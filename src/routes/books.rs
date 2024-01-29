use chrono::NaiveDate;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Book {
    id: u32,
    title: String,
    author: String,
    rating: f32,
    finished: bool,
    finish_date: Option<NaiveDate>,
}

#[component]
pub fn Books() -> impl IntoView {
    let books = vec![
        Book {
            id: 1,
            title: "Sapiens".to_string(),
            author: "Yuval Noah Harari".to_string(),
            rating: 9.5,
            finished: true,
            finish_date: NaiveDate::from_ymd_opt(2020, 5, 23),
        },
        Book {
            id: 2,
            title: "Legacy".to_string(),
            author: "James Kerr".to_string(),
            rating: 7.0,
            finished: true,
            finish_date: NaiveDate::from_ymd_opt(2020, 6, 13),
        },
    ];

    view! {
        <div class="main">
            <div id="books">
            {books.into_iter()
                .map(|book|
                     view! {
                        <div class="book-card">
                            <p class="title">{book.title}</p>
                            <p class="author">by {book.author}</p>
                            <p class="rating">Rating: {book.rating}</p>
                            <p class="finish_date">Finished: {format!("{:?}", book.finish_date.unwrap())}</p>
                        </div>
                    })
                .collect_view()
            }
            </div>
        </div>
    }
}

#[server]
pub async fn get_books() -> Result<Vec<Book>, ServerFnError> {
    let mut books = Vec::<Book>::new();
    let book = Book {
        id: 1,
        title: "Mindset".to_string(),
        author: "Carol Dweck".to_string(),
        rating: 4.5,
        finished: false,
        finish_date: None,
    };

    books.push(book);

    Ok(books)
}
