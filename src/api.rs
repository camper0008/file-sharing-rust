use crate::file_storage::{clear_files_stored, data_to_file, file_as_bytes, file_name_vector};
use rocket::http::Status;
use rocket::response::content::Json;
use rocket::response::status::{Custom, NotFound};
use rocket::Data;

#[get("/filelist")]
pub fn route_file_list<'a>() -> Result<Json<String>, Custom<Json<String>>> {
    match file_name_vector() {
        Ok(v) => {
            let fold = v.iter().fold("".to_string(), |prev, curr| {
                if prev == "" {
                    format!("\"{}\"", curr)
                } else {
                    format!("{}, \"{}\"", prev, curr)
                }
            });
            Ok(Json(format!("{{\"files\":[{}]}}", fold).to_string()))
        }
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json("{\"msg\":\"an error occurred trying to get file list\"}".to_string()),
        )),
    }
}

#[get("/files/<file_name>")]
pub fn route_download_file(file_name: String) -> Result<Vec<u8>, NotFound<Json<String>>> {
    let res = file_as_bytes(file_name);
    match res {
        Ok(v) => Ok(v),
        Err(_) => Err(NotFound(Json("{\"msg\": \"file not found\"}".to_string()))),
    }
}

#[post("/upload", data = "<data>")]
pub fn route_upload_files(data: Data) -> &'static str {
    match data_to_file(data) {
        Ok(_) => {}
        Err(_) => {}
    }
    "bruh"
}

#[post("/clear")]
pub fn route_clear_files() -> Result<Json<String>, Custom<Json<String>>> {
    match clear_files_stored() {
        Ok(_) => Ok(Json("{\"msg\":\"files cleared successfully\"}".to_string())),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json("{\"msg\":\"an error occurred trying to clear files\"}".to_string()),
        )),
    }
}
