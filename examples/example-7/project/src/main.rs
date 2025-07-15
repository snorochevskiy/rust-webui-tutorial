use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (input_getter, input_setter) = signal::<String>("".to_string());

    view! {
        <input type="text"
            on:input:target=move |ev| {
                input_setter.set(ev.target().value());
            }
        />
        <p>"Hello, "{input_getter}"!"</p>
    }
}
