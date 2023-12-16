use leptos::*;

use crate::params;

#[component]
fn Title() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:justify-content="center"
            style:align-items="center"
            style:font-size="1.1em"
        >
            <h2 style:margin="0.2em 0.5em">"Software Developer"</h2>
            <div style:width="2px" style:height="1.5em" style:background-color=params::TEXT_COLOR></div>        // the border
            <h2 style:margin="0.2em 0.5em">"C.S. Student"</h2>
        </div>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:align-items="center"
            style:flex-direction="column"
            style:margin-left="auto"
            style:margin-right="auto"
        >
            <h1 style:font-size="4em" style:margin="0.2em 0">"Raunak Chhatwal"</h1>
            <Title />
            <p
                style:width="25%"
                style:text-align="justify"
                style:hyphens="auto"
                style:line-height="2"
            >{include_str!("./description.txt")}</p>        // FIXME: convert to markdown
        </div>
    }
}