use std::io::prelude::Read;

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_unzip_acmi_file() {
        let file = "F15_SU27_BVR.zip.acmi";
        let acmi_header = unzip(file)
            .to_owned()
            .map_or_else(|e| e.to_string(), |s| s[3..21].to_string());

        assert_eq!(acmi_header, "FileType=text/acmi".to_string());
    }
}
