use crate::application::dtos::{AuthResponseDto, RegisterUserDto, UserResponseDto};
use crate::domain::entities::User;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use crate::infrastructure::security::{generate_jwt, hash_password};
use std::sync::Arc;
use uuid::Uuid;

/// Caso de uso: Registrar novo usuário
pub struct RegisterUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl RegisterUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Executa o registro de usuário
    pub async fn execute(&self, dto: RegisterUserDto) -> DomainResult<AuthResponseDto> {
        // Validar email
        let email = Email::new(dto.email)?;

        // Verificar se email já existe
        if self.user_repository.email_exists(&email).await? {
            return Err(DomainError::Conflict("Email já está em uso".to_string()));
        }

        // Hash da senha
        let password_hash =
            hash_password(&dto.password).map_err(|e| DomainError::Internal(e.to_string()))?;

        // Criar usuário
        let user = User::new(Uuid::new_v4(), email, password_hash, dto.name);

        // Salvar no repositório
        let saved_user = self.user_repository.create(&user).await?;

        // Gerar token JWT
        let token =
            generate_jwt(&saved_user.id).map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(AuthResponseDto {
            token,
            user: UserResponseDto {
                id: saved_user.id,
                email: saved_user.email.as_str().to_string(),
                name: saved_user.name,
            },
        })
    }
}
