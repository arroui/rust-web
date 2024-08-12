use crate::types::{Product, CartProduct};
use crate::components::ProductCard;

use yew::prelude::*;
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

struct State {
  // The products stored in a vector, that will be fetched from the server
  products: Vec<Product>,
  error_fetch_products: Option<Error>,
  is_products_fetched: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub cart_products: Vec<CartProduct>,
  pub add_cart_button: Callback<Product>,
}

pub enum Msg {
  FetchProducts,
  SuccessFetchProducts(Vec<Product>),
  ErrorFetchProducts(Error),
}

pub struct ShopPage {
  props: Props,
  state: State,
  link: ComponentLink<Self>,
  fetch_task: Option<FetchTask>,
}

impl Component for ShopPage {
  type Message = Msg;
  type Properties = Props;

  // this method is invoked when the component is created, so we will set the initial state here
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    // list of products
    let products = vec![];

    // Send a message "Msg::FetchProducts"  when the component is created so that the products
    // are fetched at the start of the application 
    link.send_message(Msg::FetchProducts);

    Self {
      props,
      link,
      fetch_task: None,
      state: State {
        products,
        error_fetch_products: None,
        is_products_fetched: false
      },
    }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Msg::FetchProducts => {
        self.state.is_products_fetched = false;
        
        // 1. Build the request to fetch the product data
        let request = Request::get("https://fakestoreapi.com/products?limit=15")
          .body(Nothing)
          .expect("Could not build that request");

        // 2. Construct a callback
        let callback =
          self.link
            .callback(|response: Response<Json<Result<Vec<Product>, anyhow::Error>>>| {
              let (_, Json(data)) = response.into_parts();
              match data {
                Ok(products) => {
                  Msg::SuccessFetchProducts(products)
                },
                Err(err) => {
                  Msg::ErrorFetchProducts(err)
                },
              }
            });

        // 3. Pass the request and the callback to the fetch service
        let task = FetchService::fetch(request, callback).expect("failed to start request");

        // 4. Store the `FetchTask` so it doesn't get dropped before the network request has finished
        self.fetch_task = Some(task);
        
        true
      }
      
      Msg::SuccessFetchProducts(products) => {
        self.state.products = products;
        self.state.is_products_fetched = true;
        true
      }
      
      Msg::ErrorFetchProducts(error) => {
        self.state.error_fetch_products = Some(error);
        self.state.is_products_fetched = true;
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    let products: Vec<Html> = self.state.products.iter().map(|product: &Product| {
      html! {
        <ProductCard product={product} on_add_cart=self.props.add_cart_button.clone()/>
      }
    })
    .collect();
  
    // Based on the value of the is_products_fetched state, 
    // render the loading text or error text or display the products on the screen
    if !self.state.is_products_fetched {
      html! {
        <div>{"Fetching Products..."}</div>
      }
    } 
    else if let Some(_) = self.state.error_fetch_products {
      html! {
        <div>
          <span>{"Products not fetched!!!"}</span>
        </div>
      }
    } 
    else {
      html! {
        <div class="product-list">
          {products}
        </div>
      }
    }
  }
}