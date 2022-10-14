use yew::prelude::*;

use crate::{components::*, msg, props};

pub struct App;

impl Component for App {
    type Message = msg::BaseMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <p>{"hello."}</p>
                <Header />
                <SearchBar />
            </div>
        }
    }
}
