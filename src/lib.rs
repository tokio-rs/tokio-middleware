//! A collection of middleware `Service` middleware.
//!
//! Middleware can be used to define *application-agnostic* behavior in a
//! reusable way. A common example of this is implementing timeouts: if a
//! request fails to complete within a certain time, the timeout mechanism
//! fails it with a timeout error.
//!
//! A middleware component sits in the middle of a service stack. In the case
//! of an HTTP server, the server implementation takes a `Service<Req =
//! HttpRequest, Resp = HttpResponse>` and uses this for handling inbound
//! requests. The application provides an implementation of `Service` that
//! satisfies its business requirements, for example, responding with "hello
//! world". The `Timeout` middleware is created with the `HelloWorld` upstream
//! service and the `Timeout` middleware itself implements `Service`. This
//! enables initializing the HTTP server with `Timeout<HelloWorld>` such that
//! the timeout middleware decorates the behavior of the upstream service.
//!
//! There is a huge range of potential behaviors that can be implemented as
//! middleware. This library contains a collection of useful protocol-agnostic
//! middleware, but middleware that is protocol specific can be found in crates
//! that implement those protocols.

extern crate tokio_service;
extern crate tokio_timer;

mod log;
mod timeout;

pub use log::Log;
pub use timeout::Timeout;
