use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow, PartialEq)]
pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub age: i16,
}

pub async fn get_default_user(
    State(appstate): State<Arc<AppState>>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
    Ok(Json(
        sqlx::query_as::<_, User>("SELECT * from users WHERE usertype = 'default'")
            .fetch_one(&appstate.pg_pool)
            .await
            .map_err(|err| {
                dbg!(err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not get default user",
                )
            })?,
    ))
}
