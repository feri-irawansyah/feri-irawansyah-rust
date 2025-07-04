use gloo_net::http::Request;
use serde::Deserialize;
use leptos::*;

use crate::app::BACKEND_URL;

#[derive(Deserialize, Debug, Clone)]
pub struct SessionData {
    pub user_id: i32,
    // tambah field lain kalau perlu
}

#[derive(Deserialize)]
struct SessionResponse {
    data: SessionData,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

pub async fn check_session() -> Result<SessionData, String> {

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
                let err: ErrorResponse = response.json().await.unwrap_or(ErrorResponse {
                    error: "Unknown error".to_string(),
                });
                // navigate("/login", Default::default());
                Err(err.error)
            }
        }
        Err(_) => Err("Failed to connect".to_string()),
    }
}
