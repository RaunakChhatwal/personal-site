use leptos::*;
use leptos_router::*;

#[allow(unused_imports)]
use leptos_dom::log;

mod config;
mod header;
mod overview;
mod resume;
use crate::{header::Header, overview::Overview, resume::Resume};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main style:flex-grow="1">
                <Routes>
                    <Route path="/" view=Overview />
                    <Route path="/resume" view=Resume />
                    <Route path="/*any" view=|| view! { <h1>Not found</h1> } />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}