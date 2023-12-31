use leptos::*;

#[component]
pub fn Hire() -> impl IntoView {
    let (text, show_text) = create_signal("".to_string());
    view! {
        <button on:click=move |_| show_text.set("ryan@gannon.one".to_string())>"Hire Me?"</button>
            <p>
                <a href="mailto:ryan@gannon.one">{ text }</a>
            </p>
    }
}
