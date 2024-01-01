use leptos::*;

mod contact;
use contact::*;

turf::style_sheet!("src/main.scss");

fn main() {
    mount_to_body(|| {
        view! {
            <style>{STYLE_SHEET}</style>
            <div id="main">
                <h1>"Ryan Paul Gannon"</h1>

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
    })
}
