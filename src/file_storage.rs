use std::fs;

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
                panic!("Unexpected error while creating file_storage directory: {:?}", e);
            }
        },
    } 
}

pub fn file_as_bytes(file_name: String) -> std::io::Result<Vec<u8>> {
    Ok(fs::read(format!("file_storage/{}", file_name))?)
}

pub fn file_name_vector() -> std::io::Result<Vec<String>> {
    let mut file_names = Vec::new();
    for entry in fs::read_dir("file_storage")? {
        if let Ok(entry) = entry {
            let compatible_file_name = entry.file_name().into_string();
            match compatible_file_name {
                Ok(v) => file_names.push(v),
                Err(_) => {}, 
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
                    Ok(_) => {},
                    Err(err) => println!("Unable to delete file {}: {}", file_name, err), 
                } 
            });
            Ok(())
        },
        Err(err) => Err(err),
    }    
}
