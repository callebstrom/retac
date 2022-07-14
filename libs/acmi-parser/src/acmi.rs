use itertools::Itertools;

#[derive(Default, Debug, PartialEq)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Default, Debug, PartialEq)]
pub struct World {
    pub reference_time: String,
    pub reference_coordinates: Coordinates,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Flight {
    pub id: i8,
}

#[derive(Default, Debug, PartialEq)]
pub struct Timeline {
    pub timeframes: Vec<Timeframe>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Timeframe {
    pub time: i16,
    pub transforms: Vec<Transform>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Transform {
    pub flight: i8,
    pub coordinates: Coordinates,
    pub roll: i8,
    pub pitch: i8,
    pub yaw: i8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AcmiFile {
    raw: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AcmiError {
    InvalidAcmi,
    InvalidZip,
    InvalidAttributEntry,
    Empty,
}

#[derive(Default, Debug, PartialEq)]
pub struct Recording {
    pub world: World,
    pub flights: Vec<Flight>,
    pub timeline: Timeline,
}

impl AcmiFile {
    pub fn try_from(raw: String) -> Result<Self, AcmiError> {
        AcmiFile::is_valid(&raw).map(|()| AcmiFile { raw })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    fn is_valid(raw: &String) -> Result<(), AcmiError> {
        let with_magic_bytes = raw
            .get(3..21)
            .and_then(
                |file_type| match file_type.starts_with("FileType=text/acmi") {
                    true => Some(()),
                    false => None,
                },
            )
            .map_or(Err(AcmiError::InvalidAcmi), |_| Ok(()));

        with_magic_bytes.or_else(|_| match raw.starts_with("FileType=text/acmi") {
            true => Ok(()),
            false => Err(AcmiError::InvalidAcmi),
        })
    }
}

pub fn parse_attributes(line: &str) -> Result<AttributeEntry, AcmiError> {
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
        .ok_or_else(|| AcmiError::InvalidAttributEntry)
}

fn parse_world_attribute(line: &str, recording: &mut Recording) -> () {
    let world_attribute_entry =
        parse_attributes(line).map_or(vec![], |attribute_entry| attribute_entry.attributes);

    for (key, value) in world_attribute_entry {
        match key.as_str() {
            "ReferenceTime" => recording.world.reference_time = value,
            "ReferenceLongitude" => {
                recording.world.reference_coordinates = Coordinates {
                    latitude: recording.world.reference_coordinates.latitude,
                    longitude: value.parse::<f32>().unwrap(),
                }
            }
            "ReferenceLatitude" => {
                recording.world.reference_coordinates = Coordinates {
                    latitude: value.parse::<f32>().unwrap(),
                    longitude: recording.world.reference_coordinates.longitude,
                }
            }
            _ => {}
        }
    }
}

pub fn parse_acmi(acmi_raw: String) -> Result<Recording, AcmiError> {
    AcmiFile::try_from(acmi_raw).map(|acmi| -> _ {
        acmi.raw().lines().fold(
            Recording {
                ..Default::default()
            },
            |mut recording, line| -> Recording {
                if line.starts_with("0,") {
                    parse_world_attribute(line, &mut recording);
                } else if line.starts_with(r"^([0-9])+,") {
                    let _b = parse_attributes(line);
                }

                recording
            },
        )
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct AttributeEntry {
    pub id: i8,
    pub attributes: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn given_valid_acmi_line_should_parse() {
        let given_id = "102";
        let given_attribute_name = "T".to_string();
        let given_attribute_value =
            "4.7941868|4.9813289|2433.77|-0.6|1.7||640699.63|-424017.5|".to_string();

        let given_line = format!(
            "{},{}={}",
            given_id, given_attribute_name, given_attribute_value
        )
        .to_string();

        let actual = parse_attributes(&given_line).unwrap();
        let expected = AttributeEntry {
            id: 102,
            attributes: vec![(given_attribute_name, given_attribute_value)],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_invalid_acmi_should_err() {
        let invalid_acmi = "FileType=text/html".to_string();
        assert_eq!(
            AcmiFile::try_from(invalid_acmi),
            Err(AcmiError::InvalidAcmi)
        );
    }

    #[test]
    fn given_valid_acmi_should_err() {
        let valid_acmi = "FileType=text/acmi".to_string();
        assert!(AcmiFile::try_from(valid_acmi).is_ok());
    }
}
