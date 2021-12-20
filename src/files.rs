use std::fs;

fn attempt_create_file_dir() -> std::io::Result<()> {
    fs::create_dir("file_storage")?;
    Ok(())
}

pub fn create_file_dir_if_not_exists() {
    println!("Creating directory 'file_storage'.");
    match attempt_create_file_dir() {
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
