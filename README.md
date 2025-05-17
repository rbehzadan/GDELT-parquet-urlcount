# GDELT-parquet-urlcount

A fast Rust utility for counting unique URLs in GDELT Parquet files. This tool complements [gdelt-fetcher](https://github.com/rbehzadan/gdelt-fetcher) for analyzing URL data from previously collected GDELT datasets.

## Features

- Count unique URLs in GDELT Parquet files
- Process individual files or entire directories
- Display progress with a customizable progress bar
- Static compilation options for maximum portability
- Cross-platform support (AMD64/ARM64)

## Installation

### From Releases

Download the pre-built binary for your platform from the [Releases](https://github.com/rbehzadan/GDELT-parquet-urlcount/releases) page.

### From Source

```bash
# Clone the repository
git clone https://github.com/rbehzadan/GDELT-parquet-urlcount.git
cd GDELT-parquet-urlcount

# Build with Cargo
cargo build --release

# The binary will be in target/release/urlcount
```

## Usage

```bash
# Get help
urlcount --help

# Process a single file
urlcount path/to/file.parquet

# Process a directory of Parquet files
urlcount path/to/directory/

# Enable verbose output
urlcount -v path/to/directory/
```

## Examples

```bash
# Count unique URLs in a single file with verbose output
$ urlcount -v gdelt_data/20240310.parquet
Processing file: gdelt_data/20240310.parquet
  - Found 57829 URLs in file
  - Added 57829 new unique URLs to the total

Summary:
  - Total files processed: 1
  - Total unique URLs: 57829

# Count unique URLs across all files in a directory
$ urlcount gdelt_data/
[██████████████████████████████████] 12/12 files (100%) - 00:01:07
Processed 12 files, found 211245 unique URLs
```

## Building for Different Platforms

This project supports cross-compilation for various platforms:

```bash
# For AMD64 Linux
cargo build --release --target x86_64-unknown-linux-gnu

# For ARM64 Linux (using cross)
cross build --release --target aarch64-unknown-linux-musl
```

## Related Projects

If you need to fetch and convert GDELT CSV data to Parquet format, check out [gdelt-fetcher](https://github.com/rbehzadan/gdelt-fetcher), a Python tool that automates downloading GDELT updates and converting them to Parquet files.

## License

MIT

## About GDELT

The [GDELT Project](https://www.gdeltproject.org/) monitors the world's news media and creates a free open platform for analyzing global events. The Parquet files processed by this tool contain event data with source URLs from global news coverage.
