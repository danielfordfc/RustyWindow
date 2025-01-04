# news-api/README.md

# My Rust Library

This library provides functionality to fetch and deserialize news articles from a specified URL. It utilizes the `serde` library for serialization and deserialization of data structures.

## Features

- Fetch news articles from a given URL.
- Deserialize JSON data into Rust structs.

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
news-api = { path = "../news-api" }
serde = { version = "1.0", features = ["derive"] }
ureq = "2.3"
```

## Usage

Here is a basic example of how to use the library:

```rust
use my_rust_library::fetch_news;

fn main() {
    let url = "https://example.com/news";
    match fetch_news(url) {
        Ok(articles) => {
            for article in articles.articles {
                println!("Title: {}, URL: {}", article.title, article.url);
            }
        }
        Err(e) => eprintln!("Error fetching news: {}", e),
    }
}
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.