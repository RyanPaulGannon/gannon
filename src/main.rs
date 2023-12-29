use leptos::*;

turf::style_sheet!("src/main.scss");
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>
        <div>
            <h1>"Ryan Paul Gannon"</h1>

            <div>
                <p>
                "Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
                🏴󠁧󠁢󠁥󠁮󠁧󠁿 Manchester, UK"
                <br />
                "Currently working at "
                    <a href="https://ekatree.com">Ekatree</a>.
                </p>

                <p>
                "You can find me contributing to Open Source on GitHub (with a particular interest in memory management and "
                "Rust), and working on"
                "some personal projects ou apprendre le francais."
                </p>

                <p>
                "I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈. I'll talk almost anything sport!"
                </p>
                // <p>"Some projects I'm working on/contributing to:"</p>
            </div>
        </div>
    }
}
