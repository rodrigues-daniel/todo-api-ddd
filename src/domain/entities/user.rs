use crate::domain::value_objects::Email;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade User (Agregado Root)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Cria um novo usuário
    pub fn new(id: Uuid, email: Email, password_hash: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            email,
            password_hash,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    /// Atualiza o nome do usuário
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// Atualiza o hash da senha
    pub fn update_password(&mut self, password_hash: String) {
        self.password_hash = password_hash;
        self.updated_at = Utc::now();
    }
}
