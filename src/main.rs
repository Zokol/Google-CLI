use clap::{App, Arg};
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::convert::From;
use indicatif::ProgressBar;

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

#[derive(Debug)]
struct MyError(reqwest::Error);

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError(error)
    }
}

impl Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const GOOGLE_SEARCH_API: &str = "https://www.googleapis.com/customsearch/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parsing command line arguments
    let matches = App::new("Google CLI")
        .version("1.2")
        .author("Heikki Juva <heikki@juva.lu>")
        .about("A command line tool for searching Google")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("Specifies the search term")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("filetype")
                .short("f")
                .long("filetype")
                .value_name("FILETYPE")
                .help("Allows you to filter results by file type")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .value_name("DOMAIN")
                .help("Restricts the search to a particular domain")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("download")
                .short("o")
                .long("output")
                .help("Enables file downloading and specifies the output directory. Converts HTML-files into PDF. Downloads PDF, TXT, MD, JPG, PNG, GIF, MP3, MP4, WAV files. To download all files regardless of the file type, use the --unsafe flag")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("unsafe")
                .long("unsafe")
                .help("Downloads all files regardless of the file type. NOTE: This can be dangerous!")
                .takes_value(true),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();

    // Reading the environment variables
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY not set");
    let search_engine_id = env::var("SEARCH_ENGINE_ID").expect("SEARCH_ENGINE_ID not set");

    // Prepare the client and make a request
    let client = reqwest::Client::new();
    let mut request = client.get(GOOGLE_SEARCH_API)
        .query(&[("key", &api_key), ("cx", &search_engine_id), ("q", &query.to_string())]);

    // Add filetype and site parameters if specified
    if let Some(filetype) = matches.value_of("filetype") {
        request = request.query(&[("fileType", filetype)]);
    }

    if let Some(site) = matches.value_of("domain") {
        request = request.query(&[("siteSearch", site)]);
    }

    let response = request.send().await?;

    // Deserialize the response
    let google_results: GoogleSearchResult = response.json().await?;

    // Map Google's search results to our application's structure
    let search_results: Vec<SearchResult> = google_results.items.into_iter().map(|item| {
        let title = item.title;
        let link = item.link;
        let description = item.snippet;

        SearchResult { title, link, description }
    }).collect();

    

    // Download each search result if the download flag is set
    if matches.is_present("download") {

        // Create a new progress bar with a total count of downloaded files
        let progress_bar = ProgressBar::new(search_results.len() as u64);

        let output_dir = matches.value_of("download").unwrap_or(".");
        for result in search_results {
            progress_bar.inc(1);
            match client.get(&result.link).send().await {
                Ok(response) => {
                    let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();

                    let ext = content_type.split('/').nth(1).unwrap_or("unknown");
                    let allowed_exts = ["pdf", "txt", "md", "jpg", "png", "gif", "mp3", "mp4", "wav"];
                    let is_allowed = allowed_exts.iter().any(|&e| e == ext);

                    // Only download files that are allowed or if the unsafe flag is set
                    if matches.is_present("unsafe") || is_allowed {
                        let filename = format!("{}/{}.{}", output_dir, result.title, ext);
                        let mut file = File::create(&filename)?;
                        let content = response.bytes().await?;
                        file.write_all(&content)?;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
            
        }

        // Finish the progress bar
        progress_bar.finish();
    } else {
        // Print each search result to the console
        for result in search_results {
            println!("{}\n{}\n{}\n", result.title, result.link, result.description);
        }
    }

    Ok(())
}

