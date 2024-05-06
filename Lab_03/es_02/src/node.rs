// Define this enum in order to be able to store different types in the same vector
use crate::dir::Dir;
use crate::file::File;
use crate::fs_error::FSError;

#[derive(PartialEq)]
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

impl<'a> TryInto<&'a Dir> for &'a Node {
    type Error = FSError;

    fn try_into(self) -> Result<&'a Dir, Self::Error> {
        match self {
            Node::File(_) => Err(FSError::NotADir),
            Node::Dir(dir) => Ok(dir),
        }
    }
}

impl<'a> TryInto<&'a File> for &'a Node {
    type Error = FSError;

    fn try_into(self) -> Result<&'a File, Self::Error> {
        match self {
            Node::Dir(_) => Err(FSError::GenericError),
            Node::File(f) => Ok(f),
        }
    }
}

impl<'a> TryInto<&'a mut File> for &'a mut Node {
    type Error = FSError;

    fn try_into(self) -> Result<&'a mut File, Self::Error> {
        match self {
            Node::Dir(_) => Err(FSError::GenericError),
            Node::File(f) => Ok(f),
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

    pub fn is_file(&self) -> bool {
        let tmp: Result<&File, FSError> = self.try_into();
        tmp.is_ok()
    }

    pub fn is_dir(&self) -> bool {
        let tmp: Result<&Dir, FSError> = self.try_into();
        tmp.is_ok()
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

    pub fn walk(&self, f: &impl Fn(&str, &Node), path: &String) {
        let mut this_path = path.clone();

        this_path.push_str(self.name());

        match self {
            Node::File(_) => f(this_path.as_str(), self),
            Node::Dir(dir) => {
                this_path.push('/');
				f(this_path.as_str(), self);
                dir.walk(f, &this_path);
            }
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
