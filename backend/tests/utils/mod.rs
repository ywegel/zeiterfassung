use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::Response;
use tower::Service;
use tower::ServiceExt;

/// Extension trait for Axum Router to simplify multiple service calls in tests.
///
/// This trait provides a `call_request` method that wraps the `ready` and`call`
/// methods from Tower's `ServiceExt`. It exists to reduce boilerplate in
/// integration tests when making multiple HTTP requests to the same Router
/// instance, as `oneshot` consumes the Router. By using `call_request`, tests
/// can repeatedly call the Router without cloning it.
///
/// To use this trait in integration tests, import the `utils` module in your
/// test file, as integration tests are compiled one file at a time:
/// ``` rust
/// mod utils;
/// ```
pub trait RouterExt {
    async fn call_request(&mut self, request: Request<Body>) -> Response<Body>;
}

impl<S> RouterExt for Router<S>
where
    S: Clone + Send + Sync + 'static,
    Router<S>: Service<Request<Body>, Response = Response<Body>, Error = std::convert::Infallible>,
{
    async fn call_request(&mut self, request: Request<Body>) -> Response<Body> {
        self.ready()
            .await
            .expect("Service not ready")
            .call(request)
            .await
            .expect("Service call failed")
    }
}
