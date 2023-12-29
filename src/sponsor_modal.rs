use leptos::*;

turf::style_sheet!("src/sponsor_modal.scss");

#[component]
pub fn SponsorModal() -> impl IntoView {
    view! {
        <style>{STYLE_SHEET}</style>
        <div class=ClassName::SPONSOR_MODAL>
            <h1>"Ryan Paul Gannon"</h1>
        </div>
    }
}
