use crate::domain::value_objects::{TaskPriority, TaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// DTO para criação de tarefa
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: String,

    #[validate(length(max = 5000, message = "Descrição não pode exceder 5000 caracteres"))]
    pub description: Option<String>,

    pub priority: Option<TaskPriority>,

    pub due_date: Option<DateTime<Utc>>,
}

/// DTO para atualização de tarefa
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Título deve ter entre 1 e 255 caracteres"
    ))]
    pub title: Option<String>,

    #[validate(length(max = 5000, message = "Descrição não pode exceder 5000 caracteres"))]
    pub description: Option<String>,

    pub status: Option<TaskStatus>,

    pub priority: Option<TaskPriority>,

    pub due_date: Option<DateTime<Utc>>,
}

/// DTO de resposta de tarefa
#[derive(Debug, Serialize)]
pub struct TaskResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_overdue: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO para filtros de listagem
#[derive(Debug, Deserialize)]
pub struct TaskFilterDto {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub overdue_only: Option<bool>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// DTO de resposta paginada
#[derive(Debug, Serialize)]
pub struct PaginatedResponseDto<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// DTO de histórico de tarefa
#[derive(Debug, Serialize)]
pub struct TaskHistoryResponseDto {
    pub id: Uuid,
    pub task_id: Uuid,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: DateTime<Utc>,
}
