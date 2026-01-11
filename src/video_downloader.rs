// im attempting to make a vide downloader tho i have no clue wtf im doing lol
// found some docs and example code online from this amazing repo: // https://github.com/boul2gom/yt-dlp?tab=readme-ov-file

use yt_dlp::Youtube;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;

// You use :: to call associated functions (like static methods in JS) or access constants: MyStruct::new(), MyStruct::CONSTANT

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    let url = String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    // need to find a way to get video title and set as the second param in download_audio_stream_from_url fn;
    // fetcher.download_audio_stream_from_url(url, "audio.mp3").await?;
    fetcher.download_audio_stream_from_url(url, "audio.m4a").await?;
    Ok(())
}