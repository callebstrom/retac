use acmi::{parse_acmi, AcmiError, Recording};

mod acmi;
mod zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    AcmiError(AcmiError),
    ZipError(zip::ZipError),
}

pub fn parse(file: &str) -> Result<Recording, Error> {
    zip::unzip(file)
        .map_err(|e| Error::ZipError(e))
        .and_then(|content| parse_acmi(content).map_err(|e| Error::AcmiError(e)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn given_valid_acmi_zip_should_parse_world_attributes() {
        let recording = parse("F15_SU27_BVR.zip.acmi").unwrap();
        assert_eq!(recording.world.reference_coordinates.latitude, 36.0);
        assert_eq!(recording.world.reference_coordinates.longitude, 37.0);
    }

    #[test]
    fn given_non_existing_zip_should_err() {
        let result = parse("some_non_existing_file.zip");
        assert_eq!(result, Err(Error::ZipError(zip::ZipError::InvalidZip)))
    }
}
