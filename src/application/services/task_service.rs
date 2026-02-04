use crate::application::dtos::TaskHistoryResponseDto;
use crate::domain::errors::DomainResult;
use crate::domain::repositories::TaskRepository;
use std::sync::Arc;
use uuid::Uuid;

/// Serviço de aplicação para operações auxiliares de tarefas
pub struct TaskService {
    task_repository: Arc<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Obtém histórico de alterações de uma tarefa
    pub async fn get_task_history(
        &self,
        task_id: Uuid,
        user_id: Uuid,
    ) -> DomainResult<Vec<TaskHistoryResponseDto>> {
        // Verificar se tarefa existe e pertence ao usuário
        let task = self.task_repository.find_by_id(&task_id).await?.ok_or(
            crate::domain::errors::DomainError::NotFound("Tarefa não encontrada".to_string()),
        )?;

        if !task.is_owned_by(&user_id) {
            return Err(crate::domain::errors::DomainError::Unauthorized);
        }

        // Buscar histórico
        let history = self.task_repository.get_history(&task_id).await?;

        // Converter para DTOs
        Ok(history
            .into_iter()
            .map(|h| TaskHistoryResponseDto {
                id: h.id,
                task_id: h.task_id,
                field_name: h.field_name,
                old_value: h.old_value,
                new_value: h.new_value,
                changed_at: h.changed_at,
            })
            .collect())
    }
}
