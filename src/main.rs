#![feature(decl_macro)]
#[macro_use] extern crate rocket;

mod api;
mod file_storage;

use rocket_contrib::serve::StaticFiles;

fn main() {
    file_storage::create_dir_if_not_exists();
    rocket::ignite()
        .mount("/api", routes![api::route_file_list, api::route_download_file, api::route_upload_files, api::route_clear_files])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .launch();
}
