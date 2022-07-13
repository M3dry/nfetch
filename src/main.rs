mod news;

use chrono::prelude::Utc;

#[tokio::main]
async fn main() {
    let news = news::get_news(
        std::env::var("NEWSAPI_KEY").unwrap(),
        vec!["apnews.com", "reuters.com"],
        &(Utc::today().and_hms(0, 0, 0)),
    )
    .await;

    for (i, new) in news.iter().enumerate() {
        if i < 5 {
            println!("{}\n", new.to_string())
        }
    }

    for (i, new) in news.iter().enumerate() {
        if i < 5 {
            println!("{}\n", new.to_html())
        }
    }
}
