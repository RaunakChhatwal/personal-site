use leptos::*;
use leptos_router::*;

#[allow(unused_imports)]
use leptos_dom::log;

mod params;
mod about;
mod header;
use crate::{about::Overview, header::Header};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <Header />
            </nav>
            <main style:flex-grow="1">
                <Routes>
                    <Route path="/" view=Overview />
                    <Route path="/*any" view=|| view! { <h1>Not found</h1> } />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}