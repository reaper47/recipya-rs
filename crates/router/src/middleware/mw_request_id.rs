use axum::{
    Router,
    http::{HeaderName, Request},
};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::info_span;

const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Extension trait that adds a request id tracing to a router.
pub trait RequestIdExt {
    /// Wraps the router with a request id generation, tracing and propagation middleware.
    #[must_use]
    fn with_request_id(self) -> Self;
}

impl<S: Clone + Send + Sync + 'static> RequestIdExt for Router<S> {
    fn with_request_id(self) -> Self {
        self.layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    X_REQUEST_ID,
                    MakeRequestUuid,
                ))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|req: &Request<_>| {
                            let id = req
                                .headers()
                                .get(X_REQUEST_ID)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("-");

                            info_span!("http_request", method = %req.method(), uri = %req.uri(), request_id = %id)
                        }),
                )
                .layer(PropagateRequestIdLayer::new(X_REQUEST_ID)),
        )
    }
}
