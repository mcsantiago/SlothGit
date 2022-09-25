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
    let _ = fs::create_dir(OBJECTS_DIR);
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
                let _ = fs::create_dir(OBJECTS_DIR);
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

pub mod base {
    use std::fs;
    use std::path::Path;
    pub fn write_tree(dir: &Path) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if is_ignored(&path) {
                    continue;
                }
                if path.is_dir() {
                    write_tree(&path)?;
                } else {
                    match fs::read_to_string(&path) {
                        Ok(contents) => {
                            println!("{}", contents);
                        },
                        Err(e) => {
                            println!("Error while reading string {}", e);
                        }
                    };
                }
            }
        }
        Ok(())
    }

    fn is_ignored(path: &Path) -> bool {
        match path.to_str() {
            Some(path_str) => path_str.contains(".slothgit/"),
            None => {
                println!("Ignoring none UTF-8 path");
                false
            }
        }
    }
}
