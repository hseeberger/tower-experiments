use futures::future::{ready, Ready};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::Service;

/// A request to the [AlternatingReadyService].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternatingReadyRequest;

/// A response from the [AlternatingReadyService], just wrapping a `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternatingReadyResponse;

/// AlternatingReady service, responding to a [AlternatingReadyRequest] with an
/// [AlternatingReadyResponse].
#[derive(Debug, Clone)]
pub struct AlternatingReadyService {
    ready: bool,
}

impl AlternatingReadyService {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for AlternatingReadyService {
    fn default() -> Self {
        Self { ready: true }
    }
}

/// Tower `Service` implementation for [AlternatingReadyService]: readiness alternates between
/// pending and ready, calls never fail and responses are ready immediately.
impl Service<AlternatingReadyRequest> for AlternatingReadyService {
    type Response = AlternatingReadyResponse;

    /// This service never fails.
    type Error = Infallible;

    /// Responses of this service are ready immediately.
    type Future = Ready<Result<Self::Response, Self::Error>>;

    /// Alternately return `Poll::Pending` and `Poll::Ready`.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.ready {
            // Wake up the task to make sure the executor calls `poll_ready` again.
            cx.waker().wake_by_ref();

            self.ready = false;
            Poll::Pending
        } else {
            self.ready = true;
            Poll::Ready(Ok(()))
        }
    }

    /// Always return [AlternatingReadyResponse].
    ///
    /// # Panics
    /// Panics if called without prior calling `poll_ready`.
    fn call(&mut self, _req: AlternatingReadyRequest) -> Self::Future {
        if !self.ready {
            panic!("service not ready; poll_ready must be called first");
        }
        self.ready = false;
        ready(Ok(AlternatingReadyResponse))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::noop_waker_ref;
    use std::task;

    #[tokio::test]
    async fn test_poll_ready() {
        let mut service = AlternatingReadyService::new();

        let result = service.poll_ready(&mut task::Context::from_waker(noop_waker_ref()));
        assert!(result.is_pending());

        let result = service.poll_ready(&mut task::Context::from_waker(noop_waker_ref()));
        assert!(result.is_ready());
    }
}
