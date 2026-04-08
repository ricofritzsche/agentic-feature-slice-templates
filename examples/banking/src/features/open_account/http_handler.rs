use super::{
    append_fact,
    request::{OpenAccountRequest, ValidationError, invalid_field_type_error, invalid_json_error},
    response,
};
use crate::AppState;
use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::{error, info, warn};

pub async fn handle(State(app_state): State<AppState>, body: Bytes) -> Response {
    let request = match parse_request(&body) {
        Ok(request) => request,
        Err(error) => return validation_failure(error),
    };

    let request = match request.validate() {
        Ok(request) => request,
        Err(error) => return validation_failure(error),
    };

    info!(
        feature = "open_account",
        result = "accepted",
        "request accepted"
    );

    match append_fact::append_fact(&app_state, request).await {
        Ok(opened_account) => {
            info!(
                feature = "open_account",
                result = "created",
                account_id = %opened_account.account_id,
                "account created"
            );
            response::created(opened_account).into_response()
        }
        Err(error) => {
            error!(
                feature = "open_account",
                result = "error",
                error_code = "INTERNAL_ERROR",
                %error,
                "internal failure"
            );
            response::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "The account could not be opened due to an internal error.",
            )
            .into_response()
        }
    }
}

fn parse_request(body: &[u8]) -> Result<OpenAccountRequest, ValidationError> {
    serde_json::from_slice(body).map_err(|error| {
        if error.is_data() {
            invalid_field_type_error()
        } else {
            invalid_json_error()
        }
    })
}

fn validation_failure(error: ValidationError) -> Response {
    warn!(
        feature = "open_account",
        result = "validation_failed",
        error_code = error.code,
        "request rejected"
    );

    let status = StatusCode::from_u16(error.status).expect("validation status should be valid");
    response::error(status, error.code, error.message).into_response()
}
