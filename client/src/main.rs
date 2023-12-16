use leptos::*;
use leptos_router::*;

#[allow(unused_imports)]
use leptos_dom::log;

mod params;
mod about;
mod header;
use crate::{about::About, header::Header};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <Header />
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=About />
                    <Route path="/*any" view=|| view! { <h1>Not found</h1> } />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}