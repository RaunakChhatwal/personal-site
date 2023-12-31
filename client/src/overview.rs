use leptos::*;
use comrak::{markdown_to_html, ComrakOptions};

use crate::config::TEXT_COLOR;

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
            <div style:width="2px" style:height="1.5em" style:background-color=TEXT_COLOR></div>        // the border
            <h2 style:margin="0.2em 0.5em">"C.S. Student"</h2>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    let description_html = markdown_to_html(include_str!("./description.md"), &ComrakOptions::default());

    view! {
        <div
            style:display="flex"
            style:justify-content="flex-start"
            style:align-items="center"
            style:flex-direction="column"
            style:margin-left="4vw"
        >
            <h1 style:font-size="4em" style:margin="0.2em 0">"Raunak Chhatwal"</h1>
            <Title />
            <p
                style:width="26em"
                style:text-align="justify"
                style:hyphens="auto"
                style:line-height="2"
                inner_html=description_html />
        </div>
    }
}

#[component]
pub fn Overview() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:justify-content="center"
            style:align-items="center"
            style:margin-top="10vh"
        >
            <img
                src="profile-picture.jpg"
                alt="Picture of Raunak"
                style:width="30vh"
                style:height="30vh"
                style:margin-right="8vw" />
            <div style:width="2px" style:height="50vh" style:background-color=TEXT_COLOR></div>
            <About />
        </div>
    }
}