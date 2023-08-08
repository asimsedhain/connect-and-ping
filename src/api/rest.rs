use std::borrow::Cow;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::app::App;

#[derive(Serialize)]
pub struct ListClientResponse {
    clients: Vec<String>,
}

pub async fn get_clients(State(app): State<App>) -> Json<ListClientResponse> {
    Json(ListClientResponse {
        clients: app.list_clients().await,
    })
}

pub async fn ping_client(Path(client_id): Path<String>, State(app): State<App>) {
    app.send_message(&client_id, &Cow::from("Ping")).await;
}
