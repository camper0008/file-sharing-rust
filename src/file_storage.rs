use std::fs;

fn attempt_create_dir() -> std::io::Result<()> {
    fs::create_dir("file_storage")?;
    Ok(())
}

pub fn create_dir_if_not_exists() {
    println!("Creating directory 'file_storage'.");
    match attempt_create_dir() {
        Ok(_) => println!("Directory 'file_storage' created successfully."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Directory 'file_storage' already exists.");
            } else {
                panic!("Unexpected error while creating file_storage directory: {:?}", e);
            }
        },
    } 
}

pub fn file_as_bytes(filename: String) -> std::io::Result<Vec<u8>> {
    Ok(fs::read(format!("file_storage/{}", filename))?)
}
