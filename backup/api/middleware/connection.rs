#[cfg(feature = "ssr")]
pub mod ssr {
    use dotenvy::dotenv;
    use leptos::{prelude::{ServerFnError, *}};
    use sqlx::{PgPool, postgres::PgPoolOptions};
    use std::env;

    pub async fn get_connection() -> Result<PgPool, ServerFnError<serde_json::Value>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|e| {
            let json = serde_json::json!({
                "message": "Failed to read DATABASE_URL",
                "error": e.to_string()
            });
            ServerFnError::ServerError(serde_json::to_string(&json).unwrap())
        })?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| {
                let json = serde_json::json!({
                    "message": "Failed to connect to database",
                    "error": e.to_string()
                });
                ServerFnError::ServerError(serde_json::to_string(&json).unwrap())
            })?;

        Ok(pool)
    }
}
