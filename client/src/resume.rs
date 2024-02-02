use leptos::*;
use serde::{Serialize, Deserialize};

use crate::config::{TEXT_COLOR, LINKEDIN_URL};

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center mx-auto">
            <h1 class="text-[1.5em]" style:margin="0.1em 0">Raunak Chhatwal</h1>
            <div class="flex flex-col md:flex-row items-center serif">
                <p style:margin="0 0.5em 0 0">"Phone: +1-813-807-1581"</p>
                <div class="hidden md:inline-block w-[2px] h-[1em]" style:background-color=TEXT_COLOR></div>        // the border
                <p style:margin="0 0.5em">"Email: raunakchhatwal001@gmail.com"</p>
            </div>
            <p style:margin=0>"LinkedIn: "<a class="text-blue-500 underline" href=LINKEDIN_URL>{LINKEDIN_URL}</a></p>
        </div>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <div class="resume" style:margin-top="6vh">
            <h2 class="text-[1.2em] font-bold">"Education"</h2>
            <div class="w-full h-[3px]" style:background-color=TEXT_COLOR></div>
            <p class="text-[1.2em]">"University of South Florida"</p>
            <ul class="list-disc" style:list-style-position="inside">{[
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
            <h2 class="text-[1.2em] font-bold">"Work Experience"</h2>
            <div class="w-full h-[3px]" style:background-color=TEXT_COLOR></div>
            {work_experience.iter().enumerate().map(|(i, JobEntry { title, employer, duration, about })| view! {
                <div>
                    <p
                        class="text-[1.2em] underline"
                        style:margin-top=move || (i > 0).then_some("3vh")
                    >{title}</p>
                    <p class="text-[0.9em]">{format!("{employer} | {duration}")}</p>
                    <ul class="list-disc" style:list-style-position="inside">{about.iter().map(|description|
                        view! { <li style:font-size="0.9em">{description}</li> }
                    ).collect_view()}</ul>
                </div>
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn Resume() -> impl IntoView {
    view !{
        <div
            class="flex flex-col md:text-[1.1em] w-[90vw] md:w-[50vw]"
            style:margin="13vh auto"
        >
            <Header />
            <Education />
            <WorkExperience />
        </div>
    }
}