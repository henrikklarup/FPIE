extern crate clap; 
use clap::{App, Arg}; 

mod utils;

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
    let final_list = do_it(inc, context);
    for itm in final_list {
        println!("{}", itm);
    }
}


pub fn do_it(includefile_path: &str, context_path: &str) -> Vec<String> {
    let includes = utils::get_include_list(includefile_path);
    let excludes = utils::get_exclude_list(includefile_path);
    let includedfiles = utils::expand_globs_to_files(context_path, includes);
    let excludedfiles = utils::expand_globs_to_files(context_path, excludes);

    let list_with_context_path = utils::except(includedfiles, excludedfiles);

    let mut context_path_with_trailing_slash = context_path.to_string(); 
    if !context_path.ends_with("/") {
        context_path_with_trailing_slash.push_str("/");
    }
    
    let final_list = utils::trim_context(list_with_context_path, &context_path_with_trailing_slash);
    return final_list;
}


#[cfg(test)]
mod tests {
    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_include() {
        let expected = vec!["foo.txt"];
        let actual = do_it("../testfixtures/simple/include.txt", "../testfixtures/simple");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simple_exclude() {
        let expected = vec!["foo.txt"];
        let actual = do_it("../testfixtures/simple/exclude.txt", "../testfixtures/simple");
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
            "../testfixtures/complex"
        );

        assert_eq!(expected, actual);
    }
}
