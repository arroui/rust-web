use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub image: String,
  pub price: f64,
}

#[derive(Clone, Debug)]
pub struct CartProduct {
  pub product: Product,
  pub quantity: i32,
}