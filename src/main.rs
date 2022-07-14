mod config;
mod news;
mod stocks;

use chrono::prelude::Utc;
use std::io::Write;

pub(crate) trait Fmt {
    fn to_string(&self) -> String;
    fn to_html(&self) -> String;
}

struct Active {
    news: bool,
    stocks: bool,
    currencies: bool,
}

impl Active {
    pub(crate) fn new(conf: &config::Conf) -> Active {
        Active {
            news: match conf.news {
                None => false,
                Some(_) => true,
            },
            stocks: match conf.stock_companies {
                None => false,
                Some(_) => true,
            },
            currencies: match conf.currencies {
                None => false,
                Some(_) => true,
            },
        }
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
    let active = Active::new(&conf);

    if active.news {
        let news = news::get_news(
            &conf.keys.news,
            match &conf.news {
                Some(x) => x.domains.to_vec(),
                None => vec!["".to_string()],
            },
            &Utc::today().and_hms(0, 0, 0),
        )
        .await;

        for (i, new) in news.iter().enumerate() {
            if let Some(x) = &conf.news {
                if i as i32 == x.number_of_articles {
                    break;
                }
            }

            println!("{}\n", new.to_string());

            if let Err(e) = writeln!(html_file, "{}", new.to_html()) {
                eprintln!("Couldn't write to a html file: {e}");
            }
        }
    }

    if active.stocks {
        let stonks = stocks::get_stocks(
            &conf.keys.stocks,
            match &conf.stock_companies {
                Some(x) => x.to_vec(),
                None => vec!["".to_string()],
            },
        )
        .await;

        for stonk in &stonks {
            println!("{}", stonk.to_string());

            if let Err(e) = writeln!(html_file, "{}", stonk.to_html()) {
                eprintln!("Couldn't write to a html file: {e}");
            }
        }
    }
    if active.currencies {
        let exchange_rates = stocks::get_currencies_rates(
            &conf.keys.stocks,
            match &conf.currencies {
                Some(x) => x.to_vec(),
                None => vec![vec!["".to_string()]]
            },
        )
        .await;

        println!();
        for exchange_rate in &exchange_rates {
            println!("{}", exchange_rate.to_string());

            if let Err(e) = writeln!(html_file, "{}", exchange_rate.to_html()) {
                eprintln!("Couldn't write to a html file: {e}");
            }
        }
    }
}
