use tokio_service::Service;
use tokio_timer::{self as timer, Timer};
use std::time::Duration;

pub struct Timeout<S> {
    upstream: S,
    timer: Timer,
    duration: Duration,
}

impl<S> Timeout<S> {
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
    type Req = S::Req;
    type Resp = S::Resp;
    type Error = S::Error;
    type Fut = timer::Timeout<S::Fut>;

    fn call(&self, request: Self::Req) -> Self::Fut {
        let resp = self.upstream.call(request);
        self.timer.timeout(resp, self.duration)
    }
}
