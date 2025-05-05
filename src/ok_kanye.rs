use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KanyeQuote {
    pub quote: String,
}

/// Fetch a random Kanye West quote from the API
pub async fn fetch_kanye_quote() -> Result<KanyeQuote, Error> {

    let url = "https://api.kanye.rest";

    let response = reqwest::get(url).await?;

    let quote: KanyeQuote = response.json().await?;

    println!("Kanye says: {}", quote.quote);

    Ok(quote)
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
 
    #[tokio::test]
    async fn test_fetch_kanye_quote() {
        let result = fetch_kanye_quote().await;
        assert!(result.is_ok());
        let quote = result.unwrap();
        assert!(!quote.quote.is_empty());
    }
}