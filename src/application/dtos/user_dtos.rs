use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// DTO para registro de usuário
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(length(min = 8, message = "Senha deve ter pelo menos 8 caracteres"))]
    pub password: String,

    #[validate(length(min = 2, message = "Nome deve ter pelo menos 2 caracteres"))]
    pub name: String,
}

/// DTO para login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

/// DTO de resposta de autenticação
#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}

/// DTO de resposta de usuário
#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}
