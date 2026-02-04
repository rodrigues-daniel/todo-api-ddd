use crate::application::dtos::{AuthResponseDto, LoginDto, RegisterUserDto};
use crate::application::use_cases::{LoginUseCase, RegisterUseCase};
use crate::domain::repositories::UserRepository;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use validator::Validate;

/// State compartilhado para handlers de autenticação
#[derive(Clone)]
pub struct AuthState {
    pub user_repository: Arc<dyn UserRepository>,
}

/// Handler: Registrar novo usuário
pub async fn register_handler(
    State(state): State<AuthState>,
    Json(dto): Json<RegisterUserDto>,
) -> Result<Json<AuthResponseDto>, AppError> {
    // Validar DTO
    dto.validate()?;

    // Executar caso de uso
    let use_case = RegisterUseCase::new(state.user_repository);
    let response = use_case.execute(dto).await?;

    Ok(Json(response))
}

/// Handler: Login de usuário
pub async fn login_handler(
    State(state): State<AuthState>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponseDto>, AppError> {
    // Validar DTO
    dto.validate()?;

    // Executar caso de uso
    let use_case = LoginUseCase::new(state.user_repository);
    let response = use_case.execute(dto).await?;

    Ok(Json(response))
}

/// Erro genérico da aplicação
#[derive(Debug)]
pub enum AppError {
    Validation(validator::ValidationErrors),
    Domain(crate::domain::errors::DomainError),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::Validation(err)
    }
}

impl From<crate::domain::errors::DomainError> for AppError {
    fn from(err: crate::domain::errors::DomainError) -> Self {
        AppError::Domain(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use crate::domain::errors::DomainError;

        let (status, message) = match self {
            AppError::Validation(err) => (
                StatusCode::BAD_REQUEST,
                format!("Erro de validação: {}", err),
            ),
            AppError::Domain(DomainError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            AppError::Domain(DomainError::Unauthorized) => {
                (StatusCode::UNAUTHORIZED, "Não autorizado".to_string())
            }
            AppError::Domain(DomainError::Conflict(msg)) => (StatusCode::CONFLICT, msg),
            AppError::Domain(DomainError::ValidationError(msg)) => (StatusCode::BAD_REQUEST, msg),
            AppError::Domain(err) => {
                tracing::error!("Erro interno: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor".to_string(),
                )
            }
        };

        (status, message).into_response()
    }
}
