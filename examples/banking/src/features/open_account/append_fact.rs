use crate::{AppState, Store};
use chrono::{SecondsFormat, Utc};
use factstore::{EventStore, EventStoreError, NewEvent};
use serde_json::json;
use ulid::Ulid;

use super::request::ValidOpenAccountRequest;

const ACCOUNT_OPENED_EVENT_TYPE: &str = "account_opened";

pub struct OpenedAccount {
    pub account_id: String,
    pub status: &'static str,
    pub currency: &'static str,
    pub created_at: String,
    pub holder: OpenedAccountHolder,
}

pub struct OpenedAccountHolder {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

pub async fn append_fact(
    app_state: &AppState,
    request: ValidOpenAccountRequest,
) -> Result<OpenedAccount, EventStoreError> {
    let account_id = format!("acc_{}", Ulid::new());
    let created_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let holder_date_of_birth = request.date_of_birth.format("%Y-%m-%d").to_string();
    let opened_account = OpenedAccount {
        account_id: account_id.clone(),
        status: "open",
        currency: "EUR",
        created_at: created_at.clone(),
        holder: OpenedAccountHolder {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            date_of_birth: holder_date_of_birth.clone(),
        },
    };

    let new_event = NewEvent::new(
        ACCOUNT_OPENED_EVENT_TYPE,
        json!({
            "account_id": account_id,
            "status": opened_account.status,
            "currency": opened_account.currency,
            "created_at": created_at,
            "holder": {
                "first_name": request.first_name,
                "last_name": request.last_name,
                "date_of_birth": holder_date_of_birth,
            },
            "residential_address": {
                "street_line_1": request.residential_address.street_line_1,
                "street_line_2": request.residential_address.street_line_2,
                "postal_code": request.residential_address.postal_code,
                "city": request.residential_address.city,
                "country_code": request.residential_address.country_code,
            },
            "government_id": request.government_id,
            "initial_balance": 0
        }),
    );

    match app_state.store() {
        Store::Sqlite(store) => {
            store.append(vec![new_event])?;
        }
        Store::Memory(store) => {
            store.lock().await.append(vec![new_event])?;
        }
        Store::FailAppend => {
            return Err(EventStoreError::BackendFailure {
                message: "simulated append failure".to_owned(),
            });
        }
    }

    Ok(opened_account)
}
