extern crate futures;
extern crate tokio_timer as timer;
extern crate tokio_middleware as middleware;
extern crate tokio_service as service;

use futures::{Future, Poll, Async};
use timer::Timer;
use middleware::Timeout;
use service::Service;
use std::io;
use std::time::Duration;

#[test]
fn test_request_succeeds() {
    let service = service::simple_service(|req| {
        assert_eq!("request", req);
        Ok::<&'static str, io::Error>("response")
    });

    let service = Timeout::new(
        service, Timer::default(),
        Duration::from_millis(200));

    let response = service.call("request").wait();

    assert_eq!("response", response.unwrap())
}

#[test]
fn test_request_times_out() {
    let service = service::simple_service(|req| {
        assert_eq!("request", req);
        Never
    });

    let service = Timeout::new(
        service, Timer::default(),
        Duration::from_millis(200));

    let response = service.call("request").wait();

    assert_eq!(io::ErrorKind::TimedOut, response.unwrap_err().kind());
}

struct Never;

impl Future for Never {
    type Item = &'static str;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}
