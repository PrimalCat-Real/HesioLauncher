use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::command;
use futures_util::StreamExt;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use zip::read::ZipArchive;


#[tauri::command]
pub fn hello_world() -> String {
  String::from("Hello, World!")
}

#[tauri::command]
pub fn create_directory(path: String) -> Result<(), String> {
  fs::create_dir_all(path).map_err(|err| err.to_string())
}


#[derive(serde::Deserialize)]
pub struct DownloadArgs {
  pub url: String,
  pub dest: String
}

#[tauri::command]
pub async fn unzip_file(source: String, destination: String) -> Result<String, String> {
    let source_path = std::path::Path::new(&source);
    let destination_path = std::path::Path::new(&destination);

    // Unzip the file
    let file = File::open(&source_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    archive.extract(&destination_path).map_err(|e| e.to_string())?;

    // Delete the zip file
    std::fs::remove_file(&source_path).map_err(|e| e.to_string())?;

    Ok(format!("Files successfully extracted to: {}", destination))
}

#[command]
pub async fn download_file(window: tauri::Window, args: DownloadArgs) -> Result<String, String> {
  // Check the response status
  let resp = reqwest::get(&args.url).await.map_err(|e| e.to_string())?;
  if !resp.status().is_success() {
    return Err(format!("Failed to download file: HTTP {}", resp.status()));
  }

  // Get the total size of the file
  let total_size = resp.content_length().unwrap_or(0);

  // Create the destination file
  let mut file = File::create(&args.dest).map_err(|e| e.to_string())?;

  let mut downloaded = 0u64;
  let mut stream = resp.bytes_stream();

  // Read the response bytes
  while let Some(item) = stream.next().await {
    let chunk = item.map_err(|e| e.to_string())?;
    file.write_all(&chunk).map_err(|e| e.to_string())?;

    downloaded += chunk.len() as u64;
    
    // Calculate the percentage of the download
    let percentage = (downloaded as f64 / total_size as f64 * 100.0) as u64;

    // Emit the progress to the frontend
    window.emit("download-progress", Some(percentage)).unwrap();
  }

  Ok("Download successful".into())
}