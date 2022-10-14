use props::ListProps;
use yew::{virtual_dom::VNode, Children};

mod app;
mod components;
mod msg;
mod props;

fn main() {
    yew::start_app::<app::App>();
}
