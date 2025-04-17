use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, Ready};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use uuid::Uuid;
use actix_web::http::header::HeaderName;
use log::info;

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-request-id");

pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware { service }))
    }
}

pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Generate a new request ID
        let request_id = Uuid::new_v4().to_string();
        
        // Store the request ID in the request extensions
        req.extensions_mut().insert(request_id.clone());

        // Log the request with its ID
        info!("Request started: {} {}", req.method(), req.path());

        // Call the service
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            
            // Add the request ID to the response headers
            res.headers_mut().insert(
                REQUEST_ID_HEADER,
                request_id.parse().unwrap(),
            );

            Ok(res)
        })
    }
}

pub fn get_request_id(req: &actix_web::HttpRequest) -> Option<String> {
    req.extensions().get::<String>().cloned()
} 