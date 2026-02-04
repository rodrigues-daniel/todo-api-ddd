use crate::application::dtos::{TaskResponseDto, UpdateTaskDto};
use crate::domain::entities::TaskHistory;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::TaskRepository;
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Atualizar tarefa existente
pub struct UpdateTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl UpdateTaskUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Executa a atualização de tarefa
    pub async fn execute(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        dto: UpdateTaskDto,
    ) -> DomainResult<TaskResponseDto> {
        // Buscar tarefa existente
        let mut task = self
            .task_repository
            .find_by_id(&task_id)
            .await?
            .ok_or(DomainError::NotFound("Tarefa não encontrada".to_string()))?;

        // Verificar se o usuário é dono da tarefa
        if !task.is_owned_by(&user_id) {
            return Err(DomainError::Unauthorized);
        }

        // Atualizar campos e registrar histórico
        if let Some(title) = dto.title {
            let old_title = task.title.clone();
            task.update_title(title.clone())?;

            self.add_history_entry(task_id, user_id, "title", &old_title, &title)
                .await?;
        }

        if let Some(description) = dto.description {
            let old_desc = task.description.clone().unwrap_or_default();
            task.update_description(Some(description.clone()));

            self.add_history_entry(task_id, user_id, "description", &old_desc, &description)
                .await?;
        }

        if let Some(status) = dto.status {
            let old_status = task.status;
            task.update_status(status)?;

            self.add_history_entry(
                task_id,
                user_id,
                "status",
                &old_status.to_string(),
                &status.to_string(),
            )
            .await?;
        }

        if let Some(priority) = dto.priority {
            let old_priority = task.priority;
            task.update_priority(priority);

            self.add_history_entry(
                task_id,
                user_id,
                "priority",
                &old_priority.to_string(),
                &priority.to_string(),
            )
            .await?;
        }

        if dto.due_date.is_some() {
            let old_due = task.due_date.map(|d| d.to_string()).unwrap_or_default();
            task.update_due_date(dto.due_date);
            let new_due = task.due_date.map(|d| d.to_string()).unwrap_or_default();

            self.add_history_entry(task_id, user_id, "due_date", &old_due, &new_due)
                .await?;
        }

        // Salvar alterações
        let updated_task = self.task_repository.update(&task).await?;

        // Converter para DTO
        Ok(TaskResponseDto {
            id: updated_task.id,
            user_id: updated_task.user_id,
            title: updated_task.title,
            description: updated_task.description,
            status: updated_task.status,
            priority: updated_task.priority,
            due_date: updated_task.due_date,
            completed_at: updated_task.completed_at,
            is_overdue: updated_task.is_overdue(),
            created_at: updated_task.created_at,
            updated_at: updated_task.updated_at,
        })
    }

    /// Helper para adicionar entrada no histórico
    async fn add_history_entry(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        field_name: &str,
        old_value: &str,
        new_value: &str,
    ) -> DomainResult<()> {
        let history = TaskHistory::new(
            task_id,
            user_id,
            field_name.to_string(),
            Some(old_value.to_string()),
            Some(new_value.to_string()),
        );

        self.task_repository.add_history(&history).await
    }
}
