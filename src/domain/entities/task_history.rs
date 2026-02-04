use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade TaskHistory - Histórico de alterações de tarefas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistory {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_at: DateTime<Utc>,
}

impl TaskHistory {
    /// Cria um novo registro de histórico
    pub fn new(
        task_id: Uuid,
        user_id: Uuid,
        field_name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_id,
            user_id,
            field_name,
            old_value,
            new_value,
            changed_at: Utc::now(),
        }
    }
}
