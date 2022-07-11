#[derive(Default)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

#[derive(Default)]
pub struct World {
    reference_time: i64,
    reference_coordinates: Coordinates,
}

#[derive(Default)]
pub struct Flight {
    id: i8,
}

#[derive(Default)]
pub struct Timeline {
    timeframes: Vec<Timeframe>,
}

#[derive(Default)]
pub struct Timeframe {
    time: i16,
    transforms: Vec<Transform>,
}

#[derive(Default)]
pub struct Transform {
    flight: i8,
    coordinates: Coordinates,
    roll: i8,
    pitch: i8,
    yaw: i8,
}
