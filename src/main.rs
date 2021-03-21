use std::io;
use std::fs;
use std::process;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/* This is a simple command line tool to print the content of a directory as a
 * string. 
 * arguments: 
 * dir : The name of the dir to tree-ify; 
 *
 * TODO:
 * - Print nodes as a tree (This is the goal);
 * - Add feature for user to introduce a depth limit;
 * - Refactor code to make it modular;
 * - Add colours based on the kind of node; (Optional)
 * - Add "No argument show current dir tree" feature;
 * - by default
 */

//Tag that indicates the kind of node (File or Dir)
enum NodeKind{
    Dir,
    File
}

//Struct that represents a node in the file tree
struct Node{
    path : String,
    kind : NodeKind,
    depth : u64,
    children : Option<Vec<Node>> //File nodes do not have children
}

impl Node{
    fn new(path: String, kind : NodeKind, depth : u64) -> Node{
        Node{
            children : match &kind{
                NodeKind::Dir => Some(Vec::<Node>::new()), 
                NodeKind::File => None
            },
            path,
            kind,
            depth,
        }
    }

    fn add_child(&mut self, child: Node){
        self.children.as_mut().unwrap().push(child);
    }

    fn print_node(&self){
        let mut depth = String::new();
        for i in 0..self.depth{
            depth.push_str("---");
        }
        println!("|{}{}",depth, self.path);
        if self.children.is_some(){
            for child in self.children.as_ref().unwrap(){
                child.print_node();
            }
        }
    }
}

//Struct that represents the directory tree
struct Tree{
    root: Node
}


impl Tree{
    fn new(path: String) -> Tree{
        Tree{
            root: Node::new(path, NodeKind::Dir, 0)
        }
    }

    fn print_tree(&self){
        self.root.print_node()

    }

}

fn build_tree(parent: &mut Node, dir : &Path, depth : u64) -> io::Result<()>{
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path : PathBuf = entry.path();
        let mut node;
        let mut path_str = String::from(
            path
            .file_name()
            .unwrap_or(OsStr::new("")) // In case a path ends in ''
            .to_str()
            .unwrap()
        );
        if path_str.starts_with("."){
            continue;
        }
        if path.is_dir(){
            path_str.push_str("/"); //Add slash to directories
            node = Node::new(path_str, NodeKind::Dir, depth);
            build_tree(&mut node, path.as_path(), depth + 1)?;
            parent.add_child(node);
        } 
        else{
            node = Node::new(path_str, NodeKind::File, depth);
            parent.add_child(node); 
        }
    }
    Ok(())
}

fn run(dir : &Path) -> io::Result<()> {
    let mut path = String::from(dir
        .file_name()
        .unwrap_or(OsStr::new("")) // In case a path end in ''
        .to_str().unwrap()
    );
    path.push_str("/");
    let mut tree = Tree::new(path);
    build_tree(&mut tree.root, dir, 0)?;
    //Print directory tree
    tree.print_tree();
    Ok(())
}

fn parse_args(args : &Vec<String>) -> PathBuf{
    if args.len() <= 1 {
        env::current_dir().unwrap() //Print current directory tree
    }
    else{
        Path::new(&args[1]).to_path_buf() 
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    //Check if correct number of arguments have been supplied
    let dir = parse_args(&args); 
    
    //Check if file is a directory
    if !dir.is_dir(){
        eprintln!("{:?} is not a directory.", dir.canonicalize().unwrap());
        process::exit(1);
    }

    match run(dir.as_path()){
        Ok(()) => {
            println!("");
        },
        Err(error) => {
            eprintln!("{:?}", error.kind());
            panic!("Some error that i do not care about for now has happened");
        }
    };
}
