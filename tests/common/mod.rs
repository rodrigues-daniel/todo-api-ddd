use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::sync::Arc;
use todo_api::{Config, infrastructure};
use tokio::sync::Mutex;
use uuid::Uuid;

/// Pool de banco de dados compartilhado entre testes
pub static TEST_POOL: Lazy<Arc<Mutex<Option<PgPool>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// Configura o ambiente de testes
pub async fn setup() -> PgPool {
    // Carregar variáveis de ambiente
    dotenv::dotenv().ok();

    // Configurar logging para testes
    let _ = tracing_subscriber::fmt().with_test_writer().try_init();

    // Criar pool de conexões
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/todo_test".to_string());

    let pool = infrastructure::database::create_pool(&database_url)
        .await
        .expect("Falha ao criar pool de teste");

    // Executar migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Falha ao executar migrations");

    pool
}

/// Limpa o banco de dados entre testes
pub async fn cleanup(pool: &PgPool) {
    sqlx::query!("TRUNCATE users, tasks, task_history RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .expect("Falha ao limpar banco de dados");
}

/// Cria um usuário de teste
pub async fn create_test_user(pool: &PgPool) -> (Uuid, String) {
    use todo_api::infrastructure::security::hash_password;

    let user_id = Uuid::new_v4();
    let email = format!("test-{}@example.com", Uuid::new_v4());
    let password = "test_password_123";
    let password_hash = hash_password(password).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, name)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        email,
        password_hash,
        "Test User",
    )
    .execute(pool)
    .await
    .expect("Falha ao criar usuário de teste");

    (user_id, email)
}

/// Gera um token JWT para testes
pub fn generate_test_token(user_id: &Uuid) -> String {
    use todo_api::infrastructure::security::generate_jwt;

    std::env::set_var("JWT_SECRET", "test-secret-key");
    std::env::set_var("JWT_EXPIRATION_HOURS", "24");

    generate_jwt(user_id).expect("Falha ao gerar token de teste")
}
