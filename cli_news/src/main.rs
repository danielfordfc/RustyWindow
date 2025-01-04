use std::error::Error;
use std::env;

use colour::{red, green};
use news_api::{fetch_news, models::Articles};
use dotenv::dotenv;

// The serde crate is a popular Rust library for serializing and deserializing data structures.
// The Deserialize trait is used to define how a data structure should be deserialized from a given format, such as JSON or YAML.
// The derive attribute is used to automatically implement the Deserialize trait for the Articles and Article structs.

// Result<(), Box<dyn Error>> is a type that represents either success or failure.
// The Ok(()) value represents success, and the Err(ureq::Error) value represents failure.

// Box<dyn Error> is a type that represents a trait object that implements the Error trait.
// The Box type is a smart pointer that allows you to store values of any type that implements the Error trait on the heap.

// The ? operator is used to propagate errors. If the result of the call() method is an Err value, the ? operator will return the error from the function.

// String vs &str in Rust
// String is a growable, heap-allocated data structure that allows you to store and manipulate text data.
// &str is a string slice that points to a sequence of bytes in memory. It is a view into a String or a byte array.
// The main difference between String and &str is that String is mutable and growable, while &str is immutable and fixed-size.

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("NEWS_API_KEY")?;
    let url = format!("https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey={}", api_key);
    let articles: Articles = fetch_news(&url)?;

    for i in &articles.articles {
        red!("{}", i.title);
        green!("\n ---> {} \n\n", i.url);
    }

    Ok(())
}


