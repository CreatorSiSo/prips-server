use axum::{Extension, Router};
use color_eyre::eyre::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod user;

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;

	{
		let tracing_layer = tracing_subscriber::fmt::layer();

		let filter = tracing_subscriber::filter::Targets::new()
			.with_target("tower_http::trace::make_span", Level::DEBUG)
			.with_target("tower_http::trace::on_response", Level::TRACE)
			.with_target("tower_http::trace::on_request", Level::TRACE)
			.with_default(Level::INFO);

		tracing_subscriber::registry()
			.with(tracing_layer)
			.with(filter)
			.init();
	}

	let pool = connect_database().await?;
	sqlx::migrate!("./migrations").run(&pool).await?;

	let routes = Router::new()
		.route("/users", axum::routing::get(user::get_all))
		.route("/user/:id", axum::routing::get(user::get_by_id))
		.route("/user", axum::routing::post(user::create))
		.layer(Extension(pool))
		.layer(TraceLayer::new_for_http());

	let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
	tracing::info!("Listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(routes.into_make_service())
		.await?;

	Ok(())
}

async fn connect_database() -> Result<PgPool> {
	let database_url =
		dotenvy::var("DATABASE_URL").expect("could not load DATABASE_URL from .env file");
	tracing::info!("Connecting to {database_url}");

	PgPoolOptions::new()
		.max_connections(50)
		.connect(&database_url)
		.await
		.context("Could not connect to database")
}
