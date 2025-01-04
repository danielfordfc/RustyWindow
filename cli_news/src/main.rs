use std::error::Error;
use std::env;

use colour::{red, green};
use news_api::{fetch_news, Articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        red!("{}", i.title);
        green!("\n ---> {} \n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("NEWS_API_KEY")?; // add this value to your .env file in this directory
    let url = format!("https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey={}", api_key);
    let articles: Articles = fetch_news(&url)?;

    render_articles(&articles);

    Ok(())
}


