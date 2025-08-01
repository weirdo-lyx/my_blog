# 后端项目文档 (Rust + Axum)

本文档详细解释了博客后端项目的代码结构和每一部分的功能。

## 核心技术栈

*   **语言**: Rust
*   **Web 框架**: Axum (构建在 Tokio 之上)
*   **数据库**: SQLite (通过 `sqlx` 库进行异步操作)
*   **异步运行时**: Tokio
*   **环境变量处理**: `dotenvy`

---

## 文件结构解析

```
backend/
├── src/
│   ├── main.rs         # 应用入口，负责初始化和路由配置
│   ├── database.rs     # 数据库连接和初始化逻辑
│   ├── handlers.rs     # 所有 API 请求的处理函数
│   └── models.rs       # 数据结构定义 (Post 等)
├── .env              # 环境变量，存放数据库连接字符串
├── blog.db           # SQLite 数据库文件
└── Cargo.toml        # 项目依赖和配置
```

---

## 代码详解

### 1. `Cargo.toml` (项目依赖)

这是 Rust 项目的清单文件，定义了项目元数据和所有依赖的库 (crates)。

```toml
[dependencies]
axum = "0.8.4"          # 高性能、符合人体工程学的 Web 框架
tokio = { version = "1.47.0", features = ["full"] } # 异步运行时，"full" 特性开启所有功能
serde = { version = "1.0.219", features = ["derive"] } # 用于数据结构的序列化和反序列化 (如 JSON)
sqlx = { version = "0.8.6", features = ["sqlite", "runtime-tokio-rustls"] } # 现代化的、异步的 SQL 工具包
dotenvy = "0.15.7"      # 从 .env 文件加载环境变量
tower-http = { version = "0.6.6", features = ["cors"] } # 提供处理 HTTP 的中间件，这里主要用 CORS
```

### 2. `.env` (环境变量)

这个文件用于在本地开发时存储配置，避免将敏感信息硬编码在代码中。

```env
# 定义数据库连接的 URL
# `sqlite:` 协议告诉 sqlx 使用 SQLite 驱动
# `blog.db` 是数据库文件的名字
DATABASE_URL=sqlite:blog.db
```

### 3. `src/main.rs` (应用主入口)

这是程序的起点。它的职责是：初始化、配置路由、启动服务器。

```rust
// 引入 axum 的路由功能和 Router
use axum::{
    routing::{get, post, put, delete}, // 导入 HTTP 方法对应的函数
    Router, // Axum 的核心，用于构建路由
};
// 引入标准库的网络地址模块
use std::net::SocketAddr;
// 引入 tower_http 的 CORS 中间件
use tower_http::cors::{Any, CorsLayer};

// 声明我们自己创建的模块，让 main.rs 可以使用它们内部的代码
mod database;
mod handlers;
mod models;

// 从我们自己的 handlers 模块中，导入所有内容 (*)
use handlers::*;

// `#[tokio::main]` 是一个宏，它将一个普通的 main 函数转换成一个异步的 main 函数
// 这样我们就可以在 main 函数里使用 .await 了
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从 .env 文件加载环境变量。如果文件不存在或加载失败，.ok() 会忽略错误
    dotenvy::dotenv().ok();

    // 从环境中读取 DATABASE_URL 变量
    // .expect() 会在变量未设置时让程序崩溃并显示提示信息
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // 调用 database 模块的 init_pool 函数来初始化数据库连接池
    // `?` 操作符用于错误传播，如果 init_pool 返回错误，main 函数会立即返回这个错误
    let pool = database::init_pool(&db_url).await?;
    // 创建应用的状态，将数据库连接池包裹进去，以便在所有 handler 中共享
    let app_state = AppState { pool };

    // 配置 CORS (跨源资源共享)
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许任何来源的请求 (开发时常用)
        .allow_methods(Any) // 允许任何 HTTP 方法 (GET, POST, etc.)
        .allow_headers(Any); // 允许任何 HTTP 头

    // 创建一个新的 Axum Router
    let app = Router::new()
        // 定义 /posts 路由：GET 请求由 get_posts 函数处理，POST 请求由 create_post 处理
        .route("/posts", get(get_posts).post(create_post))
        // 定义 /posts/:id 路由：GET, PUT, DELETE 请求分别由对应的函数处理
        // `{id}` 是一个路径参数，可以捕获 URL 中的动态部分
        .route("/posts/{id}", get(get_post).put(update_post).delete(delete_post))
        // 将我们创建的 app_state (包含数据库连接池) 注入到所有 handler 中
        .with_state(app_state)
        // 将 CORS 配置作为一个中间件层应用到所有路由上
        .layer(cors);

    // 定义服务器监听的地址和端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // 打印日志，告知服务器正在监听
    println!("listening on {}", addr);
    // 绑定到指定地址，创建一个 TCP 监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    // 启动服务器，开始接受请求
    axum::serve(listener, app).await?;

    // 如果程序正常结束，返回 Ok
    Ok(())
}
```

### 4. `src/database.rs` (数据库模块)

这个模块专门负责数据库的连接和初始化。

```rust
// 引入 sqlx 的 SQLite 连接池和配置选项
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// 这个函数创建一个到 SQLite 数据库的连接池，并运行“迁移”
pub async fn init_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // 创建一个新的连接池配置
    let pool = SqlitePoolOptions::new()
        .max_connections(5) // 设置最大并发连接数为 5
        .connect(db_url) // 使用传入的数据库 URL 进行连接
        .await?; // 等待连接完成

    // 运行数据库迁移。这里我们直接用一个 SQL 查询字符串
    // `CREATE TABLE IF NOT EXISTS` 确保这个表只在它不存在的时候被创建
    sqlx::query(
        // 定义 posts 表的结构
        // id: 整数，主键，自动增长
        // title: 文本，不能为空
        // content: 文本，不能为空
        "CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, content TEXT NOT NULL)"
    )
    .execute(&pool) // 在连接池上执行这个查询
    .await?; // 等待执行完成

    // 返回创建好的连接池
    Ok(pool)
}
```

### 5. `src/models.rs` (数据模型模块)

这个模块定义了我们应用中核心的数据结构。

```rust
// 引入 serde 的序列化和反序列化功能
use serde::{Deserialize, Serialize};

