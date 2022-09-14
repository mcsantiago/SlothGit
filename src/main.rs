use std::env;
mod data;

fn display_help_menu() -> std::io::Result<()> {
    println!("usage: slothgit <command> [<args>]");
    println!();
    println!("These are common SlothGit commands used in various situations:");
    println!();
    println!("start a working area:");
    println!("  init               Create an empty SlothGit repository or reinitialize an existing one");
    Ok(())
}

fn main() {
    // Parse out the command. If no command exists, display the help menu
    let command : Option<String> = env::args().nth(1);
    let operand : Option<String> = env::args().nth(2);
    let _result: Result<(), std::io::Error> = match command.as_ref().map(String::as_ref) {
        Some("init") => data::initialize(),
        Some("hash-object") => data::store_hash_object(operand, true),
        Some("--help") | Some("help") => display_help_menu(),
        _  => display_help_menu(),
    };
}
