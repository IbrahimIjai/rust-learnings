use axum::{
    Json, Router,
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode, response},
    middleware::{Next, from_fn},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let app: Router = app();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on {}!", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let shared_states = Arc::new(Mutex::new(Person {
        name: "Alice".to_string(),
        age: 30,
    }));
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(hello_yoo))
        .route("/hello", get(hello_handler))
        .with_state(shared_states)
        .fallback(not_found)
        .layer(from_fn(mid_ware))
}

async fn mid_ware(req: Request, next: Next) -> impl IntoResponse {
    println!("This is a middleware");
    let response = next.run(req).await;
    response
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
async fn hello_handler(State(person): State<Arc<Mutex<Person>>>) -> impl IntoResponse {
    let mut person = person.lock().await;
    println!("{:?}", person);
    (*person).name = "Bob".to_string();
    (*person).age = 25;
    println!("{:?}", person);
    (StatusCode::OK, "Hello, World!").into_response()
}

async fn hello_yoo(State(person): State<Arc<Mutex<Person>>>) -> String {
    println!("{:?}", person);
    "Hello, World!".to_string()
}
