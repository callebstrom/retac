use itertools::Itertools;
use std::collections::HashMap;

use acmi::{AcmiFile, AttributeEntry, Flight, Timeline, World};

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
    InvalidAttributEntry,
    Empty,
}

pub fn parse(file: &str) -> Result<Recording, AcmiParseError> {
    zip::unzip(file)
        .map_err(|_| AcmiParseError::InvalidZip)
        .and_then(parse_acmi)
}

fn parse_acmi(acmi_raw: String) -> Result<Recording, AcmiParseError> {
    AcmiFile::try_from(acmi_raw).map(|acmi| -> Recording {
        Recording {
            ..Default::default()
        }
    })
}

const WORLD_PREFIX: &str = "0,";

fn parse_attributes(line: String) -> Result<AttributeEntry, AcmiParseError> {
    let mut raw_attributes = line.split(",");

    let maybe_id = raw_attributes
        .next()
        .map(|id| id.parse::<i8>())
        .and_then(|id| match id {
            Ok(i) => Some(i),
            Err(_) => None,
        });

    let attributes = raw_attributes
        .map(|attribute| attribute.split("=").tuples().unzip())
        .collect_vec();

    maybe_id
        .map(|id| AttributeEntry { id, attributes })
        .ok_or_else(|| AcmiParseError::InvalidAttributEntry)
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
