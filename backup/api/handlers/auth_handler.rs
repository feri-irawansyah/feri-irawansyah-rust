use leptos::*;
use leptos::server_fn::ServerFnError;

#[cfg(feature = "ssr")]
use crate::api::middleware::{ connection::ssr::get_connection, crypto::encrypt_text };
use crate::api::models::auth_model::{LoginRequest, User};
use crate::api::models::global_model::ActionResult;

#[cfg(feature = "ssr")]
use sqlx::Row; // PgRow implements Row

#[server]
pub async fn login_handler(request: LoginRequest) -> Result<ActionResult<User, String>, ServerFnError<serde_json::Value>> {
    let mut result = ActionResult::default();
    let pool = get_connection().await?;

    let hash_password = encrypt_text(request.password.clone());

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
    .map_err(|e| {
        result.message = "Database error".to_string();
        result.error = Some(e.to_string());
        ServerFnError::ServerError(serde_json::to_string(&result).unwrap())
    })?;

    if let Some(row) = row {
        result.result = true;
        result.message = "Login Success".to_string();
        result.data = Some(User {
            usernid: row.get("user_id"),
            fullname: row.get("fullname"),
            email: row.get("email"),
            disabled_login: row.get("disable_login"),
            picture: row.get("picture"),
            register_date: row.get("register_date"),
            comp_name: Some("".to_string()),
            ip_address: Some("".to_string()),
            app_name: Some("".to_string()),
        });

        Ok(result)
    } else {
        result.message = "Login Failed".to_string();
        result.error = Some("User not found".to_string());

        // Serialize result ke JSON string dulu
        Err(ServerFnError::ServerError(serde_json::to_string(&result).unwrap()))
    }
}
