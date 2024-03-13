mod api;

use api::message::*;
use clap::Parser;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(
    name = "message-info-localizer",
    version = "0.1.0",
    author = "dei",
    about = "A tool to localize messageInfo strings to other languages."
)]
struct Args {
    #[clap(short, long)]
    auth_key: String,
    #[clap(short, long)]
    dir: String,
    #[clap(short, long)]
    source_lang: String,
}

fn main() {
    let args = Args::parse();

    // Make sure the key is set and valid
    if args.auth_key.is_empty() {
        eprintln!("No Authentication Key provided. Exiting...");
        std::process::exit(1);
    }

    let paths = collect_files(Path::new(args.dir.as_str()));

    add_translations(paths, args.source_lang.as_str(), args.auth_key.as_str());

    
    // Print the Deepl usage statistics
    println!(
        "Usage statistics: {}",
        api::deepl::usage_statistics(args.auth_key.as_str())
    );
}

fn collect_files(directory: &Path) -> Vec<String> {
    let mut files = Vec::new();

    for entry in WalkDir::new(directory).follow_links(true) {
        match entry {
            Ok(entry) => {
                // Also only collect .xfbin files
                if entry.file_type().is_file() && entry.path().extension().unwrap() == "xfbin" {
                    files.push(entry.path().to_path_buf());
                }
            }
            Err(e) => eprintln!("Error accessing entry: {}", e),
        }
    }

    files
        .iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
}
