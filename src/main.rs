use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up command-line argument parsing with clap
    let matches = Command::new("urlcount")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Counts unique URLs in GDELT Parquet files")
        .arg(Arg::new("INPUT")
            .help("Input file or directory path")
            .required(true)
            .index(1))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose output")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // Get the input path
    let input_path = PathBuf::from(matches.get_one::<String>("INPUT").unwrap());
    
    // Check if verbose mode is enabled
    let verbose = matches.get_flag("verbose");
    
    // Create a set to store unique URLs
    let mut unique_urls = HashSet::new();
    let mut file_count = 0;
    
    if input_path.is_file() {
        // Process single file
        if verbose {
            println!("Processing single file: {}", input_path.display());
        }
        process_parquet_file(&input_path, &mut unique_urls, verbose)?;
        file_count = 1;
    } else if input_path.is_dir() {
        // Process all parquet files in directory
        if verbose {
            println!("Processing all parquet files in directory: {}", input_path.display());
        }
        let entries = fs::read_dir(input_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
                if verbose {
                    println!("Processing file: {}", path.display());
                }
                process_parquet_file(&path, &mut unique_urls, verbose)?;
                file_count += 1;
            }
        }
    } else {
        eprintln!("Error: {} is neither a file nor a directory", input_path.display());
        std::process::exit(1);
    }
    
    // Print the summary
    if verbose {
        println!("\nSummary:");
        println!("  - Total files processed: {}", file_count);
        println!("  - Total unique URLs: {}", unique_urls.len());
    } else {
        println!("Processed {} files, found {} unique URLs", file_count, unique_urls.len());
    }
    
    Ok(())
}

fn process_parquet_file(file_path: &Path, unique_urls: &mut HashSet<String>, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Open the Parquet file
    let file = File::open(file_path)?;
    let reader = SerializedFileReader::new(file)?;
    
    // Get the file metadata
    let metadata = reader.metadata();
    let schema = metadata.file_metadata().schema();
    
    // Find the index of the SOURCEURL column
    let url_column_index = schema
        .get_fields()
        .iter()
        .position(|field| field.name() == "SOURCEURL")
        .ok_or(format!("SOURCEURL column not found in file: {}", file_path.display()))?;
    
    // Track statistics for this file
    let initial_unique_count = unique_urls.len();
    let mut file_urls_count = 0;
    
    // Read the file row by row
    let mut row_iter = reader.get_row_iter(None)?;
    
    while let Some(record) = row_iter.next() {
        let record = record?;
        
        // Get the URL value from the correct column
        if let Ok(url) = record.get_string(url_column_index) {
            unique_urls.insert(url.to_string());
            file_urls_count += 1;
        }
    }
    
    if verbose {
        let new_unique_urls = unique_urls.len() - initial_unique_count;
        println!("  - Found {} URLs in file", file_urls_count);
        println!("  - Added {} new unique URLs to the total", new_unique_urls);
    }
    
    Ok(())
}
