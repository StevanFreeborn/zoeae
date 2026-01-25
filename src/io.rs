use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

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

pub fn save_file_to_disk(path: PathBuf, text: String) {
    let save_result = write(path, text);

    match save_result {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn load_file_from_disk(path: PathBuf) -> String {
    let read_result = read_to_string(path);
    read_result.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, remove_file};

    #[test]
    fn test_save_file_to_disk() {
        let test_path = PathBuf::from("test_file.txt");
        let test_content = String::from("Hello, Chat!");

        save_file_to_disk(test_path.clone(), test_content.clone());

        let saved_content = fs::read_to_string(&test_path).unwrap();

        assert_eq!(saved_content, test_content);

        let _ = remove_file(test_path);
    }

    #[test]
    fn test_load_file_from_disk() {
        let test_path = PathBuf::from("test_file.txt");
        let test_content = String::from("Hello, Chat!");
        let _ = fs::write(test_path.clone(), test_content.clone());

        let saved_content = load_file_from_disk(test_path.clone());

        assert_eq!(saved_content, test_content);

        let _ = remove_file(test_path);
    }
}
