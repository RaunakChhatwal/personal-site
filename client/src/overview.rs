use leptos::*;
use comrak::{markdown_to_html, ComrakOptions};

use crate::config::TEXT_COLOR;

#[component]
fn Title() -> impl IntoView {
    view! {
        <div class="flex flex-col md:flex-row justify-center items-center text-[1.3em] md:text-[1.6em]">
            <h2 style:margin="0.2em 0.5em">"Software Developer"</h2>
            <div class="hidden md:block w-[2px] h-[1em]" style:background-color=TEXT_COLOR></div>        // the border
            <h2 class="hidden md:block" style:margin="0.2em 0.5em">"C.S. Student"</h2>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    let mut description_html = markdown_to_html(include_str!("./description.md"), &ComrakOptions::default());

    for i in 0..(description_html.len() - "<a".len() + 1) {
        if &description_html[i..i+"<a".len()] == "<a" {
            let (left, right) = description_html.split_at(i + 2);
            description_html = left.to_string() + " class=\"text-blue-500 underline\"" + right;
        }
    }

    view! {
        <div
            class="flex flex-col justify-start items-center"
            style:margin-left="4vw"
        >
            <h1 class="m-0 text-[2em] md:text-[4em]">"Raunak Chhatwal"</h1>
            <Title />
            <p
                class="md:w-[26em]"
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
        <div class="flex flex-col md:flex-row justify-center items-center mx-auto w-[90%] h-full">
            <img
                src="profile-picture.jpg"
                alt="Picture of Raunak"
                class="w-[20vw] h-[20vw] mb-[1em] md:mb-0 md:mr-[8vw]" />
            <div class="hidden md:block w-[2px] h-[50vh]" style:background-color=TEXT_COLOR></div>
            <About />
        </div>
    }
}