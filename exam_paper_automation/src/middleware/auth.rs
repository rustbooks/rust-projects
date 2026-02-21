use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::utils::auth::{decode_jwt, Claims};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract JWT token from Authorization header
        let auth_header = req.headers().get("Authorization");
        
        let token = if let Some(auth) = auth_header {
            if let Ok(auth_str) = auth.to_str() {
                if auth_str.starts_with("Bearer ") {
                    Some(&auth_str[7..])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Missing or invalid Authorization header"
                            }))
                            .into_body()
                    ))
                });
            }
        };

        // TODO: Get JWT secret from app data
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        
        let claims = match decode_jwt(token, &jwt_secret) {
            Ok(claims) => claims,
            Err(e) => {
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": format!("Invalid token: {}", e)
                            }))
                            .into_body()
                    ))
                });
            }
        };

        // Store claims in request extensions
        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

pub struct AdminMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AdminMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminMiddlewareService { service }))
    }
}

pub struct AdminMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // First, do regular auth
        let auth_header = req.headers().get("Authorization");
        
        let token = if let Some(auth) = auth_header {
            if let Ok(auth_str) = auth.to_str() {
                if auth_str.starts_with("Bearer ") {
                    Some(&auth_str[7..])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Missing or invalid Authorization header"
                            }))
                            .into_body()
                    ))
                });
            }
        };

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        
        let claims = match decode_jwt(token, &jwt_secret) {
            Ok(claims) => claims,
            Err(e) => {
                return Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": format!("Invalid token: {}", e)
                            }))
                            .into_body()
                    ))
                });
            }
        };

        // Check if user is admin
        if claims.role != "admin" {
            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::Forbidden()
                        .json(serde_json::json!({
                            "error": "Admin access required"
                        }))
                        .into_body()
                ))
            });
        }

        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
