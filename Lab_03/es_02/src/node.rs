// Define this enum in order to be able to store different types in the same vector
use crate::dir::Dir;
use crate::file::File;
use crate::fs_error::FSError;

pub enum Node {
    File(File),
    Dir(Dir),
}

impl<'a> TryInto<&'a mut Dir> for &'a mut Node {
    type Error = FSError;

    fn try_into(self) -> Result<&'a mut Dir, Self::Error> {
        match self {
            Node::File(_) => Err(FSError::NotADir),
            Node::Dir(dir) => Ok(dir),
        }
    }
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Dir(dir) => dir.name(),
            Node::File(file) => file.name(),
        }
    }

    pub fn get<'a, 'b>(&'a self, path: &'b str) -> Result<&'a Node, FSError> {
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

    pub fn get_mut<'a, 'b>(&'a mut self, path: &'b str) -> Result<&'a mut Node, FSError> {
        if path.contains("/") {
            return match self {
                Node::Dir(dir) => {
                    let (dir_name, rest_of_path) = path.split_once("/").unwrap();
                    if dir_name == dir.name() {
                        dir.get_mut(rest_of_path)
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

    pub fn test_root_node() -> Self {
        Node::Dir(Dir::test_root_dir_with_subdir())
    }

    pub fn test_dir_node() -> Self {
        Node::Dir(Dir::test_dir_with_file())
    }

    pub fn test_file_node() -> Self {
        Node::File(File::test_file())
    }
}
