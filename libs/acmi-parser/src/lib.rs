use acmi::{AcmiFile, Flight, Timeline, World};

mod acmi;
mod zip;

#[derive(Default)]
pub struct Recording {
    pub world: World,
    pub flights: Vec<Flight>,
    pub timeline: Timeline,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AcmiParseError {
    InvalidAcmi,
    InvalidZip,
    Empty,
}

pub fn parse(file: &str) -> Result<Recording, AcmiParseError> {
    zip::unzip(file)
        .map_err(|_| AcmiParseError::InvalidZip)
        .and_then(parse_acmi)
}

fn parse_acmi(acmi_raw: String) -> Result<Recording, AcmiParseError> {
    AcmiFile::try_from(acmi_raw).map(|acmi| -> Recording {
        let _world = parse_world(acmi);
        Recording {
            ..Default::default()
        }
    })
}

fn parse_world(_acmi: AcmiFile) -> World {
    World {
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn given_invalid_acmi_should_err() {
        let invalid_acmi = "FileType=text/html".to_string();
        assert_eq!(
            AcmiFile::try_from(invalid_acmi),
            Err(AcmiParseError::InvalidAcmi)
        );
    }

    #[test]
    fn given_valid_acmi_should_err() {
        let valid_acmi = "FileType=text/acmi".to_string();
        assert!(AcmiFile::try_from(valid_acmi).is_ok());
    }
}
