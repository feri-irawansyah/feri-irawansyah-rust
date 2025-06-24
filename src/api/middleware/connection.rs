#[cfg(feature = "ssr")]
pub mod ssr {
    use dotenvy::dotenv;
    use leptos::{prelude::{ServerFnError, *}, server_fn::error::NoCustomError};
    use sqlx::{PgPool, postgres::PgPoolOptions};
    use std::env;

    pub async fn get_connection() -> Result<PgPool, ServerFnError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("DATABASE_URL error: {e}")))?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| ServerFnError::<NoCustomError>::ServerError(format!("Postgres connect error: {e}")))?;

        Ok(pool)
    }
}
