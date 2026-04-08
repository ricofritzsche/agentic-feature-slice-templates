use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenAccountRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub residential_address: ResidentialAddressRequest,
    pub government_id: String,
}

#[derive(Deserialize)]
pub struct ResidentialAddressRequest {
    pub street_line_1: String,
    pub street_line_2: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub country_code: String,
}

pub struct ValidOpenAccountRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub residential_address: ValidResidentialAddress,
    pub government_id: String,
}

pub struct ValidResidentialAddress {
    pub street_line_1: String,
    pub street_line_2: String,
    pub postal_code: String,
    pub city: String,
    pub country_code: String,
}

#[derive(Clone, Copy)]
pub struct ValidationError {
    pub status: u16,
    pub code: &'static str,
    pub message: &'static str,
}

impl OpenAccountRequest {
    pub fn validate(self) -> Result<ValidOpenAccountRequest, ValidationError> {
        Ok(ValidOpenAccountRequest {
            first_name: validate_required(
                self.first_name,
                100,
                "FIRST_NAME_REQUIRED",
                "first_name is required.",
            )?,
            last_name: validate_required(
                self.last_name,
                100,
                "LAST_NAME_REQUIRED",
                "last_name is required.",
            )?,
            date_of_birth: parse_date_of_birth(&self.date_of_birth)?,
            residential_address: self.residential_address.validate()?,
            government_id: validate_required(
                self.government_id,
                64,
                "GOVERNMENT_ID_REQUIRED",
                "government_id is required.",
            )?,
        })
    }
}

impl ResidentialAddressRequest {
    fn validate(self) -> Result<ValidResidentialAddress, ValidationError> {
        Ok(ValidResidentialAddress {
            street_line_1: validate_required(
                self.street_line_1,
                200,
                "ADDRESS_REQUIRED",
                "residential_address is incomplete.",
            )?,
            street_line_2: validate_optional(
                self.street_line_2,
                200,
                "ADDRESS_REQUIRED",
                "residential_address is incomplete.",
            )?,
            postal_code: validate_required(
                self.postal_code,
                32,
                "ADDRESS_REQUIRED",
                "residential_address is incomplete.",
            )?,
            city: validate_required(
                self.city,
                120,
                "ADDRESS_REQUIRED",
                "residential_address is incomplete.",
            )?,
            country_code: validate_country_code(self.country_code)?,
        })
    }
}

fn validate_required(
    value: String,
    max_len: usize,
    code: &'static str,
    message: &'static str,
) -> Result<String, ValidationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() || trimmed.chars().count() > max_len {
        return Err(unprocessable(code, message));
    }

    Ok(trimmed.to_owned())
}

fn validate_optional(
    value: Option<String>,
    max_len: usize,
    code: &'static str,
    message: &'static str,
) -> Result<String, ValidationError> {
    let trimmed = value.unwrap_or_default().trim().to_owned();

    if trimmed.chars().count() > max_len {
        return Err(unprocessable(code, message));
    }

    Ok(trimmed)
}

fn validate_country_code(value: String) -> Result<String, ValidationError> {
    let trimmed = value.trim();

    if trimmed.len() != 2
        || !trimmed
            .chars()
            .all(|character| character.is_ascii_alphabetic())
    {
        return Err(unprocessable(
            "COUNTRY_CODE_INVALID",
            "country_code must contain exactly two letters.",
        ));
    }

    Ok(trimmed.to_ascii_uppercase())
}

fn parse_date_of_birth(value: &str) -> Result<NaiveDate, ValidationError> {
    let date_of_birth = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| {
        bad_request(
            "INVALID_DATE_FORMAT",
            "date_of_birth must use the format YYYY-MM-DD.",
        )
    })?;
    let today = Utc::now().date_naive();

    if date_of_birth >= today {
        return Err(unprocessable(
            "DATE_OF_BIRTH_INVALID",
            "date_of_birth must be in the past.",
        ));
    }

    if !is_adult(date_of_birth, today) {
        return Err(unprocessable(
            "APPLICANT_MUST_BE_ADULT",
            "Applicant must be at least 18 years old.",
        ));
    }

    Ok(date_of_birth)
}

fn is_adult(date_of_birth: NaiveDate, today: NaiveDate) -> bool {
    let mut age = today.year() - date_of_birth.year();

    if (today.month(), today.day()) < (date_of_birth.month(), date_of_birth.day()) {
        age -= 1;
    }

    age >= 18
}

pub fn invalid_json_error() -> ValidationError {
    bad_request("INVALID_JSON", "The request body is not valid JSON.")
}

pub fn invalid_field_type_error() -> ValidationError {
    bad_request(
        "INVALID_FIELD_TYPE",
        "One or more request fields have the wrong type.",
    )
}

fn bad_request(code: &'static str, message: &'static str) -> ValidationError {
    ValidationError {
        status: 400,
        code,
        message,
    }
}

fn unprocessable(code: &'static str, message: &'static str) -> ValidationError {
    ValidationError {
        status: 422,
        code,
        message,
    }
}
