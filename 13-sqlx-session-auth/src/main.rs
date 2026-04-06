// use std::process::Output;

// use async_trait::async_trait;
// use axum::{
//     Router,
//     http::StatusCode,
//     middleware::from_fn,
//     response::IntoResponse,
//     routing::{get, post},
// };
// use axum_session::{Key, Session, SessionConfig, SessionLayer, SessionStore};
// use axum_session_auth::{AuthConfig, AuthSessionLayer, Authentication};
// use axum_session_sqlx::SessionSqlitePool;
// use serde::Deserialize;
// use sqlx::{Pool, Sqlite, SqlitePool, prelude::FromRow, query, query_as};
// use tokio::net::TcpListener;

use anyhow::Ok;
use async_trait::async_trait;
use axum::{
    Extension, Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{Next, from_fn},
    response::IntoResponse,
    routing::{get, post},
};
use axum_session::{Key, SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use axum_session_sqlx::SessionSqlitePool;
use serde::Deserialize;
use sqlx::{Executor, Pool, Sqlite, SqlitePool, prelude::FromRow};

#[tokio::main]
async fn main() {
    let pool = db().await;
    let session_store = session(pool.clone()).await;
    let app = app(pool, session_store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

async fn db() -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
        .await
        .unwrap();

    pool.execute(
        "
    CREATE TABLE IF NOT EXISTS user (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      username TEXT,
      password TEXT
    )
  ",
    )
    .await
    .unwrap();

    let rows: Vec<UserSql> = sqlx::query_as("SELECT * FROM user WHERE id = ?1")
        .bind(&1)
        .fetch_all(&pool)
        .await
        .unwrap();

    if rows.len() == 0 {
        sqlx::query("INSERT INTO user (username, password) VALUES (?1, ?2)")
            .bind(&"guest")
            .bind(&"guest")
            .execute(&pool)
            .await
            .unwrap();
    };

    pool
}

async fn session(pool: Pool<Sqlite>) -> SessionStore<SessionSqlitePool> {
    let config = SessionConfig::default()
        .with_table_name("session_table")
        .with_key(Key::generate());

    let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config)
        .await
        .unwrap();

    session_store
}

fn app(pool: Pool<Sqlite>, session_store: SessionStore<SessionSqlitePool>) -> Router {
    let config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(log_out))
        .route("/protected", get(protected).route_layer(from_fn(auth)))
        .layer(
            AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
                .with_config(config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(pool)
}

async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
}

async fn login(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
    State(pool): State<Pool<Sqlite>>,
    Json(user): Json<UserRequest>,
) -> impl IntoResponse {
}

async fn log_out(auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>) -> impl IntoResponse {
    auth.logout_user();
    (StatusCode::OK, "Log out successful!").into_response()
}

async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    let msg = format!("Hello , {} , your id is {}", user.username, user.id);
    (StatusCode::OK, msg).into_response()
}

async fn auth(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
}

#[derive(Deserialize)]
struct UserRequest {
    username: String,
    password: String,
}

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}

#[async_trait]
impl Authentication<User, i64, SqlitePool> for User {
    async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
        if userid == 1 {
            Ok(User {
                id: userid,
                anonymous: true,
                username: "guest".to_string(),
            })
        } else {
            let user: UserSql = sqlx::query_as("SELECT * FROM user WHERE id = ?1")
                .bind(&userid)
                .fetch_one(pool.unwrap())
                .await
                .unwrap();
            Ok(User {
                id: user.id as i64,
                anonymous: false,
                username: user.username,
            })
        }
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }
}

#[derive(FromRow)]
struct UserSql {
    id: i32,
    username: String,
    password: String,
}

//--------------------------------------------------------------------------------------------
// use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
// use axum_session::{Key, Session, SessionConfig, SessionLayer, SessionStore};
// use axum_session_sqlx::SessionSqlitePool;
// use sqlx::{Pool, Sqlite, SqlitePool};
// use tokio::net::TcpListener;
// #[tokio::main]
// async fn main() {
//     let pool = db().await;
//     let session_store = session(pool).await;
//     let app = app(session_store);

//     let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("Listening on http://127.0.0.1:3000");

//     axum::serve(listener, app.into_make_service())
//         .await
//         .unwrap()
// }

// fn app(session_store: SessionStore<SessionSqlitePool>) -> Router {
//     Router::new()
//         .route("/", get(hello))
//         .route("/world", get(world))
//         .layer(SessionLayer::new(session_store))
// }

// async fn db() -> Pool<Sqlite> {
//     SqlitePool::connect("sqlite://db.sqlite").await.unwrap()
// }

// async fn session(pool: Pool<Sqlite>) -> SessionStore<SessionSqlitePool> {
//     let config = SessionConfig::default()
//         .with_table_name("session_table")
//         .with_key(Key::generate());
//     SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config)
//         .await
//         .unwrap()
// }

// async fn hello(session: Session<SessionSqlitePool>) -> impl IntoResponse {
//     session.set("message", "Hello, world! cccccccccccccccccccccc");
//     (StatusCode::OK, "Hello, world!").into_response()
// }

// async fn world(session: Session<SessionSqlitePool>) -> impl IntoResponse {
//     let msg: String = session.get("message").unwrap();
//     (StatusCode::OK, msg).into_response()
// }
