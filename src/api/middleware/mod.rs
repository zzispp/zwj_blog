pub mod jwt;

pub use jwt::{generate_jwt_token, JwtMiddleware};

// ServiceContextMaintenanceCheck middleware (existing)
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct ServiceContextMaintenanceCheck;

impl<S, B> Transform<S, ServiceRequest> for ServiceContextMaintenanceCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ServiceContextMaintenanceCheckService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServiceContextMaintenanceCheckService { service }))
    }
}

pub struct ServiceContextMaintenanceCheckService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ServiceContextMaintenanceCheckService<S>
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
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
