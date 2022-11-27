use std::path::PathBuf;

struct VolumeServerFile<'a> {
    hkey: &'a str,
    path: PathBuf,
}

impl<'a> VolumeServerFile<'a> {}
