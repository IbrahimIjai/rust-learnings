use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{Sender, channel};

#[tokio::main]
async fn main() {
    let (tx, _) = channel(100);
    let app = app(tx);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app(tx: Sender<String>) -> Router {
    Router::new()
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(tx)
}

#[axum::debug_handler]
async fn ws_handler(State(tx): State<Sender<String>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(tx, socket))
}

async fn handle_ws(tx: Sender<String>, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::from(msg)).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(content) => {
                let _ = tx.send(content.to_string());
            }
            _ => (),
        }
    }
    // while let Some(Ok(msg)) = socket.recv().await {
    //     if let ws::Message::Text(text) = msg {
    //         tx.send(text).unwrap();
    //     }
    // }
}
