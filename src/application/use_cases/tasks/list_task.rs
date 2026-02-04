use crate::application::dtos::{PaginatedResponseDto, TaskFilterDto, TaskResponseDto};
use crate::domain::errors::DomainResult;
use crate::domain::repositories::{Pagination, TaskFilter, TaskRepository};
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Listar tarefas com filtros e paginação
pub struct ListTasksUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl ListTasksUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// Executa a listagem de tarefas
    pub async fn execute(
        &self,
        user_id: Uuid,
        filter_dto: TaskFilterDto,
    ) -> DomainResult<PaginatedResponseDto<TaskResponseDto>> {
        // Construir filtro
        let filter = TaskFilter {
            user_id,
            status: filter_dto.status,
            priority: filter_dto.priority,
            overdue_only: filter_dto.overdue_only.unwrap_or(false),
            search_query: filter_dto.search,
        };

        // Construir paginação
        let pagination = Pagination {
            page: filter_dto.page.unwrap_or(1).max(1),
            page_size: filter_dto.page_size.unwrap_or(10).min(100),
        };

        // Buscar tarefas
        let result = self.task_repository.list(filter, pagination).await?;

        // Converter para DTOs
        let items: Vec<TaskResponseDto> = result
            .items
            .into_iter()
            .map(|task| TaskResponseDto {
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
            .collect();

        Ok(PaginatedResponseDto {
            items,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
        })
    }
}
