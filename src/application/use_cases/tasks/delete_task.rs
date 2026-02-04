use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::TaskRepository;
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Deletar tarefa
pub struct DeleteTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl DeleteTaskUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Executa a deleção de tarefa
    pub async fn execute(&self, task_id: Uuid, user_id: Uuid) -> DomainResult<()> {
        // Buscar tarefa para verificar propriedade
        let task = self
            .task_repository
            .find_by_id(&task_id)
            .await?
            .ok_or(DomainError::NotFound("Tarefa não encontrada".to_string()))?;

        // Verificar se o usuário é dono
        if !task.is_owned_by(&user_id) {
            return Err(DomainError::Unauthorized);
        }

        // Deletar tarefa (histórico é deletado em cascata)
        self.task_repository.delete(&task_id).await
    }
}
