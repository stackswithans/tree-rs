use std::process;
use std::env;
use std::path::{PathBuf};
use clap::{Arg, App, ArgMatches};
use fstree::Options;

/* This is a simple command line tool to print the content of a directory as a
 * string. 
 * arguments: 
 * dir : The name of the dir to tree-ify; 
 *
 * TODO:
 *** Print nodes as a tree (This is the goal);
 *** Refactor code to make it modular;
 *** Add tests
 *** Add "No argument show current dir tree" feature;
 *** Add flag for hidden files
 *** Add count of files and subfolders 
 * - Add feature for user to introduce a depth limit;
 * - Add colours based on the kind of node; (Optional)
 * - Add proper error handling
 */
fn get_options(args : ArgMatches) -> Options{
    let mut path = PathBuf::new();
    path.push(args.value_of("dir").unwrap());
    Options{
        dir: path,
        all: args.is_present("all"),
        count: args.is_present("count"),
    }
}

fn main(){
    let default_path = env::current_dir().unwrap();
    let matches = App::new("treers")
                      .version("0.1")
                      .author("Sténio J. <stexor12@gmail.com>")
                      .about("A simple command line program for showing contents of a directory as a tree")
                      .arg(Arg::with_name("dir")
                           .required(true)
                           .value_name("DIR")
                           .default_value(default_path.to_str().unwrap())
                           .help("Path of directory to 'tree-ify'.")
                           )
                      .arg(Arg::with_name("all")
                           .short("a")
                           .help("Show all files and dirs.")
                           )
                      .arg(Arg::with_name("count")
                           .short("c")
                           .help("Show number of files and subdirectories in DIR.")
                           )
                      .get_matches();
    let options = get_options(matches); 
    //Check if file is a directory
    if !options.dir.is_dir(){
        eprintln!("{:?} is not a directory.", options.dir);
        process::exit(1);
    }

    match fstree::run(&options){
        Ok(result) => {
            println!("{}", result.tree);
            if options.count{
                println!("Found {} Subdirectories and {} files",
                          result.subdirs, 
                          result.files
                );
            }
        },
        Err(error) => {
            eprintln!("{:?}", error.kind());
            panic!("Some error that i do not care about for now has happened");
        }
    };
}
