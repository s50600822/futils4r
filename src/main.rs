use std::fs;
use std::path::PathBuf;
use clap::{arg, command, Command};

fn is_singleton(dir_path: &std::path::Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir_path) {
        let mut count = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().ok().map_or(false, |t| t.is_dir()) {
                    count += 1;
                    if count > 1 {
                        return false;
                    }
                } else {
                    // Ignore hidden files
                    if let Some(file_name) = entry.file_name().to_str() {
                        if !file_name.starts_with('.') {
                            count += 1;
                            if count > 1 {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        count == 1
    } else {
        false
    }
}

fn find_singleton_directories(dir_path: &std::path::Path) -> Vec<PathBuf> {
    let mut singletons = vec![];
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().ok().map_or(false, |t| t.is_dir()) {
                    if is_singleton(entry.path().as_path()) {
                        singletons.push(entry.path());
                    }
                }
            }
        }
    }
    singletons
}

fn main() {
    let matches = command!()
        // Show usage, options and subcommands
        .arg_required_else_help(true)
        .subcommand(
            Command::new("flatten")
                .about("Find directory that has a single child")
                .arg(arg!(--dir <VALUE>))
        )
        .subcommand(
            Command::new("find_h264")
                .about("Find H.264")
                .arg(arg!(--dir <VALUE>))
        )
        .get_matches();

        match matches.subcommand() {
            Some(("flatten", sub_matches)) => {
                let dir = sub_matches.get_one::<String>("dir").expect("required");
                println!("Flattening: {}",dir);
                let singletons = find_singleton_directories(std::path::Path::new(dir));
                for dir in singletons {
                    println!("{}", dir.display());
                }
            }
            _ => unreachable!("Unsupported subcommands."),
        }    

    let singletons = find_singleton_directories(std::path::Path::new("."));
    for dir in singletons {
        println!("{}", dir.display());
    }
}
