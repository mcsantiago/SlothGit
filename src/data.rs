use std::fs;
use std::env;

pub fn initialize() -> std::io::Result<()> {
    // Remember, the binary is not where the pwd is on the console
    let path = env::current_dir()?;
    match fs::create_dir("./.slothgit") {
        Ok(_) => println!("Initialized empty SlothGit repository in {}/.slothgit", path.display()),
        Err(_) => println!("Reinitialized existing SlothGit repository in {}/.slothgit", path.display())
    };
    Ok(())
}
