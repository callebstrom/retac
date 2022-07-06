mod zip;

pub fn parse(file: &str) -> String {
    match zip::unzip(file) {
        Ok(res) => res,
        Err(err) => err.to_string(),
    }
}
