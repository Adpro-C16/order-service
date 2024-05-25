use std::env;

use rocket::{get, routes};

use autometrics::prometheus_exporter;
use dotenvy::dotenv;
use rocket_cors::AllowedOrigins;
use routes::order::{create_order, get_order, get_orders, pay_order};
use sqlx::postgres::PgPoolOptions;

pub mod guard;
pub mod model;
pub mod repository;
pub mod routes;
pub mod typing;

#[get("/")]
fn index() -> &'static str {
    "Heymart C14 Payment Service"
}

#[get("/metrics")]
pub fn metrics() -> String {
    prometheus_exporter::encode_to_string().unwrap()
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    prometheus_exporter::init();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        // .max_connections(3)
        .connect(&url)
        .await
        .unwrap();

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .manage(pool)
        .attach(cors)
        .mount("/", routes![index, metrics])
        .mount(
            "/orders",
            routes![create_order, get_orders, pay_order, get_order],
        )
        .launch()
        .await
        .unwrap();
}