/// 代表数据库中的一篇博客文章
// `#[derive(...)]` 是一个宏，可以自动为我们的 struct 实现一些通用的功能 (trait)
#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i64, // 对应数据库的 INTEGER
    pub title: String, // 对应数据库的 TEXT
    pub content: String, // 对应数据库的 TEXT
}
// - `Serialize`: 允许将这个 struct 转换成 JSON 字符串发送给前端
// - `Deserialize`: 允许从前端的 JSON 请求体中解析出这个 struct
// - `Clone`: 允许我们创建这个 struct 的副本
// - `Debug`: 允许我们使用 `println!("{:?}", post)` 来打印调试信息
// - `sqlx::FromRow`: 允许 sqlx 将数据库查询的一行结果直接映射成这个 Post struct

/// 代表创建一个新文章时，从前端接收到的请求体结构
#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}
```

### 6. `src/handlers.rs` (请求处理模块)

这个模块包含了所有 API 端点的具体实现逻辑。

```rust
// 引入 axum 的提取器 (Extractor)，用于从请求中获取数据
use axum::{
    extract::{Path, State}, // `Path` 用于获取路径参数 (如 /posts/1)，`State` 用于获取共享状态
    http::StatusCode, // 提供了标准的 HTTP 状态码，如 200 OK, 404 NOT FOUND
    response::Json, // 一个响应类型，可以将数据自动序列化为 JSON 并设置正确的 Content-Type 头
};
// 引入我们自己定义的模型
use crate::models::{CreatePost, Post};
// 引入 sqlx 的数据库连接池类型
use sqlx::SqlitePool;

// 定义应用共享状态的结构体
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

// --- 路由处理函数 ---

// 获取所有文章
pub async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<Post>>, StatusCode> {
    // `sqlx::query_as` 可以将查询结果自动映射到指定的类型 (这里是 Post)
    sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts ORDER BY id DESC")
        .fetch_all(&state.pool) // 从连接池获取所有结果
        .await // 等待数据库操作完成
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts)) // 如果成功，将文章列表包裹在 Json 中返回
}

// 创建新文章
pub async fn create_post(
    State(state): State<AppState>,
    Json(input): Json<CreatePost>, // 从请求体中提取 JSON 数据并反序列化为 CreatePost
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    // `?` 用于参数化查询，可以防止 SQL 注入
    let result = sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?)")
        .bind(&input.title) // 将 title 绑定到第一个 `?`
        .bind(&input.content) // 将 content 绑定到第二个 `?`
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid(); // 获取新插入行的 ID
    // 创建一个新的 Post 实例用于返回给前端
    let post = Post {
        id,
        title: input.title,
        content: input.content,
    };

    // 返回 201 CREATED 状态码和新创建的文章数据
    Ok((StatusCode::CREATED, Json(post)))
}

// 获取单篇文章
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i64>, // 从路径中提取 id
) -> Result<Json<Post>, StatusCode> {
    sqlx::query_as::<_, Post>("SELECT id, title, content FROM posts WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool) // 只获取一行结果
        .await
        .map(Json) // 如果成功，将结果包裹在 Json 中
        .map_err(|e| match e { // 对错误进行匹配
            sqlx::Error::RowNotFound => StatusCode::NOT_FOUND, // 如果没找到，返回 404
            _ => StatusCode::INTERNAL_SERVER_ERROR, // 其他错误，返回 500
        })
}

// 更新文章
pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<CreatePost>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE posts SET title = ?, content = ? WHERE id = ?")
        .bind(&input.title)
        .bind(&input.content)
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 { // 检查是否有行被更新
        Err(StatusCode::NOT_FOUND) // 如果没有行被影响，说明文章不存在，返回 404
    } else {
        Ok(StatusCode::OK) // 否则返回 200 OK
    }
}

// 删除文章
pub async fn delete_post(State(state): State<AppState>, Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT) // 成功删除，返回 204 NO CONTENT
    }
}
```
