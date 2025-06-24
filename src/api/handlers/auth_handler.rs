use leptos::*;
use leptos::server_fn::{ServerFnError, error::NoCustomError};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::api::middleware::{ connection::ssr::get_connection, crypto::encrypt_text };

#[cfg(feature = "ssr")]
use sqlx::Row; // PgRow implements Row

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub result: bool,
    pub message: String,
}

#[server] // leptos server function macro
pub async fn login_handler(request: LoginRequest) -> Result<LoginResponse, ServerFnError> {
    let pool = get_connection().await?; // pakai koneksi dari middleware

    let hash_password = encrypt_text(request.password.clone());

    // Cek user dari DB
    let row = sqlx::query(r#"
            SELECT 
                B.autonid AS user_id, 
                B.fullname,
                A.email, 
                A.disable_login, 
                A.last_login, 
                A.picture, 
                A.register_date
            FROM users A
            LEFT JOIN user_kyc B ON A.web_cif_id = B.autonid
            WHERE A.email = $1 AND A.password = $2
            "#)
        .bind(&request.email)
        .bind(&hash_password)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    if let Some(row) = row {
        return Ok(LoginResponse {
            result: true,
            message: "Login success".to_string(),
        });
    } else {
        return Ok(LoginResponse {
            result: false,
            message: "Invalid credentials".to_string(),
        });
    }
}
