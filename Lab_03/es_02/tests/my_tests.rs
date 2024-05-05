use es_02::{fs_error::FSError, Filesystem};

mod my_tests_get {
    use super::*;

    #[test]
    fn test_01_get_dir_root() {
        assert!(Filesystem::get_test_fs().get("/").is_ok());
    }

    #[test]
    fn test_02_get_dir() {
        assert!(Filesystem::get_test_fs().get("/testDir").is_ok());
    }

    #[test]
    fn test_03_get_file() {
        assert!(Filesystem::get_test_fs().get("/testFile").is_ok());
    }

    #[test]
    fn test_04_get_file_in_dir() {
        assert!(Filesystem::get_test_fs().get("/testDir/testFile").is_ok());
    }

    #[test]
    fn test_05_get_wrong_dir() {
        assert!(Filesystem::get_test_fs()
            .get("/testDir2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn test_06_get_wrong_file() {
        assert!(Filesystem::get_test_fs()
            .get("/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn test_07_get_wrong_file_in_dir() {
        assert!(Filesystem::get_test_fs()
            .get("/testFolder/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn test_08_get_dir_root_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/").is_ok());
    }

    #[test]
    fn test_09_get_dir_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/testDir").is_ok());
    }

    #[test]
    fn test_10_get_file_mut() {
        assert!(Filesystem::get_test_fs().get_mut("/testFile").is_ok());
    }

    #[test]
    fn test_11_get_file_in_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testDir/testFile")
            .is_ok());
    }

    #[test]
    fn test_12_get_wrong_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testDir2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn test_13_get_wrong_file_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }

    #[test]
    fn test_14_get_wrong_file_in_dir_mut() {
        assert!(Filesystem::get_test_fs()
            .get_mut("/testFolder/testFile2")
            .is_err_and(|e| e == FSError::NotFound));
    }
}

mod my_tests_mkdir {
    use super::*;

    #[test]
    fn test_15_mkdir_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs.get("/myFolder").is_ok());
    }

    #[test]
    fn test_16_mkdir_subdir_ok() {
        let mut fs = Filesystem::new();
        assert!(fs.mkdir("/", "myFolder").is_ok());
        assert!(fs.mkdir("/myFolder", "subfolder").is_ok());
        assert!(fs.get("/myFolder/subfolder").is_ok());
    }
}
