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
    let includedfiles = utils::expand_globs_to_files(context, includes);
    let excludedfiles = utils::expand_globs_to_files(context, excludes);

    let final_list = utils::except(includedfiles, excludedfiles);

    for itm in final_list {
        println!("{}", itm);
    }
}
