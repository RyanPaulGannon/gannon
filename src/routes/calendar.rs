use chrono::{Datelike, Local};
use leptos::*;

use dotenv::dotenv;
use dotenv_codegen::dotenv;
use serde_json::Value;

fn display_month(month: u32) -> Option<&'static str> {
    match month {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("Aug"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
}

#[component]
pub fn Calendar() -> impl IntoView {
    // let (weather, _set_weather) = create_signal<String>(None);
    let date = Local::now();

    view! {
        <div class="main">
            <div id="calendar">
                <div id="year">
                    { format!("{} {}", display_month(date.month()).unwrap(), date.year()) }
                        <ul class="calendar-header">
                            <li>"Mon"</li>
                            <li>"Tue"</li>
                            <li>"Wed"</li>
                            <li>"Thu"</li>
                            <li>"Fri"</li>
                            <li>"Sat"</li>
                            <li>"Sun"</li>
                            <li>{ format!("{:2}", date.day()) }</li>
                        </ul>
                </div>
            </div>
        </div>
    }
}

#[server(GetWeather, "/weather", "GetJson")]
async fn fetch_weather() -> Result<(), ServerFnError> {
    dotenv().ok();

    let met_office_api_key = dotenv!("MET_OFFICE_API_KEY");
    let westhoughton_id: u32 = 354159;

    let url = format!(
        "http://datapoint.metoffice.gov.uk/public/data/val/wxfcs/all/json/{}?res=3hourly&key={}",
        westhoughton_id, met_office_api_key
    );

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_data: Value = serde_json::from_str(&body)?;
        println!("{:#?}", json_data);
        // set_weather(json_data);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
