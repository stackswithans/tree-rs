
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

//Tag that indicates the kind of node (File or Dir)
pub enum NodeKind{
    Dir,
    File
}

//Struct that represents a node in the file tree
pub struct Node{
    pub path : String,
    pub kind : NodeKind,
    pub depth : u64,
    pub children : Option<Vec<Node>> //File nodes do not have children
}

impl Node{
    pub fn new(path: String, kind : NodeKind, depth : u64) -> Node{
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

    pub fn add_child(&mut self, child: Node){
        self.children.as_mut().unwrap().push(child);
    }

    pub fn print_node(&self){
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

pub fn build_tree(parent: &mut Node, dir : &Path, depth : u64) -> io::Result<()>{
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

pub fn run(dir : &Path) -> io::Result<()> {
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
