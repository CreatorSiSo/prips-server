use axum::{Extension, Router};
use color_eyre::eyre::Result;
use sqlx::postgres::PgPoolOptions;

mod error;
mod user;

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://simon:1234&&@localhost:5432/prips")
		.await?;

	sqlx::migrate!("./migrations").run(&pool).await?;

	let routes = Router::new()
		.route("/users", axum::routing::get(user::get_all))
		.route("/user/:id", axum::routing::get(user::get_by_id))
		.route("/user", axum::routing::post(user::create))
		.layer(Extension(pool));

	axum::Server::bind(&([0, 0, 0, 0], 5000).into())
		.serve(routes.into_make_service())
		.await?;

	Ok(())
}

// fn read_env_file(path: &str) -> Result<HashMap<String, String>> {
// 	let string = std::fs::read_to_string(path)?;
// 	let pairs = string.lines().map(|line| {
// 		let (key, value) = line.split_once("=").unwrap();
// 		(key.trim().into(), value.trim().into())
// 	});
// 	Ok(pairs.collect())
// }
