mod zip;

pub fn acmi_parser() -> String {
    match zip::unzip("F15_SU27_BVR.zip.acmi") {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(acmi_parser(), "acmi_parser".to_string());
    }
}
