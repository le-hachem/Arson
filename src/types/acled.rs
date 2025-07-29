use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AcledParams {
    pub start_date: String,
    pub end_date: String,
    pub country: String,
    pub event_type: String,
    pub limit: u32,
}

impl Default for AcledParams {
    fn default() -> Self {
        Self {
            start_date: "2024-01-01".to_string(),
            end_date: "2024-12-31".to_string(),
            country: "Lebanon".to_string(),
            event_type: "Battles".to_string(),
            limit: 50,
        }
    }
}

impl AcledParams {
    pub fn is_valid(&self) -> bool {
        !self.start_date.is_empty()
            && !self.end_date.is_empty()
            && !self.country.is_empty()
            && self.limit > 0
            && self.limit <= 5000
    }

    pub fn to_query_params(&self) -> Vec<(String, String)> {
        vec![
            ("start".to_string(), self.start_date.clone()),
            ("end".to_string(), self.end_date.clone()),
            ("country".to_string(), self.country.clone()),
            ("event_type".to_string(), self.event_type.clone()),
            ("limit".to_string(), self.limit.to_string()),
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AcledEvent {
    pub data_id: u32,
    pub event_date: String,
    pub event_type: String,
    pub actor1: String,
    pub actor2: String,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: String,
    pub fatalities: Option<u32>,
}

impl AcledEvent {
    #[allow(dead_code)]
    pub fn has_coordinates(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }

    #[allow(dead_code)]
    pub fn coordinates(&self) -> Option<(f64, f64)> {
        match (self.latitude, self.longitude) {
            (Some(lat), Some(lng)) => Some((lat, lng)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn fatality_count(&self) -> u32 {
        self.fatalities.unwrap_or(0)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AcledError {
    pub status: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AcledResponse {
    pub success: Option<bool>,
    pub count: Option<u32>,
    pub data: Option<Vec<AcledEvent>>,
    pub pagination: Option<serde_json::Value>,
    pub status: Option<u32>,
    pub error: Option<AcledError>,
    pub message: Option<String>,
}

impl AcledResponse {
    pub fn is_success(&self) -> bool {
        self.success.unwrap_or(false) && self.error.is_none()
    }

    pub fn get_events(&self) -> Vec<AcledEvent> {
        self.data.clone().unwrap_or_default()
    }

    pub fn get_error_message(&self) -> Option<String> {
        if let Some(error) = &self.error {
            Some(error.message.clone())
        } else {
            self.message.clone()
        }
    }
} 