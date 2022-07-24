mod config;
mod news;
mod stocks;

use chrono::prelude::Utc;
use std::io::Write;

pub(crate) trait Fmt {
    fn to_string(&self) -> String;
    fn to_html(&self) -> String;
    fn feed(&self, str: &mut String, html: &mut String) {
        str.push_str(&format!("{}", self.to_string()));
        html.push_str(&format!("{}", self.to_html()));
    }
}

#[tokio::main]
async fn main() {
    let conf = config::Conf::new();
    std::fs::File::create(&conf.html).expect("Can't create a file");
    let mut html_file = std::fs::File::options()
        .append(true)
        .open(&conf.html)
        .expect("Can't write to a file");
    let mut feed_str: String = String::new();
    let mut feed_html: String = String::from(r#"<link rel="stylesheet" href="style.css"> "#);
    let news;
    let stonks;
    let exchange_rates;

    news = news::get_news(
        &conf.keys.news,
        match &conf.news {
            Some(x) => Some(&x.domains),
            None => None,
        },
        Utc::today().and_hms(0, 0, 0),
    );
    stonks = stocks::get_stocks(
        &conf.keys.stocks,
        match &conf.stock_companies {
            Some(x) => Some(x),
            None => None,
        },
    );

    exchange_rates = stocks::get_currencies_rates(
        &conf.keys.stocks,
        match &conf.currencies {
            Some(x) => Some(x),
            None => None,
        },
    );

    let (news, stonks, exchange_rates) = tokio::join!(news, stonks, exchange_rates);

    for (i, new) in news.iter().enumerate() {
        if let Some(x) = &conf.news {
            if i as i32 == x.number_of_articles {
                break;
            }
        }

        new.feed(&mut feed_str, &mut feed_html);
    }

    for stonk in &stonks {
        stonk.feed(&mut feed_str, &mut feed_html);
    }

    if stonks.len() != 0 {
        feed_str.push_str(&format!("\n"));
    }

    for exchange_rate in &exchange_rates {
        exchange_rate.feed(&mut feed_str, &mut feed_html);
    }

    print!("{}", feed_str);
    if let Err(e) = writeln!(html_file, "{}", feed_html) {
        eprintln!("Couldn't write to a html file: {e}");
    }
}
