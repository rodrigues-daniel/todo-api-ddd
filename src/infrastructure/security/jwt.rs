use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Claims do JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user_id)
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

impl Claims {
    /// Extrai o user_id dos claims
    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub).context("ID de usuário inválido no token")
    }
}

/// Gera um JWT para um usuário
pub fn generate_jwt(user_id: &Uuid) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").context("JWT_SECRET não definido")?;

    let expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24);

    let now = Utc::now();
    let expiration = now + Duration::hours(expiration_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .context("Falha ao gerar JWT")
}

/// Valida um JWT e retorna os claims
pub fn validate_jwt(token: &str) -> Result<Claims> {
    let secret = std::env::var("JWT_SECRET").context("JWT_SECRET não definido")?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .context("Token inválido ou expirado")?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_jwt() {
        std::env::set_var("JWT_SECRET", "test-secret-key");
        std::env::set_var("JWT_EXPIRATION_HOURS", "1");

        let user_id = Uuid::new_v4();
        let token = generate_jwt(&user_id).unwrap();

        let claims = validate_jwt(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.user_id().unwrap(), user_id);
    }

    #[test]
    fn test_invalid_token() {
        std::env::set_var("JWT_SECRET", "test-secret-key");

        let result = validate_jwt("invalid.token.here");
        assert!(result.is_err());
    }
}
