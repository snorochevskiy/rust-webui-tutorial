use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, path};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                | <a href="/tab_1">tab 1</a> | <a href="/tab_2">tab 2</a> |
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/")      view=|| view! { <p>Default</p> } />
                    <Route path=path!("/tab_1") view=Tab1 />
                    <Route path=path!("/tab_2") view=Tab2 />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Tab1() -> impl IntoView {
    view! { <p>Tab 1</p> }
}

#[component]
fn Tab2() -> impl IntoView {
    view! { <p>Tab 2</p> }
}
