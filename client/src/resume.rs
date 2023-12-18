use leptos::*;
use serde::{Serialize, Deserialize};

use crate::config::{TEXT_COLOR, LINKEDIN_URL};

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
                <p style:margin="0 0.5em 0 0">"Phone: +1-813-807-1581"</p>
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
        <div class="resume" style:margin-top="8vh">
            <h2 style:font-weight="bold">"Education"</h2>
            <div style:width="100%" style:height="3px" style:background-color=TEXT_COLOR></div>
            <p style:font-size="1.1em">"University of South Florida"</p>
            <ul>{[
                "Bachelors of Science in Computer Science",
                "GPA: 3.49",
                "Expect graduation date: May 2024",
                "Member of the Society of Competitive Programmers club"
            ].into_iter().map(|entry| view! { <li style:font-size="0.9em">{entry}</li> }).collect_view()}</ul>
        </div>
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct JobEntry {
    title: String,
    employer: String,
    duration: String,
    about: Vec<String>
}

#[component]
fn WorkExperience() -> impl IntoView {
    let work_experience = serde_json::from_str::<Vec<JobEntry>>(include_str!("./work-experience.json")).unwrap();

    view! {
        <div class="resume" style:margin-top="4vh">
            <h2 style:font-weight="bold">"Work Experience"</h2>
            <div style:width="100%" style:height="3px" style:background-color=TEXT_COLOR></div>
            {work_experience.iter().enumerate().map(|(i, JobEntry { title, employer, duration, about })| view! {
                <div>
                    <p
                        style:font-size="1.1em"
                        style:margin-top=move || (i > 0).then_some("3vh")
                    >{title}</p>
                    <p style:font-size="0.9em">{format!("{employer} | {duration}")}</p>
                    <ul>{about.iter().map(|description| view! { <li style:font-size="0.9em">{description}</li> }).collect_view()}</ul>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    view !{
        <div
            style:margin-top="10vh"
            style:margin-left="auto"
            style:margin-right="auto"
            style:width="50vw"
            style:display="flex"
            style:flex-direction="column"
        >
            <Header />
            <Education />
            <WorkExperience />
        </div>
    }
}