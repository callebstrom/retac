use std::io::prelude::Read;

#[derive(Debug)]
pub enum ZipError {
    InvalidZip,
    NoFilesInArchive,
}

pub fn unzip(path: &str) -> Result<String, ZipError> {
    let fname = std::path::Path::new(path);
    let zipfile = std::fs::File::open(&fname).unwrap();

    zip::ZipArchive::new(zipfile)
        .map_err(|_| ZipError::InvalidZip)
        .and_then(|mut archive| {
            archive
                .by_index(0)
                .map_err(|_| ZipError::NoFilesInArchive)
                .and_then(|mut file| -> _ {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .map(|_| contents)
                        .map_err(|_| ZipError::InvalidZip)
                })
        })
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
