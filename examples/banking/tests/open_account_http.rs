use banking_feature_slices_example::{AppState, run};
use chrono::{DateTime, Utc};
use factstore::{EventQuery, EventStore};
use factstore_memory::MemoryStore;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex, task::JoinHandle};

#[tokio::test]
async fn open_account_happy_path_returns_full_response_shape() {
    let memory_store = Arc::new(Mutex::new(MemoryStore::new()));
    let (address, handle) = spawn_app(AppState::from_memory_store(Arc::clone(&memory_store))).await;

    let response = reqwest::Client::new()
        .post(format!("http://{address}/accounts"))
        .json(&valid_request())
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let body: Value = response.json().await.expect("body should be valid json");
    let account_id = body["account"]["account_id"]
        .as_str()
        .expect("account_id should be present")
        .to_owned();
    let created_at = body["account"]["created_at"]
        .as_str()
        .expect("created_at should be present")
        .to_owned();

    assert!(account_id.starts_with("acc_"));
    DateTime::parse_from_rfc3339(&created_at).expect("created_at should be rfc3339");
    assert_eq!(
        body,
        json!({
            "account": {
                "account_id": account_id,
                "status": "open",
                "currency": "EUR",
                "created_at": created_at,
                "holder": {
                    "first_name": "Anna",
                    "last_name": "Schneider",
                    "date_of_birth": "1992-04-18"
                }
            }
        })
    );

    let query_result = memory_store
        .lock()
        .await
        .query(&EventQuery::for_event_types(["account_opened"]))
        .expect("query should succeed");
    assert_eq!(query_result.event_records.len(), 1);

    handle.abort();
}

#[tokio::test]
async fn blank_first_name_returns_first_name_required() {
    assert_error_response(
        valid_request_with("first_name", json!("   ")),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "FIRST_NAME_REQUIRED",
        "first_name is required.",
    )
    .await;
}

#[tokio::test]
async fn blank_last_name_returns_last_name_required() {
    assert_error_response(
        valid_request_with("last_name", json!("   ")),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "LAST_NAME_REQUIRED",
        "last_name is required.",
    )
    .await;
}

#[tokio::test]
async fn malformed_date_of_birth_returns_invalid_date_format() {
    assert_error_response(
        valid_request_with("date_of_birth", json!("1992/04/18")),
        reqwest::StatusCode::BAD_REQUEST,
        "INVALID_DATE_FORMAT",
        "date_of_birth must use the format YYYY-MM-DD.",
    )
    .await;
}

#[tokio::test]
async fn non_past_date_of_birth_returns_date_of_birth_invalid() {
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    assert_error_response(
        valid_request_with("date_of_birth", json!(today)),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "DATE_OF_BIRTH_INVALID",
        "date_of_birth must be in the past.",
    )
    .await;
}

#[tokio::test]
async fn younger_than_18_returns_applicant_must_be_adult() {
    assert_error_response(
        valid_request_with("date_of_birth", json!("2010-04-18")),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "APPLICANT_MUST_BE_ADULT",
        "Applicant must be at least 18 years old.",
    )
    .await;
}

#[tokio::test]
async fn blank_address_field_returns_address_required() {
    let payload = json!({
        "first_name": "Anna",
        "last_name": "Schneider",
        "date_of_birth": "1992-04-18",
        "residential_address": {
            "street_line_1": "   ",
            "street_line_2": "",
            "postal_code": "10435",
            "city": "Berlin",
            "country_code": "DE"
        },
        "government_id": "D1234567X"
    });

    assert_error_response(
        payload,
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "ADDRESS_REQUIRED",
        "residential_address is incomplete.",
    )
    .await;
}

#[tokio::test]
async fn invalid_country_code_returns_country_code_invalid() {
    let payload = json!({
        "first_name": "Anna",
        "last_name": "Schneider",
        "date_of_birth": "1992-04-18",
        "residential_address": {
            "street_line_1": "Kastanienallee 15",
            "street_line_2": "",
            "postal_code": "10435",
            "city": "Berlin",
            "country_code": "D3"
        },
        "government_id": "D1234567X"
    });

    assert_error_response(
        payload,
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "COUNTRY_CODE_INVALID",
        "country_code must contain exactly two letters.",
    )
    .await;
}

#[tokio::test]
async fn blank_government_id_returns_government_id_required() {
    assert_error_response(
        valid_request_with("government_id", json!("   ")),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "GOVERNMENT_ID_REQUIRED",
        "government_id is required.",
    )
    .await;
}

#[tokio::test]
async fn malformed_json_returns_invalid_json() {
    let (address, handle) = spawn_app(AppState::from_memory_store(Arc::new(Mutex::new(
        MemoryStore::new(),
    ))))
    .await;

    let response = reqwest::Client::new()
        .post(format!("http://{address}/accounts"))
        .header("content-type", "application/json")
        .body("{\"first_name\":")
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    assert_eq!(
        response.json::<Value>().await.expect("body should be json"),
        json!({
            "error": {
                "code": "INVALID_JSON",
                "message": "The request body is not valid JSON."
            }
        })
    );

    handle.abort();
}

