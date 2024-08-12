use crate::components::Header;
use crate::types::{Product, CartProduct};
use crate::pages::{ShopPage, CheckoutPage};
use crate::routes::Route;

use yew::prelude::*;
use yew_router::prelude::*;

struct State {
  cart_items: i32,
  cart_products: Vec<CartProduct>,
}

pub struct App {
  state: State,
  link: ComponentLink<Self>,
}

pub enum Msg {
  AddToCart(Product),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let cart_products = vec![];
    let cart_items = 0;
    
    Self {
      state: State { 
        cart_items, 
        cart_products 
      },
      link,
    }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Msg::AddToCart(product) => {
        let cart_product = self
          .state
          .cart_products
          .iter_mut()
          .find(|cp: &&mut CartProduct| cp.product.id == product.id);
  
        if let Some(cp) = cart_product {
          cp.quantity += 1;
        } else {
          self.state.cart_products.push(CartProduct {
            product: product.clone(),
            quantity: 1,
          });
        }
        self.state.cart_items = self.state.cart_items + 1;
        true
      }
    }
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let handle_add_to_cart = self.link.callback(|product: Product| Msg::AddToCart(product));
    
    let cart_products = self.state.cart_products.clone();
    let cart_items = self.state.cart_items.clone();

    let render = Router::render(move |switch: Route| match switch {
      Route::CheckoutPage => {
        html! {<CheckoutPage cart_items=cart_items.clone() cart_products=cart_products.clone()/>}
      },
      Route::ShopPage => {
        html! {<ShopPage cart_products=cart_products.clone() add_cart_button=handle_add_to_cart.clone() />}
      }
    });

    html! {
      <>
        <Header cart_items=self.state.cart_items.clone()/>
        <Router<Route, ()> render=render/>
      </>
    }
  }
}