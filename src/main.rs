use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

// Define the data structures we expect from Google Search API.
#[derive(Debug, Deserialize)]
struct GoogleSearchResult {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    link: String,
    title: String,
    snippet: String,
}

// This is what we will serialize to JSON as output.
#[derive(Debug, Serialize)]
struct SearchResult {
    order: usize,
    url: String,
    title: String,
    content_preview: String,
}

const GOOGLE_API_URL: &str = "https://www.googleapis.com/customsearch/v1";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch your API key and search engine ID from environment variables.
    let api_key = env::var("GOOGLE_API_KEY")?;
    let search_engine_id = env::var("SEARCH_ENGINE_ID")?;

    // Here, for demonstration purposes, we'll just search for "rust programming".
    let query = "rust programming";

    let resp: GoogleSearchResult = reqwest::blocking::get(&format!(
        "{}?key={}&cx={}&q={}",
        GOOGLE_API_URL, api_key, search_engine_id, query
    ))?
    .json()?;

    // Process results to create our custom SearchResult structs.
    let search_results: Vec<SearchResult> = resp.items.iter().enumerate().map(|(index, item)| {
        SearchResult {
            order: index + 1,
            url: item.link.clone(),
            title: item.title.clone(),
            content_preview: item.snippet.chars().take(100).collect(),
        }
    }).collect();

    // Serialize our search results to JSON.
    let output = serde_json::to_string_pretty(&search_results)?;
    println!("{}", output);

    Ok(())
}
