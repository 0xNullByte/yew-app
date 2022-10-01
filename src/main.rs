use yew::prelude::*;
use yew_router::prelude::*;

// modules 
mod include;
use crate::include::routes::{
    switch,
    AppRoute
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
fn main() {
    yew::start_app::<App>();
}