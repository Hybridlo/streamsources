use std::{future::{ready, Ready}, rc::Rc};

use actix_web::{dev::{
    forward_ready, Service, ServiceRequest, ServiceResponse, Transform
}, Error, web::Data, HttpResponse, http::header, body::EitherBody};
use actix_web::FromRequest;
use futures_util::future::LocalBoxFuture;
use twitch_sources_rework::TWITCH_AUTH_URL;

use crate::{DbPool, db::AuthState, routes::LoginUrlResponse, errors::{MyErrors, IntoResultMyErr}, util::session_state::TypedSession};

pub struct AutoTwitchLoginFactory;
impl<S, B> Transform<S, ServiceRequest> for AutoTwitchLoginFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AutoTwitchLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AutoTwitchLoginMiddleware { service: Rc::new(service) }))
    }
}

pub struct AutoTwitchLoginMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AutoTwitchLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let mut logged_in = false;

            let (r, mut pl) = req.into_parts();
            // check if we're logged in, if we are - process request as normal
            if let Ok(session) = TypedSession::from_request(&r, &mut pl).await {
                if session.get_user_id()?.is_some() {
                    logged_in = true;
                }
            }
            
            let req = ServiceRequest::from_parts(r, pl);
            
            if logged_in {
                let fut = svc.call(req);
                let res = fut.await?;
                return Ok(res.map_into_left_body())
            }
            // otherwise - redirect user to twitch api login
            let db_pool = req
                .app_data::<Data<DbPool>>()
                .ok_or(MyErrors::InternalServerError("DB access error".to_string()))?;
            let mut db_conn = db_pool.get().await.into_my()?;

            let full_uri = req.uri().to_string();
            let new_state = AuthState::get_new_state(&full_uri, &mut db_conn).await
                .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;
            let host = req.connection_info().scheme().to_string() + "://" + req.connection_info().host();

            let twitch_api_query = serde_urlencoded::ser::to_string(LoginUrlResponse::new(&host, &new_state))?;

            Ok(
                req
                    .into_response(
                    HttpResponse::TemporaryRedirect().append_header((
                        header::LOCATION,
                        header::HeaderValue::from_str(&(TWITCH_AUTH_URL.to_string() + "?" + &twitch_api_query))
                            .map_err(|err| MyErrors::InternalServerError(err.to_string()))?
                    ))
                    .body("")
                    .map_into_right_body()
                )
            )
        })
    }
}
