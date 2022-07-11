use acmi::{Flight, Timeline, World};

mod acmi;
mod zip;

#[derive(Default)]
pub struct Recording {
    pub world: World,
    pub flights: Vec<Flight>,
    pub timeline: Timeline,
}

struct AcmiFile {
    raw: String,
}

pub enum AcmiParseError {
    InvalidAcmi,
    InvalidZip,
    Empty,
}

impl AcmiFile {
    pub fn try_from(raw: String) -> Result<Self, AcmiParseError> {
        AcmiFile::is_valid(&raw).map(|()| AcmiFile { raw })
    }

    fn is_valid(raw: &String) -> Result<(), AcmiParseError> {
        if !raw.starts_with("FileType=text/acmi") {
            return Err(AcmiParseError::InvalidAcmi);
        }

        Ok(())
    }
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
