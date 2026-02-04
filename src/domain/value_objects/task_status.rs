use crate::domain::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Status possíveis de uma tarefa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl TaskStatus {
    /// Verifica se a transição de status é válida
    pub fn can_transition_to(&self, new_status: TaskStatus) -> bool {
        use TaskStatus::*;

        match (self, new_status) {
            (Pending, InProgress) => true,
            (Pending, Cancelled) => true,
            (InProgress, Completed) => true,
            (InProgress, Cancelled) => true,
            (InProgress, Pending) => true,
            (_, _) if self == &new_status => true,
            _ => false,
        }
    }

    /// Valida e cria uma nova transição de status
    pub fn transition(&self, new_status: TaskStatus) -> DomainResult<TaskStatus> {
        if self.can_transition_to(new_status) {
            Ok(new_status)
        } else {
            Err(DomainError::ValidationError(format!(
                "Transição inválida de {:?} para {:?}",
                self, new_status
            )))
        }
    }
}

impl FromStr for TaskStatus {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "in_progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            "cancelled" => Ok(TaskStatus::Cancelled),
            _ => Err(DomainError::InvalidTaskStatus),
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(TaskStatus::Pending.can_transition_to(TaskStatus::InProgress));
        assert!(TaskStatus::InProgress.can_transition_to(TaskStatus::Completed));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!TaskStatus::Completed.can_transition_to(TaskStatus::Pending));
        assert!(!TaskStatus::Cancelled.can_transition_to(TaskStatus::InProgress));
    }
}
