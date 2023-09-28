#[derive(Debug)]
pub enum Device {
    Factory { multiplier: f32 },
    Farm { multiplier: f32 },
    MissileSilo,
    Mine { multiplier: f32 },
    PowerPlant { multiplier: f32 },
    Satellite,
}
