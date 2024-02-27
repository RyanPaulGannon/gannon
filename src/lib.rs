use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod routes;
use components::contact::*;
use routes::{books::*, calendar::*, login::*, not_found::*};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/gannon.css"/>

        <Title text="Ryan Paul Gannon"/>

        <div class="container">
            <nav>
                <a href="/">"Home"</a>
                <a href="/calendar">"Calendar"</a>
                <a href="/books">"Books"</a>
                // <a href="/login">"Login"</a>
            </nav>
            <Router>
            <main>
                <Routes>
                    <Route path="/" view=Index/>
                    <Route path="/books" view=Books/>
                    <Route path="/calendar" view=Calendar/>
                    // <Route path="/login" view=Login/>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
            </Router>
        </div>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <h1>"Ryan Paul Gannon"</h1>
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

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            use leptos::*;

            console_error_panic_hook::set_once();

            leptos::mount_to_body(App);
        }
    }
}
