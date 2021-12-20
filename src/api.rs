#[get("/filelist")]
pub fn route_file_list() -> &'static str {
    "Hello, world!"
}

#[get("/files/<filename>")]
pub fn route_download_file(filename: String) -> String {
    format!("fuck dig {}", filename) 
}

#[post("/upload")]
pub fn route_upload_files() -> &'static str {
    "Hello, world!"
}

#[post("/clear")]
pub fn route_clear_files() -> &'static str {
    "Hello, world!"
}
