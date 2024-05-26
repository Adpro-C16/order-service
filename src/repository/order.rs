use std::{env, str::FromStr};

use sqlx::{query, Result};

use crate::{
    guard::services::{
        user_service_client::UserServiceClient, TransactionType, UpdateBalanceRequest,
    },
    model::order::{Order, OrderDto, OrderStatus},
};

pub struct OrderRepository;

impl OrderRepository {
    pub async fn find_all(pool: &sqlx::PgPool, id: i32) -> Result<Vec<Order>> {
        let query = query!(
            "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC",
            id
        )
        .fetch_all(pool)
        .await;
        match query {
            Ok(orders) => Ok(orders
                .iter()
                .map(|row| Order {
                    id: row.id,
                    user_id: row.user_id,
                    supermarket_id: row.supermarket_id,
                    product_name: row.product_name.clone(),
                    subtotal: row.subtotal,
                    quantity: row.quantity.unwrap_or_else(|| 0),
                    status: OrderStatus::from_str(&row.status)
                        .unwrap_or(OrderStatus::WaitingPayment),
                    created_at: row.created_at.unwrap(),
                })
                .collect()),
            Err(err) => Err(err),
        }
    }
    pub async fn create(pool: &sqlx::PgPool, order: OrderDto, user_id: i32) -> Option<Order> {
        let query = query!(
            "INSERT INTO orders (user_id, product_name, quantity, subtotal, supermarket_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            user_id,
            order.product_name,
            order.quantity,
            order.subtotal,
            order.supermarket_id
        )
        .fetch_one(pool)
        .await;
        match query {
            Ok(row) => Some(Order {
                id: row.id,
                user_id: row.user_id,
                supermarket_id: row.supermarket_id,
                product_name: row.product_name,
                subtotal: row.subtotal,
                quantity: row.quantity.unwrap_or_else(|| 0),
                status: OrderStatus::from_str(&row.status).unwrap_or(OrderStatus::WaitingPayment),
                created_at: row.created_at.unwrap(),
            }),
            Err(_) => None,
        }
    }

    pub async fn find(pool: &sqlx::PgPool, order_id: i32, user_id: i32) -> Option<Order> {
        let query = query!(
            "SELECT * FROM orders WHERE id = $1 AND user_id = $2",
            order_id,
            user_id,
        )
        .fetch_one(pool)
        .await;
        match query {
            Ok(row) => Some(Order {
                id: row.id,
                user_id: row.user_id,
                supermarket_id: row.supermarket_id,
                product_name: row.product_name,
                subtotal: row.subtotal,
                quantity: row.quantity.unwrap_or_else(|| 0),
                status: OrderStatus::from_str(&row.status).unwrap_or(OrderStatus::WaitingPayment),
                created_at: row.created_at.unwrap(),
            }),
            Err(_) => None,
        }
    }
    pub async fn pay(
        pool: &sqlx::PgPool,
        order_id: i32,
        user_id: i32,
        token: String,
    ) -> Option<Order> {
        let order = OrderRepository::find(&pool, order_id, user_id).await;
        if order.is_none()
            || order.as_ref().unwrap().status.to_string() != OrderStatus::WaitingPayment.to_string()
        {
            return None;
        }
        let order = order.unwrap();
        let auth_url = env::var("AUTH_SERVICE_URL").expect("AUTH_SERVICE_URL must be set.");
        let mut user_client = UserServiceClient::connect(auth_url).await.unwrap();
        let request = tonic::Request::new(UpdateBalanceRequest {
            amount: order.subtotal,
            token,
            transaction_type: TransactionType::Withdraw.to_i32(),
        });
        let response = match user_client.update_balance(request).await {
            Ok(response) => response.into_inner(),
            Err(_) => return None,
        };
        if !response.success {
            return None;
        }
        let query = query!(
            "UPDATE orders SET status = $1 WHERE id = $2 AND user_id = $3 RETURNING *",
            OrderStatus::Paid.to_string(),
            order_id,
            user_id,
        )
        .fetch_one(pool)
        .await;
        match query {
            Ok(row) => Some(Order {
                id: row.id,
                user_id: row.user_id,
                supermarket_id: row.supermarket_id,
                product_name: row.product_name,
                subtotal: row.subtotal,
                quantity: row.quantity.unwrap_or_else(|| 0),
                status: OrderStatus::from_str(&row.status).unwrap_or(OrderStatus::WaitingPayment),
                created_at: row.created_at.unwrap(),
            }),
            Err(_) => None,
        }
    }
}
