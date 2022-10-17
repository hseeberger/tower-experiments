use futures::{future::BoxFuture, FutureExt};
use std::{
    fmt::Debug,
    task::{Context, Poll},
};
use tower::{Layer, Service};

/// Log layer, wrapping a service, logging its request and reponse.
#[derive(Debug, Clone)]
pub struct LogLayer;

/// Tower `Layer` implementation for [LogService].
impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

/// Log service, wrapping another service, logging its request and reponse.
#[derive(Debug, Clone)]
pub struct LogService<S> {
    inner: S,
}

/// Tower `Service` implementation for [LogService]: readiness and calls are delegated to the
/// wrapped service, requests and responses are logged.
impl<R, S> Service<R> for LogService<S>
where
    S: Service<R>,
    S::Response: Debug,
    S::Error: Debug,
    S::Future: Send + 'static,
    R: Debug,
{
    /// Same response type as wrapped service.
    type Response = S::Response;

    /// Same error type as wrapped service.
    type Error = S::Error;

    /// Dynamically typed boxed future to account for calling `inspect` to log the request.
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// Always delegate readiness to the inner service.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    /// First log the request, then call the inner service and finally log the response.
    fn call(&mut self, req: R) -> Self::Future {
        println!("Service called with {req:?}");

        let res = self
            .inner
            .call(req)
            .inspect(|res| println!("Service responded with {res:?}"));

        Box::pin(res)
    }
}
