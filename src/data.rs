use std::fs;
use std::env;
use sha1::{Sha1, Digest};

const SLOTH_DIR: &str = "./.slothgit";
const OBJECTS_DIR: &str = "./.slothgit/objects";

pub fn initialize() -> std::io::Result<()> {
    let path = env::current_dir()?;
    match fs::create_dir(SLOTH_DIR) {
        Ok(_) => println!("Initialized empty SlothGit repository in {}/.slothgit", path.display()),
        Err(_) => println!("Reinitialized existing SlothGit repository in {}/.slothgit", path.display())
    };
    match fs::create_dir(OBJECTS_DIR) {
        Ok(_) => (),
        Err(_) => ()
    };
    Ok(())
}

pub fn store_hash_object(file: Option<String>, write_to_db: bool, object_type: &str) -> std::io::Result<()> {
    let mut hasher = Sha1::new();

    match file {
        Some(file_path) => {
            let mut contents: String = fs::read_to_string(file_path).expect("Unable to read file {file_path}");
            contents = format!("{object_type}\x00{contents}");
            hasher.update(&contents);
            let oid = hasher.finalize();
            let output_path = format!("{}/{:#01x}", OBJECTS_DIR, oid);
            if write_to_db {
                match fs::create_dir(OBJECTS_DIR) {
                    Ok(_) => (),
                    Err(_) => (),
                };
                match fs::write(&output_path, contents) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing to {}: {}", output_path, e),
                };
            };
            println!("{:#01x}", oid);
            Ok(())
        },
        None => Ok(())
    }
}

pub fn read_hash_object(input: Option<String>, _alleged_object_type: &str, _expected: bool) -> std::io::Result<()> {
    match input {
        Some(oid) => {
            let path: String = format!("{}/{}", OBJECTS_DIR, oid);
            let contents: String = fs::read_to_string(path).expect("Unable to find {oid}");
            match contents.find('\x00') {
                Some(pos) => {
                    let object_type = contents.get(0..pos);
                    println!("Type: {}", object_type.unwrap());
                    println!("{}", contents.get(pos..).unwrap());
                },
                None => println!("Unable to find object type: null byte not found"),
            }
            Ok(())
        },
        None => Ok(())
    }
}
