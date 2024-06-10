use glm::{Vec2, Vec3};
use crate::color::Color;
use crate::texture::Texture;

pub struct Perlin<const RES: usize> {
    rand_vec: [Vec3; RES],
    perm_x: [i32; RES],
    perm_y: [i32; RES],
    perm_z: [i32; RES],
}

impl<const RES: usize> Default for Perlin<RES> {
    fn default() -> Self {
        let rand_vec = std::array::from_fn(|_| {
            Vec3::from_fn(|_,_| rand::random_range(-1.0..1.0) )
        });

        Self {
            rand_vec,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }
}

impl<const RES: usize> Perlin<RES> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn turbulence(&self, mut p: Vec3, freq: f32, depth: u8) -> f32 {
        let mut acc = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(p * freq);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let floor = p.map(f32::floor);
        let uvw = p - floor;

        let mut c = [[[Vec3::default();2];2];2];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let and = self.rand_vec.len() as i32 - 1;

                    c[i as usize][j as usize][k as usize] = self.rand_vec[(
                        self.perm_x[( (floor.x as i32 + i) & and ) as usize] ^
                        self.perm_y[( (floor.y as i32 + j) & and ) as usize] ^
                        self.perm_z[( (floor.z as i32 + k) & and ) as usize]
                    ) as usize ];
                }
            }
        }


        Self::perlin_interpolation(&c, uvw)
    }

    fn perlin_interpolation(c: &[[[Vec3;2];2];2], coords: Vec3) -> f32 {
        let coords2 = coords.map(|n| n * n * (3.0 - 2.0 * n) );

        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    let weight = coords - Vec3::new(i, j, k);

                    acc += ( i * coords2.x + (1.0 - i) * (1.0 - coords2.x ) )
                         * ( j * coords2.y + (1.0 - j) * (1.0 - coords2.y ) )
                         * ( k * coords2.z + (1.0 - k) * (1.0 - coords2.z ) )
                         * c[i as usize][j as usize][k as usize].dot(&weight);
                }
            }
        }

        acc
    }

    fn generate_perm() -> [i32; RES] {
        let mut p = std::array::from_fn(|i| i as i32);

        for i in (1..RES).rev() {
            p.swap(i, rand::random_range(0..=i));
        }

        p
    }
}

pub struct NoiseTexture {
    noise: Perlin<256>,
    pub freq: f32,
    pub depth: u8,
}

impl NoiseTexture {
    pub fn new(freq: f32, depth: u8) -> Self {
        assert!(freq > 0.0, "frequency must be positive");

        Self {
            noise: Perlin::new(),
            freq,
            depth,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: Vec2, p: Vec3) -> Color {
        Color::from_element(1.0) * self.noise.turbulence(p, self.freq, self.depth)
    }
}

pub struct MarbleTexture {
    noise: Perlin<256>,
    pub freq: f32,
    pub depth: u8,
}

impl MarbleTexture {
    pub fn new(freq: f32, depth: u8) -> Self {
        Self {
            noise: Perlin::new(),
            freq,
            depth,
        }
    }
}

impl Texture for MarbleTexture {
    fn value(&self, _uv: Vec2, p: Vec3) -> Color {
        Color::from_element(0.5) * (1.0 + f32::sin(self.freq * p.z + 10.0 * self.noise.turbulence(p, 1.0, self.depth)))
    }
}