use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use std::num::NonZeroU32;
use std::sync::Arc;

/// Middleware de rate limiting
pub struct RateLimitMiddleware {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl RateLimitMiddleware {
    /// Cria novo middleware com limite de requisições por minuto
    pub fn new(requests_per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
        let limiter = Arc::new(RateLimiter::direct(quota));

        Self { limiter }
    }

    /// Handler do middleware
    pub async fn handle(
        limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
        req: Request,
        next: Next,
    ) -> Result<Response, RateLimitError> {
        // Verificar se pode processar a requisição
        if limiter.check().is_err() {
            return Err(RateLimitError);
        }

        Ok(next.run(req).await)
    }
}

/// Erro de rate limit excedido
#[derive(Debug)]
pub struct RateLimitError;

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "Muitas requisições. Tente novamente mais tarde.",
        )
            .into_response()
    }
}
