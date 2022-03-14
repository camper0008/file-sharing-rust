use crate::file_storage::{clear_files_stored, file_as_bytes, file_name_vector};
use rocket::http::Status;
use rocket::response::content::Json;
use rocket::response::status::{Custom, NotFound};
use rocket::response::Redirect;
use rocket_upload::MultipartDatas;
use std::path::Path;

#[get("/filelist")]
pub fn route_file_list<'a>() -> Result<Json<String>, Custom<Json<String>>> {
    match file_name_vector() {
        Ok(v) => {
            let fold = v.iter().fold("".to_string(), |prev, curr| {
                if prev == "" {
                    format!(r#""{}""#, curr)
                } else {
                    format!(r#"{}, "{}""#, prev, curr)
                }
            });
            Ok(Json(format!(r#"{{"files":[{}]}}"#, fold).to_string()))
        }
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(r#"{"msg":"an error occurred trying to get file list"}"#.to_string()),
        )),
    }
}

#[get("/files/<file_name>")]
pub fn route_download_file(file_name: String) -> Result<Vec<u8>, NotFound<Json<String>>> {
    let res = file_as_bytes(file_name);
    match res {
        Ok(v) => Ok(v),
        Err(_) => Err(NotFound(Json(r#"{"msg": "file not found"}"#.to_string()))),
    }
}

#[post("/upload", data = "<data>")]
pub fn route_upload_files(data: MultipartDatas) -> Redirect {
    for f in data.files {
        f.persist(Path::new("file_storage"));
    }
    Redirect::to("/")
}

#[post("/clear")]
pub fn route_clear_files() -> Result<Json<String>, Custom<Json<String>>> {
    match clear_files_stored() {
        Ok(_) => Ok(Json(r#"{"msg":"files cleared successfully"}"#.to_string())),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(r#"{"msg":"an error occurred trying to clear files"}"#.to_string()),
        )),
    }
}
