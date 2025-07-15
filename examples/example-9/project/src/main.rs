use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let input_sig = RwSignal::new("".to_string());
    let hello_str_sig = RwSignal::new("".to_string());

    view! {
        <input type="text" bind:value=input_sig />

        <button
            on:click=move |_| hello_str_sig.set(format!("Hello, {}!", input_sig.get())) 
        >Greet</button>

        <p>{hello_str_sig}</p>
    }
}
