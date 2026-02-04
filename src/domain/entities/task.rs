use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::value_objects::{TaskPriority, TaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade Task (Agregado Root)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Cria uma nova tarefa
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        priority: TaskPriority,
        due_date: Option<DateTime<Utc>>,
    ) -> DomainResult<Self> {
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Título não pode ser vazio".to_string(),
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id,
            user_id,
            title,
            description,
            status: TaskStatus::Pending,
            priority,
            due_date,
            completed_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Atualiza o título
    pub fn update_title(&mut self, title: String) -> DomainResult<()> {
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Título não pode ser vazio".to_string(),
            ));
        }
        self.title = title;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Atualiza a descrição
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    /// Atualiza o status da tarefa
    pub fn update_status(&mut self, new_status: TaskStatus) -> DomainResult<()> {
        let validated_status = self.status.transition(new_status)?;
        self.status = validated_status;

        if validated_status == TaskStatus::Completed {
            self.completed_at = Some(Utc::now());
        } else if self.completed_at.is_some() && validated_status != TaskStatus::Completed {
            self.completed_at = None;
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Atualiza a prioridade
    pub fn update_priority(&mut self, priority: TaskPriority) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }

    /// Atualiza a data de vencimento
    pub fn update_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
        self.updated_at = Utc::now();
    }

    /// Verifica se a tarefa está atrasada
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            if self.status != TaskStatus::Completed && self.status != TaskStatus::Cancelled {
                return Utc::now() > due_date;
            }
        }
        false
    }

    /// Verifica se o usuário é o dono da tarefa
    pub fn is_owned_by(&self, user_id: &Uuid) -> bool {
        &self.user_id == user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = Task::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test Task".to_string(),
            None,
            TaskPriority::Medium,
            None,
        );
        assert!(task.is_ok());
    }

    #[test]
    fn test_empty_title_fails() {
        let task = Task::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "".to_string(),
            None,
            TaskPriority::Medium,
            None,
        );
        assert!(task.is_err());
    }

    #[test]
    fn test_status_transition() {
        let mut task = Task::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test".to_string(),
            None,
            TaskPriority::Medium,
            None,
        )
        .unwrap();

        assert!(task.update_status(TaskStatus::InProgress).is_ok());
        assert_eq!(task.status, TaskStatus::InProgress);
    }
}
