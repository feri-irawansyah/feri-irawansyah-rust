use gloo_net::http::Request;
use serde::Deserialize;
use leptos::*;

use crate::app::BACKEND_URL;

#[derive(Deserialize, Debug, Clone)]
pub struct SessionData {
    pub user_id: i32,
    // tambah field lain kalau perlu
}

impl SessionData {
    pub fn new() -> Self {
        SessionData {
            user_id: 0,
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
