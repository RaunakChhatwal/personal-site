use leptos::*;
use leptos_router::*;

use crate::params::BACKGROUND_COLOR;

#[component]
fn HeaderLink(text_content: &'static str, href: &'static str) -> impl IntoView {
    let location = use_location();
    let (hover, set_hover) = create_signal(false);
    let (background_color, set_background_color) = create_signal(BACKGROUND_COLOR);

    create_effect(move |_| {
        if location.pathname.get() == href.to_string() || hover.get() {
            set_background_color.set("rgb(32, 35, 37)")
        } else {
            set_background_color.set(BACKGROUND_COLOR)
        }
    });

    view! {
        <a href={href}
            on:mouseenter=move |_| set_hover.set(true)
            on:mouseleave=move |_| set_hover.set(false)
            style:background-color=background_color
            style:margin="0"
            style:width="7vw"
            style:text-align="center"
            style:display="block"
            style:text-decoration="none"
            style:color="inherit"
            style:font="inherit"
            style:line-height="5vh"
        >{text_content}</a>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            style:height="5vh"
            style:font-size="1.3em"
            style:display="flex"
            style:justify-content="flex-start"
        >
            <HeaderLink text_content="About" href="/" />
            <HeaderLink text_content="Resume" href="/resume" />
        </header>
    }
}