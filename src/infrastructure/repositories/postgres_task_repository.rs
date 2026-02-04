use crate::domain::entities::{Task, TaskHistory};
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::{PaginatedResult, Pagination, TaskFilter, TaskRepository};
use crate::domain::value_objects::{TaskPriority, TaskStatus};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// Implementação PostgreSQL do TaskRepository
pub struct PostgresTaskRepository {
    pool: PgPool,
}

impl PostgresTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PostgresTaskRepository {
    async fn create(&self, task: &Task) -> DomainResult<Task> {
        sqlx::query_as!(
            TaskRow,
            r#"
            INSERT INTO tasks (id, user_id, title, description, status, priority, due_date, completed_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING 
                id, user_id, title, description,
                status as "status: TaskStatus",
                priority as "priority: TaskPriority",
                due_date, completed_at, created_at, updated_at
            "#,
            task.id,
            task.user_id,
            task.title,
            task.description,
            task.status as TaskStatus,
            task.priority as TaskPriority,
            task.due_date,
            task.completed_at,
            task.created_at,
            task.updated_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .into()
    }

    async fn find_by_id(&self, id: &Uuid) -> DomainResult<Option<Task>> {
        let row = sqlx::query_as!(
            TaskRow,
            r#"
            SELECT 
                id, user_id, title, description,
                status as "status: TaskStatus",
                priority as "priority: TaskPriority",
                due_date, completed_at, created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn list(
        &self,
        filter: TaskFilter,
        pagination: Pagination,
    ) -> DomainResult<PaginatedResult<Task>> {
        // Construir query dinamicamente
        let mut query = String::from(
            r#"
            SELECT 
                id, user_id, title, description,
                status as "status: TaskStatus",
                priority as "priority: TaskPriority",
                due_date, completed_at, created_at, updated_at
            FROM tasks
            WHERE user_id = $1
            "#,
        );

        let mut param_count = 1;
        let mut conditions = Vec::new();

        // Adicionar filtros
        if filter.status.is_some() {
            param_count += 1;
            conditions.push(format!("status = ${}", param_count));
        }

        if filter.priority.is_some() {
            param_count += 1;
            conditions.push(format!("priority = ${}", param_count));
        }

        if filter.overdue_only {
            conditions
                .push("due_date < NOW() AND status NOT IN ('completed', 'cancelled')".to_string());
        }

        if filter.search_query.is_some() {
            param_count += 1;
            conditions.push(format!(
                "(title ILIKE ${} OR description ILIKE ${})",
                param_count, param_count
            ));
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        // Ordenação
        query.push_str(" ORDER BY created_at DESC");

        // Paginação
        let offset = pagination.offset();
        query.push_str(&format!(
            " LIMIT {} OFFSET {}",
            pagination.page_size, offset
        ));

        // Executar query (versão simplificada - em produção use querybuilder)
        let mut query_builder = sqlx::query_as::<_, TaskRow>(&query);
        query_builder = query_builder.bind(filter.user_id);

        if let Some(status) = filter.status {
            query_builder = query_builder.bind(status);
        }

        if let Some(priority) = filter.priority {
            query_builder = query_builder.bind(priority);
        }

        if let Some(ref search) = filter.search_query {
            let search_pattern = format!("%{}%", search);
            query_builder = query_builder.bind(search_pattern);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        // Contar total
        let total = self.count_total(&filter).await?;

        let total_pages = ((total as f64) / (pagination.page_size as f64)).ceil() as u32;

        Ok(PaginatedResult {
            items: rows.into_iter().map(Into::into).collect(),
            total,
            page: pagination.page,
            page_size: pagination.page_size,
            total_pages,
        })
    }

    async fn update(&self, task: &Task) -> DomainResult<Task> {
        sqlx::query_as!(
            TaskRow,
            r#"
            UPDATE tasks
            SET title = $2, description = $3, status = $4, priority = $5,
                due_date = $6, completed_at = $7, updated_at = $8
            WHERE id = $1
            RETURNING 
                id, user_id, title, description,
                status as "status: TaskStatus",
                priority as "priority: TaskPriority",
                due_date, completed_at, created_at, updated_at
            "#,
            task.id,
            task.title,
            task.description,
            task.status as TaskStatus,
            task.priority as TaskPriority,
            task.due_date,
            task.completed_at,
            task.updated_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .into()
    }

    async fn delete(&self, id: &Uuid) -> DomainResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM tasks WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn add_history(&self, history: &TaskHistory) -> DomainResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO task_history (id, task_id, user_id, field_name, old_value, new_value, changed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            history.id,
            history.task_id,
            history.user_id,
            history.field_name,
            history.old_value,
            history.new_value,
            history.changed_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn get_history(&self, task_id: &Uuid) -> DomainResult<Vec<TaskHistory>> {
        let rows = sqlx::query_as!(
            TaskHistoryRow,
            r#"
            SELECT id, task_id, user_id, field_name, old_value, new_value, changed_at
            FROM task_history
            WHERE task_id = $1
            ORDER BY changed_at DESC
            "#,
            task_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn count_by_status(&self, user_id: &Uuid, status: TaskStatus) -> DomainResult<i64> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM tasks
            WHERE user_id = $1 AND status = $2
            "#,
            user_id,
            status as TaskStatus,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(result.count)
    }
}

impl PostgresTaskRepository {
    /// Helper para contar total de tarefas
    async fn count_total(&self, filter: &TaskFilter) -> DomainResult<i64> {
        let mut query = String::from("SELECT COUNT(*) FROM tasks WHERE user_id = $1");

        let mut conditions = Vec::new();

        if filter.status.is_some() {
            conditions.push("status = $2");
        }

        if filter.priority.is_some() {
            conditions.push("priority = $3");
        }

        if filter.overdue_only {
            conditions.push("due_date < NOW() AND status NOT IN ('completed', 'cancelled')");
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        // Versão simplificada - em produção use query builder apropriado
        let count: i64 = sqlx::query_scalar(&query)
            .bind(filter.user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(count)
    }
}

/// Row structs para mapeamento
#[derive(Debug)]
struct TaskRow {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    status: TaskStatus,
    priority: TaskPriority,
    due_date: Option<chrono::DateTime<chrono::Utc>>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TaskRow> for Task {
    fn from(row: TaskRow) -> Self {
        Task {
            id: row.id,
            user_id: row.user_id,
            title: row.title,
            description: row.description,
            status: row.status,
            priority: row.priority,
            due_date: row.due_date,
            completed_at: row.completed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug)]
struct TaskHistoryRow {
    id: Uuid,
    task_id: Uuid,
    user_id: Uuid,
    field_name: String,
    old_value: Option<String>,
    new_value: Option<String>,
    changed_at: chrono::DateTime<chrono::Utc>,
}

impl From<TaskHistoryRow> for TaskHistory {
    fn from(row: TaskHistoryRow) -> Self {
        TaskHistory {
            id: row.id,
            task_id: row.task_id,
            user_id: row.user_id,
            field_name: row.field_name,
            old_value: row.old_value,
            new_value: row.new_value,
            changed_at: row.changed_at,
        }
    }
}
