
extern crate glob;
use self::glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn get_include_list(includefile_path: &str) -> Vec<&str> {
    let mut includes: Vec<&str> = Vec::new();
    if let Ok(lines) = read_lines(includefile_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.starts_with("!") {
                    includes.push(&ip.to_owned());
                }
            }
        }
    }
    return includes;
}

pub fn get_exclude_list(includefile_path: &str) -> Vec<&str> {
    let mut excludes: Vec<&str> = Vec::new();
    if let Ok(lines) = read_lines(includefile_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("!") {
                    excludes.push(remove_first(&ip).unwrap());
                }
            }
        }
    }
    // .into_iter().map(|x| utils::remove_first(&x).unwrap().to_owned()
    return excludes;
}

pub fn expand_globs_to_files(context_dir: &str, glob_list: Vec<&str>) -> Vec<String> {
    let mut filelist: Vec<String> = Vec::new();
    for expand in glob_list {
        for entry in glob(&format!("{}/{}", context_dir, expand)).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => filelist.push(path.display().to_string()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    return filelist;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
