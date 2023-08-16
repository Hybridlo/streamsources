use std::{future::{ready, Ready}, rc::Rc};

use actix_session::SessionExt;
use actix_web::{dev::{
    forward_ready, Service, ServiceRequest, ServiceResponse, Transform
}, Error};
use futures_util::future::LocalBoxFuture;
use serde::Deserialize;

use crate::{errors::IntoResultMyErr, util::Context};
use crate::errors::MyErrors;
use crate::domain::login_token::LoginToken;
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
            let ctx = req
                .app_data::<Context>()
                .ok_or(MyErrors::InternalServerError("DB access error".to_string()))?;
            let session: TypedSession = req.get_session().into();
            let query = req.query_string();
            
            // if there is no login token, or it's invalid - we just ignore it
            if let Ok(login_data) = serde_urlencoded::de::from_str::<LoginTokenQuery>(query) {
                if let Ok(user_id) = LoginToken::validate_user_token(&ctx.repository, &login_data.login_token).await {
                    if session.get_user_id()?.is_none() {
                        session.renew();
                        session
                            .insert_user_id(user_id).into_my()?;
                    }
                }
            }

            let fut = svc.call(req);
            let res = fut.await?;

            Ok(res)
        })
    }
}

