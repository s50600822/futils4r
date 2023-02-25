use std::fs;
use std::path::PathBuf;

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
    let singletons = find_singleton_directories(std::path::Path::new("."));
    for dir in singletons {
        println!("{}", dir.display());
    }
}
