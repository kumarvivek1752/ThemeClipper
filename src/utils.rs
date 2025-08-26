use crate::cli::Config;
use crate::extractor;
use crate::finder;

use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};

/// Helper: Create styled progress bar
fn create_progress_bar(len: usize, msg: &str) -> ProgressBar {
    let progress_bar = ProgressBar::new(len as u64);
    let style = ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.magenta/blue}] {percent:>3}% ({pos}/{len}) {msg}",
        )
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  ");
    progress_bar.set_style(style);
    progress_bar.println(msg.to_string());
    progress_bar
}

///Process Movie directory
pub fn process_movies(movies_dir: &str, config: &Config) {
    let movies = finder::find_movie_files(movies_dir);
    let progress_bar =
        create_progress_bar(movies.len(), &format!("Scanning movies in: {}", movies_dir));

    progress_bar.println(format!("Found {} movie(s)", movies.len()));

    for (folder, file) in movies {
        progress_bar.println(format!("Processing: {}", file));

        let backdrop_dir = Path::new(&folder).join("Backdrops");

        if !backdrop_dir.exists() {
            progress_bar.println(format!("Creating Backdrops folder at {:?}", backdrop_dir));
            if let Err(e) = fs::create_dir_all(&backdrop_dir) {
                progress_bar.println(format!("Failed to create Backdrops folder: {}", e));
                continue;
            }
        } else {
            progress_bar.println(format!("Exists Backdrops folder at {:?}", backdrop_dir));
            progress_bar.println("Skipping..");
            continue;
        }

        let output_path = backdrop_dir.join("theme.mp4");

        let start = extractor::get_random_clip_start(600, config.clip_length, 180, 0.5);

        let extractor_result = extractor::extract_clip(
            &file,
            output_path.to_str().unwrap(),
            start,
            config.clip_length,
        );
        match extractor_result {
            Ok(_) => progress_bar.println(format!("Saved clip: {:?}", output_path)),
            Err(e) => progress_bar.println(format!("Failed to extract clip from {}: {}", file, e)),
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
}

/// Process shows directory (series)
pub fn process_shows(shows_dir: &str, config: &Config) {
    let shows_path = Path::new(shows_dir);

    if !shows_path.exists() {
        eprintln!("Shows directory does not exist: {}", shows_dir);
        return;
    }
    let shows: Vec<PathBuf> = fs::read_dir(shows_path)
        .expect("Failed to read shows directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    let progress_bar =
        create_progress_bar(shows.len(), &format!("Scanning shows in: {}", shows_dir));
    progress_bar.println(format!("Found {} show(s)", shows.len()));

    for show_path in shows {
        progress_bar.println(format!("Processing show: {}", show_path.display()));

        // Find "Season XX" subfolders
        let mut seasons: Vec<PathBuf> = fs::read_dir(&show_path)
            .expect("Failed to read show folder")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.is_dir()
                    && p.file_name()
                        .map(|n| {
                            n.to_string_lossy()
                                .to_lowercase()
                                .starts_with("season")
                        })
                        .unwrap_or(false)
            })
            .collect();

        seasons.sort();
        seasons.reverse();

        if let Some(latest_season) = seasons.first() {
            progress_bar.println(format!("Latest season: {}", latest_season.display()));

            // Find Episode 01
            if let Some(episode1) = fs::read_dir(latest_season)
                .expect("Failed to read season folder")
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_file())
                .find(|p| {
                    p.file_name()
                        .map(|n| {
                            n.to_string_lossy()
                                .to_lowercase()
                                .contains("e01")
                        })
                        .unwrap_or(false)
                })
            {
                progress_bar.println(format!("Selected episode 1: {}", episode1.display()));

                let backdrop_dir = show_path.join("Backdrops");
                if !backdrop_dir.exists() {
                    progress_bar
                        .println(format!("Creating Backdrops folder: {}", backdrop_dir.display()));
                    fs::create_dir_all(&backdrop_dir).expect("Failed to create Backdrops folder");
                } else {
                    progress_bar.println(format!("Exists Backdrops folder at {:?}", backdrop_dir));
                    progress_bar.println("Skipping..");
                    continue;
                }

                let theme_file = backdrop_dir.join("theme.mp4");
                let start = extractor::get_random_clip_start(600, config.clip_length, 180, 0.5);

                progress_bar.println(format!("Extracting theme to: {}", theme_file.display()));
                match extractor::extract_clip(
                    episode1.to_str().unwrap(),
                    theme_file.to_str().unwrap(),
                    start,
                    config.clip_length,
                ) {
                    Ok(_) => progress_bar.println(format!("Saved clip: {:?}", theme_file)),
                    Err(e) => progress_bar.println(format!(
                        "Failed to extract clip from {}: {}",
                        episode1.display(),
                        e
                    )),
                }
            }
        }

        progress_bar.inc(1);
    }
    progress_bar.finish();
}
