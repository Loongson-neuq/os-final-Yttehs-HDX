use std::fs::ReadDir;

fn main() {
    let entris = ls();

    for entry in entris {
        let entry = entry.unwrap();

        match entry.file_type().unwrap() {
            ft if ft.is_dir() => println!("{} is a directory", entry.file_name().to_string_lossy()),
            ft if ft.is_file() => println!("{} is a file", entry.file_name().to_string_lossy()),
            _ => println!("{} is unknown", entry.file_name().to_string_lossy()),
        }
    }
}

// ls -al
fn ls() -> ReadDir {
    use std::{fs, env};

    let current_dir = env::current_dir().unwrap();
    fs::read_dir(current_dir).unwrap()
}