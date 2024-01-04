use leptos::*;
use std::time::Duration;

#[component]
pub fn Contact() -> impl IntoView {
    let (text, show_text) = create_signal("".to_string());

    view! {
        <button on:click=move |_| {
            show_text.set("ryan@gannon.one".to_string());

            set_timeout(move || {
                show_text.set("".to_string());
            }, Duration::from_secs(5));
        }>
            "Contact Me?"
        </button>

        <p>
            <a href="mailto:{ text.get() }">{ text.clone() }</a>
        </p>
    }
}

