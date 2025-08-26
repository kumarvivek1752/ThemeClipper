use clap::Parser;
use serde::Deserialize;
use std::fs;

/// ThemeClipper - Generate theme clips from movies and series
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "💡 Created by Vivek (kumarvivek1752)\n💝 Paypal: https://paypal.me/kumarvivek1752\n🐙 GitHub: https://github.com/kumarvivek1752\n\n🙏 If you like this project, consider supporting or donating to keep it alive!"
)]
pub struct CliArgs {
    /// Path to movies directory
    #[arg(long)]
    pub movies_directory: Option<String>,

    /// Path to shows/series directory
    #[arg(long)]
    pub shows_directory: Option<String>,

    /// Clip length in seconds (default: 10)
    #[arg(short, long)]
    pub clip_length: Option<u32>,

    /// Method: random | audio | visual | music (default: random)
    /// | note: currently only suppoted random
    #[arg(short, long)]
    pub method: Option<String>,

    /// Method: Delete all backdrops folders form given directory
    #[arg(long)]
    pub delete: bool,

    /// Path to config JSON file
    #[arg(long)]
    pub config: Option<String>,
}

/// Config structure (used for JSON + merged with CLI)
#[derive(Debug, Deserialize)]
pub struct Config {
    pub movies_directory: Option<String>,
    pub shows_directory: Option<String>,
    pub clip_length: u32,
    pub delete: bool,
    pub method: String,
}

impl Config {
    pub fn from_args_and_file(args: CliArgs) -> Self {
        // Load from file if provided
        let mut file_config = if let Some(path) = args.config {
            let content = fs::read_to_string(path).expect("Failed to read config file");
            serde_json::from_str::<Config>(&content).expect("Invalid config JSON")
        } else {
            Config {
                movies_directory: None,
                shows_directory: None,
                clip_length: 10,
                method: "random".to_string(),
                delete: false,
            }
        };

        // CLI overrides
        if let Some(movies) = args.movies_directory {
            file_config.movies_directory = Some(movies);
        }
        if let Some(shows) = args.shows_directory {
            file_config.shows_directory = Some(shows);
        }
        if let Some(length) = args.clip_length {
            file_config.clip_length = length;
        }
        if let Some(method) = args.method {
            file_config.method = method;
        }
        if args.delete {
            file_config.delete = true;
        }
        file_config
    }
}
