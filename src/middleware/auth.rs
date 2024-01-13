use crate::state;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage, HttpResponse,
};
use jwt_simple::prelude::*;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    #[tracing::instrument(name = "auth_middleware", skip_all)]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let token = match headers.get("Authorization") {
            Some(data) => {
                let Ok(value) = data.to_str() else {
                    tracing::event!(tracing::Level::WARN, "Auth token data is invalid(non-str)");

                    return Box::pin(async {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().finish().map_into_right_body(),
                        ))
                    });
                };

                let Some(token) = value.split(' ').last() else {
                    tracing::event!(
                        tracing::Level::WARN,
                        "Auth token data uses wrong format / layout"
                    );

                    return Box::pin(async {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().finish().map_into_right_body(),
                        ))
                    });
                };

                token.to_string()
            }
            None => {
                tracing::event!(tracing::Level::TRACE, "Auth token is not found");

                return Box::pin(async {
                    Ok(req
                        .into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
                });
            }
        };

        let app_state = req
            .app_data::<web::Data<state::App>>()
            .expect("Unexpected app data not found");

        let Ok(jwt_claims) = app_state
            .keypair
            .public_key()
            .verify_token::<state::JwtClaim>(&token, None)
        else {
            tracing::event!(tracing::Level::WARN, "Can't verify auth token(jwt claims)");

            return Box::pin(async {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
            });
        };

        req.extensions_mut().insert(jwt_claims.custom);

        let service_fut = self.service.call(req);

        Box::pin(async move {
            let res = service_fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
