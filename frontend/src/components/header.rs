use yew::prelude::*;

pub struct Header;

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1 class="text-center text-2xl">{ "Nova" }</h1>
            </div>
        }
    }
}
