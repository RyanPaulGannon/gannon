use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <a href="mailto:ryan@gannon.one">
            <button>"Contact Me?"</button>
        </a>
    }
}
