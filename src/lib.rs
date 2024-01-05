use leptos::*;
use leptos_router::*;
use std::time::Duration;

turf::style_sheet!("src/main.scss");

pub mod calendar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>
            <div class="container">
                <Router>
                    <nav class="nav">
                        <h1>"Ryan Paul Gannon"</h1>
                        <a href="/">"Home"</a>
                        <a href="/calendar">"Calendar"</a>
                        <a href="/login">"Login"</a>
                    </nav>
                    <Routes>
                        <Route path="/" view=Index/>
                        <Route path="/calendar" view=calendar::Calendar/>
                        <Route path="/login" view=Login/>
                    </Routes>
                </Router>
            </div>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div id="main">
            <Contact />
            <div class="card">
                <p>
                "Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
                🏴󠁧󠁢󠁥󠁮󠁧󠁿 Manchester, UK"
                <br />
                "Currently working at "
                    <a href="https://ekatree.com" target="_blank">Ekatree</a>.
                </p>

                <p id="main-text">
                "You can find me contributing to Open Source on "
                    <a href="https://github.com/ryanpaulgannon" target="_blank">GitHub</a>
                " (with a particular interest in memory management "
                "and Rust), and working on some personal projects ou apprendre le francais."
                </p>

                <p>
                "I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈. I'll talk almost anything sport!"
                </p>
                // <p>"Some projects I'm working on/contributing to:"</p>
            </div>
        </div>
    }
}

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

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div id="main">
            <div class="form-container">
                <input type="email" />
                <button>Enter</button>
            </div>
        </div>
    }
}
