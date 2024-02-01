use leptos::*;
use leptos_router::*;

use crate::config::{TEXT_COLOR, BACKGROUND_COLOR, LINKEDIN_URL};

const HEADER_HEIGHT: &str = "5vh";

#[component]
fn HeaderLink(text_content: &'static str, href: &'static str, new_tab: bool) -> impl IntoView {
    let location = use_location();
    let (hover, set_hover) = create_signal(false);
    let (background_color, set_background_color) = create_signal(BACKGROUND_COLOR);

    create_effect(move |_| {
        if location.pathname.get() == *href || hover.get() {
            let hover_color = "rgb(32, 35, 37)";
            set_background_color.set(hover_color)
        } else {
            set_background_color.set(BACKGROUND_COLOR)
        }
    });

    let header_link = view! {
        <a href={href}
            on:mouseenter=move |_| set_hover.set(true)
            on:mouseleave=move |_| set_hover.set(false)
            class="m-0 text-center block no-underline text-inherit"
            style:background-color=background_color
            style:width="7vw"
            style:font="inherit"
            style:line-height=HEADER_HEIGHT
        >{text_content}</a>
    };

    if new_tab {
        header_link.set_attribute("target", "_black").unwrap();
        header_link.set_attribute("rel", "noopener noreferrer").unwrap();    // this is to prevent tabnipping
    }

    header_link
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header
            class="flex"
            style:height=HEADER_HEIGHT
            style:font-size="1.1em"
        >
            <nav>
                <div class="flex justify-start">
                    <HeaderLink text_content="About" href="/" new_tab=false />
                    <HeaderLink text_content="Resume" href="/resume" new_tab=false />
                    <HeaderLink text_content="Projects" href="/projects" new_tab=false />
                    <HeaderLink text_content="Interests" href="/interests" new_tab=false />
                </div>
            </nav>
            <div class="flex justify-end items-center w-full">
                <p class="text-center" style:margin="0 1em" style:line-height=HEADER_HEIGHT>"raunakchhatwal001@gmail.com"</p>
                <div class="m-0" style:width="1px" style:height="1em" style:background-color=TEXT_COLOR></div>
                <HeaderLink text_content="LinkedIn" href=LINKEDIN_URL new_tab=true />
                <HeaderLink text_content="Github" href="https://github.com/RaunakChhatwal" new_tab=true />
            </div>
        </header>
    }
}