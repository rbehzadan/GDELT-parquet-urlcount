use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use clap::{Arg, Command};
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up command-line argument parsing with clap
    let matches = Command::new("urlcount")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
    
    if input_path.is_file() {
        // Process single file
        if verbose {
            println!("Processing single file: {}", input_path.display());
        }
        process_parquet_file(&input_path, &mut unique_urls, verbose)?;
        
        // Print the summary for a single file
        if verbose {
            println!("\nSummary:");
            println!("  - Total files processed: 1");
            println!("  - Total unique URLs: {}", unique_urls.len());
        } else {
            println!("Processed 1 file, found {} unique URLs", unique_urls.len());
        }
    } else if input_path.is_dir() {
        // Process all parquet files in directory
        // First, count the total number of parquet files for the progress bar
        let total_files = count_parquet_files(&input_path)?;
        
        if total_files == 0 {
            println!("No parquet files found in the directory.");
            return Ok(());
        }
        
        if verbose {
            println!("Processing all parquet files in directory: {}", input_path.display());
            println!("Found {} parquet files to process", total_files);
        }
        
        // Set up a progress bar if not in verbose mode
        let progress_bar = if !verbose {
            let pb = ProgressBar::new(total_files);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files ({percent}%) - {elapsed_precise}")
                .unwrap()
                .progress_chars("#>-"));
            Some(pb)
        } else {
            None
        };
        
        // Process the files
        let entries = fs::read_dir(input_path)?;
        let mut processed_files = 0;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
                if verbose {
                    println!("Processing file: {}", path.display());
                }
                process_parquet_file(&path, &mut unique_urls, verbose)?;
                processed_files += 1;
                
                // Update the progress bar if we have one
                if let Some(pb) = &progress_bar {
                    pb.set_position(processed_files);
                }
            }
        }
        
        // Finish and clear the progress bar if we have one
        if let Some(pb) = progress_bar {
            pb.finish_and_clear();
        }
        
        // Print the summary
        if verbose {
            println!("\nSummary:");
            println!("  - Total files processed: {}", processed_files);
            println!("  - Total unique URLs: {}", unique_urls.len());
        } else {
            println!("Processed {} files, found {} unique URLs", processed_files, unique_urls.len());
        }
    } else {
        eprintln!("Error: {} is neither a file nor a directory", input_path.display());
        std::process::exit(1);
    }
    
    Ok(())
}

fn count_parquet_files(dir_path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?;
    let mut count = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
            count += 1;
        }
    }
    
    Ok(count)
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
