# Todo API - Sistema Avançado de Gerenciamento de Tarefas

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.7-blue.svg)
![SQLx](https://img.shields.io/badge/SQLx-0.7-green.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## 📋 Índice

- [Sobre o Projeto](#sobre-o-projeto)
- [Propósito](#propósito)
- [Características](#características)
- [Arquitetura](#arquitetura)
- [Tecnologias Utilizadas](#tecnologias-utilizadas)
- [Pré-requisitos](#pré-requisitos)
- [Instalação](#instalação)
- [Configuração](#configuração)
- [Uso](#uso)
- [Endpoints da API](#endpoints-da-api)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Testes](#testes)
- [Segurança](#segurança)
- [Contribuição](#contribuição)
- [Roadmap](#roadmap)
- [Licença](#licença)

---

## 🎯 Sobre o Projeto

**Todo API** é uma aplicação backend robusta e escalável para gerenciamento avançado de tarefas (To-Do), construída com **Rust** e seguindo os princípios de **Domain-Driven Design (DDD)**. Este projeto demonstra as melhores práticas de desenvolvimento em Rust, incluindo arquitetura em camadas, segurança, testes automatizados e documentação completa.

### Por que este projeto?

Este projeto foi desenvolvido como uma demonstração prática de:

- ✅ Como estruturar aplicações Rust complexas usando DDD
- ✅ Implementação de APIs RESTful com Axum
- ✅ Integração com bancos de dados PostgreSQL usando SQLx
- ✅ Autenticação e autorização com JWT
- ✅ Aplicação de princípios SOLID e Clean Architecture
- ✅ Testes automatizados (unitários e de integração)
- ✅ Segurança em aplicações web

---

## 🎓 Propósito

### Objetivos Educacionais

Este projeto serve como **referência completa** para desenvolvedores que desejam:

1. **Aprender Rust para Backend**: Demonstra padrões e práticas recomendadas para desenvolvimento de APIs em Rust
2. **Entender Domain-Driven Design**: Implementação prática dos conceitos de DDD em um projeto real
3. **Arquitetura em Camadas**: Como separar responsabilidades e criar código manutenível
4. **Segurança Web**: Implementação de autenticação JWT, hash de senhas, proteção contra rate limiting
5. **Banco de Dados**: Uso de SQLx para queries type-safe e migrations

### Casos de Uso Reais

Embora seja um projeto educacional, pode ser usado como base para:

- 📱 Aplicações de produtividade pessoal
- 👥 Sistemas de gerenciamento de projetos
- 🏢 Ferramentas corporativas de task tracking
- 🎓 Plataformas de gestão acadêmica
- 🔧 Backend para aplicativos mobile/web de tarefas

---

## ✨ Características

### Funcionalidades Principais

#### 🔐 Autenticação e Autorização
- Registro e login de usuários
- Autenticação via JWT (JSON Web Tokens)
- Hash seguro de senhas com bcrypt
- Proteção de rotas por autenticação
- Validação de tokens em middleware

#### 📝 Gerenciamento de Tarefas (CRUD Completo)
- **Criar** tarefas com título, descrição, prioridade e data de vencimento
- **Listar** tarefas com filtros avançados e paginação
- **Atualizar** tarefas individualmente ou em lote
- **Deletar** tarefas com soft delete opcional
- **Busca** por texto em título e descrição

#### 🎯 Funcionalidades Avançadas
- **Prioridades**: Low, Medium, High, Urgent
- **Status**: Pending, In Progress, Completed, Cancelled
- **Transições de Status**: Validação de mudanças de estado válidas
- **Data de Vencimento**: Controle de deadlines
- **Detecção de Atraso**: Identificação automática de tarefas atrasadas
- **Relacionamento Usuário-Tarefa**: Cada usuário possui suas próprias tarefas (1:N)

#### 📊 Histórico de Alterações
- Rastreamento completo de mudanças em tarefas
- Registro de campo alterado, valor antigo e novo
- Timestamp de cada modificação
- Auditoria completa para compliance

#### 🔍 Filtros e Paginação
- Filtrar por status, prioridade, atraso
- Busca textual em tarefas
- Paginação eficiente com metadados
- Ordenação customizável

#### 🛡️ Segurança
- Rate limiting para prevenir abuso
- Validação de input com Validator
- Proteção contra SQL Injection (SQLx type-safe)
- CORS configurável
- Logging estruturado para auditoria

---

## 🏗️ Arquitetura

### Domain-Driven Design (DDD)

O projeto segue uma arquitetura em **4 camadas** baseada em DDD:

```
┌─────────────────────────────────────────┐
│         Interface Layer (HTTP)          │
│   Routes, Handlers, Extractors          │
│   (Axum Controllers)                    │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│       Application Layer (Use Cases)     │
│   Business Logic, DTOs, Services        │
│   (Orchestration)                       │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│          Domain Layer (Core)            │
│   Entities, Value Objects, Aggregates   │
│   Repository Interfaces, Business Rules │
│   (Pure Business Logic - Framework Free)│
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Infrastructure Layer (Details)     │
│   Database, JWT, Password Hash          │
│   Repository Implementations, Middleware│
│   (External Dependencies)               │
└─────────────────────────────────────────┘
```

### Princípios Aplicados

- **Separation of Concerns**: Cada camada tem responsabilidades bem definidas
- **Dependency Inversion**: Dependências apontam para abstrações (traits)
- **Single Responsibility**: Cada módulo tem uma única razão para mudar
- **Open/Closed**: Aberto para extensão, fechado para modificação
- **Clean Architecture**: Regras de negócio independentes de frameworks

### Padrões de Design

- **Repository Pattern**: Abstração de acesso a dados
- **Use Case Pattern**: Encapsulamento de lógica de aplicação
- **Value Objects**: Validação e imutabilidade
- **Aggregates**: Consistência de entidades relacionadas
- **DTOs**: Separação entre modelos de domínio e API

---

## 🛠️ Tecnologias Utilizadas

### Core Framework
- **[Rust](https://www.rust-lang.org/)** (1.75+) - Linguagem de programação
- **[Axum](https://github.com/tokio-rs/axum)** (0.7) - Web framework assíncrono
- **[Tokio](https://tokio.rs/)** (1.x) - Runtime assíncrono

### Banco de Dados
- **[PostgreSQL](https://www.postgresql.org/)** (14+) - Banco de dados relacional
- **[SQLx](https://github.com/launchbadge/sqlx)** (0.7) - Driver assíncrono type-safe
- **Migrations** - Controle de versão do schema

### Segurança
- **[bcrypt](https://crates.io/crates/bcrypt)** - Hash de senhas
- **[jsonwebtoken](https://crates.io/crates/jsonwebtoken)** - JWT authentication
- **[validator](https://crates.io/crates/validator)** - Validação de dados

### Utilidades
- **[serde](https://serde.rs/)** - Serialização/deserialização
- **[uuid](https://crates.io/crates/uuid)** - Geração de UUIDs
- **[chrono](https://crates.io/crates/chrono)** - Manipulação de datas
- **[tracing](https://crates.io/crates/tracing)** - Logging estruturado
- **[governor](https://crates.io/crates/governor)** - Rate limiting
- **[tower-http](https://crates.io/crates/tower-http)** - Middlewares HTTP

### Testes
- **[reqwest](https://crates.io/crates/reqwest)** - Cliente HTTP para testes
- Testes unitários nativos do Rust
- Testes de integração

---

## 📋 Pré-requisitos

Antes de começar, certifique-se de ter instalado:

- **Rust** 1.75 ou superior
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **PostgreSQL** 14 ou superior
  ```bash
  # Ubuntu/Debian
  sudo apt install postgresql postgresql-contrib
  
  # macOS
  brew install postgresql@14
  
  # Windows
  # Baixe de https://www.postgresql.org/download/windows/
  ```

- **SQLx CLI** (para migrations)
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

- **Git**
  ```bash
  # Ubuntu/Debian
  sudo apt install git
  
  # macOS
  brew install git
  ```

---

## 🚀 Instalação

### 1. Clone o Repositório

```bash
git clone https://github.com/seu-usuario/todo-api.git
cd todo-api
```

### 2. Configure as Variáveis de Ambiente

```bash
cp .env.example .env
```

Edite o arquivo `.env` com suas configurações:

```env
DATABASE_URL=postgres://postgres:sua_senha@localhost:5432/todo_db
JWT_SECRET=seu-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION_HOURS=24
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
```

### 3. Crie o Banco de Dados

```bash
# Criar banco de dados
createdb todo_db

# Ou via psql
psql -U postgres
CREATE DATABASE todo_db;
\q
```

### 4. Execute as Migrations

```bash
sqlx migrate run
```

### 5. Compile e Execute

```bash
# Desenvolvimento
cargo run

# Produção (otimizado)
cargo build --release
./target/release/todo-api
```

O servidor estará disponível em `http://localhost:8080`

---

## ⚙️ Configuração

### Variáveis de Ambiente

| Variável | Descrição | Padrão | Obrigatória |
|----------|-----------|--------|-------------|
| `DATABASE_URL` | String de conexão PostgreSQL | - | ✅ |
| `JWT_SECRET` | Chave secreta para assinar JWTs | - | ✅ |
| `JWT_EXPIRATION_HOURS` | Tempo de expiração do token (horas) | 24 | ❌ |
| `SERVER_HOST` | Host do servidor | 0.0.0.0 | ❌ |
| `SERVER_PORT` | Porta do servidor | 8080 | ❌ |
| `RUST_LOG` | Nível de logging (trace, debug, info, warn, error) | info | ❌ |

### Migrations

As migrations estão em `migrations/` e são executadas em ordem:

1. `20240101_create_users.sql` - Cria tabela de usuários
2. `20240102_create_tasks.sql` - Cria tabela de tarefas
3. `20240103_create_task_history.sql` - Cria tabela de histórico

Para criar uma nova migration:

```bash
sqlx migrate add nome_da_migration
```

Para reverter a última migration:

```bash
sqlx migrate revert
```

---

## 📖 Uso

### Exemplos com cURL

#### 1. Registrar Novo Usuário

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "senha123456",
    "name": "João Silva"
  }'
```

**Resposta:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "name": "João Silva"
  }
}
```

#### 2. Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "senha123456"
  }'
```

#### 3. Criar Tarefa

```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer SEU_TOKEN_JWT" \
  -d '{
    "title": "Implementar autenticação",
    "description": "Adicionar JWT auth ao projeto",
    "priority": "high",
    "due_date": "2024-12-31T23:59:59Z"
  }'
```

#### 4. Listar Tarefas com Filtros

```bash
curl -X GET "http://localhost:8080/api/tasks?status=pending&priority=high&page=1&page_size=10" \
  -H "Authorization: Bearer SEU_TOKEN_JWT"
```

#### 5. Atualizar Tarefa

```bash
curl -X PUT http://localhost:8080/api/tasks/TASK_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer SEU_TOKEN_JWT" \
  -d '{
    "status": "in_progress",
    "priority": "urgent"
  }'
```

#### 6. Obter Histórico de Tarefa

```bash
curl -X GET http://localhost:8080/api/tasks/TASK_ID/history \
  -H "Authorization: Bearer SEU_TOKEN_JWT"
```

#### 7. Deletar Tarefa

```bash
curl -X DELETE http://localhost:8080/api/tasks/TASK_ID \
  -H "Authorization: Bearer SEU_TOKEN_JWT"
```

---

## 🔌 Endpoints da API

### Autenticação

| Método | Endpoint | Descrição | Auth |
|--------|----------|-----------|------|
| POST | `/api/auth/register` | Registrar novo usuário | ❌ |
| POST | `/api/auth/login` | Login de usuário | ❌ |

### Tarefas

| Método | Endpoint | Descrição | Auth |
|--------|----------|-----------|------|
| POST | `/api/tasks` | Criar nova tarefa | ✅ |
| GET | `/api/tasks` | Listar tarefas (com filtros) | ✅ |
| GET | `/api/tasks/:id` | Obter tarefa específica | ✅ |
| PUT | `/api/tasks/:id` | Atualizar tarefa | ✅ |
| DELETE | `/api/tasks/:id` | Deletar tarefa | ✅ |
| GET | `/api/tasks/:id/history` | Obter histórico de tarefa | ✅ |

### Filtros Disponíveis (Query Params)

- `status`: pending, in_progress, completed, cancelled
- `priority`: low, medium, high, urgent
- `overdue_only`: true/false
- `search`: texto para busca
- `page`: número da página (padrão: 1)
- `page_size`: itens por página (padrão: 10)

### Health Check

| Método | Endpoint | Descrição | Auth |
|--------|----------|-----------|------|
| GET | `/health` | Status da API | ❌ |

---

## 📁 Estrutura do Projeto

```
todo-api/
├── src/
│   ├── main.rs                    # Entry point da aplicação
│   ├── lib.rs                     # Library root
│   ├── config.rs                  # Configurações gerais
│   │
│   ├── domain/                    # Camada de Domínio (Core Business)
│   │   ├── entities/              # Entidades de negócio
│   │   │   ├── user.rs            # User aggregate
│   │   │   ├── task.rs            # Task aggregate
│   │   │   └── task_history.rs    # Task history entity
│   │   ├── value_objects/         # Objetos de valor
│   │   │   ├── email.rs           # Email VO com validação
│   │   │   ├── task_status.rs     # Status enum
│   │   │   └── task_priority.rs   # Priority enum
│   │   ├── repositories/          # Interfaces de repositórios
│   │   │   ├── user_repository.rs # User repo trait
│   │   │   └── task_repository.rs # Task repo trait
│   │   └── errors.rs              # Domain errors
│   │
│   ├── application/               # Camada de Aplicação (Use Cases)
│   │   ├── dtos/                  # Data Transfer Objects
│   │   │   ├── user_dto.rs        # User DTOs
│   │   │   └── task_dto.rs        # Task DTOs
│   │   ├── use_cases/             # Casos de uso
│   │   │   ├── auth/              # Autenticação
│   │   │   │   ├── register.rs    # Registro de usuário
│   │   │   │   └── login.rs       # Login
│   │   │   └── tasks/             # Tarefas
│   │   │       ├── create_task.rs # Criar tarefa
│   │   │       ├── update_task.rs # Atualizar tarefa
│   │   │       ├── delete_task.rs # Deletar tarefa
│   │   │       ├── get_task.rs    # Obter tarefa
│   │   │       └── list_tasks.rs  # Listar tarefas
│   │   └── services/              # Serviços de aplicação
│   │       └── task_service.rs    # Task service
│   │
│   ├── infrastructure/            # Camada de Infraestrutura
│   │   ├── database/              # Database setup
│   │   │   └── connection.rs      # Pool de conexões
│   │   ├── repositories/          # Implementações de repositórios
│   │   │   ├── postgres_user_repository.rs
│   │   │   └── postgres_task_repository.rs
│   │   ├── security/              # Segurança
│   │   │   ├── jwt.rs             # JWT generation/validation
│   │   │   └── password.rs        # Password hashing
│   │   └── middleware/            # Middlewares
│   │       ├── auth.rs            # Auth middleware
│   │       ├── logging.rs         # Request logging
│   │       └── rate_limit.rs      # Rate limiting
│   │
│   └── interface/                 # Camada de Interface (HTTP)
│       ├── routes/                # Definição de rotas
│       │   ├── auth_routes.rs     # Rotas de auth
│       │   └── task_routes.rs     # Rotas de tasks
│       ├── handlers/              # Request handlers
│       │   ├── auth_handlers.rs   # Auth handlers
│       │   └── task_handlers.rs   # Task handlers
│       └── extractors/            # Custom extractors
│           └── claims.rs          # JWT claims extractor
│
├── migrations/                    # Database migrations
│   ├── 20240101_create_users.sql
│   ├── 20240102_create_tasks.sql
│   └── 20240103_create_task_history.sql
│
├── tests/                         # Testes de integração
│   ├── common/                    # Helpers de teste
│   ├── auth_tests.rs              # Testes de autenticação
│   └── task_tests.rs              # Testes de tarefas
│
├── Cargo.toml                     # Dependências e metadados
├── .env.example                   # Exemplo de variáveis de ambiente
├── .gitignore                     # Arquivos ignorados pelo Git
└── README.md                      # Este arquivo
```

---

## 🧪 Testes

### Executar Testes

```bash
# Todos os testes
cargo test

# Testes unitários apenas
cargo test --lib

# Testes de integração apenas
cargo test --test '*'

# Com output detalhado
cargo test -- --nocapture

# Testes específicos
cargo test auth
```

### Cobertura de Testes

```bash
# Instalar tarpaulin
cargo install cargo-tarpaulin

# Gerar relatório de cobertura
cargo tarpaulin --out Html
```

### Tipos de Testes

1. **Testes Unitários**: Em cada módulo (`#[cfg(test)]`)
   - Value Objects (Email, Status, Priority)
   - Entities (User, Task)
   - Business Rules

2. **Testes de Integração**: Em `tests/`
   - Endpoints completos
   - Fluxos de autenticação
   - CRUD de tarefas
   - Validações end-to-end

---

## 🔒 Segurança

### Medidas de Segurança Implementadas

#### 1. Autenticação JWT
- Tokens assinados com HS256
- Expiração configurável
- Validação em cada request protegido

#### 2. Proteção de Senhas
- Hash com bcrypt (cost factor 12)
- Senhas nunca armazenadas em texto plano
- Validação de força mínima

#### 3. Validação de Input
- Validação de email
- Sanitização de strings
- Limites de tamanho
- Type-safety do Rust

#### 4. SQL Injection Prevention
- SQLx com queries parametrizadas
- Compilação em tempo de build
- Type-safe queries

#### 5. Rate Limiting
- Limite de requisições por IP
- Proteção contra brute force
- Configurável por endpoint

#### 6. CORS
- Configuração de origens permitidas
- Headers de segurança
- Métodos HTTP controlados

### Boas Práticas Recomendadas

- ✅ Sempre use HTTPS em produção
- ✅ Rode migrations em ambientes controlados
- ✅ Use secrets managers (AWS Secrets, Vault)
- ✅ Implemente rate limiting agressivo
- ✅ Monitore logs de segurança
- ✅ Mantenha dependências atualizadas
- ✅ Use `cargo audit` regularmente

---

## 🤝 Contribuição

Contribuições são bem-vindas! Siga estas etapas:

### 1. Fork o Projeto

```bash
git clone https://github.com/seu-usuario/todo-api.git
```

### 2. Crie uma Branch

```bash
git checkout -b feature/nova-funcionalidade
```

### 3. Commit suas Mudanças

```bash
git commit -m 'feat: adiciona nova funcionalidade X'
```

### 4. Push para a Branch

```bash
git push origin feature/nova-funcionalidade
```

### 5. Abra um Pull Request

### Convenções de Commit

Seguimos [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Nova funcionalidade
- `fix:` Correção de bug
- `docs:` Documentação
- `test:` Testes
- `refactor:` Refatoração
- `chore:` Manutenção

---

## 🗺️ Roadmap

### Versão 1.0 (Atual)
- [ ] CRUD completo de tarefas
- [ ] Autenticação JWT
- [ ] Histórico de alterações
- [ ] Filtros e paginação
- [ ] Testes unitários e de integração

### Versão 2.0 (Planejado)
- [ ] Tags e categorias
- [ ] Compartilhamento de tarefas entre usuários
- [ ] Notificações (email/webhook)
- [ ] Upload de anexos
- [ ] Comentários em tarefas
- [ ] Dashboard com estatísticas
- [ ] Export para PDF/Excel

### Versão 3.0 (Futuro)
- [ ] GraphQL API
- [ ] WebSockets para real-time
- [ ] Integração com calendários (Google, Outlook)
- [ ] Mobile app (Flutter/React Native)
- [ ] AI-powered task suggestions
- [ ] Multi-tenancy

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

```
MIT License

Copyright (c) 2024 [Seu Nome]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 👨‍💻 Autor

**Seu Nome**

- GitHub: [@rodrigues-daniel](https://github.com/rodrigues-daniel/)
- LinkedIn: [Daniel Rodrigues](https://www.linkedin.com/in/daniel-c-rodrigues/)
- Email: daniel.rodrighes@gmail.com

---

## 🙏 Agradecimentos

- Comunidade Rust pela linguagem incrível
- Tokio team pelo runtime assíncrono
- Axum contributors pelo framework elegante
- SQLx maintainers pelo driver type-safe
- Todos os contributors de crates open-source

---

## 📚 Recursos Adicionais

### Documentação
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Guide](https://github.com/launchbadge/sqlx)
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)

### Tutoriais
- [Building REST APIs with Rust](https://blog.logrocket.com/rust-web-apps-using-rocket-framework/)
- [Async Rust](https://rust-lang.github.io/async-book/)
- [PostgreSQL with Rust](https://diesel.rs/guides/getting-started/)

---

## 🐛 Reportar Bugs

Encontrou um bug? Por favor, abra uma [issue](https://github.com/seu-usuario/todo-api/issues) com:

- Descrição do bug
- Passos para reproduzir
- Comportamento esperado vs atual
- Screenshots (se aplicável)
- Informações do ambiente (OS, versão do Rust)

---

## ❓ FAQ

**P: Posso usar este projeto em produção?**
R: Sim, mas recomendamos revisão de segurança e ajustes para seu caso específico.

**P: Qual versão mínima do Rust?**
R: Rust 1.75 ou superior.

**P: Suporta outros bancos além do PostgreSQL?**
R: Atualmente apenas PostgreSQL, mas pode ser adaptado para MySQL/SQLite.

**P: Como faço deploy?**
R: Veja seção de deployment no [DEPLOYMENT.md](DEPLOYMENT.md) (a ser criado).

---

**⭐ Se este projeto foi útil, considere dar uma estrela no GitHub!**
