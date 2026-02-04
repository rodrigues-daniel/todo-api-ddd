use crate::domain::entities::User;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// Implementação PostgreSQL do UserRepository
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> DomainResult<User> {
        sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO users (id, email, password_hash, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, password_hash, name, created_at, updated_at
            "#,
            user.id,
            user.email.as_str(),
            user.password_hash,
            user.name,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("users_email_key") {
                    return DomainError::Conflict("Email já existe".to_string());
                }
            }
            DomainError::Internal(e.to_string())
        })?
        .try_into()
    }

    async fn find_by_id(&self, id: &Uuid) -> DomainResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(r.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, password_hash, name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email.as_str(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(r.try_into()?)),
            None => Ok(None),
        }
    }

    async fn update(&self, user: &User) -> DomainResult<User> {
        sqlx::query_as!(
            UserRow,
            r#"
            UPDATE users
            SET email = $2, password_hash = $3, name = $4, updated_at = $5
            WHERE id = $1
            RETURNING id, email, password_hash, name, created_at, updated_at
            "#,
            user.id,
            user.email.as_str(),
            user.password_hash,
            user.name,
            user.updated_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?
        .try_into()
    }

    async fn delete(&self, id: &Uuid) -> DomainResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn email_exists(&self, email: &Email) -> DomainResult<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!"
            "#,
            email.as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(result.exists)
    }
}

/// Row struct para mapeamento do banco
#[derive(Debug)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = DomainError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            email: Email::new(row.email)?,
            password_hash: row.password_hash,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}
