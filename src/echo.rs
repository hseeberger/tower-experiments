use std::{
    convert::Infallible,
    fmt::{self, Display},
    future::{ready, Ready},
    task::{Context, Poll},
};
use tower::Service;

/// A request to the [EchoService], just wrapping a `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoRequest(String);

/// Delegate to the wrapped `String`.
impl Display for EchoRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Anything that can be turned into a `String` can be turned into a [EchoRequest].
impl<T> From<T> for EchoRequest
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        EchoRequest(text.into())
    }
}

/// A response from the [EchoService], just wrapping a `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoResponse(String);

/// Delegate to the wrapped `String`.
impl Display for EchoResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Echo service, responding to an [EchoRequest] with an [EchoResponse] with the same content.
#[derive(Debug, Clone)]
pub struct EchoService;

/// Tower `Service` implementation for [EchoService]: always ready, calls never fail and returned
/// `Future`s are ready immediately.
impl Service<EchoRequest> for EchoService {
    type Response = EchoResponse;

    /// This service never fails.
    type Error = Infallible;

    /// Responses of this service are ready immediately.
    type Future = Ready<Result<Self::Response, Self::Error>>;

    /// Always return `Poll::Ready`: this service is always ready.
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    /// Always return an [EchoResponse] with the same content like the [EchoRequest].
    fn call(&mut self, req: EchoRequest) -> Self::Future {
        ready(Ok(EchoResponse(req.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::noop_waker_ref;
    use std::task;

    #[tokio::test]
    async fn test_poll_ready() {
        let result = EchoService.poll_ready(&mut task::Context::from_waker(noop_waker_ref()));
        assert!(result.is_ready());
        if let Poll::Ready(result) = result {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_call() {
        let response = EchoService.call("Tower".into()).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response, EchoResponse("Tower".to_string()));
    }
}
