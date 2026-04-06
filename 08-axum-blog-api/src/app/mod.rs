use crate::app::router::create_router;
use crate::app::state::{AppState, SharedState};
use crate::config::{AppConfig, connect_db};
use tokio::net::TcpListener;

pub mod router;
pub mod state;

pub async fn create_app() {
    let app_config = AppConfig::from_env().expect("Failed to load configuration from environment");

    let db_pool = connect_db(&app_config.database_url)
        .await
        .expect("Failed to connect to the database");

    let app_state = SharedState::new(AppState::new(db_pool));
    let app = create_router(app_state);

    let server_address = format!("127.0.0.1:{}", app_config.server_port);
    let listener = TcpListener::bind(&server_address).await.unwrap();

    println!("Server started successfully on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}
