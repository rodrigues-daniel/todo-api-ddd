use crate::application::dtos::{
    CreateTaskDto, PaginatedResponseDto, TaskFilterDto, TaskHistoryResponseDto, TaskResponseDto,
    UpdateTaskDto,
};
use crate::application::services::TaskService;
use crate::application::use_cases::{
    CreateTaskUseCase, DeleteTaskUseCase, GetTaskUseCase, ListTasksUseCase, UpdateTaskUseCase,
};
use crate::domain::repositories::TaskRepository;
use crate::interface::extractors::AuthenticatedUser;
use crate::interface::handlers::auth_handlers::AppError;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// State compartilhado para handlers de tarefas
#[derive(Clone)]
pub struct TaskState {
    pub task_repository: Arc<dyn TaskRepository>,
}

/// Handler: Criar nova tarefa
pub async fn create_task_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(dto): Json<CreateTaskDto>,
) -> Result<(StatusCode, Json<TaskResponseDto>), AppError> {
    // Validar DTO
    dto.validate()?;

    // Executar caso de uso
    let use_case = CreateTaskUseCase::new(state.task_repository);
    let response = use_case.execute(user_id, dto).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler: Listar tarefas com filtros
pub async fn list_tasks_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Query(filter): Query<TaskFilterDto>,
) -> Result<Json<PaginatedResponseDto<TaskResponseDto>>, AppError> {
    // Executar caso de uso
    let use_case = ListTasksUseCase::new(state.task_repository);
    let response = use_case.execute(user_id, filter).await?;

    Ok(Json(response))
}

/// Handler: Obter tarefa específica
pub async fn get_task_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TaskResponseDto>, AppError> {
    // Executar caso de uso
    let use_case = GetTaskUseCase::new(state.task_repository);
    let response = use_case.execute(task_id, user_id).await?;

    Ok(Json(response))
}

/// Handler: Atualizar tarefa
pub async fn update_task_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
    Json(dto): Json<UpdateTaskDto>,
) -> Result<Json<TaskResponseDto>, AppError> {
    // Validar DTO
    dto.validate()?;

    // Executar caso de uso
    let use_case = UpdateTaskUseCase::new(state.task_repository);
    let response = use_case.execute(task_id, user_id, dto).await?;

    Ok(Json(response))
}

/// Handler: Deletar tarefa
pub async fn delete_task_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Executar caso de uso
    let use_case = DeleteTaskUseCase::new(state.task_repository);
    use_case.execute(task_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Handler: Obter histórico de tarefa
pub async fn get_task_history_handler(
    State(state): State<TaskState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
) -> Result<Json<Vec<TaskHistoryResponseDto>>, AppError> {
    // Executar serviço
    let service = TaskService::new(state.task_repository);
    let response = service.get_task_history(task_id, user_id).await?;

    Ok(Json(response))
}
