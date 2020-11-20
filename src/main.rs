use std::fs::ReadDir;
use std::{fs, path::Path};

use anyhow::Result;
use std::env;
use walkdir::WalkDir;

const SIZE: usize = 1024;

struct FileEntry {
    name: String,
    size: usize,
}

impl Clone for FileEntry {
    fn clone(&self) -> Self {
        FileEntry {
            name: self.name.clone(),
            size: self.size,
        }
    }
}

impl FileEntry {
    pub fn new(name: String, size: usize) -> FileEntry {
        FileEntry {
            name: name,
            size: size,
        }
    }

    pub fn sort(vec: &mut Vec<FileEntry>) {
        vec.sort_by(|a, b| b.size.cmp(&a.size));
    }
}

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        println!("Usage - {} <path>", args.nth(0).unwrap());
        return;
    }
    let args = args.skip(1);
    let args = args.collect::<Vec<String>>().join(" ");
    if let Err(err) = start(&args) {
        println!("Error: {}", err);
    }
}

fn start(path: &str) -> Result<()> {
    let mut vec: Vec<FileEntry> = vec![];
    for folder in get_contents(path) {
        let folder = folder.unwrap();
        let name = folder.file_name();
        let name = name.to_str().unwrap();

        let path = folder.path();
        let size = get_size(&path)?;

        let entry = FileEntry::new(String::from(name), size);
        vec.push(entry);
    }
    FileEntry::sort(&mut vec);
    let mut total: usize = 0;
    for entry in vec {
        let name = entry.name;
        total += entry.size;
        println!("{}: {}", name, get_formatted_size(entry.size));
    }
    println!("Total: {}", get_formatted_size(total));
    Ok(())
}

fn get_formatted_size(bytes: usize) -> String {
    let mut formatted: String = String::new();
    if bytes > SIZE * SIZE * SIZE {
        let num = bytes as f32;
        let num = num / (SIZE * SIZE * SIZE) as f32;
        formatted.push_str(&num.to_string());
        formatted.push_str(" GB");
        return formatted;
    }
    if bytes > SIZE * SIZE {
        let num = bytes as f32;
        let num = num / (SIZE * SIZE) as f32;
        formatted.push_str(&num.to_string());
        formatted.push_str(" MB");
        return formatted;
    }
    if bytes > SIZE {
        let num = bytes as f32;
        let num = num / SIZE as f32;
        formatted.push_str(&num.to_string());
        formatted.push_str(" KB");
        return formatted;
    }
    let num = bytes as f32;
    formatted.push_str(&num.to_string());
    formatted.push_str(" bytes");
    return formatted;
}

fn get_contents(root: &str) -> ReadDir {
    return fs::read_dir(root).unwrap();
}

fn walk(root: &str) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path: &str = entry.path().to_str().unwrap();
        vec.push(String::from(path));
    }
    return vec;
}

fn get_size(path: &Path) -> Result<usize> {
    let folder = path.to_str().unwrap();
    let mut size: usize = 0;
    if path.is_file() {
        return Ok(get_file_size(path.to_str().unwrap()));
    }
    for file in walk(folder) {
        size += get_file_size(&file);
    }
    return Ok(size);
}

fn get_file_size(path: &str) -> usize {
    let res = fs::metadata(path);
    let metadata = match res {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    if metadata.is_none() {
        return 0;
    }
    let size = metadata.unwrap().len();
    return size as usize;
}
