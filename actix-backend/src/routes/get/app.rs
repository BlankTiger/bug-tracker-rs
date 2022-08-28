use actix_files as fs;
use actix_web::Result;
use std::path::PathBuf;

pub async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("../yew-frontend/dist/index.html");
    Ok(fs::NamedFile::open(path)?)
}
