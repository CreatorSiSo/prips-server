use axum::{extract::Path, http::StatusCode, response::Result, Extension, Json};
use sqlx::PgPool;

use crate::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewUser {
	display_name: String,
	user_name: String,
	email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserId {
	id: u32,
}

#[axum::debug_handler]
pub async fn create(
	Extension(pool): Extension<PgPool>,
	Json(user): Json<NewUser>,
) -> Result<(StatusCode, Json<UserId>), Error> {
	let query = sqlx::query!(
		"INSERT INTO user_data (display_name, user_name, email) VALUES ($1, $2, $3) RETURNING id",
		user.display_name,
		user.user_name,
		user.email
	);
	let id = query.fetch_one(&pool).await?.id as u32;

	Ok((StatusCode::CREATED, Json(UserId { id })))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
	id: i32,
	display_name: String,
	user_name: String,
	email: String,
}

#[axum::debug_handler]
pub async fn get_all(
	Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<User>>), Error> {
	let query = sqlx::query_as!(User, "SELECT * FROM user_data");
	let users = query.fetch_all(&pool).await?;

	Ok((StatusCode::OK, Json(users)))
}

#[axum::debug_handler]
pub async fn get_by_id(
	Path(id): Path<u32>,
	Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<User>), Error> {
	let query = sqlx::query_as!(User, "SELECT * FROM user_data where id=$1", id as i32);
	let user = query
		.fetch_one(&pool)
		.await
		.map_err(|_| Error::NotFound(format!("User with id ({id}) does not exist")))?;

	Ok((StatusCode::OK, Json(user)))
}
