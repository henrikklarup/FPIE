extern crate clap;
use clap::{App, Arg};

extern crate tar;
use tar::Builder;
use std::fs::File;

mod utils;

fn main() {

    let matches = App::new("FPIE")
       .version("1.0")
       .about("File Packer with Include and Exclude")
       .author("Henrik Klarup")
        .arg(Arg::with_name("includefile")
        .short("i")
        .long("includefile")
        .value_name("PATH")
        .help("Specify the includefile path")
        .takes_value(true).required(true))
        .arg(Arg::with_name("context")
        .short("c")
        .long("context")
        .value_name("PATH")
        .help("Specify the context path")
        .takes_value(true).required(true))
        .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("PATH")
        .help("Specify output file path")
        .takes_value(true))
        .arg(Arg::with_name("dry-run")
        .short("d")
        .help("Print a list of files to be packed"))
       .get_matches();

    let inc = matches.value_of("includefile").unwrap();
    let context = matches.value_of("context").unwrap();
    let mut context_path_with_trailing_slash = context.to_string(); 
    if !context.ends_with("/") {
        context_path_with_trailing_slash.push_str("/");
    }
    let dry_run = matches.is_present("dry-run");
    let final_list = do_it(inc, &context_path_with_trailing_slash);
    if dry_run {
        for itm in final_list {
            println!("{}", itm);
        }
        return;
    }

    let mut output_file = "output.tar";
    let output_val = matches.value_of("output");
    if let Some(output) = output_val {
        output_file = output;
    }

    let file = File::create(output_file).unwrap();
    let mut a = Builder::new(file);

    for itm in final_list {
        a.append_path_with_name(format!("{}{}", context_path_with_trailing_slash, itm), itm).unwrap();
    }
}


pub fn do_it(includefile_path: &str, context_path: &str) -> Vec<String> {
    let includes = utils::get_include_list(includefile_path);
    let excludes = utils::get_exclude_list(includefile_path);
    let includedfiles = utils::expand_globs_to_files(context_path, includes);
    let excludedfiles = utils::expand_globs_to_files(context_path, excludes);

    let list_with_context_path = utils::except(includedfiles, excludedfiles);
    
    let final_list = utils::trim_context(list_with_context_path, &context_path);
    return final_list;
}


#[cfg(test)]
mod tests {
    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_include() {
        let expected = vec!["foo.txt"];
        let actual = do_it("../testfixtures/simple/include.txt", "../testfixtures/simple/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simple_exclude() {
        let expected = vec!["foo.txt"];
        let actual = do_it("../testfixtures/simple/exclude.txt", "../testfixtures/simple/");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_complex() {
        let expected = vec![
            "includefile",
            "includedir/.letsinclude",
            "includedir/alsoincludethisfile",
            "includedir/innerincludedir/innerincludefile",
            "file with spaces",
        ];

        let actual = do_it(
            "../testfixtures/complex/includedir/.includefile_many",
            "../testfixtures/complex/"
        );

        assert_eq!(expected, actual);
    }
}
