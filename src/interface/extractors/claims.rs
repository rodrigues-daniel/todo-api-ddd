use crate::infrastructure::security::Claims;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

/// Extractor para obter usuário autenticado
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extrair claims inseridos pelo middleware de auth
        let claims = parts
            .extensions
            .get::<Claims>()
            .ok_or(AuthError::Unauthorized)?;

        let user_id = claims.user_id().map_err(|_| AuthError::Unauthorized)?;

        Ok(AuthenticatedUser { user_id })
    }
}

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Não autorizado").into_response()
    }
}
