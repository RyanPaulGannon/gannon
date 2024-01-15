use chrono::{Local, TimeZone}; 
use leptos::*;

use serde_json::Value;
use dotenv::dotenv;
use dotenv_codegen::dotenv;

#[component]
pub fn Calendar() -> impl IntoView {
    let start_date = Local.with_ymd_and_hms(2024, 1, 1, 1, 1, 1);
    let _today = Local::now();

    let (weather, _set_weather) = create_signal(1);
    let _weather = create_resource(move || weather.get(), fetch_weather);

    view! {
        <div class="main">
            <div id="calendar">
                <ul class="calendar-header">
                    <li>"Mon"</li>
                    <li>"Tue"</li>
                    <li>"Wed"</li>
                    <li>"Thu"</li>
                    <li>"Fri"</li>
                    <li>"Sat"</li>
                    <li>"Sun"</li>
                    <li>{ format!("{:?}", start_date) }</li>
                </ul>
            </div>
        </div>
    }
}

#[server(GetWeather, "/weather", "GetJson")]
async fn fetch_weather(_i: i32) -> Result<(), ServerFnError> {
    dotenv().ok();

    let met_office_api_key = dotenv!("MET_OFFICE_API_KEY");
    let westhoughton_id: u32 = 354159;

    let url = format!("http://datapoint.metoffice.gov.uk/public/data/val/wxfcs/all/json/{}?res=3hourly&key={}", westhoughton_id, met_office_api_key);

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

