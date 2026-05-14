use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{Database, org::ShareCode};

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "metoffice-api-key")]
    metoffice_api_key: String,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
pub enum Response {
    Revoked {
        #[serde(rename = "share-code")]
        share_code: String,
    },
    Error {
        reason: String,
    },
}

impl Response {
    pub fn code(&self) -> u16 {
        match self {
            Self::Revoked { .. } => 200,
            Self::Error { reason } if reason == "not found" => 404,
            Self::Error { .. } => 500,
        }
    }
}

pub async fn endpoint(Json(body): Json<Request>) -> impl IntoResponse {
    let mut conn = Database::conn();

    let res = match ShareCode::remove_by_metoffice_key(&mut conn, &body.metoffice_api_key) {
        Ok(Some(removed)) => Response::Revoked {
            share_code: removed.share_code().to_string(),
        },
        Ok(None) => Response::Error {
            reason: "not found".to_string(),
        },
        Err(err) => Response::Error {
            reason: err.to_string(),
        },
    };

    (StatusCode::from_u16(res.code()).unwrap(), Json(res))
}
