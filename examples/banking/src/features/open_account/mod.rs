mod append_fact;
mod http_handler;
mod request;
mod response;

use crate::AppState;
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/accounts", post(http_handler::handle))
}
