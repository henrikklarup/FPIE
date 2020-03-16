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

    let includes = utils::get_include_list(inc);
    let excludes = utils::get_exclude_list(inc);
    for exclude in excludes {
        println!("{}", exclude);
    }
    // let includefiles = utils::expand_globs_to_files(context, &includes);
    // let excludefiles = utils::expand_globs_to_files(context, excludes);

    // for include in includes {
    //     for entry in glob(&format!("{}/{}", context, include)).expect("Failed to read glob pattern") {
    //         match entry {
    //             Ok(path) => filestoinclude.push(path.display().to_string()),
    //             Err(e) => println!("{:?}", e),
    //         }
    //     }
    // }
    // for exclude in excludes {
    //     println!("{}", exclude);

    //     for entry in glob(&format!("{}/{}", context, exclude)).expect("Failed to read glob pattern") {
    //         match entry {
    //             Ok(path) => filestoexclude.push(path.display().to_string()),
    //             Err(e) => println!("{:?}", e),
    //         }
    //     }
    // }

    // let mut final_list = Vec::new();
    // let mut finalexclude = filestoexclude.iter();
    // for include in filestoinclude {
    //     match finalexclude.find(|&x| x.to_string() == include.to_string()) {
    //         Some(_) => assert!(true),
    //         None => final_list.push(include)
    //     }
    // }

    // for pr in final_list {
    //     // println!("{}", pr);
    // }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

