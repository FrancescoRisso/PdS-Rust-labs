#[derive(Debug)]
pub enum FSError {
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}
