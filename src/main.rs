mod news;
mod stocks;

use chrono::prelude::Utc;
use std::env;

struct Keys {
    news: String,
    stocks: String,
}

pub(crate) trait Fmt {
    fn to_string(&self) -> String;
    fn to_html(&self) -> String;
}

#[tokio::main]
async fn main() {
    let keys = Keys {
        news: env::var("NEWSAPI_KEY").unwrap(),
        stocks: env::var("ALPHAVANTAGE_KEY").unwrap(),
    };
    let news = news::get_news(
        &keys.news,
        vec!["apnews.com", "reuters.com"],
        &Utc::today().and_hms(0, 0, 0),
    )
    .await;
    let stonks =
        stocks::get_stocks(&keys.stocks, vec!["GOOG".to_string(), "AAPL".to_string()]).await;
    let exchange_rates = stocks::get_currencies_rates(
        &keys.stocks,
        vec!["CZK".to_string(), "USD".to_string()],
        vec!["EUR".to_string(), "CZK".to_string()],
    )
    .await;

    for (i, new) in news.iter().enumerate() {
        if i < 5 {
            println!("{}\n", new.to_string())
        }
    }

    for stonk in &stonks {
        println!("{}", stonk.to_string());
    }

    for exchange_rate in &exchange_rates {
        println!("{}", exchange_rate.to_string());
    }


    for (i, new) in news.iter().enumerate() {
        if i < 5 {
            println!("{}\n", new.to_html())
        }
    }

    for stonk in &stonks {
        println!("{}", stonk.to_html());
    }

    for exchange_rate in &exchange_rates {
        println!("{}", exchange_rate.to_html());
    }
}
