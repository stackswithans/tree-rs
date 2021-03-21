
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

//Tag that indicates the kind of node (File or Dir)
#[derive(Debug, PartialEq)]
enum NodeKind{
    Dir,
    File
}

//Struct that represents a node in the file tree
#[derive(Debug)]
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
#[derive(Debug)]
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
        if path_str.starts_with("."){ // Make this optional base on params
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

#[cfg(test)]
mod tests{
    use std::fs::{DirBuilder, File};
    use std::fs;
    use super::Path;
    use super::{Tree, Node, NodeKind};

    #[test]
    fn test_new_tree(){
        let tree = Tree::new(String::from("foo/"));
        assert_eq!(tree.root.path, "foo/");
        assert_eq!(tree.root.children.unwrap().len(), 0);
        assert_eq!(tree.root.kind, NodeKind::Dir);
    }

    #[test]
    fn test_file_node_children(){
        let node = Node::new(
            String::from("foo.txt"),
            NodeKind::File, 
            1
        );
        assert_eq!(node.path, "foo.txt");
        assert_eq!(node.depth, 1);
        assert!(node.children.is_none());
        assert_eq!(node.kind, NodeKind::File);
    }

    #[test]
    fn test_build_tree(){
        //Create test data
        let builder = DirBuilder::new();
        let root = Path::new("./_test");
        builder.create(root).unwrap();
        builder.create(root.join("foo")).unwrap();
        builder.create(root.join("foo1")).unwrap();
        File::create(root.join("foo.txt")).unwrap();
        
        let mut tree = Tree::new(
            root
            .file_name()
            .unwrap()
            .to_str()
            .unwrap().to_string(),
        );
        tree.root.path.push_str("/");

        super::build_tree(&mut tree.root, root, 0).unwrap();

        assert_eq!(tree.root.path, "_test/");
        assert_eq!(tree.root.kind, NodeKind::Dir);
        assert_eq!(tree.root.children.as_ref().unwrap().len(), 3);

        //Only testing this way because there are no guarantees about
        //the sorting of the entries. There is probably a better to do this
        for node in tree.root.children.as_ref().unwrap(){
            match (node.path.as_str(), &node.kind) {
                ("foo/", NodeKind::Dir) => {
                    assert_eq!(
                        node.children.as_ref()
                        .unwrap().len(), 0
                    )},
                ("foo1/", NodeKind::Dir) => {
                    assert_eq!(
                        node.children.as_ref()
                        .unwrap().len(), 0
                    )},
                ("foo.txt", NodeKind::File) => (),
                _ => panic!("Bad child node found")
            }
        }
        //Clean up test data
        fs::remove_dir(root.join("foo")).unwrap();
        fs::remove_dir(root.join("foo1")).unwrap();
        fs::remove_file(root.join("foo.txt")).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
