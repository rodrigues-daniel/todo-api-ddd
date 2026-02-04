pub mod auth_routes;
pub mod task_routes;

use crate::Config;
use crate::domain::repositories::{TaskRepository, UserRepository};
use crate::infrastructure::middleware::{auth_middleware, logging_middleware};
use crate::interface::handlers::{AuthState, TaskState};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

/// Cria todas as rotas da aplicação
pub fn create_routes(
    _config: Config,
    user_repository: Arc<dyn UserRepository>,
    task_repository: Arc<dyn TaskRepository>,
) -> Router {
    // States
    let auth_state = AuthState {
        user_repository: user_repository.clone(),
    };

    let task_state = TaskState {
        task_repository: task_repository.clone(),
    };

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Rotas públicas
    let public_routes = Router::new().merge(auth_routes::routes(auth_state));

    // Rotas protegidas (requerem autenticação)
    let protected_routes = Router::new()
        .merge(task_routes::routes(task_state))
        .layer(middleware::from_fn(auth_middleware));

    // Health check
    let health_routes = Router::new().route("/health", get(health_check));

    // Combinar todas as rotas
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", protected_routes)
        .merge(health_routes)
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
}

/// Handler de health check
async fn health_check() -> &'static str {
    "OK"
}
