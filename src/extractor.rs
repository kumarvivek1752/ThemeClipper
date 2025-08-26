use rand::Rng;
use std::io;
use std::process::Command;

pub fn get_random_clip_start(
    duration: u32,
    clip_length: u32,
    start_buffer: u32,
    end_ignore_pct: f32,
) -> u32 {
    let max_valid_time = (duration as f32 * (1.0 - end_ignore_pct)) as u32;
    let mut rng = rand::thread_rng();
    rng.gen_range(start_buffer..(max_valid_time - clip_length))
}

pub fn extract_clip(
    input: &str,
    output: &str,
    start_time: u32,
    clip_length: u32,
) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .args([
            "-ss",
            &start_time.to_string(),
            "-i",
            input,
            "-t",
            &clip_length.to_string(),
            "-c:v",
            "libx264",
            "-preset",
            "fast",
            "-crf",
            "23",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            "-y",
            "-hide_banner",
            "-loglevel",
            "error",
            output,
        ])
        .status()
        .expect("failed to run ffmpeg");

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "ffmpeg failed"))
    }
}
