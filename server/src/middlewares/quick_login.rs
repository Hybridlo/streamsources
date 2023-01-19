use std::{future::{ready, Ready}, rc::Rc};

use actix_web::{dev::{
    forward_ready, Service, ServiceRequest, ServiceResponse, Transform
}, Error, web::Data};
use actix_web::FromRequest;
use futures_util::future::LocalBoxFuture;
use serde::Deserialize;

use crate::DbPool;
use crate::errors::MyErrors;
use crate::db::LoginToken;
use crate::util::session_state::TypedSession;

pub struct QuickLoginFactory;
impl<S, B> Transform<S, ServiceRequest> for QuickLoginFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = QuickLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(QuickLoginMiddleware { service: Rc::new(service) }))
    }
}

pub struct QuickLoginMiddleware<S> {
    service: Rc<S>,
}

#[derive(Deserialize)]
struct LoginTokenQuery {
    login_token: String
}

impl<S, B> Service<ServiceRequest> for QuickLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let db_pool = req
                .app_data::<Data<DbPool>>()
                .ok_or(MyErrors::InternalServerError("DB access error".to_string()))?;
            let mut db_conn = db_pool.get()
                .await
                .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;
            
            let (r, mut pl) = req.into_parts();
            let query = r.query_string();
            
            // if there is no login token, or it's invalid - we just ignore it
            if let Ok(login_data) = serde_urlencoded::de::from_str::<LoginTokenQuery>(query) {
                println!("got login");
                if let Ok(user_id) = LoginToken::validate_token(&login_data.login_token, &mut db_conn).await {
                    println!("got uid");
                    if let Ok(session) = TypedSession::from_request(&r, &mut pl).await {
                        println!("got request");
                        session.renew();
                        session
                            .insert_user_id(user_id)
                            .map_err(|err| MyErrors::InternalServerError(err.to_string()))?;
                        println!("session renewed");
                    }
                }
            }

            let req = ServiceRequest::from_parts(r, pl);

            let fut = svc.call(req);
            let res = fut.await?;

            Ok(res)
        })
    }
}

