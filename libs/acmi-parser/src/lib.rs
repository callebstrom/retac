use acmi::{Flight, Timeline, World};

mod acmi;
mod zip;

#[derive(Default)]
pub struct Recording {
    world: World,
    flights: Vec<Flight>,
    timeline: Timeline,
}

pub fn parse(file: &str) -> Result<Recording, &str> {
    zip::unzip(file).map(parse_acmi)
}

fn parse_acmi(acmi_raw: String) -> Recording {
    Recording {
        ..Default::default()
    }
}
