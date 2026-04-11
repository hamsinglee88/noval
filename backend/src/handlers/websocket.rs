use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{StreamExt, SinkExt};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                axum::extract::ws::Message::Text(text) => {
                    // 处理消息并推送进度
                    let response = format!("{{\"type\":\"progress\",\"data\":\"处理中...\"}}");
                    let _ = socket.send(axum::extract::ws::Message::Text(response)).await;
                }
                axum::extract::ws::Message::Close(_) => break,
                _ => {}
            }
        }
    }
}
