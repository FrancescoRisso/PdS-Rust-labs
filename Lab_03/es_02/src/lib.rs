mod dir;
mod file;
pub mod fs_error;
mod match_result;
pub mod node;

use crate::dir::Dir;
use crate::file::File;
use crate::fs_error::FSError;
use crate::node::Node;

pub struct Filesystem {
    root: Node,
}

impl Filesystem {
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    pub fn new() -> Self {
        Filesystem {
            root: Node::Dir(Dir::new("".to_string())),
        }
    }

    //     // create a new filesystem reading from disk all the structure under the given path
    //     // in the file content just write the firt 1k bytes of the file
    //     // return the root node of the filesystem
    //     // (implement this function at the end, after all the other methods, the only purpose is to take a look std::fs functions, use std::fs:read_dir)
    //     pub fn from(path: &str) -> Self {
    //         unimplemented!()
    //     }

    // create a new directory in the filesystem under the given path
    // return a reference the created dir
    // possible errors: NotFound, path NotADir, Duplicate
    pub fn mkdir(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> {
        match self.get_mut(path)? {
            Node::File(_) => Err(FSError::NotADir),
            Node::Dir(dir) => {
                if dir.get(name).is_ok() {
                    Err(FSError::Duplicate)
                } else {
                    dir.mkdir(name.to_string()).try_into()
                }
            }
        }
    }

    // possible errors: NotFound, path is NotADir, Duplicate
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> {
        match self.get_mut(path)? {
            Node::File(_) => Err(FSError::NotADir),
            Node::Dir(dir) => {
                if dir.get(name).is_ok() {
                    Err(FSError::Duplicate)
                } else {
                    dir.mkfile(Node::File(File::new(name.to_string())))
                        .try_into()
                }
            }
        }
    }

    //     // updated modification time of the file or the dir
    //     // possible errors: NotFound
    //     pub fn touch(&mut self, path: &str) -> Result<(), FSError> {
    //         unimplemented!()
    //     }

    //     // remove a node from the filesystem and return it
    //     // if it's a dir, it must be empty
    //     // possible errors: NotFound, DirNotEmpty
    //     pub fn delete(&mut self, path: &str) -> Result<Node, FSError> {
    //         unimplemented!()
    //     }

    // get a reference to a node in the filesystem, given the path
    pub fn get(&mut self, path: &str) -> Result<&Node, FSError> {
        match path {
            "/" => Ok(&self.root),
            _ => self.root.get(path),
        }
    }

    // get a mutable reference to a node in the filesystem, given the path
    pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> {
        match path {
            "/" => Ok(&mut self.root),
            _ => self.root.get_mut(path),
        }
    }

    //     // search for a list of paths in the filesystem
    //     // qs is a list query strings with constraints
    //     // the constraints must be matched in or (it's returned any node matching at least one constraint)
    //     // constraint format: "type:pattern"
    //     // constraints:
    //     // - "type:dir" -> match only directories
    //     // - "type:file" -> match only files
    //     // - "name:value" -> match only nodes with the given name
    //     // - "partname:value" -> match only nodes with the given string in the name

    //     pub fn find<'a>(&'a self, qs: &[&'a str]) -> Vec<MatchResult> {
    //         unimplemented!()
    //     }

    //     // walk the filesystem, starting from the root, and call the closure for each node with its path
    //     // the first parameter of the closure is the path of the node, second is the node itself
    //     pub fn walk(&self, f: impl Fn(&str, &Node)) {
    //         unimplemented!()
    //     }

    pub fn get_test_fs() -> Self {
        Filesystem {
            root: Node::test_root_node(),
        }
    }
}
