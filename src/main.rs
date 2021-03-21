use std::io;
use std::fs;
use std::env;
use std::path::Path;
/* This is a simple command line tool to print the content of a directory as a
 * string. 
 * arguments: 
 * dir : The name of the dir to tree-ify; 
 */
fn run(dir : &Path) -> io::Result<String> {
    let mut paths = String::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        paths.push_str(
            &format!("{}\n", path.to_str().unwrap())
        );
    }
    Ok(paths)
}

fn parse_args(args : &Vec<String>) -> &Path{
    Path::new(&args[1])
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let dir = parse_args(&args); 
    
    //Check if file is a directory
    if !dir.is_dir(){
        panic!("{:?} is not a directory.", dir.canonicalize().unwrap());
    }

    //Print contents directory
    match run(dir){
        Ok(paths) => {
            println!("{}", paths);
        },
        Err(error) => {
            panic!("Some error that i do not care about for now has happened");
        }
    };
}
