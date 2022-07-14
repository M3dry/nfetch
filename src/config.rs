use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Keys {
    pub(crate) news: String,
    pub(crate) stocks: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct NewsConf {
    pub(crate) domains: Vec<String>,
    pub(crate) number_of_articles: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Conf {
    pub(crate) html: String,
    pub(crate) keys: Keys,
    pub(crate) news: Option<NewsConf>,
    pub(crate) stock_companies: Option<Vec<String>>,
    pub(crate) currencies: Option<Vec<Vec<String>>>,
}

impl Conf {
    pub(crate) fn new() -> Conf {
        let filename = Conf::get_config_path();
        serde_json::from_str::<Conf>(
            &std::fs::read_to_string(&filename).expect("Opening config didn't work"),
        )
        .unwrap_or(Conf {
            html: "nfetch.html".to_string(),
            keys: Keys {
                news: "".to_string(),
                stocks: "".to_string(),
            },
            news: Some(NewsConf {
                domains: vec!["apnews.com".to_string(), "reuters.com".to_string()],
                number_of_articles: 10,
            }),
            stock_companies: Some(vec!["AAPL".to_string(), "GOOG".to_string()]),
            currencies: Some(vec![vec!["CZK".to_string(), "EUR".to_string()]]),
        })
        .check_keys(filename)
    }

    fn check_keys(self, filename: PathBuf) -> Self {
        if self.keys.news == "" || self.keys.stocks == "" {
            eprintln!("Please add missing api key");
            self.save(filename);
            std::process::exit(-1)
        }

        self
    }

    fn get_config_path() -> PathBuf {
        xdg::BaseDirectories::with_prefix("nfetch").unwrap().place_config_file("config.json").expect("Can't create config directory")
    }

    pub(crate) fn save(&self, filename: PathBuf) {
        std::fs::write(filename, serde_json::to_string_pretty(self).unwrap())
            .expect("Saving config failed");
    }
}
