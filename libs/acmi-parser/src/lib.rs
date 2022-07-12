use acmi::{parse_acmi, AcmiError, Recording};

mod acmi;
mod zip;

enum Error {
    AcmiError(AcmiError),
    ZipError(zip::ZipError),
}

pub fn parse(file: &str) -> () {
    let content = zip::unzip(file)
        .map_err(|e| Error::ZipError(e))
        .and_then(parse_acmi);
}
