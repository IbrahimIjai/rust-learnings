use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Pool, Sqlite, sqlite};

#[derive(Serialize, Deserialize, FromRow)]
struct Person {
    id: Option<i32>,
    name: String,
}

#[tokio::main]
async fn main() {
    let pool = db().await;

    let app = app(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://{}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn db() -> Pool<Sqlite> {
    let opt = sqlite::SqliteConnectOptions::new()
        .filename("test.db")
        .create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(opt).await.unwrap();

    pool.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT
        )",
    )
    .await
    .unwrap();

    pool
}

fn app(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/list", get(get_person_list))
        .route("/add_person", post(add_new_person))
        .route("/person/{id}", get(get_single_person))
        .route("/remove_person/{id}", delete(remove_person))
        .route("/update_person/{id}", patch(update_person))
        .with_state(pool)
}

async fn get_person_list(State(pool): State<Pool<Sqlite>>) -> Json<Vec<Person>> {
    let persons = sqlx::query_as::<_, Person>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(persons)
}

async fn add_new_person(
    State(pool): State<Pool<Sqlite>>,
    Json(person): Json<Person>,
) -> &'static str {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(&person.name)
        .execute(&pool)
        .await
        .unwrap();
    "Person added successfully"
}

async fn get_single_person(State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> Json<Person> {
    let person = sqlx::query_as::<_, Person>("SELECT id, name FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(person)
}

async fn remove_person(State(pool): State<Pool<Sqlite>>, Path(id): Path<i32>) -> &'static str {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    "Person removed successfully"
}

async fn update_person(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i32>,
    Json(person): Json<Person>,
) -> &'static str {
    sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(&person.name)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    "Person updated successfully"
}

//9:43
