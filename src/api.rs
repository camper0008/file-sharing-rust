use rocket::response::content::Json;
use rocket::response::status::{NotFound, Custom};
use rocket::http::Status;
use crate::file_storage::{file_as_bytes, file_name_vector, clear_files_stored};

#[get("/filelist")]
pub fn route_file_list<'a>() -> Result<Json<String>, Custom<Json<String>>> {
    match file_name_vector() {
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
            Json(format!("{{\"files\":[{}]}}", fold).to_string())
        } 
        Err(_) => Err(Custom(Status::InternalServerError, Json("{\"msg\":\"an error occurred trying to get file list\"}".to_string())))
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

#[post("/upload")]
pub fn route_upload_files() -> &'static str {
    "Hello, world!"
}

#[post("/clear")]
pub fn route_clear_files() -> Result<Json<String>, Custom<Json<String>>> {
    match clear_files_stored() {
        Ok(_) => Ok(Json("{\"msg\":\"files cleared successfully\"}".to_string())),
        Err(_) => Err(Custom(Status::InternalServerError, Json("{\"msg\":\"an error occurred trying to clear files\"}".to_string()))),
    }
}
