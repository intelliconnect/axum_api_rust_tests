use std::{env, sync::Arc};

use axum::{routing::get, Router, Server};
use routes::User;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod routes;
#[cfg(test)]
mod test;

pub struct AppState {
    pg_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let test_db_url = env::var("TEST_DB").expect("could not get test database url");
    let pg_pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&test_db_url)
        .await
        .expect("could not connect to database");
    let shared_state = Arc::new(AppState { pg_pool });

    let app = app(shared_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to run the server");
}

pub fn app(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async move { "welcome to tdd" }))
        .route("/defaultuser", get(routes::get_default_user))
        .with_state(shared_state)
}

pub fn get_sample_user() -> User {
    User {
        firstname: "Unit".to_owned(),
        lastname: "Test".to_owned(),
        age: 18,
    }
}
