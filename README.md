# Google CLI Search Tool

A versatile Rust-based command-line tool to conduct advanced Google searches

## Design

1. **Google Search**: At its core, this tool harnesses the power of Google search from the command line, allowing users to input queries and retrieve results without needing a browser.

2. **Google Dorking**: This is a standout feature, enabling users to perform advanced search techniques. The tool provides parameters that align with typical "dorking" operations, such as site-specific searches and filtering by file type.

3. **Download Functionality**: The tool goes beyond just listing URLs. If the search result points to a direct file (like a PDF), it will be downloaded.

## Usage

1. **Installing**:

   - Clone the repository: `git clone https://github.com/Zokol/Google-CLI`
   - Navigate into the project directory: `cd Google-CLI`
   - Build the project: `cargo build --release`

2. **Running the Tool**:

   Basic command:

   ```bash
   ./target/release/google-cli -q "testing"
   ```

   Search PDF-files:

   ```bash
   ./target/release/google-cli -q "testing" -f pdf
   ```

   Search PDF-files from example.com:

   ```bash
   ./target/release/google-cli -q "testing" -f pdf -s example.com
   ```

   Search PDF-files from example.com and save all resulting files to downloads-folder:

   ```bash
   ./target/release/google-cli -q "testing" -f pdf -s example.com -o ./downloads/
   ```

   - `-q` or `--query`: Specifies the search term.
   - `-f` or `--filetype`: Allows you to filter results by file type.
   - `-s` or `--site`: Restricts the search to a particular site.
   - `-o` or `--output`: Specifies the output directory for downloads.
   - `--unsafe`: Allows download of any filetype.

## Output

1. **Search Results**: Displays a list of URLs corresponding to the Google search results.

2. **Downloads**: If an output directory is set, the tool will store search results therein. Direct links to files (like PDFs) are directly downloaded, while web pages are saved as their PDF representation.

## Contributing

If you'd like to contribute to the project, pull requests are very much appreciated. For significant changes, it's always a good idea to open an issue first to chat about what you'd like to modify. Please ensure that tests are updated as needed.

## License

[MIT](https://choosealicense.com/licenses/mit/)

>>>>>>> download
