use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: f64,
}

pub async fn fetch_prices_from_coingecko(symbols: &[String]) -> Result<Vec<PriceData>, reqwest::Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        symbols.join(",")
    );

    let response = reqwest::get(&url).await?;
    let prices: HashMap<String, HashMap<String, f64>> = response.json().await?;

    let price_data = prices
        .into_iter()
        .map(|(symbol, price_map)| PriceData {
            symbol,
            price: price_map.get("usd").cloned().unwrap_or(0.0),
        })
        .collect();

    Ok(price_data)
}

pub async fn fetch_sentiment_from_lunarcrush(symbol: &str) -> Result<f64, reqwest::Error> {
    let url = format!(
        "https://lunarcrush.com/api3/coinsentiment?symbol={}",
        symbol
    );

    let response = reqwest::get(&url).await?;
    let sentiment_data: serde_json::Value = response.json().await?;

    let sentiment = sentiment_data
        .get("data")
        .and_then(|data| data.get(0))
        .and_then(|data| data.get("sentiment"))
        .and_then(|sentiment| sentiment.as_f64())
        .unwrap_or(0.0);

    Ok(sentiment)
}