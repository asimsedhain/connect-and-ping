use crate::app::App;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures::StreamExt;
use futures_util::SinkExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SocketParams {
    user_id: String,
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<SocketParams>,
    State(app): State<App>,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, params.user_id, app))
}

// sockets is closed when both the ends of the sockets are dropped
async fn handle_socket(socket: WebSocket, user_id: String, app: App) {
    let (sender, mut receiver) = socket.split();

    tracing::debug!("Adding user: {} to pool", &user_id);

    if app.add_client(&user_id, sender).await.is_err() {
        tracing::debug!("Could not add user: {user_id} to app");
        return;
    }

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }
    let client = app.remove_client(&user_id).await;
    if let Some(mut client) = client {
        let _ = client.close().await;
    }
}
