use chrono::{Datelike, Local, Month, TimeZone};
use dotenv::dotenv;
use dotenv_codegen::dotenv;
use leptos::*;
use serde_json::Value;

#[component]
pub fn Calendar() -> impl IntoView {
    // let (weather, _set_weather) = create_signal("".to_string());
    let date = Local::now();
    let start_date = Local::with_ymd_and_hms(&Local, 2023, 1, 1, 0, 0, 0);
    let month = Month::try_from(u8::try_from(date.month()).unwrap())
        .ok()
        .unwrap();

    let mut previous_dates: Vec<u32> = Vec::new();
    let mut following_dates: Vec<u32> = Vec::new();

    for i in start_date.unwrap().day()..=date.day() {
        previous_dates.push(i);
    }

    let days_in_month = days_in_month(date.year(), date.month());

    for i in date.day() + 1..=days_in_month {
        following_dates.push(i);
    }

    view! {
        <div class="main">
            <div id="calendar">
                <div id="year">
                { format!("{:?} {:?}", month, date.year()) }
                        <ul class="calendar-header">
                            <li>"Mon"</li>
                            <li>"Tue"</li>
                            <li>"Wed"</li>
                            <li>"Thu"</li>
                            <li>"Fri"</li>
                            <li>"Sat"</li>
                            <li>"Sun"</li>
                            { previous_dates.into_iter().map(|day| {
                                let today = day == Local::now().day();
                                let class_name = if today { "today" } else { "calendar-days" };
                                    view! { <li class={class_name}>{ day }</li> }
                            }).collect::<Vec<_>>()}
                            { following_dates.into_iter().map(|day| {
                                    view! { <li class="calendar-days">{ day }</li> }
                            }).collect::<Vec<_>>()}
                        </ul>
                </div>
            </div>
        </div>

        <button on:click=move |_| {
            spawn_local(async {
                let _ = book_slot("So much to do!".to_string()).await;
            });
        }>
            "Book Slot"
        </button>
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

#[server(BookSlot, "/api", "url", "book")]
pub async fn book_slot(event: String) -> Result<(), ServerFnError> {
    // use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    // use leptos_actix::ResponseOptions;

    // let response = expect_context::<ResponseOptions>();

    println!("{}", event);
    Ok(())
}

#[server(GetWeather, "/api", "GetJson", "weather")]
pub async fn get_weather() -> Result<(), ServerFnError> {
    println!("testing weather");
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