#[tokio::test]
async fn wrong_field_type_returns_invalid_field_type() {
    let payload = json!({
        "first_name": "Anna",
        "last_name": "Schneider",
        "date_of_birth": "1992-04-18",
        "residential_address": {
            "street_line_1": "Kastanienallee 15",
            "street_line_2": "",
            "postal_code": 10435,
            "city": "Berlin",
            "country_code": "DE"
        },
        "government_id": "D1234567X"
    });

    assert_error_response(
        payload,
        reqwest::StatusCode::BAD_REQUEST,
        "INVALID_FIELD_TYPE",
        "One or more request fields have the wrong type.",
    )
    .await;
}

#[tokio::test]
async fn identical_requests_create_two_accounts() {
    let memory_store = Arc::new(Mutex::new(MemoryStore::new()));
    let (address, handle) = spawn_app(AppState::from_memory_store(Arc::clone(&memory_store))).await;
    let client = reqwest::Client::new();
    let payload = valid_request();

    let first_response = client
        .post(format!("http://{address}/accounts"))
        .json(&payload)
        .send()
        .await
        .expect("first request should succeed");
    let second_response = client
        .post(format!("http://{address}/accounts"))
        .json(&payload)
        .send()
        .await
        .expect("second request should succeed");

    assert_eq!(first_response.status(), reqwest::StatusCode::CREATED);
    assert_eq!(second_response.status(), reqwest::StatusCode::CREATED);

    let first_body: Value = first_response
        .json()
        .await
        .expect("first body should be json");
    let second_body: Value = second_response
        .json()
        .await
        .expect("second body should be json");

    assert_ne!(
        first_body["account"]["account_id"],
        second_body["account"]["account_id"]
    );

    let query_result = memory_store
        .lock()
        .await
        .query(&EventQuery::for_event_types(["account_opened"]))
        .expect("query should succeed");
    assert_eq!(query_result.event_records.len(), 2);

    handle.abort();
}

#[tokio::test]
async fn country_code_is_normalized_to_uppercase() {
    let memory_store = Arc::new(Mutex::new(MemoryStore::new()));
    let (address, handle) = spawn_app(AppState::from_memory_store(Arc::clone(&memory_store))).await;

    let response = reqwest::Client::new()
        .post(format!("http://{address}/accounts"))
        .json(&valid_request_with_country_code("de"))
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let query_result = memory_store
        .lock()
        .await
        .query(&EventQuery::for_event_types(["account_opened"]))
        .expect("query should succeed");

    assert_eq!(
        query_result.event_records[0].payload["residential_address"]["country_code"],
        "DE"
    );

    handle.abort();
}

#[tokio::test]
async fn simulated_persistence_failure_returns_internal_error() {
    let (address, handle) = spawn_app(AppState::with_failing_append()).await;

    let response = reqwest::Client::new()
        .post(format!("http://{address}/accounts"))
        .json(&valid_request())
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(
        response.status(),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        response.json::<Value>().await.expect("body should be json"),
        json!({
            "error": {
                "code": "INTERNAL_ERROR",
                "message": "The account could not be opened due to an internal error."
            }
        })
    );

    handle.abort();
}

async fn assert_error_response(
    payload: Value,
    expected_status: reqwest::StatusCode,
    expected_code: &str,
    expected_message: &str,
) {
    let (address, handle) = spawn_app(AppState::from_memory_store(Arc::new(Mutex::new(
        MemoryStore::new(),
    ))))
    .await;

    let response = reqwest::Client::new()
        .post(format!("http://{address}/accounts"))
        .json(&payload)
        .send()
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), expected_status);
    assert_eq!(
        response.json::<Value>().await.expect("body should be json"),
        json!({
            "error": {
                "code": expected_code,
                "message": expected_message
            }
        })
    );

    handle.abort();
}

async fn spawn_app(app_state: AppState) -> (std::net::SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("ephemeral listener should bind");
    let address = listener
        .local_addr()
        .expect("local address should be available");
    let app_state_for_server = app_state.clone();

    let handle = tokio::spawn(async move {
        run(listener, app_state_for_server)
            .await
            .expect("server should run");
    });

    (address, handle)
}

fn valid_request() -> Value {
    valid_request_with_country_code("DE")
}

fn valid_request_with_country_code(country_code: &str) -> Value {
    json!({
        "first_name": "Anna",
        "last_name": "Schneider",
        "date_of_birth": "1992-04-18",
        "residential_address": {
            "street_line_1": "Kastanienallee 15",
            "street_line_2": "",
            "postal_code": "10435",
            "city": "Berlin",
            "country_code": country_code
        },
        "government_id": "D1234567X"
    })
}

fn valid_request_with(field: &str, value: Value) -> Value {
    let mut payload = valid_request();
    payload[field] = value;
    payload
}
