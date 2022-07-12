use std::io::prelude::Read;

pub enum ZipError {
    InvalidZip,
    NoFilesInArchive,
}

pub fn unzip(path: &str) -> Result<String, ZipError> {
    let fname = std::path::Path::new(path);
    let zipfile = std::fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut file = match archive.by_index(0) {
        Ok(file) => file,
        Err(..) => {
            return Err(ZipError::InvalidZip);
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_unzip_acmi_file() {
        let file = "F15_SU27_BVR.zip.acmi";
        let acmi_header = unzip(file).map_or_else(|_| "".to_string(), |s| s[3..21].to_string());

        assert_eq!(acmi_header, "FileType=text/acmi".to_string());
    }
}
