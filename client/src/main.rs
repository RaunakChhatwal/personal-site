use leptos::*;

#[allow(unused_imports)]
use leptos_dom::log;

const TEXT_COLOR: &str = "#e8e6e3";

#[component]
fn Title() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:justiny-content="center"
            style:align-items="center"
            style:flex-direction="row"
            style:font-size="1.1em"
        >
            <h2 style:margin="0.2em 0.5em">"Software Developer"</h2>
            <div style:width="2px" style:height="1.5em" style:background-color=TEXT_COLOR></div>        // the border
            <h2 style:margin="0.2em 0.5em">"C.S. Student"</h2>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:justify-content="center"
            style:align-items="center"
            style:flex-direction="column"
            style:margin-left="auto"
            style:margin-right="auto"
        >
            <h1 style:font-size="4em" style:margin="0.2em 0">"Raunak Chhatwal"</h1>
            <Title />
            <p
                style:width="25%"
                style:text-align="center"
                style:text-align="justify"
                style:hyphens="auto"
                style:line-height="2"
            >{include_str!("./description.txt")}</p>
        </div>
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