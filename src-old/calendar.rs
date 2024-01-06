use chrono::{Datelike, Local};
use leptos::*;

turf::style_sheet!("src/calendar.scss");

#[component]
pub fn Calendar() -> impl IntoView {
    let today = Local::now();

    view! {
        <style>{STYLE_SHEET}</style>
        <div id="main">
            <div id="calendar">
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
    format!("{}-{}-{}", date.day(), date.month(), date.year())
}
