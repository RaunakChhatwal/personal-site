use leptos::*;

use crate::config::{TEXT_COLOR, LINKEDIN_URL, GPA};

#[component]
fn Header() -> impl IntoView {
    view! {
        <div
            style:display="flex"
            style:align-items="center"
            style:flex-direction="column"
            style:margin-left="auto"
            style:margin-right="auto"
        >
            <h1 style:margin="0.1em 0">Raunak Chhatwal</h1>
            <div
                style:display="flex"
                style:align-items="center"
                style:font-family="serif"
            >
                <p style:margin="0 0.5em 0 0">"Phone: +1-813807-1581"</p>
                <div style:width="2px" style:height="1em" style:background-color=TEXT_COLOR></div>        // the border
                <p style:margin="0 0.5em">"Email: raunakchhatwal001@gmail.com"</p>
            </div>
            <p style:margin=0>"LinkedIn:"<a href=LINKEDIN_URL>{LINKEDIN_URL}</a></p>
        </div>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <div class="resume" style:margin-top="10vh">
            <h2 style:font-weight="bold" style:margin=0>"Education"</h2>
            <div style:width="100%" style:height="3px" style:background-color=TEXT_COLOR></div>
            <p style:font-size="1.1em" style:margin=0>"University of South Florida"</p>
            <ul style:margin=0>
                <li>"Bachelors of Science in Computer Science"</li>
                <li>{format!("GPA: {GPA}")}</li>
                <li>"Expect graduation date: May 2024"</li>
                <li>"Member of the Society of Competitive Programmers club"</li>
            </ul>
        </div>
    }
}

#[component]
fn WorkExperience() -> impl IntoView {

}

#[component]
pub fn Resume() -> impl IntoView {
    view !{
        <div
            style:margin-top="10vh"
            style:margin-left="auto"
            style:margin-right="auto"
            style:width="75vw"
            style:display="flex"
            style:flex-direction="column"
        >
            <Header />
            <Education />
        </div>
    }
}