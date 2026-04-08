use crate::features::open_account::append_fact::OpenedAccount;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub fn created(opened_account: OpenedAccount) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(OpenAccountResponse {
            account: AccountResponse {
                account_id: opened_account.account_id,
                status: opened_account.status,
                currency: opened_account.currency,
                created_at: opened_account.created_at,
                holder: HolderResponse {
                    first_name: opened_account.holder.first_name,
                    last_name: opened_account.holder.last_name,
                    date_of_birth: opened_account.holder.date_of_birth,
                },
            },
        }),
    )
}

pub fn error(status: StatusCode, code: &'static str, message: &'static str) -> impl IntoResponse {
    (
        status,
        Json(ErrorResponse {
            error: ErrorBody { code, message },
        }),
    )
}

#[derive(Serialize)]
struct OpenAccountResponse {
    account: AccountResponse,
}

#[derive(Serialize)]
struct AccountResponse {
    account_id: String,
    status: &'static str,
    currency: &'static str,
    created_at: String,
    holder: HolderResponse,
}

#[derive(Serialize)]
struct HolderResponse {
    first_name: String,
    last_name: String,
    date_of_birth: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
}
