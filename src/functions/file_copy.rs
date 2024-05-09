use std::fs::{copy, metadata};
use std::io;

pub fn file_copy(file_path: &str, destination_path: &str) -> std::io::Result<()> {
  // Check if source file exists
  if !metadata(file_path)?.is_file() {
    return Err(std::io::Error::new(io::ErrorKind::NotFound, format!("Source file '{}' not found", file_path)));
  }

  // Perform the copy operation
  copy(file_path, destination_path)?;

  println!("File '{}' copied successfully to '{}'", file_path, destination_path);
  Ok(())
}