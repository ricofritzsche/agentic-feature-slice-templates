pub mod open_account;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    open_account::router()
}
