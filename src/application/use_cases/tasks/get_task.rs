use crate::application::dtos::TaskResponseDto;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::TaskRepository;
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Obter tarefa por ID
pub struct GetTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl GetTaskUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Executa a busca de tarefa
    pub async fn execute(&self, task_id: Uuid, user_id: Uuid) -> DomainResult<TaskResponseDto> {
        // Buscar tarefa
        let task = self
            .task_repository
            .find_by_id(&task_id)
            .await?
            .ok_or(DomainError::NotFound("Tarefa não encontrada".to_string()))?;

        // Verificar propriedade
        if !task.is_owned_by(&user_id) {
            return Err(DomainError::Unauthorized);
        }

        // Converter para DTO
        Ok(TaskResponseDto {
            id: task.id,
            user_id: task.user_id,
            title: task.title,
            description: task.description,
            status: task.status,
            priority: task.priority,
            due_date: task.due_date,
            completed_at: task.completed_at,
            is_overdue: task.is_overdue(),
            created_at: task.created_at,
            updated_at: task.updated_at,
        })
    }
}
