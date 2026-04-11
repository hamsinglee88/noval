use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
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
                Message::Text(text) => {
                    // 处理消息并推送进度
                    let response = format!("{{\"type\":\"progress\",\"data\":\"处理中...\"}}");
                    let _ = socket.send(Message::Text(response.into())).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}
