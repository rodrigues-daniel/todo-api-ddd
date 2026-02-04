use crate::interface::handlers::{AuthState, login_handler, register_handler};
use axum::{Router, routing::post};

/// Rotas de autenticação
pub fn routes(state: AuthState) -> Router {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
        .with_state(state)
}
