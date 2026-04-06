mod app;

mod author;
mod config;
mod error;
mod post;


use app::create_app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    create_app().await;
}
