use std::path::PathBuf;
use temp_testdir::TempDir;

pub fn init_tmp_dir(name: &str) -> (PathBuf, TempDir) {
    let temp = temp_testdir::TempDir::default();
    let mut path = std::path::PathBuf::from(temp.as_ref());
    path.push(name);
    (path, temp)
}
