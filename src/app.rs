use crate::engine::SnowFlake;
use std::time::Instant;

pub struct App {
    pub input: String,
    pub angle_x: f32,
    pub angle_y: f32,
    pub anim_progress: f32,
    pub zoom_progress: f32,
    pub is_growing: bool,
    pub is_zooming: bool,
    pub start_time: Instant,
    pub snow: Vec<SnowFlake>,
}

impl App {
    pub fn new(snow: Vec<SnowFlake>) -> Self {
        Self {
            input: String::from("MERRY CHRISTMAS"),
            angle_x: 0.2,
            angle_y: 0.0,
            anim_progress: 0.0,
            zoom_progress: 0.0,
            is_growing: false,
            is_zooming: false,
            start_time: Instant::now(),
            snow,
        }
    }
}
