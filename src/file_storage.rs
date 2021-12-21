use rocket::Data;
use std::fs;
use std::io::{BufRead, Cursor};

fn attempt_create_dir() -> std::io::Result<()> {
    fs::create_dir("file_storage")?;
    Ok(())
}

pub fn create_dir_if_not_exists() {
    println!("Creating directory 'file_storage'");
    match attempt_create_dir() {
        Ok(_) => println!("Directory 'file_storage' created successfully"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Directory 'file_storage' already exists");
            } else {
                panic!(
                    "Unexpected error while creating file_storage directory: {:?}",
                    e
                );
            }
        }
    }
}

pub fn file_as_bytes(file_name: String) -> std::io::Result<Vec<u8>> {
    Ok(fs::read(format!("file_storage/{}", file_name))?)
}

pub fn get_data_from_vector(boundary: String, data: Vec<u8>) -> String {
    let mut cursor = Cursor::new(data);
    // has to be replaced with a vector to read non-utf8 files (e.g. images)
    // however, a vector does not support newlines
    // might have to crawl through every character and parse that way?
    // will look at tomorrow
    let mut buf = String::new();
    loop {
        let size = cursor.read_line(&mut buf).expect("an error occured reading from cursor");
        
        if size == 0 {
            println!("Reached EOF");
            break;
        }

        let pos = buf.len() - size;
        let line = &buf[pos..];
        println!("line: {}", line);
    }

    "Hello world!".to_string()

}

pub fn parse_form_data(boundary: String, data: Data) -> std::io::Result<()> {
    let mut vd = Vec::new();
    data.stream_to(&mut vd).expect("an error occurred streaming");

    let data = get_data_from_vector(boundary, vd);
    Ok(())
}

pub fn file_name_vector() -> std::io::Result<Vec<String>> {
    let mut file_names = Vec::new();
    for entry in fs::read_dir("file_storage")? {
        if let Ok(entry) = entry {
            let compatible_file_name = entry.file_name().into_string();
            match compatible_file_name {
                Ok(v) => file_names.push(v),
                Err(_) => {}
            }
        }
    }

    Ok(file_names)
}

pub fn clear_files_stored() -> std::io::Result<()> {
    match file_name_vector() {
        Ok(v) => {
            v.iter().for_each(|file_name| {
                match fs::remove_file(format!("file_storage/{}", file_name)) {
                    Ok(_) => {}
                    Err(err) => println!("Unable to delete file {}: {}", file_name, err),
                }
            });
            Ok(())
        }
        Err(err) => Err(err),
    }
}
