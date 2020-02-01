extern crate glob;
use glob::glob;

extern crate clap; 
use clap::{App, Arg}; 

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

fn main() {

    let matches = App::new("myapp")
       .version("1.0")
       .about("Does great things!")
       .author("Kevin K.")
        .arg(Arg::with_name("includefile")
        .short("i")
        .long("includefile")
        .value_name("FILE")
        .help("Specify the include file")
        .takes_value(true).required(true))
        .arg(Arg::with_name("context")
        .short("c")
        .long("context")
        .value_name("PATH")
        .help("Specify the context path")
        .takes_value(true).required(true))
       .get_matches();

    let inc = matches.value_of("includefile").unwrap();
    let context = matches.value_of("context").unwrap();

    let mut includes = Vec::new();
    let mut excludes = Vec::new();
    if let Ok(lines) = read_lines(inc) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("!") {
                    let s_mut_str = ip.clone();
                    let removed = remove_first(&s_mut_str).unwrap();
                    excludes.push(removed);
                } else {
                    includes.push(ip);
                }
            }
        }
    }
    let mut filestoinclude = Vec::new();
    let mut filestoexclude = Vec::new();

    for include in includes {
        for entry in glob(&format!("{}/{}", context, include)).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => filestoinclude.push(path.display().to_string()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    for exclude in excludes {
        println!("{}", exclude);

        for entry in glob(&format!("{}/{}", context, exclude)).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => filestoexclude.push(path.display().to_string()),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    let mut final_list = Vec::new();
    let mut finalexclude = filestoexclude.iter();
    for include in filestoinclude {
        match finalexclude.find(|&x| x.to_string() == include.to_string()) {
            Some(_) => assert!(true),
            None => final_list.push(include)
        }
    }

    for pr in final_list {
        // println!("{}", pr);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}