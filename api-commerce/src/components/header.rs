use yew::prelude::*;

use crate::routes::Route;
use yew_router::components::RouterAnchor;

pub struct Header {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub cart_items: i32,
}

impl Component for Header {
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
    type Anchor = RouterAnchor<Route>;

    html! {
      <div class="header">
        <Anchor route=Route::ShopPage classes="link-anchor">
          <div class="header-title">{"My Store"}</div>
        </Anchor>
        <Anchor route=Route::CheckoutPage classes="link-anchor">
          <div class="header-cart-value">{"Cart"}{" ("}{self.props.cart_items}{")"}</div>
        </Anchor>
        </div>
    }
  }
}  