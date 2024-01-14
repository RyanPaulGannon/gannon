use chrono::{Datelike, Local};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::time::Duration;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/gannon.css"/>

        <Title text="Ryan Paul Gannon"/>

        <div class="container">
            <nav>
                <h1>"Ryan Paul Gannon"</h1>
                <a href="/">"Home"</a>
                <a href="/calendar">"Calendar"</a>
                <a href="/login">"Login"</a>
            </nav>
            <Router>
            <main>
                <Routes>
                    <Route path="/" view=Index/>
                    <Route path="/calendar" view=Calendar/>
                    <Route path="/login" view=Login/>
                </Routes>
            </main>
            </Router>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div class="main">
            <Contact />
            <div class="card">
                <p>
                "Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
                🏴󠁧󠁢󠁥󠁮󠁧󠁿 Manchester, UK"
                <br />
                "Currently working at "
                    <a href="https://ekatree.com" target="_blank">Ekatree</a>.
                </p>

                <p class="main-text">
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
fn Contact() -> impl IntoView {
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
            <a href="mailto:ryan@gannon.one">{ text }</a>
        </p>
    }
}

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

#[component]
pub fn Calendar() -> impl IntoView {
    let today = Local::now();

    view! {
        <div class="main">
            <div class="calendar">
                <ul class="weeks">
                    <li>"Mon"</li>
                    <li>"Tue"</li>
                    <li>"Wed"</li>
                    <li>"Thu"</li>
                    <li>"Fri"</li>
                    <li>"Sat"</li>
                    <li>"Sun"</li>
                    <li>{ format_date(today) }</li>
                </ul>
            </div>
        </div>
    }
}

fn format_date(date: chrono::DateTime<chrono::Local>) -> String {
    format!("{:02}-{:02}-{:04}", date.day(), date.month(), date.year())
}
