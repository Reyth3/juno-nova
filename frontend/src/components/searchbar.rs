use yew::prelude::*;

pub struct SearchBar;

pub enum Msg {}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}
