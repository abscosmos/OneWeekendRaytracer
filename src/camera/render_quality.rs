pub struct RenderQuality {
    pub samples_per_pixel: u32,
    pub max_depth: u16,
}

impl RenderQuality {
    pub const LOW: RenderQuality = RenderQuality { samples_per_pixel: 50, max_depth: 10 };
    pub const MEDIUM: RenderQuality = RenderQuality { samples_per_pixel: 100, max_depth: 20 };
    pub const HIGH: RenderQuality = RenderQuality { samples_per_pixel: 500, max_depth: 50 };
}

impl Default for RenderQuality {
    fn default() -> Self {
        Self::MEDIUM
    }
}

pub(super) struct InternalRenderQuality {
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    max_depth: u16,
}

impl From<RenderQuality> for InternalRenderQuality {
    fn from(value: RenderQuality) -> Self {
        Self {
            samples_per_pixel: value.samples_per_pixel,
            pixel_samples_scale: (value.samples_per_pixel as f32).recip(),
            max_depth: value.max_depth,
        }
    }
}

impl InternalRenderQuality {
    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn pixel_samples_scale(&self) -> f32 {
        self.pixel_samples_scale
    }

    pub fn max_depth(&self) -> u16 {
        self.max_depth
    }
}
