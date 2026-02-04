use thiserror::Error;

/// Erros de domínio da aplicação
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Entidade não encontrada: {0}")]
    NotFound(String),

    #[error("Validação falhou: {0}")]
    ValidationError(String),

    #[error("Email inválido: {0}")]
    InvalidEmail(String),

    #[error("Status de tarefa inválido")]
    InvalidTaskStatus,

    #[error("Prioridade de tarefa inválida")]
    InvalidTaskPriority,

    #[error("Operação não autorizada")]
    Unauthorized,

    #[error("Conflito de dados: {0}")]
    Conflict(String),

    #[error("Erro interno: {0}")]
    Internal(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
