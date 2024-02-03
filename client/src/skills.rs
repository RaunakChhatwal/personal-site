use leptos::*;

#[component]
pub fn SkillsMapEntry(header: &'static str, entries: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-start items-center mt-[2em] md:mt-0 md:text-[1.3em]">
            <h1 class="text-center text-[1.5em] underline">{header}</h1>
            {entries.iter().map(|entry| {
                let capitalized_entry = entry.split(" ").map(|word|
                    (*word)[0..1].to_uppercase() + &(*word)[1..]).collect::<Vec<String>>().join(" ");
                view! { <p class="text-center my-[0.4em]">{capitalized_entry}</p> }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn SkillsMap() -> impl IntoView {
    let languages = vec!["python", "c++", "typescript", "rust", "haskell"];
    let web_technologies = vec!["node.js", "express", "flask", "react", "postgres", "axum.rs"];
    let devops_technologies = vec!["kubernetes", "google cloud platform", "NixOS"];

    view! {
        <div
            class="flex flex-col md:flex-row md:justify-evenly md:items-start w-full md:w-[85%] md:mt-[10vh]"
        >
            <SkillsMapEntry header="Languages" entries=languages />
            <SkillsMapEntry header="Web Development" entries=web_technologies />
            <SkillsMapEntry header="DevOps" entries=devops_technologies />
        </div>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center mx-auto my-[13vh]">
            <h1 class="text-[3em]">Skills Map</h1>
            <SkillsMap />
        </div>
    }
}