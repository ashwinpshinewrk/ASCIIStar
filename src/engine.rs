use glam::Vec3;
use ratatui::style::Color;

pub struct Rng {
    state: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    pub fn next_f32(&mut self) -> f32 {
        //Cononical xor shifters - DO NOT TOUCH BROO
        self.state ^= self.state >> 13;
        self.state ^= self.state << 17;
        self.state ^= self.state >> 5;
        (self.state as f32) / (u32::MAX as f32)
    }
}

pub struct SnowFlake {
    pub pos: Vec3,
    pub drift: f32,
    pub speed: f32,
}

pub struct Point3D {
    pub pos: Vec3,
    pub color: Color,
    pub is_bright: bool,
    pub is_ornament: bool,
    pub birth_threshold: f32,
    pub ornament_id: usize,
    pub is_star: bool,
    pub lod_level: f32,
    pub fixed_char: Option<char>,
}
