use std::{env::consts::FAMILY, time::SystemTime};

struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

struct Dir {
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

// Define this enum in order to be able to store different types in the same vector
enum Node {
    File(File),
    Dir(Dir),
}

enum FSError {
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}

// // define lifetimes
// struct MatchResult {
//     q: &str, // matched query string
//     path: String, // matched path
//     node: &Node, // matched node
// }

struct Filesystem {
    root: Node,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name: name,
            modified: SystemTime::now(),
            children: Vec::new(),
        }
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn get<'a, 'b>(&'a self, path: &'b str) -> Result<&'a Node, FSError> {
        match self
            .children
            .iter()
            .filter(|child| child.get(path).is_ok())
            .next()
        {
            None => Err(FSError::NotFound),
            Some(node) => Ok(node),
        }
    }
}
// let next_node_name: &str = path.split("/").next().unwrap();

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Dir(dir) => dir.name.as_str(),
            Node::File(file) => file.name.as_str(),
        }
    }

    fn get<'a, 'b>(&'a self, path: &'b str) -> Result<&'a Node, FSError> {
        if path.contains("/") {
            return match self {
                Node::Dir(dir) => {
                    let (dir_name, rest_of_path) = path.split_once("/").unwrap();
                    if dir_name == dir.name() {
                        dir.get(rest_of_path)
                    } else {
                        Err(FSError::NotFound)
                    }
                }
                Node::File(_) => Err(FSError::NotFound),
            };
        } else {
            return match self.name() == path {
                true => Ok(self),
                false => Err(FSError::NotFound),
            };
        }
    }
}

impl Filesystem {
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    fn new() -> Self {
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
    // pub fn mkdir(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> {
    //     unimplemented!()
    // }

    //     // possible errors: NotFound, path is NotADir, Duplicate
    //     pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> {
    //         unimplemented!()
    //     }

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
        self.root.get(path)
    }

    //     // get a mutable reference to a node in the filesystem, given the path
    //     pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> {
    //         unimplemented!()
    //     }

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
}

pub fn demo() {
    // let mut fs = Filesystem::new();

    //     // create a directory structure, 10 dirs with a child dir and file each one
    //     for i in 0..10 {
    //         fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
    //         fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
    //         fs.create_file(format!("/dir{}", i).as_str(), "file1").unwrap();
    //     }

    let mut fs = Filesystem {
        root: Node::Dir(Dir {
            name: "".to_string(),
            modified: SystemTime::now(),
            children: vec![Node::Dir(Dir {
                name: "dir2".to_string(),
                modified: SystemTime::now(),
                children: vec![Node::File(File {
                    name: "child1".to_string(),
                    modified: SystemTime::now(),
                    content: vec![],
                })],
            })],
        }),
    };

    println!("find /child");
    if let Ok(res) = fs.get("/dir2/child1") {
        match res {
            Node::Dir(d) => {
                println!("dir2 found");
            }
            // try to match all possible errros
            _ => {}
        }
    } else {
        println!("not found");
    }

    //     // let's try with matches
    //     let matches = fs.find(&["name:child1", "type:file"]);
    //     for m in matches {
    //         match m.node {
    //             Node::File(f) => {
    //                 // inspect content
    //             },
    //             Node::Dir(d) => {
    //                 // inspect children
    //             },
    //             _ => {}
    //         }
    //     }

    //     // see note "riferimenti mutabili" in exercise text
    //     // now let's try to modify the filesystem using the found matches
    //     // is it possible to do it? which error do you get from the compiler?
    //     let matches = fs.find(&["/dir2/child1", "/dir3/child1"]);
    //     for m in matches {
    //         let node = fs.get_mut(m.path).unwrap();
    //         match node {
    //             Node::File(f) => {
    //                 // inspect content
    //             }
    //             _ => {}
    //         }
    //     }

    //     // how can you fix the previous code?
    //     // suggestion: this code using paths which are not referenced by MatchResults should compile. Why?
    //     // Therefore how can you use the paths returned in the MatchResults to modify the filesystem?
    //     let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
    //     for p in paths {
    //         let n = fs.get_mut(p.as_str());
    //     }

    //     // now let's try to walk the filesystem
    //     fs.walk(|path, node| {
    //         match node {
    //             Node::File(f) => {
    //                 println!("file: {}", path);
    //             }
    //             Node::Dir(d) => {
    //                 println!("dir: {}", path);
    //             }
    //         }
    //     });
}
