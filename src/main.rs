use axum::{routing::get, Router};
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;
use tracing::info;

mod dtos;
mod errors;
mod misc;
mod models;
mod repositories;
mod routes;
mod services;

trait StateTrait {
    fn get_pool(&self) -> Box<SqlitePool>;
}

#[derive(Debug, Clone)]
pub struct AppState {
    pool: Box<SqlitePool>,
}

impl StateTrait for AppState {
    fn get_pool(&self) -> Box<SqlitePool> {
        self.pool.clone()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(
            #[cfg(debug_assertions)]
            tracing::Level::DEBUG,
            #[cfg(not(debug_assertions))]
            tracing::Level::INFO,
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = SqlitePool::connect(&database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    let state = AppState {
        pool: Box::new(pool),
    };

    let app = Router::new()
        .route(
            "/batches",
            get(routes::batch::list_batches).post(routes::batch::insert_batch),
        )
        .route(
            "/batches/:id",
            get(routes::batch::find_batch_by_id)
                .put(routes::batch::update_batch)
                .delete(routes::batch::delete_batch),
        )
        .route(
            "/crops",
            get(routes::crop::list_crops).post(routes::crop::insert_crop),
        )
        .route(
            "/crops/:id",
            get(routes::crop::find_crop_by_id)
                .put(routes::crop::update_crop)
                .delete(routes::crop::delete_crop),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addrs = "0.0.0.0:3333";
    let listener = tokio::net::TcpListener::bind(addrs).await?;
    info!("Listening on {}", addrs);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
