use anyhow::{Context, Result};
use serde::Deserialize;

/// Configuração da aplicação carregada de variáveis de ambiente
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// URL de conexão com o banco de dados PostgreSQL
    pub database_url: String,

    /// Chave secreta para assinar tokens JWT
    pub jwt_secret: String,

    /// Tempo de expiração do token JWT em horas
    pub jwt_expiration_hours: u64,

    /// Host do servidor (ex: 0.0.0.0 ou 127.0.0.1)
    pub server_host: String,

    /// Porta do servidor (ex: 8080)
    pub server_port: u16,
}

impl Config {
    /// Carrega a configuração das variáveis de ambiente
    ///
    /// # Variáveis Obrigatórias
    /// - `DATABASE_URL`: String de conexão PostgreSQL
    /// - `JWT_SECRET`: Chave secreta para JWT
    ///
    /// # Variáveis Opcionais (com valores padrão)
    /// - `JWT_EXPIRATION_HOURS`: Tempo de expiração em horas (padrão: 24)
    /// - `SERVER_HOST`: Host do servidor (padrão: 0.0.0.0)
    /// - `SERVER_PORT`: Porta do servidor (padrão: 8080)
    ///
    /// # Exemplo
    ///
    /// ```rust,no_run
    /// use todo_api::Config;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = Config::from_env().expect("Erro ao carregar configuração");
    ///     println!("Servidor rodando na porta: {}", config.server_port);
    /// }
    /// ```
    pub fn from_env() -> Result<Self> {
        // Carrega o arquivo .env se existir (ignora erro se não existir)
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context(
                "DATABASE_URL deve estar definida no arquivo .env ou nas variáveis de ambiente",
            )?,

            jwt_secret: std::env::var("JWT_SECRET").context(
                "JWT_SECRET deve estar definida no arquivo .env ou nas variáveis de ambiente",
            )?,

            jwt_expiration_hours: std::env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .context("JWT_EXPIRATION_HOURS deve ser um número válido")?,

            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),

            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("SERVER_PORT deve ser um número válido entre 1 e 65535")?,
        })
    }

    /// Valida se a configuração está correta
    pub fn validate(&self) -> Result<()> {
        // Validar URL do banco
        if !self.database_url.starts_with("postgres://")
            && !self.database_url.starts_with("postgresql://")
        {
            anyhow::bail!("DATABASE_URL deve começar com 'postgres://' ou 'postgresql://'");
        }

        // Validar JWT secret (mínimo de 32 caracteres para segurança)
        if self.jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET deve ter pelo menos 32 caracteres para ser seguro");
        }

        // Validar tempo de expiração
        if self.jwt_expiration_hours == 0 || self.jwt_expiration_hours > 8760 {
            anyhow::bail!("JWT_EXPIRATION_HOURS deve estar entre 1 e 8760 (1 ano)");
        }

        // Validar porta
        if self.server_port == 0 {
            anyhow::bail!("SERVER_PORT não pode ser 0");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Configura variáveis de ambiente para testes
    fn setup_test_env() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/test_db");
        std::env::set_var(
            "JWT_SECRET",
            "test-secret-key-with-at-least-32-chars-for-security",
        );
        std::env::set_var("JWT_EXPIRATION_HOURS", "48");
        std::env::set_var("SERVER_HOST", "127.0.0.1");
        std::env::set_var("SERVER_PORT", "3000");
    }

    /// Limpa variáveis de ambiente após testes
    fn cleanup_test_env() {
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("JWT_SECRET");
        std::env::remove_var("JWT_EXPIRATION_HOURS");
        std::env::remove_var("SERVER_HOST");
        std::env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_from_env() {
        setup_test_env();

        let config = Config::from_env().unwrap();

        assert_eq!(config.database_url, "postgres://localhost/test_db");
        assert_eq!(
            config.jwt_secret,
            "test-secret-key-with-at-least-32-chars-for-security"
        );
        assert_eq!(config.jwt_expiration_hours, 48);
        assert_eq!(config.server_host, "127.0.0.1");
        assert_eq!(config.server_port, 3000);

        cleanup_test_env();
    }

    #[test]
    fn test_default_values() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/test");
        std::env::set_var(
            "JWT_SECRET",
            "test-secret-key-with-at-least-32-chars-for-security",
        );
        std::env::remove_var("JWT_EXPIRATION_HOURS");
        std::env::remove_var("SERVER_HOST");
        std::env::remove_var("SERVER_PORT");

        let config = Config::from_env().unwrap();

        assert_eq!(config.jwt_expiration_hours, 24); // padrão
        assert_eq!(config.server_host, "0.0.0.0"); // padrão
        assert_eq!(config.server_port, 8080); // padrão

        cleanup_test_env();
    }

    #[test]
    fn test_missing_database_url() {
        cleanup_test_env();
        std::env::set_var(
            "JWT_SECRET",
            "test-secret-key-with-at-least-32-chars-for-security",
        );

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("DATABASE_URL"));

        cleanup_test_env();
    }

    #[test]
    fn test_missing_jwt_secret() {
        cleanup_test_env();
        std::env::set_var("DATABASE_URL", "postgres://localhost/test");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JWT_SECRET"));

        cleanup_test_env();
    }

    #[test]
    fn test_invalid_port() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/test");
        std::env::set_var(
            "JWT_SECRET",
            "test-secret-key-with-at-least-32-chars-for-security",
        );
        std::env::set_var("SERVER_PORT", "invalid");

        let result = Config::from_env();
        assert!(result.is_err());

        cleanup_test_env();
    }

    #[test]
    fn test_validate_short_jwt_secret() {
        let config = Config {
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "short".to_string(), // Muito curto
            jwt_expiration_hours: 24,
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("32 caracteres"));
    }

    #[test]
    fn test_validate_invalid_database_url() {
        let config = Config {
            database_url: "invalid://localhost/test".to_string(),
            jwt_secret: "test-secret-key-with-at-least-32-chars-for-security".to_string(),
            jwt_expiration_hours: 24,
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("postgres://"));
    }

    #[test]
    fn test_validate_success() {
        let config = Config {
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "test-secret-key-with-at-least-32-chars-for-security".to_string(),
            jwt_expiration_hours: 24,
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
        };

        assert!(config.validate().is_ok());
    }
}
