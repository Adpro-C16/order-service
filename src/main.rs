use std::env;

use rocket::{get, routes};

use dotenvy::dotenv;
use routes::order::{create_order, get_orders};
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

#[rocket::main]
async fn main() {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        // .max_connections(3)
        .connect(&url)
        .await
        .unwrap();

    rocket::build()
        .manage(pool)
        .mount("/", routes![index])
        .mount("/orders", routes![create_order, get_orders])
        .launch()
        .await
        .unwrap();
}
