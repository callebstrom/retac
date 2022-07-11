#[derive(Default)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Default)]
pub struct World {
    pub reference_time: i64,
    pub reference_coordinates: Coordinates,
}

#[derive(Default)]
pub struct Flight {
    pub id: i8,
}

#[derive(Default)]
pub struct Timeline {
    pub timeframes: Vec<Timeframe>,
}

#[derive(Default)]
pub struct Timeframe {
    pub time: i16,
    pub transforms: Vec<Transform>,
}

#[derive(Default)]
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

impl AcmiFile {
    pub fn try_from(raw: String) -> Result<Self, super::AcmiParseError> {
        AcmiFile::is_valid(&raw).map(|()| AcmiFile { raw })
    }

    pub fn raw(&self) -> &String {
        &self.raw
    }

    fn is_valid(raw: &String) -> Result<(), super::AcmiParseError> {
        if !raw.starts_with("FileType=text/acmi") {
            return Err(super::AcmiParseError::InvalidAcmi);
        }

        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub struct AttributeEntry {
    pub id: i8,
    pub attributes: Vec<(String, String)>,
}
