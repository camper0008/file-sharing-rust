use rocket::response::content;

use crate::file_storage::{file_as_bytes, file_name_array};

#[get("/filelist")]
pub fn route_file_list<'a>() -> content::Json<String> {
    match file_name_array() {
        Ok(v) => {
            let fold = v
                .iter()
                .fold("".to_string(), 
                    |prev, curr| {
                        if prev == "" {
                            format!("\"{}\"", curr) 
                        } else {
                            format!("{}, \"{}\"", prev, curr) 
                        }
                    }
                );
            content::Json(format!("{{\"files\":[{}]}}", fold).to_string())
        } 
        Err(_) => content::Json("{\"files\":[]}".to_string())
    }
}

#[get("/files/<file_name>")]
pub fn route_download_file(file_name: String) -> Vec<u8> {
    let res = file_as_bytes(file_name);
    match res {
        Ok(v) => v,
        Err(_) => "file not found".as_bytes().to_vec(),
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
