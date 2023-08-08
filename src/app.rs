use anyhow::Result;
use futures::stream::SplitSink;
use futures::SinkExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::extract::ws::{Message, WebSocket};

type Sink = SplitSink<WebSocket, Message>;
type InnerMap = HashMap<String, Sink>;

#[derive(Clone)]
pub struct App {
    connected_clients: Arc<Mutex<InnerMap>>,
}

impl App {
    fn new(connected_clients: InnerMap) -> Self {
        let connected_clients = Arc::new(Mutex::new(connected_clients));
        Self { connected_clients }
    }
}

impl Default for App {
    fn default() -> Self {
        App::new(HashMap::new())
    }
}

impl App {
    pub async fn add_client<N: Into<String>>(&self, name: N, sink: Sink) -> Result<()> {
        let mut map = self.connected_clients.lock().await;
        map.insert(name.into(), sink);

        Ok(())
    }
    pub async fn list_clients(&self) -> Vec<String> {
        let map = self.connected_clients.lock().await;

        map.keys().cloned().collect()
    }
    pub async fn remove_client(&self, name: &str) -> Result<()> {
        let mut map = self.connected_clients.lock().await;
        map.remove(name);

        Ok(())
    }

    pub async fn send_message(&self, client_id: &str, message: &str) {
        let mut client_map = self.connected_clients.lock().await;
        let Some(client) = client_map.get_mut(client_id) else{
        return;};

        let _ = client.send(Message::Text(message.to_string())).await;
    }
}
