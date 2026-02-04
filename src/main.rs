use anyhow::Result;
use std::sync::Arc;
use todo_api::{
    Config,
    infrastructure::{self, repositories},
    interface::routes,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar sistema de logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_api=debug,tower_http=debug,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Iniciando Todo API...");

    // Carregar configuração do ambiente
    let config = Config::from_env()?;
    tracing::info!("✅ Configuração carregada");

    // Configurar pool de conexões do banco de dados
    tracing::info!("📦 Conectando ao banco de dados...");
    let db_pool = infrastructure::database::create_pool(&config.database_url).await?;
    tracing::info!("✅ Conexão com banco de dados estabelecida");

    // Executar migrations automaticamente
    tracing::info!("🔄 Executando migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| {
            tracing::error!("❌ Erro ao executar migrations: {}", e);
            e
        })?;
    tracing::info!("✅ Migrations executadas com sucesso");

    // Criar repositórios (implementações concretas)
    let user_repository = Arc::new(repositories::PostgresUserRepository::new(db_pool.clone()));
    let task_repository = Arc::new(repositories::PostgresTaskRepository::new(db_pool.clone()));

    tracing::info!("✅ Repositórios inicializados");

    // Criar aplicação com todas as rotas
    let app = routes::create_routes(config.clone(), user_repository, task_repository);

    // Configurar endereço do servidor
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        tracing::error!("❌ Erro ao fazer bind no endereço {}: {}", addr, e);
        e
    })?;

    tracing::info!("🌐 Servidor rodando em http://{}", addr);
    tracing::info!("📚 Endpoints disponíveis:");
    tracing::info!("   POST   /api/auth/register      - Registrar usuário");
    tracing::info!("   POST   /api/auth/login         - Login");
    tracing::info!("   POST   /api/tasks              - Criar tarefa");
    tracing::info!("   GET    /api/tasks              - Listar tarefas");
    tracing::info!("   GET    /api/tasks/:id          - Obter tarefa");
    tracing::info!("   PUT    /api/tasks/:id          - Atualizar tarefa");
    tracing::info!("   DELETE /api/tasks/:id          - Deletar tarefa");
    tracing::info!("   GET    /api/tasks/:id/history  - Histórico da tarefa");
    tracing::info!("   GET    /health                 - Health check");
    tracing::info!("");
    tracing::info!("✨ Servidor pronto para receber requisições!");

    // Iniciar servidor
    axum::serve(listener, app).await.map_err(|e| {
        tracing::error!("❌ Erro no servidor: {}", e);
        e
    })?;

    Ok(())
}
