mod analysis;
mod cli;
mod extractor;
mod finder;
mod utils;

use clap::Parser;
use cli::CliArgs;
use cli::Config;
use std::path::Path;
use utils::{process_movies, process_shows};

use std::fs;
use std::process::Command;

fn ensure_ffmpeg_installed() {
    let ffmpeg_check = Command::new("ffmpeg")
        .arg("-version")
        .output();

    match ffmpeg_check {
        Ok(output) if output.status.success() => {}
        _ => {
            eprintln!("ffmpeg is not installed. Please install ffmpeg and try again.");
            std::process::exit(1);
        }
    }
}

/// Recursively delete all "Backdrops" folders inside the given directory
fn delete_backdrops(dir: &str) {
    let path = Path::new(dir);

    if !path.exists() {
        eprintln!("Path does not exist: {}", dir);
        return;
    }

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.file_name() == "Backdrops" {
            println!("Deleting: {}", entry.path().display());
            if let Err(err) = fs::remove_dir_all(entry.path()) {
                eprintln!("Failed to delete {}: {}", entry.path().display(), err);
            }
        }
    }
}

fn main() {
    ensure_ffmpeg_installed();
    let args = CliArgs::parse();
    let config = Config::from_args_and_file(args);

    if config.delete {
        if let Some(movies_dir) = &config.movies_directory {
            delete_backdrops(movies_dir);
        }
        if let Some(shows_dir) = &config.shows_directory {
            delete_backdrops(shows_dir);
        }
        return;
    }

    if let Some(movies_dir) = &config.movies_directory {
        process_movies(movies_dir, &config);
    }

    if let Some(shows_dir) = &config.shows_directory {
        process_shows(shows_dir, &config);
    }
}
