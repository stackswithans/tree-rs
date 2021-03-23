use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

//Stores options to be used during tree building
#[derive(Debug, PartialEq)]
pub struct Options{
    pub dir: PathBuf,
    pub all : bool, //Traverse all nodes, including hidden nodes
    pub count : bool, //Count the number of files and subdirs in dir
}
//Represents result return from the treeify_function
pub struct DirData{
    pub tree : String, 
    pub subdirs : u32,
    pub files : u32
}

impl DirData{
    fn new(tree : String) -> Self{
        DirData{
            tree,
            subdirs : 0,
            files : 0, 
        }
    }
}

fn treeify_path(
    data : &mut DirData,
    dir : &Path,
    depth : u64,
    depth_str: String,
    all : bool,
) -> io::Result<()>{
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path : PathBuf = entry.path();
        let mut path_str = String::from(
            path
            .file_name()
            .unwrap_or(OsStr::new("")) // In case a path ends in ''
            .to_str()
            .unwrap()
        );
        if path_str.starts_with(".") && !all{ // Make this optional base on params
            continue;
        }
        if path.is_dir(){
            path_str.push_str("/"); //Add slash to directories
            data.tree.push_str(&format!("|{}{}\n", depth_str, path_str));
            data.subdirs += 1;
            treeify_path(
                data, 
                path.as_path(), 
                depth + 1, 
                format!("{}---", depth_str), all
            )?;
        } 
        else{
            //TODO: Add logic for symlinks
            data.files += 1;
            data.tree.push_str(&format!("|{}{}\n", depth_str, path_str));
        }
    }
    Ok(())
}

pub fn run(options : &Options) -> io::Result<DirData> {
    //Add root dir string
    let mut tree_str = String::from("|");
    tree_str.push_str(
        options.dir.as_path()
        .file_name()
        .unwrap_or(OsStr::new("")) // In case a path end in ''
        .to_str().unwrap()
    );
    tree_str.push_str("/\n");
    let mut dir_data = DirData::new(tree_str);
    treeify_path(
        &mut dir_data,
        options.dir.as_path(), 
        1,
        String::from("---"),
        options.all
    )?;
    //Print directory tree
    Ok(dir_data)
}

#[cfg(test)]
mod tests{
    use std::fs::{DirBuilder, File};
    use std::fs;
    use super::{Options, Path};

    fn setup(root : &Path){
        //Create test data
        let builder = DirBuilder::new();
        builder.create(root).unwrap();
        builder.create(root.join("foo")).unwrap();
        builder.create(root.join("foo1")).unwrap();
        builder.create(root.join(".foo2")).unwrap();
        File::create(root.join("foo.txt")).unwrap();
    }

    fn teardown(root: &Path){
        //Clean up test data
        fs::remove_dir(root.join("foo")).unwrap();
        fs::remove_dir(root.join("foo1")).unwrap();
        fs::remove_dir(root.join(".foo2")).unwrap();
        fs::remove_file(root.join("foo.txt")).unwrap();
        fs::remove_dir(root).unwrap();
    }

   #[test] 
    fn test_run(){
        let root = Path::new("./_test");
        setup(root);
        let options = Options{
            dir : root.to_path_buf(),
            all : false,
            count : false
        };
        let result = super::run(&options).unwrap();
        let expected = "|_test/\n|---foo/\n|---foo1/\n|---foo.txt\n";
        assert_eq!(result.tree, expected);
        teardown(root);
    }

    #[test]
    fn test_run_with_hidden_true(){
        let root = Path::new("./_test");
        setup(root);
        let options = Options{
            dir : root.to_path_buf(),
            all : true,
            count : false
        };
        let result = super::run(&options).unwrap();
        let expected = "|_test/\n|---.foo2/\n|---foo/\n|---foo1/\n|---foo.txt\n";
        assert_eq!(result.tree, expected);
        teardown(root);
    }

    #[test]
    fn test_run_with_count_true(){
        let root = Path::new("./_test");
        setup(root);
        let options = Options{
            dir : root.to_path_buf(),
            all : true,  
            count: true  
        };
        let result = super::run(&options).unwrap();
        let expected = "|_test/\n|---.foo2/\n|---foo/\n|---foo1/\n|---foo.txt\n";
        assert_eq!(result.tree, expected);
        assert_eq!(result.subdirs, 3);
        assert_eq!(result.files, 1);
        teardown(root);
    }
}
