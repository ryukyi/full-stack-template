use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64, // Using Decimal for accurate financial calculations
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub meal_id: i32,
    pub quantity: i32,
    pub total_price: f64, // Using Decimal here as well
}
