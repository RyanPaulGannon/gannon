use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="main">
            <div class="form-container">
                <input type="email" />
                <button>Enter</button>
            </div>
        </div>
    }
}

