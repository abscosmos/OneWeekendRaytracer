use std::num::{NonZero, NonZeroU32};

pub struct CameraDimensions {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

impl CameraDimensions {
    const WIDESCREEN_AR: f32 = 16.0 / 9.0;
    
    pub const MEDIUM: Self = Self::from_aspect_ratio(
        NonZero::new(800).expect("nonzero"),
        Self::WIDESCREEN_AR
    );

    pub const LARGE: Self = Self::from_aspect_ratio(
        NonZero::new(1200).expect("nonzero"),
        Self::WIDESCREEN_AR
    );
    
    pub const FHD: Self = Self {
        width: NonZero::new(1920).expect("nonzero"),
        height: NonZero::new(1080).expect("nonzero"),
    };
    
    pub const fn from_aspect_ratio(width: NonZeroU32, aspect_ratio: f32) -> Self {
        let height = (width.get() as f32 / aspect_ratio).max(1.0) as u32;
        
        Self {
            width,
            height: NonZero::new(height).expect("nonzero unless aspect ratio was 0")
        }
    }

    pub const fn square(len: NonZeroU32) -> Self {
        Self { width: len, height: len }
    }

    pub const fn aspect_ratio(&self) -> f32 {
        self.width.get() as f32 / self.height.get() as f32
    }

}

impl Default for CameraDimensions {
    fn default() -> Self {
        Self::MEDIUM
    }
}

