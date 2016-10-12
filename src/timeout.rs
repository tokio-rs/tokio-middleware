
/// Proposal: Standardize this error.
///
/// Custom errors should provide a conversion to SinkError.
///
/// For example:
///     impl From<HttpError> for SinkError<HttpRequest, HttpError> {
///     }
///
pub enum SinkError<T, E> {
    NotReady(T),
    Other(E),
}

/// Example middleware that uses SinkError
impl<S> Service for Retry<S>
    where SinkError<S::Request, S::Error>: From<S::Error>,
{
    type Request = S::Request;
    type Error = S::Error;

    fn call(&self, request: S::Request) -> impl Future<S::Response, S::Error> {
        // Bump Arc count
        let retry = self.clone();

        self.inner.upstream.call(request).then(move |res| {
            match res {
                Ok(resp) => Ok(resp),
                Err(e) => {
                    match SinkError::from(e) {
                        SinkError::NotReady(request) => {
                            // Some global timer
                            timer::sleep(Duration::from_secs(10))
                                .and_then(|_| retry.call(request))
                        }
                        SinkError::Other(e) => Err(e),
                    }
                }
            }
        })
    }

    fn poll_ready(&self) -> Async<()> {
        self.inner.upstream.poll_ready()
    }
}

/// Any middleware generic over the error doesn't have to care. For example
/// timout:
///
/// In this case, the timeout will proxy the upstream error or it will short
/// circuit in the event of a timeout
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

    fn poll_ready(&self) -> Async<()> {
        self.upstream.poll_ready()
    }
}
