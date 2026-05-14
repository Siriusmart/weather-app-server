use axum::Router;

pub mod v1;

pub fn scope() -> Router {
    Router::new().nest("/v1", v1::scope())
}
