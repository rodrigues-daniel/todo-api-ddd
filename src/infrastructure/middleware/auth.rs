use crate::infrastructure::security::{Claims, validate_jwt};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// Middleware de autenticação JWT
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, AuthError> {
    // Extrair token do header Authorization
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    // Verificar formato "Bearer {token}"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidFormat)?;

    // Validar token
    let claims = validate_jwt(token).map_err(|_| AuthError::InvalidToken)?;

    // Inserir claims na request para uso nos handlers
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Erros de autenticação
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidFormat,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Token não fornecido"),
            AuthError::InvalidFormat => (StatusCode::UNAUTHORIZED, "Formato de token inválido"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token inválido ou expirado"),
        };

        (status, message).into_response()
    }
}

/// Type alias para middleware
pub type AuthMiddleware =
    fn(Request, Next) -> impl std::future::Future<Output = Result<Response, AuthError>>;
