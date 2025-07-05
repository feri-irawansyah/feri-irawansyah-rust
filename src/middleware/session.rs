use gloo_net::http::Request;
use serde::Deserialize;
use leptos::*;

use crate::app::BACKEND_URL;

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SessionData {
    pub usernid: i32,
    pub email: String,
    pub fullname: String,
    pub picture: String,
    pub register_date: String,
    pub ip_address: String,
    pub error: Option<String>,
}

impl SessionData {
    pub fn new() -> Self {
        SessionData {
            usernid: 0,
            email: "".to_string(),
            fullname: "".to_string(),
            picture: "".to_string(),
            register_date: "".to_string(),
            ip_address: "".to_string(),
            error: None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionResponse {
    pub data: SessionData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn check_session() -> Result<SessionData, ErrorResponse> {

    // let navigate = leptos_router::hooks::use_navigate();

    let resp = Request::get(format!("{}/auth/session", BACKEND_URL).as_str())
        .credentials(web_sys::RequestCredentials::Include) // penting kalau pakai cookie/session
        .send()
        .await;

    match resp {
        Ok(response) => {
            if response.status() == 200 {
                let session: SessionResponse = response.json().await.unwrap();
                Ok(session.data)
            } else {
                let error: ErrorResponse = response.json().await.unwrap();
                Err(error)
            }
        }
        Err(_) => Err(ErrorResponse { error: "Network error".to_string() }),
    }
}
