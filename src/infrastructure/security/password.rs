use anyhow::{Context, Result};
use bcrypt::{DEFAULT_COST, hash, verify};

/// Faz hash de uma senha usando bcrypt
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).context("Falha ao fazer hash da senha")
}

/// Verifica se uma senha corresponde ao hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).context("Falha ao verificar senha")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "minha_senha_secreta";
        let hash = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("senha_errada", &hash).unwrap());
    }

    #[test]
    fn test_different_hashes() {
        let password = "mesma_senha";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        // Hashes devem ser diferentes (salt diferente)
        assert_ne!(hash1, hash2);

        // Mas ambos devem validar
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}
