mod zip;

pub fn acmi_parser(file: &str) -> String {
    match zip::unzip(file) {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = "F15_SU27_BVR.zip.acmi";
        assert_eq!(acmi_parser(file)[3..21], "FileType=text/acmi".to_string());
    }
}
