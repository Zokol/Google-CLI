# Google CLI Search Tool

A versatile Rust-based command-line tool to conduct advanced Google searches

## Design

1. **Google Search**: At its core, this tool harnesses the power of Google search from the command line, allowing users to input queries and retrieve results without needing a browser.

2. **Google Dorking**: This is a standout feature, enabling users to perform advanced search techniques. The tool provides parameters that align with typical "dorking" operations, such as site-specific searches, in-url and in-text, and filtering by file type.

3. **Download Functionality**: The tool goes beyond just listing URLs. If the search result points to a direct file (like a PDF), it will be downloaded. Web pages, on the other hand, will be converted to a PDF representation and then downloaded.

## Usage

1. **Installing**:

   - Clone the repository: `git clone <repository_url>`
   - Navigate into the project directory: `cd Google-CLI`
   - Build the project: `cargo build --release`

2. **Running the Tool**:

   Basic command:

   ```bash
   ./target/release/google-cli -q "<search_query>"
   ```

   Advanced command with Google dorking:

   ```bash
   ./target/release/google-cli -q "<search_query>" --filetype pdf --site example.com -o ./downloads/
   ```

   - `-q` or `--query`: Specifies the search term.
   - `--filetype`: Allows you to filter results by file type.
   - `--site`: Restricts the search to a particular site.
   - `-o` or `--output`: Specifies the output directory for downloads.

## Output

1. **Search Results**: Displays a list of URLs corresponding to the Google search results, refined based on provided dorking parameters.

2. **Downloads**: If an output directory is set, the tool will store search results therein. Direct links to files (like PDFs) are directly downloaded, while web pages are saved as their PDF representation.

## Contributing

If you'd like to contribute to the project, pull requests are very much appreciated. For significant changes, it's always a good idea to open an issue first to chat about what you'd like to modify. Please ensure that tests are updated as needed.

## License

[MIT](https://choosealicense.com/licenses/mit/)

---

Again, remember to replace `<repository_url>` with the actual URL of your repository and adjust the usage section if there are other specific details or configurations pertaining to your implementation.