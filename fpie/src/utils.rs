
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
        let mut expand_single_asterix = expand;
        if expand_single_asterix.ends_with("/*") {
            expand_single_asterix = format!("{}*/*", expand_single_asterix);
        }
        for entry in glob(&format!("{}/{}", context_dir, expand_single_asterix)).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                if path.is_file() {
                    filelist.push(path.display().to_string());
                }
            }
        }
    }
    return filelist;
}

pub fn except(list_one: Vec<String>, list_two: Vec<String>) -> Vec<String> {
    let mut final_list: Vec<String> = Vec::new();
    for include in list_one {
        if !list_two.contains(&include) {
            final_list.push(include);
        }
    }
    return final_list;
}

pub fn trim_context(list: Vec<String>, context_path: &str) -> Vec<String> {
    return list.into_iter().map(|x| x.replace(context_path, "")).collect();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_finds_all_includes() {
        let expected = vec!("*");
        let actual = get_include_list("../testfixtures/simple/exclude.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_finds_all_excludes() {
        let expected = vec!("exclude*", "include.txt");
        let actual = get_exclude_list("../testfixtures/simple/exclude.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_asterix_gives_all_files() {
        let expected = vec!(
            "../testfixtures/simple/exclude-this.txt",
            "../testfixtures/simple/exclude.txt",
            "../testfixtures/simple/foo.txt",
            "../testfixtures/simple/include.txt"
        );
        let actual = expand_globs_to_files("../testfixtures/simple", get_include_list("../testfixtures/simple/exclude.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_exclude_files() {
        let expected = vec!(
            "../testfixtures/simple/exclude-this.txt",
            "../testfixtures/simple/exclude.txt",
            "../testfixtures/simple/include.txt"
        );
        let actual = expand_globs_to_files("../testfixtures/simple", get_exclude_list("../testfixtures/simple/exclude.txt"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn except_works() {
        let exclude_these = vec!(
            "../testfixtures/simple/exclude-this.txt".to_string(),
            "../testfixtures/simple/exclude.txt".to_string(),
            "../testfixtures/simple/include.txt".to_string()
        );
        let include_these = vec!(
            "../testfixtures/simple/exclude-this.txt".to_string(),
            "../testfixtures/simple/exclude.txt".to_string(),
            "../testfixtures/simple/foo.txt".to_string(),
            "../testfixtures/simple/include.txt".to_string()
        );

        let expected = vec!(
            "../testfixtures/simple/foo.txt".to_string()
        );
        let actual = except(include_these, exclude_these);
        assert_eq!(expected, actual);
    }
}
