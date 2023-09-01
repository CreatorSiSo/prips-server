use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub enum Error {
	DatabaseError(String),
	NotFound(String),
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::DatabaseError(value.to_string())
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		match self {
			Self::DatabaseError(msg) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(json!({"error": msg})),
			),
			Self::NotFound(msg) => (StatusCode::NOT_FOUND, Json(json!({"error": msg}))),
		}
		.into_response()
	}
}
