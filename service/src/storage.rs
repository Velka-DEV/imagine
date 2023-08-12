use std::{path::PathBuf, io::Error, ffi::OsStr};
use salvo::fs::NamedFileBuilder;
use salvo::http::form::FilePart;
use salvo::core::fs::NamedFile;
use ::entity::file::Model as File;

const STORAGE_PATH: &str = "storage";
const FILE_STORAGE_PATH: &str = "files";

pub async fn get_named_file(file: &File) -> Option<NamedFileBuilder> {
    let path = get_file_path(&file.id.to_string(), &file.extension);

    if !path.exists() {
        return None;
    }

    let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();

    Some(NamedFile::builder(&path).attached_name(file_name))
}

pub fn store_file(file: &FilePart, id: &str) -> Result<PathBuf, Error> {
    let file_extension = file
        .path()
        .extension()
        .unwrap_or(OsStr::new(""));

    let path = get_file_storage_path();
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    let mut path = path;
    path.push(id);
    path.set_extension(file_extension);

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

fn get_file_path(id: &str, extension: &str) -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push(STORAGE_PATH);
    path.push(FILE_STORAGE_PATH);
    path.push(id);

    if !extension.is_empty() {
        path.set_extension(extension);
    }

    path
}

fn get_file_storage_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path
        .push(STORAGE_PATH);
    path
        .push(FILE_STORAGE_PATH);

    path
}