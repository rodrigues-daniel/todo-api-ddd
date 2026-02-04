use crate::application::dtos::{CreateTaskDto, TaskResponseDto};
use crate::domain::entities::{Task, TaskHistory};
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::TaskRepository;
use crate::domain::value_objects::TaskPriority;
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Criar nova tarefa
pub struct CreateTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl CreateTaskUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Executa a criação de tarefa
    pub async fn execute(
        &self,
        user_id: Uuid,
        dto: CreateTaskDto,
    ) -> DomainResult<TaskResponseDto> {
        // Criar entidade de tarefa
        let task = Task::new(
            Uuid::new_v4(),
            user_id,
            dto.title.clone(),
            dto.description.clone(),
            dto.priority.unwrap_or(TaskPriority::Medium),
            dto.due_date,
        )?;

        // Salvar no repositório
        let saved_task = self.task_repository.create(&task).await?;

        // Registrar no histórico
        let history = TaskHistory::new(
            saved_task.id,
            user_id,
            "created".to_string(),
            None,
            Some(format!("Tarefa criada: {}", saved_task.title)),
        );

        self.task_repository.add_history(&history).await?;

        // Converter para DTO de resposta
        Ok(TaskResponseDto {
            id: saved_task.id,
            user_id: saved_task.user_id,
            title: saved_task.title,
            description: saved_task.description,
            status: saved_task.status,
            priority: saved_task.priority,
            due_date: saved_task.due_date,
            completed_at: saved_task.completed_at,
            is_overdue: saved_task.is_overdue(),
            created_at: saved_task.created_at,
            updated_at: saved_task.updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::{PaginatedResult, Pagination, TaskFilter};
    use crate::domain::value_objects::TaskStatus;
    use async_trait::async_trait;

    // Mock repository para testes
    struct MockTaskRepository;

    #[async_trait]
    impl TaskRepository for MockTaskRepository {
        async fn create(&self, task: &Task) -> DomainResult<Task> {
            Ok(task.clone())
        }

        async fn find_by_id(&self, _id: &Uuid) -> DomainResult<Option<Task>> {
            unimplemented!()
        }

        async fn list(
            &self,
            _filter: TaskFilter,
            _pagination: Pagination,
        ) -> DomainResult<PaginatedResult<Task>> {
            unimplemented!()
        }

        async fn update(&self, task: &Task) -> DomainResult<Task> {
            Ok(task.clone())
        }

        async fn delete(&self, _id: &Uuid) -> DomainResult<()> {
            Ok(())
        }

        async fn add_history(&self, _history: &TaskHistory) -> DomainResult<()> {
            Ok(())
        }

        async fn get_history(&self, _task_id: &Uuid) -> DomainResult<Vec<TaskHistory>> {
            Ok(vec![])
        }

        async fn count_by_status(&self, _user_id: &Uuid, _status: TaskStatus) -> DomainResult<i64> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_create_task_success() {
        let repo = Arc::new(MockTaskRepository);
        let use_case = CreateTaskUseCase::new(repo);

        let dto = CreateTaskDto {
            title: "Test Task".to_string(),
            description: Some("Description".to_string()),
            priority: Some(TaskPriority::High),
            due_date: None,
        };

        let result = use_case.execute(Uuid::new_v4(), dto).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.priority, TaskPriority::High);
    }

    #[tokio::test]
    async fn test_create_task_with_default_priority() {
        let repo = Arc::new(MockTaskRepository);
        let use_case = CreateTaskUseCase::new(repo);

        let dto = CreateTaskDto {
            title: "Test".to_string(),
            description: None,
            priority: None, // Sem prioridade
            due_date: None,
        };

        let result = use_case.execute(Uuid::new_v4(), dto).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().priority, TaskPriority::Medium);
    }
}
