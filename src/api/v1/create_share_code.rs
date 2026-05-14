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
    #[serde(rename = "created")]
    Created {
        #[serde(rename = "share-code")]
        share_code: String,
    },
    #[serde(rename = "error")]
    Error {
        reason: String,
    },
}

impl Response {
    pub fn code(&self) -> u16 {
        match self {
            Self::Created { .. } => 201,
            Self::Error { .. } => 500,
        }
    }
}

pub async fn endpoint(Json(body): Json<Request>) -> impl IntoResponse {
    let mut conn = Database::conn();

    let res = match ShareCode::create_metoffice(&mut conn, &body.metoffice_api_key) {
        Ok(entry) => Response::Created {
            share_code: entry.share_code().to_string(),
        },
        Err(err) => Response::Error {
            reason: err.to_string(),
        },
    };

    (StatusCode::from_u16(res.code()).unwrap(), Json(res))
}
