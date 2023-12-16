use leptos::*;

#[allow(unused_imports)]
use leptos_dom::log;

mod params;
mod about;
use crate::about::About;

#[component]
fn Header() -> impl IntoView {
    view! {

    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <About />
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}