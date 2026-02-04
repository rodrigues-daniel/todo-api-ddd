use crate::domain::entities::User;
use crate::domain::errors::DomainResult;
use crate::domain::value_objects::Email;
use async_trait::async_trait;
use uuid::Uuid;

/// Trait repository para User (Port do padrão Hexagonal)
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Cria um novo usuário
    async fn create(&self, user: &User) -> DomainResult<User>;

    /// Busca um usuário por ID
    async fn find_by_id(&self, id: &Uuid) -> DomainResult<Option<User>>;

    /// Busca um usuário por email
    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>>;

    /// Atualiza um usuário
    async fn update(&self, user: &User) -> DomainResult<User>;

    /// Deleta um usuário
    async fn delete(&self, id: &Uuid) -> DomainResult<()>;

    /// Verifica se um email já existe
    async fn email_exists(&self, email: &Email) -> DomainResult<bool>;
}
