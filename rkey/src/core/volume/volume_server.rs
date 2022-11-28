use path_absolutize::*;

use std::{
    fs,
    path::{Path, PathBuf},
};

use super::file::VolumeServerFile;
pub(crate) struct VolumeServer {
    base_dir: String,
}

impl VolumeServer {
    fn make_dir(p: &Path) -> PathBuf {
        let k = p.absolutize().unwrap();
        // println!("{:?}", k);
        // let full_path = Path::new()
        if (!k.exists()) {
            // println!("Should create");
            fs::create_dir_all(&k).expect("Unable to create directory, check permissions");
        }
        k.into_owned()
    }
    pub fn new(base_dir: &str) -> Self {
        let p = Path::new(base_dir);
        //
        let k = Self::make_dir(p);

        VolumeServer {
            base_dir: k.to_str().unwrap().to_string(),
        }
    }
    fn key_to_path(&self, hkey: &str, make_dir: bool) -> PathBuf {
        Self::assert_md5_key(hkey);
        let key_dir = format!("{}/{}/{}", self.base_dir.as_str(), &hkey[0..2], &hkey[0..4]);
        let p = Path::new(key_dir.as_str());
        if !p.is_dir() && make_dir {
            Self::make_dir(p);
        }
        Path::new(key_dir.as_str()).join(hkey).to_path_buf()
    }
    fn assert_md5_key(hkey: &str) {
        assert!(hkey.len() == 32, "The key has to be a MD5 hash");
    }
    pub fn exists(&self, hkey: &str) -> bool {
        Self::assert_md5_key(hkey);
        self.key_to_path(hkey, false).is_file()
    }
    pub fn remove(&self, hkey: &str) {
        let path = self.key_to_path(hkey, false);
        let file = VolumeServerFile::new(path);
        file.delete();
    }
    pub fn get(&self, hkey: &str) -> String {
        Self::assert_md5_key(hkey);
        let path = self.key_to_path(hkey, false);

        let file = VolumeServerFile::new(path);
        file.read()
    }

    pub fn put(&self, hkey: &str, value: String) {
        Self::assert_md5_key(hkey);
        let path = self.key_to_path(hkey, true);

        let file = VolumeServerFile::new(path);
        file.write(value)
    }
    // fn key_to
}
