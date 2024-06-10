#[derive(Copy, Clone, Debug)]
pub struct FocusSettings {
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

impl Default for FocusSettings {
    fn default() -> Self {
        Self { defocus_angle: 0.0, focus_dist: 10.0 }
    }
}