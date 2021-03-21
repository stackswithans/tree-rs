use std::process;
use std::env;
use std::path::{Path, PathBuf};
extern crate treers;

/* This is a simple command line tool to print the content of a directory as a
 * string. 
 * arguments: 
 * dir : The name of the dir to tree-ify; 
 *
 * TODO:
 *** Print nodes as a tree (This is the goal);
 * - Add tests
 * - Add feature for user to introduce a depth limit;
 * - Refactor code to make it modular;
 * - Add colours based on the kind of node; (Optional)
 * - Add "No argument show current dir tree" feature;
 * - Add proper error handling
 * - by default
 */
fn parse_args(args : &Vec<String>) -> PathBuf{
    if args.len() <= 1 {
        env::current_dir().unwrap() //Print current directory tree
    }
    else{
        Path::new(&args[1]).to_path_buf() 
    }
}

fn main(){
    let args : Vec<String> = env::args().collect();
    //Check if correct number of arguments have been supplied
    let dir = parse_args(&args); 
    
    //Check if file is a directory
    if !dir.is_dir(){
        eprintln!("{:?} is not a directory.", dir.canonicalize().unwrap());
        process::exit(1);
    }

    match treers::run(dir.as_path()){
        Ok(()) => {
            println!("");
        },
        Err(error) => {
            eprintln!("{:?}", error.kind());
            panic!("Some error that i do not care about for now has happened");
        }
    };
}
