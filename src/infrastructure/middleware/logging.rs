use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

/// Middleware de logging de requisições
pub async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    info!("Requisição: {} {}", method, uri);

    let response = next.run(req).await;

    info!(
        "Resposta: {} {} - Status: {}",
        method,
        uri,
        response.status()
    );

    response
}

pub type LoggingMiddleware = fn(Request, Next) -> impl std::future::Future<Output = Response>;
