use std::cmp::Ordering;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}

impl Interval {
    pub fn from_cmp(a: f32, b: f32) -> Self {
        match a.partial_cmp(&b) {
            Some(cmp) => match cmp {
                Ordering::Less | Ordering::Equal => Self { min: a, max: b },
                Ordering::Greater => Self { min: b, max: a },
            }
            None => Self { min: f32::NAN, max: f32::NAN },
        }
    }
    
    pub fn from_enclosing(a: &Self, b: &Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, n: f32) -> bool {
        (self.min..=self.max).contains(&n)
    }

    pub fn surrounds(&self, n: f32) -> bool {
        self.min < n && n < self.max
    }

    pub fn clamp(&self, n: f32) -> f32 {
        n.clamp(self.min, self.max)
    }

    pub fn expanded(&self, delta: f32) -> Interval {
        assert!(delta > 0.0, "delta must be positive");

        let pad = delta / 2.0;
        Interval { min: self.min - pad, max: self.max + pad }
    }

    pub fn expanded_if_smaller(&self, delta: f32) -> Interval {
        if self.size() < delta {
            self.expanded(delta)
        } else {
            *self
        }
    }
}

impl Add<f32> for Interval {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<Interval> for f32 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}

pub const UNIT: Interval = Interval { min: 0.0, max: 1.0 };
pub const EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
pub const UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };