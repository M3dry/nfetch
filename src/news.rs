use chrono::{prelude::Utc, DateTime};
use newsapi::api::NewsAPIClient;
use newsapi::constants::SortMethod;
use newsapi::payload::article::{Article, Articles};
use regex::Regex;

pub(crate) struct News {
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
            description: match article.description {
                Some(x) => x,
                None => "".to_string(),
            },
        }
    }
}

impl crate::Fmt for News {
    fn to_string(&self) -> String {
        format!(
            "title: {title}\nsource: {source}\nurl: {url}\nabout: {description}\n",
            title = self.title,
            source = self.source,
            url = self.url,
            description = self.description
        )
    }

    fn to_html(&self) -> String {
        format!(
            r#"<div class="article"><h2 class="title">{title}</h2><h3 class="source">{source}</h3><a class="url" href="{url}">{url}</a><p class="about">{description}</p></div>"#,
            title = self.title,
            source = self.source,
            url = self.url,
            description = self.description
        )
    }
}

pub(crate) async fn get_news(
    api_key: &String,
    domains: Vec<String>,
    from: &DateTime<Utc>,
) -> Vec<News> {
    NewsAPIClient::new(api_key.to_string())
        .domains(domains.iter().map(|s| &**s).collect())
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
