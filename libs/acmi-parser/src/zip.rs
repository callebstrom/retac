use std::io::prelude::*;

pub fn unzip(path: &str) -> Result<String, &str> {
    let fname = std::path::Path::new(path);
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut file = match archive.by_index(0) {
        Ok(file) => file,
        Err(..) => {
            return Err("No files in archive");
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents);
}
