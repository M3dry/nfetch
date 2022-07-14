use alphavantage::{exchange_rate::ExchangeRate, time_series::Entry, Client};

#[derive(Debug)]
pub(crate) struct Stock {
    name: String,
    entry: Entry,
}

impl crate::Fmt for Stock {
    fn to_string(&self) -> String {
        format!(
            "{name} - OPEN: {open} HIGH: {high} LOW: {low} CLOSE: {close} VOLUME: {volume}\n",
            name = self.name,
            open = self.entry.open,
            high = self.entry.high,
            low = self.entry.low,
            close = self.entry.close,
            volume = self.entry.volume,
        )
    }

    fn to_html(&self) -> String {
        format!(
            r#"<div class="stock"><h2 class="company">{name}</h2><div class="OHLCV">OPEN: {open}<br>HIGH: {high}<br>LOW: {low}<br>CLOSE: {close}<br>VOLUME: {volume}</div></div>"#,
            name = self.name,
            open = self.entry.open,
            high = self.entry.high,
            low = self.entry.low,
            close = self.entry.close,
            volume = self.entry.volume,
        )
    }
}

impl crate::Fmt for ExchangeRate {
    fn to_string(&self) -> String {
        format!(
            "1 {from} = {rate} {to}\n",
            from = self.from.code,
            rate = self.rate,
            to = self.to.code
        )
    }

    fn to_html(&self) -> String {
        format!(
            r#"<div class="exchange_rate"><h2>1 {from} = {rate} {to}</h2></div>"#,
            from = self.from.code,
            rate = self.rate,
            to = self.to.code
        )
    }
}

pub(crate) async fn get_stocks(api_key: &String, stocks: Vec<String>) -> Vec<Stock> {
    let client = Client::new(api_key);
    let mut ret: Vec<Stock> = vec![];

    for stock in &stocks {
        ret.push(Stock {
            name: stock.to_string(),
            entry: client
                .get_time_series_daily(&stock)
                .await
                .unwrap()
                .entries
                .last()
                .unwrap()
                .clone(),
        })
    }

    ret
}

pub(crate) async fn get_currencies_rates(
    api_key: &String,
    from_to: Vec<Vec<String>>,
) -> Vec<ExchangeRate> {
    let client = Client::new(api_key);
    let mut ret: Vec<ExchangeRate> = vec![];

    for val in &from_to {
        ret.push(client.get_exchange_rate(&val[0], &val[1]).await.unwrap());
    }

    ret
}
