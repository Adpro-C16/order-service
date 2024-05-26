use std::{fmt::Display, str::FromStr};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub supermarket_id: i32,
    pub product_name: String,
    pub subtotal: i32,
    pub quantity: i32,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrderDto {
    pub supermarket_id: i32,
    pub product_name: String,
    pub subtotal: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub enum OrderStatus {
    WaitingPayment,
    Paid,
    Shipped,
    Delivered,
}

impl FromStr for OrderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Waiting Payment" => Ok(OrderStatus::WaitingPayment),
            "Paid" => Ok(OrderStatus::Paid),
            "Shipped" => Ok(OrderStatus::Shipped),
            "Delivered" => Ok(OrderStatus::Delivered),
            _ => Err(()),
        }
    }
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::WaitingPayment => write!(f, "Waiting Payment"),
            OrderStatus::Paid => write!(f, "Paid"),
            OrderStatus::Shipped => write!(f, "Shipped"),
            OrderStatus::Delivered => write!(f, "Delivered"),
        }
    }
}
