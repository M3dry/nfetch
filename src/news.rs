use chrono::{prelude::Utc, DateTime};
use newsapi::api::NewsAPIClient;
use newsapi::constants::SortMethod;
use newsapi::payload::article::{Article, Articles};
use regex::Regex;

pub struct News {
    title: String,
    source: String,
    url: String,
    description: String,
}

impl From<Article> for News {
    fn from(article: Article) -> Self {
        News {
            title: article.title,
            source: article.source.name,
            url: article.url,
            description: match &article.description {
                Some(x) => String::from(Regex::new("/<.*>.*</.*>").unwrap().replace_all(x, "")),
                None => "".to_string(),
            },
        }
    }
}

impl News {
    pub fn to_string(&self) -> String {
        format!(
            "title: {title}\nsource: {source}\nurl: {url}\nabout: {description}\n",
            title = self.title,
            source = self.source,
            url = self.url,
            description = self.description
        )
    }

    pub fn to_html(&self) -> String {
        format!(
            r#"<div class="article"><h2 class="title">{title}</h2><h3 class="source">{source}</h3><a class="url" href="{url}">{url}</a><p class="about">{description}</p></div>"#,
            title = self.title,
            source = self.source,
            url = self.url,
            description = self.description
        )
    }
}

pub async fn get_news(api: String, domains: Vec<&str>, from: &DateTime<Utc>) -> Vec<News> {
    NewsAPIClient::new(api)
        .domains(domains)
        .from(from)
        .sort_by(SortMethod::PublishedAt)
        .everything()
        .send_async::<Articles>()
        .await
        .unwrap()
        .articles
        .into_iter()
        .map(|x| -> News { News::from(x) })
        .collect()
}
