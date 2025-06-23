// use leptos::prelude::*;

// #[server]
// pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
//     let mut conn = db().await?;

//     match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
//         .bind(title)
//         .execute(&mut conn)
//         .await
//     {
//         Ok(_row) => Ok(()),
//         Err(e) => Err(ServerFnError::ServerError(e.to_string())),
//     }
// }
