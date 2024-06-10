pub mod dimensions;
pub mod render_quality;
pub mod focus_settings;

use std::ops::RangeInclusive;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use glm::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use crate::color::{Color, transform_color_to_pixel};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use crate::camera::dimensions::CameraDimensions;
use crate::camera::focus_settings::FocusSettings;
use crate::camera::render_quality::{InternalRenderQuality, RenderQuality};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::ray::Ray;
use crate::util::timer::ScopedTimer;
use crate::util::vec3_random::random_vec_in_unit_disk;

pub struct Camera {
    pub center: Vec3,

    dimensions: CameraDimensions,
    render_quality: InternalRenderQuality,
    background_color: Color,

    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    const UP_VECTOR: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    const MIN_RAY_DISTANCE: f32 = 0.001;
    const RANDOM_RAY_OFFSET_RANGE: RangeInclusive<f32> = -0.5..=0.5f32;

    pub fn new(
        center: Vec3,
        look_at: Vec3,
        background_color: Color,
        dimensions: CameraDimensions,
        render_quality: RenderQuality,
        focus_settings: FocusSettings,
        fov: f32,

    ) -> Self {
        let h = f32::tan(fov.to_radians() / 2.0);
        let viewport_height = 2.0 * h * focus_settings.focus_dist;
        let viewport_width = viewport_height * dimensions.aspect_ratio();

        let w = (center - look_at).normalize();
        let u = Self::UP_VECTOR.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / dimensions.width.get() as f32;
        let pixel_delta_v = viewport_v / dimensions.height.get() as f32;

        let viewport_upper_left = center - focus_settings.focus_dist * w
            - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_settings.focus_dist * f32::tan(f32::to_radians(focus_settings.defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let render_quality = render_quality.into();

        let defocus_angle = focus_settings.defocus_angle;

        Self {
            center,
            dimensions,
            render_quality,
            background_color,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render_screen(&self, world: &impl Hittable, debug_frequency: u8) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.dimensions.width.get(), self.dimensions.height.get());

        for y in 0..self.dimensions.height.get() {
            for x in 0..self.dimensions.width.get() {
                img.put_pixel(x, y, self.render_pixel(world, x, y));
            }

            if debug_frequency != 0 && (y + 1) % debug_frequency as u32 == 0 {
                println!("Lines completed: {}/{}", y / self.dimensions.width, self.dimensions.height);
            }
        }

        img
    }

    pub fn render_screen_par(&self, world: &impl Hittable, debug_frequency: u8, timer_name: Option<&'static str>) -> RgbImage {
        let _timer = timer_name.map(|name| ScopedTimer::new(name, false));
        
        let mut img: RgbImage = ImageBuffer::new(self.dimensions.width.get(), self.dimensions.height.get());

        let lines = Arc::new(AtomicU32::new(0));

        img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
            *pixel = self.render_pixel(world, x, y);

            let curr = lines.fetch_add(1, Ordering::Relaxed) + 1;

            if debug_frequency != 0 && curr % self.dimensions.width == 0 &&  curr / self.dimensions.width % debug_frequency as u32 == 0 {
                println!("Lines completed: {}/{}", curr / self.dimensions.width, self.dimensions.height);
            }
        });

        img
    }

    fn render_pixel(&self, world: &impl Hittable, x: u32, y: u32) -> Rgb<u8> {
        let mut pixel_color = Color::default();

        for _ in 0..self.render_quality.samples_per_pixel() {
            let ray = self.get_ray(x, y);
            pixel_color += self.ray_color(self.render_quality.max_depth(), ray, world);
        }

        transform_color_to_pixel(self.render_quality.pixel_samples_scale() * pixel_color)
    }

    fn get_ray(&self, x :u32, y: u32) -> Ray {
        let offset_x = rand::random_range(Self::RANDOM_RAY_OFFSET_RANGE);
        let offset_y = rand::random_range(Self::RANDOM_RAY_OFFSET_RANGE);

        let pixel_sample = self.pixel00_loc
            + ((x as f32 + offset_x) * self.pixel_delta_u)
            + ((y as f32 + offset_y) * self.pixel_delta_v);

        let origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };

        Ray { origin, direction: pixel_sample - origin }
    }

    fn ray_color(&self, depth: u16, ray: Ray, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Vec3::zeros();
        }

        if let Some(rec) = world.hit(ray, Interval { min: Self::MIN_RAY_DISTANCE, max: f32::INFINITY }) {
            let emission_color = rec.material.emitted(rec.uv, rec.p);

            if let Some(ScatterResult { attenuation, scattered }) = rec.material.scatter(ray, &rec) {
                let ray_color = self.ray_color(depth - 1, scattered, world);

                emission_color + attenuation.component_mul(&ray_color)
            } else {
                emission_color
            }
        } else {
            self.background_color
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let v = random_vec_in_unit_disk();
        self.center + (v.x * self.defocus_disk_u) + (v.y * self.defocus_disk_v)
    }
}