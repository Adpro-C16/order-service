use crate::{
    guard::AuthGuard,
    model::order::{Order, OrderDto},
    repository::order::OrderRepository,
    typing::{ErrorResponse, Result},
};
use autometrics::autometrics;
use rocket::{get, post, serde::json::Json, State};
use sqlx::PgPool;

#[post("/", data = "<order>", format = "json")]
#[autometrics]
pub async fn create_order(
    pool: &State<PgPool>,
    order: Json<OrderDto>,
    auth_ctx: AuthGuard,
) -> Result<Json<Order>, Json<ErrorResponse>> {
    let order = match OrderRepository::create(
        &pool.inner().clone(),
        order.into_inner(),
        auth_ctx.claims.user_id,
    )
    .await
    {
        Some(order) => order,
        None => {
            return Err(Json(ErrorResponse {
                status_code: rocket::http::Status::InternalServerError,
                message: "Failed to create order",
            }))
        }
    };
    Ok(Json(order))
}

#[get("/")]
#[autometrics]
pub async fn get_orders(
    pool: &State<PgPool>,
    auth_ctx: AuthGuard,
) -> Result<Json<Vec<Order>>, Json<ErrorResponse>> {
    let orders =
        match OrderRepository::find_all(&pool.inner().clone(), auth_ctx.claims.user_id).await {
            Ok(orders) => orders,
            Err(_) => {
                return Err(Json(ErrorResponse {
                    status_code: rocket::http::Status::InternalServerError,
                    message: "Failed to get orders",
                }))
            }
        };
    Ok(Json(orders))
}

#[post("/<order_id>")]
#[autometrics]
pub async fn pay_order(
    pool: &State<PgPool>,
    order_id: i32,
    auth_ctx: AuthGuard,
) -> Result<Json<Order>, Json<ErrorResponse>> {
    let order = match OrderRepository::pay(
        &pool.inner().clone(),
        order_id,
        auth_ctx.claims.user_id,
        auth_ctx.token,
    )
    .await
    {
        Some(order) => order,
        None => {
            return Err(Json(ErrorResponse {
                status_code: rocket::http::Status::InternalServerError,
                message: "Failed to pay order",
            }))
        }
    };
    Ok(Json(order))
}

#[get("/<order_id>")]
#[autometrics]
pub async fn get_order(
    pool: &State<PgPool>,
    order_id: i32,
    auth_ctx: AuthGuard,
) -> Result<Json<Order>, Json<ErrorResponse>> {
    let order =
        match OrderRepository::find(&pool.inner().clone(), order_id, auth_ctx.claims.user_id).await
        {
            Some(order) => order,
            None => {
                return Err(Json(ErrorResponse {
                    status_code: rocket::http::Status::InternalServerError,
                    message: "Failed to get order",
                }))
            }
        };
    Ok(Json(order))
}
