mod zip;

pub fn parse(file: &str) -> String {
    match zip::unzip(file) {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_unzip_acmi_file() {
        let file = "F15_SU27_BVR.zip.acmi";
        assert_eq!(parse(file)[3..21], "FileType=text/acmi".to_string());
    }
}
