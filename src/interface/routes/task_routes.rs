use crate::interface::handlers::{
    TaskState, create_task_handler, delete_task_handler, get_task_handler,
    get_task_history_handler, list_tasks_handler, update_task_handler,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

/// Rotas de tarefas (todas protegidas por autenticação)
pub fn routes(state: TaskState) -> Router {
    Router::new()
        .route("/tasks", post(create_task_handler))
        .route("/tasks", get(list_tasks_handler))
        .route("/tasks/:id", get(get_task_handler))
        .route("/tasks/:id", put(update_task_handler))
        .route("/tasks/:id", delete(delete_task_handler))
        .route("/tasks/:id/history", get(get_task_history_handler))
        .with_state(state)
}
