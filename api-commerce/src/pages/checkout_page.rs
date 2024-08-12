use crate::types::{CartProduct};

use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
  pub cart_items: i32,
  pub cart_products: Vec<CartProduct>,
}

pub struct CheckoutPage {
  props: Props,
}

impl Component for CheckoutPage {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    let cart_value = self.props.cart_products
      .iter()
      .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

    let prod = &self.props.cart_products;

    html! {
      <>
        <div class="heading">{"Checkout"}</div>
        <div class="table">
          <div class="table-header">
            <div class="header__item">{"Product"}</div>
            <div class="header__item">{"Name"}</div>
            <div class="header__item">{"Quantity"}</div>
            <div class="header__item">{"Price"}</div>
          </div>
          <hr class="line"/>
          <div class="table-content">   
            {
              prod.into_iter().map(|cartItem| {
                html!{
                  <div class="table-row">
                    <div class="table-data"><img class="card_image" src={&cartItem.product.image}/></div>
                    <div class="table-data">{format!("{}", cartItem.product.title)}</div>
                    <div class="table-data">{cartItem.quantity}</div>
                    <div class="table-data">{"$"}{cartItem.product.price}</div>
                  </div>
                }
              })
              .collect::<Html>()
            }
          <div class="totalprice">{"Total:"}{" $"}{cart_value}</div>
          </div>
        </div>
      </>
    }
  }
}