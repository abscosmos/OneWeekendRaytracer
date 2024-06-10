use std::time::{Duration, Instant};

pub struct ScopedTimer {
    pub name: &'static str,
    pub start_time: Instant,
    pub as_millis: bool,
}

impl ScopedTimer {
    pub fn new(name: &'static str, as_millis: bool) -> Self {
        Self { name, start_time: Instant::now(), as_millis }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        match self.as_millis {
            true => println!("[{}: {}ms]", self.name, self.start_time.elapsed().as_millis()),
            false => println!("[{}: {:.3}s]", self.name, self.start_time.elapsed().as_secs_f32()),
        }
    }
}

pub fn test_multiple(iterations: usize, test: fn(usize)) -> Duration {
    let mut acc = Duration::default();

    for i in 0..iterations {
        let start = Instant::now();
        test(i);
        acc += start.elapsed();
    }

    acc / iterations as u32
}