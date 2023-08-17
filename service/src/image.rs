use std::{path::PathBuf, io::Error, ffi::OsStr};
use salvo::http::form::FilePart;

const STORAGE_PATH: &str = "storage";
const IMAGE_STORAGE_PATH: &str = "images";

pub fn store_image(file: &FilePart, id: &str) -> Result<PathBuf, Error> {
    let file_extension = file
        .path()
        .extension()
        .unwrap_or(OsStr::new(""));

    let mut path = get_image_storage_path();
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    path.push(id);

    std::fs::create_dir(&path)?;

    path.push("original");
    
    if !file_extension.is_empty() {
        path.set_extension(file_extension);
    }

    let path_as_str = path.to_str().ok_or(Error::new(std::io::ErrorKind::Other, "Path is not valid UTF-8"))?;

    let file_info = std::fs::copy(
        &file.path(), 
        path_as_str
    )?;

    if file_info != file.size() {
        return Err(Error::new(std::io::ErrorKind::Other, "File size mismatch"));
    }

    Ok(path)
}

fn get_image_path(id: &str, extension: &str) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push(STORAGE_PATH);
    path.push(IMAGE_STORAGE_PATH);
    path.push(id);

    if !extension.is_empty() {
        path.set_extension(extension);
    }

    path
}

fn get_image_storage_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path
        .push(STORAGE_PATH);
    path
        .push(IMAGE_STORAGE_PATH);

    path
}