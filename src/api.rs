#[get("/filelist")]
pub fn route_file_list() -> &'static str {
    "Hello, world!"
}

use crate::file_storage::file_as_bytes;

#[get("/files/<filename>")]
pub fn route_download_file(filename: String) -> Vec<u8> {
    let res = file_as_bytes(filename);
    match res {
        Ok(v) => v,
        Err(_e) => "file not found".as_bytes().to_vec(),
    } 
}

#[post("/upload")]
pub fn route_upload_files() -> &'static str {
    "Hello, world!"
}

#[post("/clear")]
pub fn route_clear_files() -> &'static str {
    "Hello, world!"
}
