use std::borrow::Cow;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::app::App;

pub async fn get_clients(State(app): State<App>) -> Json<Vec<String>> {
    Json(app.list_clients())
}

pub async fn ping_client(Path(client_id): Path<String>, State(app): State<App>) {
    app.send_message(&client_id, &Cow::from("Ping"));
}
