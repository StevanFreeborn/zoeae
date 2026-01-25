use std::path::PathBuf;

use rfd::AsyncFileDialog;
use tokio::fs;

pub async fn open_file() -> Result<(PathBuf, String), String> {
  let handle_result = AsyncFileDialog::new()
    .set_directory("/")
    .pick_file()
    .await
    .ok_or(String::from("Dialog cancelled"));

  match handle_result {
    Ok(handle) => {
      let path = handle.path().to_owned();
      let content_result = fs::read_to_string(&path).await.map_err(|e| e.to_string());

      match content_result {
        Ok(content) => Ok((path, content)),
        Err(e) => Err(e),
      }
    }
    Err(e) => Err(e),
  }
}

pub async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, String> {
  let save_path = match path {
    Some(p) => p,
    None => AsyncFileDialog::new()
      .set_directory("/")
      .save_file()
      .await
      .ok_or(String::from("Dialog cancelled"))?
      .path()
      .to_owned(),
  };

  let save_result = fs::write(&save_path, text).await.map_err(|e| e.to_string());

  match save_result {
    Ok(_) => Ok(save_path),
    Err(err) => Err(err),
  }
}
