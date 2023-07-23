use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct GoogleSearchItem {
    title: String,
    link: String,
    snippet: String,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchResult {
    items: Vec<GoogleSearchItem>,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    title: String,
    link: String,
    description: String,
}

const GOOGLE_SEARCH_API: &str = "https://www.googleapis.com/customsearch/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reading command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a search phrase.");
        std::process::exit(1);
    }

    let query = &args[1];

    // Reading the environment variables
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not set");
    let search_engine_id = env::var("SEARCH_ENGINE_ID").expect("SEARCH_ENGINE_ID not set");

    // Prepare the client and make a request
    let client = reqwest::Client::new();
    let response = client.get(GOOGLE_SEARCH_API)
        .query(&[("key", &api_key), ("cx", &search_engine_id), ("q", query)])
        .send()
        .await?;

    // Deserialize the response
    let google_results: GoogleSearchResult = response.json().await?;

    // Map Google's search results to our application's structure
    let search_results: Vec<SearchResult> = google_results.items.into_iter().map(|item| {
        SearchResult {
            title: item.title,
            link: item.link,
            description: item.snippet,
        }
    }).collect();

    // Serialize our search results to JSON (pretty-printed)
    let json_output = serde_json::to_string_pretty(&search_results)?;
    println!("{}", json_output);

    Ok(())
}
