use tokio_service::Service;
use tokio_timer::{self as timer, Timer};
use std::time::Duration;

/// Abort requests that are taking too long
#[derive(Clone)]
pub struct Timeout<S> {
    upstream: S,
    timer: Timer,
    duration: Duration,
}

impl<S> Timeout<S> {
    /// Crate a new `Timeout` with the given `upstream` service.
    ///
    /// Requests will be limited to `duration` and aborted once the limit has
    /// been reached.
    pub fn new(upstream: S, timer: Timer, duration: Duration) -> Timeout<S> {
        Timeout {
            upstream: upstream,
            duration: duration,
            timer: timer,
        }
    }
}

impl<S, E> Service for Timeout<S>
    where S: Service<Error = E>,
          E: From<timer::TimeoutError>,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = timer::Timeout<S::Future>;

    fn call(&self, request: Self::Request) -> Self::Future {
        let resp = self.upstream.call(request);
        self.timer.timeout(resp, self.duration)
    }
}
