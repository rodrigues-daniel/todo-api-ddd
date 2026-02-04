use crate::domain::entities::{Task, TaskHistory};
use crate::domain::errors::DomainResult;
use crate::domain::value_objects::{TaskPriority, TaskStatus};
use async_trait::async_trait;
use uuid::Uuid;

/// Parâmetros de filtro para listagem de tarefas
#[derive(Debug, Clone)]
pub struct TaskFilter {
    pub user_id: Uuid,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub overdue_only: bool,
    pub search_query: Option<String>,
}

/// Parâmetros de paginação
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

impl Pagination {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }
}

/// Resultado paginado
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Trait repository para Task (Port do padrão Hexagonal)
#[async_trait]
pub trait TaskRepository: Send + Sync {
    /// Cria uma nova tarefa
    async fn create(&self, task: &Task) -> DomainResult<Task>;

    /// Busca uma tarefa por ID
    async fn find_by_id(&self, id: &Uuid) -> DomainResult<Option<Task>>;

    /// Lista tarefas com filtros e paginação
    async fn list(
        &self,
        filter: TaskFilter,
        pagination: Pagination,
    ) -> DomainResult<PaginatedResult<Task>>;

    /// Atualiza uma tarefa
    async fn update(&self, task: &Task) -> DomainResult<Task>;

    /// Deleta uma tarefa
    async fn delete(&self, id: &Uuid) -> DomainResult<()>;

    /// Adiciona um registro ao histórico
    async fn add_history(&self, history: &TaskHistory) -> DomainResult<()>;

    /// Lista o histórico de uma tarefa
    async fn get_history(&self, task_id: &Uuid) -> DomainResult<Vec<TaskHistory>>;

    /// Conta tarefas por status para um usuário
    async fn count_by_status(&self, user_id: &Uuid, status: TaskStatus) -> DomainResult<i64>;
}
