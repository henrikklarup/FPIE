
extern crate glob;
use self::glob::glob;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn get_include_list(includefile_path: &str) -> Vec<String> {
    let mut includes: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(includefile_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !ip.starts_with("!") {
                    includes.push(ip);
                }
            }
        }
    }
    return includes;
}

pub fn get_exclude_list(includefile_path: &str) -> Vec<String> {
    let mut excludes: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(includefile_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("!") {
                    excludes.push(remove_first(&ip).unwrap().to_string());
                }
            }
        }
    }
    return excludes;
}

pub fn expand_globs_to_files(context_dir: &str, glob_list: Vec<String>) -> Vec<String> {
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

pub fn except(list_one: Vec<String>, list_two: Vec<String>) -> Vec<String> {
    let mut final_list: Vec<String> = Vec::new();
    let mut finalexclude = list_two.iter();
    for include in list_one {
        match finalexclude.find(|&x| x.to_string() == include.to_string()) {
            Some(_) => assert!(true),
            None => final_list.push(include)
        }
    }
    return final_list;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
