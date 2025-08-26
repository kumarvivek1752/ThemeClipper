// use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_movie_files(base_path: &str) -> Vec<(String, String)> {
    let mut movies = Vec::new();
    let exts = ["mp4", "mkv", "mov", "avi", "flv", "wmv", "webm", "m4v"];

    for entry in WalkDir::new(base_path)
        .min_depth(1)
        .max_depth(2)
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if let Some(ext) = path
            .extension()
            .and_then(|e| e.to_str())
        {
            if exts.contains(&ext.to_lowercase().as_str()) {
                let folder = path
                    .parent()
                    .unwrap_or_else(|| path)
                    .to_string_lossy()
                    .to_string();
                let file = path
                    .to_string_lossy()
                    .to_string();
                movies.push((folder, file));
            }
        }
    }
    movies
}
