use axum::{Router, routing::post};

mod create_share_code;
mod revoke_share_code;

pub fn scope() -> Router {
    Router::new()
        .route("/create-share-code", post(create_share_code::endpoint))
        .route("/revoke-share-code", post(revoke_share_code::endpoint))
}
