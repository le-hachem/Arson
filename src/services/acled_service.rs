use crate::config::ACLED_BASE_URL;
use crate::errors::{AppError, AppResult, IntoAppError};
use crate::types::{AcledEvent, AcledParams, AcledResponse, UserData};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub struct AcledService;

impl AcledService {
    pub fn fetch_events(
        user_data: &UserData,
        params: &AcledParams,
        on_success: Callback<Vec<AcledEvent>>,
        on_error: Callback<AppError>,
    ) {
        // Validate parameters before making request
        if !params.is_valid() {
            on_error.emit(AppError::validation(
                "parameters",
                "Invalid API parameters provided",
            ));
            return;
        }

        if !user_data.is_valid() {
            on_error.emit(AppError::validation(
                "credentials",
                "Invalid user credentials provided",
            ));
            return;
        }

        let user_data = user_data.clone();
        let params = params.clone();

        spawn_local(async move {
            match Self::fetch_events_async(&user_data, &params).await {
                Ok(events) => on_success.emit(events),
                Err(error) => on_error.emit(error),
            }
        });
    }

    async fn fetch_events_async(
        user_data: &UserData,
        params: &AcledParams,
    ) -> AppResult<Vec<AcledEvent>> {
        let url = Self::build_url(user_data, params);

        let response = Request::get(&url).send().await.into_app_error()?;

        if !response.ok() {
            return Err(AppError::api(
                response.status().into(),
                format!("HTTP {}: {}", response.status(), response.status_text()),
            ));
        }

        let text = response.text().await.into_app_error()?;

        // Try to parse as AcledResponse first
        if let Ok(api_response) = serde_json::from_str::<AcledResponse>(&text) {
            if let Some(error_msg) = api_response.get_error_message() {
                return Err(AppError::api(api_response.status.unwrap_or(400), error_msg));
            }

            if api_response.is_success() {
                return Ok(api_response.get_events());
            }
        }

        // Fallback: try to parse as direct array of events
        let events: Vec<AcledEvent> = serde_json::from_str(&text).into_app_error()?;

        Ok(events)
    }

    fn build_url(user_data: &UserData, params: &AcledParams) -> String {
        let mut url = format!(
            "{}?key={}&email={}",
            ACLED_BASE_URL, user_data.api_key, user_data.email
        );

        for (key, value) in params.to_query_params() {
            url.push_str(&format!("&{}={}", key, urlencoding::encode(&value)));
        }

        url
    }

    #[allow(dead_code)]
    pub fn validate_params(params: &AcledParams) -> AppResult<()> {
        if params.start_date.is_empty() {
            return Err(AppError::validation("start_date", "Start date is required"));
        }

        if params.end_date.is_empty() {
            return Err(AppError::validation("end_date", "End date is required"));
        }

        if params.country.is_empty() {
            return Err(AppError::validation("country", "Country is required"));
        }

        if params.limit == 0 {
            return Err(AppError::validation(
                "limit",
                "Limit must be greater than 0",
            ));
        }

        if params.limit > crate::config::MAX_EVENTS_LIMIT {
            return Err(AppError::validation(
                "limit",
                format!("Limit cannot exceed {}", crate::config::MAX_EVENTS_LIMIT),
            ));
        }

        Ok(())
    }
}
