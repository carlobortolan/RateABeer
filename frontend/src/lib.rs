mod api;
mod components;
mod pages;
mod routes;

use routes::{switch, Route};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html};
use yew_router::{BrowserRouter, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn main() {
    println!("STARTED main 0");
    yew::start_app::<RateBeer>();
    println!("STARTED main 1");
}

#[function_component(RateBeer)]
pub fn rate_beer() -> Html {
    println!("STARTED rate_beer");

    html! {
        <div class="container mt-5">
          <h1>{ "Rate Beer" }</h1>
          <div>
            <BrowserRouter>
              <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
          </div>
        </div>
    }
}


