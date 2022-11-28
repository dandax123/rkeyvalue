use std::fs;
use std::hash::Hasher;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
pub struct VolumeServerFile {
    path: PathBuf,
}

impl VolumeServerFile {
    pub fn new(path: PathBuf) -> Self {
        VolumeServerFile { path }
    }
    pub fn write(&self, value: String) {
        let mut file =
            fs::File::create(&self.path).expect("Can't create the file, check permissions error");
        file.write_all(value.as_bytes());
    }
    pub fn delete(&self) {
        fs::remove_file(&self.path).expect("Can't delete file!");
    }
    pub fn read(&self) -> String {
        let file =
            fs::File::open(&self.path).expect("Can't open the file, check permissions error");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Can't read the file");
        contents
    }
}
