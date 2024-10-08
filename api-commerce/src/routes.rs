use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
  #[to = "/checkout"]
  CheckoutPage,
  #[to = "/"]
  ShopPage,
}