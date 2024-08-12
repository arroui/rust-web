use crate::types::Product;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
  pub product: Product,
  pub on_add_cart: Callback<Product>,
}

// the actions to be taken when the button is pressed
pub enum Msg {
  AddToCart,
}

pub struct ProductCard {
  props: Props,
  link: ComponentLink<Self>,
}

impl Component for ProductCard {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddToCart => self.props.on_add_cart.emit(self.props.product.clone()),
    }
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    let onclick = self.link.callback(|_| Msg::AddToCart);

    html! {
      <div class="card">
        <img class="card-image" src={&self.props.product.image}/>
        <div class="card-title">
          {&self.props.product.title}
        </div>
        <div class="card-price">
          {"$"}{&self.props.product.price}
        </div>
        <button class="cart-button" onclick=onclick>{"Add To Cart"}</button>
      </div>
    }
  } 
}