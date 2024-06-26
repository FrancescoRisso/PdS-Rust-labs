use es_02::{fs_error::FSError, node::Node, Filesystem};

mod my_tests_get {
    use super::*;

    #[test]
    fn get_dir_root() {
        assert!(Filesystem::get_test_fs().get("/").is_ok());
    }

    #[test]
    fn get_dir() {
        assert!(Filesystem::get_test_fs().get("/testDir").is_ok());
    }

    #[test]
    fn get_file() {
        assert!(Filesystem::get_test_fs().get("/testFile").is_ok());
    }

    #[test]
    fn get_file_in_dir() {
        assert!(Filesystem::get_test_fs()
            .get("/testDir/testFile")
            .is_ok_and(|f| f.is_file()));
    }

    #[test]
    fn get_wrong_dir() {
        assert!(Filesystem::get_test_fs()
            .get("/testDir2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_wrong_file() {
        assert!(Filesystem::get_test_fs()
            .get("/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_wrong_file_in_dir() {
        assert!(Filesystem::get_test_fs()
            .get("/testFolder/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_nothing() {
        assert!(Filesystem::get_test_fs().get("").is_ok());
    }

    #[test]
    fn get_dir_root_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/").is_ok());
    }

    #[test]
    fn get_dir_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/testDir").is_ok());
    }

    #[test]
    fn get_file_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/testFile").is_ok());
    }

    #[test]
    fn get_file_in_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testDir/testFile")
            .is_ok_and(|f| f.is_file()));
    }

    #[test]
    fn get_wrong_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testDir2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_wrong_file_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_wrong_file_in_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testFolder/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn get_nothing_mut() {
        assert!(Filesystem::get_test_fs().get_mut("").is_ok());
    }
}

mod my_tests_mkdir {
    use super::*;

    #[test]
    fn mkdir_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs.get("/myFolder").is_ok());
    }

    #[test]
    fn mkdir_subdir_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs.mkdir("/myFolder", "subfolder").is_ok());
        assert!(fs.get("/myFolder/subfolder").is_ok());
    }

    #[test]
    fn mkdir_path_not_found() {
        let mut fs = Filesystem::new();
        assert!(fs
            .mkdir("/myFolder", "subfolder")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn mkdir_folder_exists() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs
            .mkdir("/", "myFolder")
            .is_err_and(|e| e == FSError::Duplicate));
    }

    #[test]
    fn mkdir_not_a_folder() {
        let mut fs = Filesystem::get_test_fs();
        assert!(fs
            .mkdir("/testDir/testFile", "myFolder")
            .is_err_and(|e| e == FSError::NotADir));
    }
}

mod my_tests_mkfile {
    use super::*;

    #[test]
    fn mkfile_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.create_file("/", "myFile").is_ok());
        assert!(fs.get("/myFile").is_ok_and(|res| res.is_file()));
    }

    #[test]
    fn mkfile_subdir_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs.create_file("/myFolder", "myFile").is_ok());
        assert!(fs.get("/myFolder/myFile").is_ok_and(|res| res.is_file()));
    }

    #[test]
    fn mkfile_path_not_found() {
        let mut fs = Filesystem::new();
        assert!(fs
            .create_file("/myFolder", "myFile")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn mkfile_file_exists() {
        let mut fs = Filesystem::new();
        assert!(fs.create_file("/", "myFile").is_ok());
        assert!(fs
            .create_file("/", "myFile")
            .is_err_and(|e| e == FSError::Duplicate));
    }

    #[test]
    fn mkfile_folder_same_name_exists() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "test").is_ok());
        assert!(fs
            .create_file("/", "test")
            .is_err_and(|e| e == FSError::Duplicate));
    }

    #[test]
    fn mkfile_not_a_folder() {
        let mut fs = Filesystem::get_test_fs();
        assert!(fs
            .create_file("/testDir/testFile", "myFile")
            .is_err_and(|e| e == FSError::NotADir));
    }
}

mod my_tests_touch {
    use super::*;

    #[test]
    fn touch_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.create_file("/", "myFile").is_ok());
        assert!(fs.touch("/myFile").is_ok());
    }

    #[test]
    fn touch_not_found() {
        let mut fs = Filesystem::new();
        assert!(fs.touch("/myFile").is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn touch_is_dir() {
        let mut fs = Filesystem::new();
        _ = fs.mkdir("/", "test");
        assert!(fs.touch("/test").is_err_and(|e| e == FSError::NotFound));
    }
}

mod my_tests_rm {
    use super::*;

    #[test]
    fn rm_file() {
        let mut fs = Filesystem::new();
        assert!(fs.create_file("/", "myFile").is_ok());
        assert!(fs.delete("/myFile").is_ok_and(|f| f.is_file()));
    }

    #[test]
    fn rm_empty_dir() {
        let mut fs = Filesystem::new();
        _ = fs.mkdir("/", "myFolder");
        assert!(fs.delete("/myFolder").is_ok_and(|f| f.is_dir()));
    }

    #[test]
    fn rm_dir_with_file() {
        let mut fs = Filesystem::new();
        _ = fs.mkdir("/", "myFolder");
        _ = fs.create_file("/myFolder", "myFile");
        assert!(fs
            .delete("/myFolder")
            .is_err_and(|e| e == FSError::DirNotEmpty));
    }

    #[test]
    fn rm_dir_with_subdir() {
        let mut fs = Filesystem::new();
        _ = fs.mkdir("/", "myFolder");
        _ = fs.mkdir("/myFolder", "subdir");
        assert!(fs
            .delete("/myFolder")
            .is_err_and(|e| e == FSError::DirNotEmpty));
    }

    #[test]
    fn rm_non_existing() {
        let mut fs = Filesystem::new();
        assert!(fs
            .delete("/myFolder")
            .is_err_and(|e| e == FSError::NotFound));
    }
}

mod my_tests_walk {
    use super::*;

    #[test]
    fn rm_file() {
        let fs = Filesystem::get_test_fs();
        let f = |path: &str, node: &Node| {
            println!("{} {}", if node.is_dir() { "D" } else { "F" }, path)
        };
        fs.walk(f);
    }
}
mod my_tests_find {
    use super::*;

    #[test]
    fn find_file() {
        let mut fs = Filesystem::get_test_fs();
        _ = fs.create_file("/", "otherFile");
        assert_eq!(fs.find(&["type:file"]).len(), 3)
    }

    #[test]
    fn find_dir() {
        let mut fs = Filesystem::get_test_fs();
        _ = fs.create_file("/", "otherFile");
        assert_eq!(fs.find(&["type:dir"]).len(), 2)
    }

    #[test]
    fn find_name() {
        let mut fs = Filesystem::get_test_fs();
        _ = fs.create_file("/", "otherFile");
        assert_eq!(fs.find(&["name:otherFile"]).len(), 1)
    }

    #[test]
    fn find_partname() {
        let mut fs = Filesystem::get_test_fs();
        _ = fs.create_file("/", "otherFile");
        assert_eq!(fs.find(&["partname:other"]).len(), 1)
    }

    #[test]
    fn find_multiple_finds() {
        let fs = Filesystem::get_test_fs();
        assert_eq!(fs.find(&["partname:test"]).len(), 3)
    }

    #[test]
    fn find_multiple_queries() {
        let fs = Filesystem::get_test_fs();
        assert_eq!(fs.find(&["type:dir", "type:file"]).len(), 4)
    }

    #[test]
    fn find_same_thing_found_more_times() {
        let fs = Filesystem::get_test_fs();
        assert_eq!(fs.find(&["name:testFile", "type:file"]).len(), 4)
    }
}
