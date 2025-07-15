use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let input_sig = RwSignal::new("".to_string());
    let (name_getter, name_setter) = signal("".to_string());
    let gist_text = LocalResource::new(move || hello(name_getter.get()));

    view! {
        <input type="text" bind:value=input_sig />
        <button on:click=move |_| name_setter.set(input_sig.get())>Greet</button>
        <p>
            {move || {
                match gist_text.get().unwrap_or_default() {
                    Some(greetings) => greetings,
                    None => "Please enter a name to greet".to_string(),
                }
            }}
        </p>
    }
}

async fn hello(name: String) -> Option<String> {
    if name.is_empty() { return None };

    let response = gloo_net::http::RequestBuilder::new(&format!(
        "http://localhost:8080/api/hello/{name}",
    ))
    .method(gloo_net::http::Method::GET)
    .send().await.unwrap()
    .text().await.unwrap();
    Some(response)
}
