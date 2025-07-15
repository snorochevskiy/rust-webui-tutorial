use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (input_getter, input_setter) = signal::<String>("".to_string());
    view! {
        <NameInput setter=input_setter />
        <DisplayHellow getter=input_getter />
    }
}

#[component]
fn NameInput(setter: WriteSignal<String>) -> impl IntoView {
    view! {
        <input type="text"
            on:input:target=move |ev| {
                setter.set(ev.target().value());
            }
        />
    }
}

#[component]
fn DisplayHellow(getter: ReadSignal<String>) -> impl IntoView {
    view! {
        <p>"Hello, "{getter}"!"</p>
    }
}
