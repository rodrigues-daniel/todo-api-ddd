use crate::application::dtos::{AuthResponseDto, LoginDto, UserResponseDto};
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::Email;
use crate::infrastructure::security::{generate_jwt, verify_password};
use std::sync::Arc;

/// Caso de uso: Login de usuário
pub struct LoginUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl LoginUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Executa o login
    pub async fn execute(&self, dto: LoginDto) -> DomainResult<AuthResponseDto> {
        // Validar email
        let email = Email::new(dto.email)?;

        // Buscar usuário
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(DomainError::Unauthorized)?;

        // Verificar senha
        if !verify_password(&dto.password, &user.password_hash)
            .map_err(|e| DomainError::Internal(e.to_string()))?
        {
            return Err(DomainError::Unauthorized);
        }

        // Gerar token JWT
        let token = generate_jwt(&user.id).map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(AuthResponseDto {
            token,
            user: UserResponseDto {
                id: user.id,
                email: user.email.as_str().to_string(),
                name: user.name,
            },
        })
    }
}
