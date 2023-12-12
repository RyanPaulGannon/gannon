use leptos::*;

mod sponsor_modal;
use sponsor_modal::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
               <div>
           <div>
     //         <SponsorModal />
             <div>
              <SponsorModal />

    //           <h1>Ryan Paul Gannon</h1>
               <div>
                "Bonjour, I'm Ryan, enchanté! I'm a Full Stack Developer from
                 🏴󠁧󠁢󠁥󠁮󠁧󠁿 Manchester, UK"
                 <br />
                 "Currently working at "
                 <a href="https://ekatree.com">Ekatree</a>.

                 <br />
                 <br />
                 "You can find me contributing to Open Source on GitHub (with a particular interest in memory management and"
                 "Rust), and working on"
                 "some personal projects ou apprendre le francais."

                 <br />
                 <br />
                 "I've a keen sporting interest, mostly for ⚽️, 🏏 and the NFL 🏈. I'll talk almost anything sport!"

                 <div>"Some projects I'm working on/contributing to:"</div>

                 <div>
                   // <Carousel />
                 </div>
               </div>
             </div>
           </div>
         </div>
           }
}
