use std::f64::consts::TAU;
use std::time::Instant;
use crate::theme::{WatchTheme, submariner::SubmarinerTheme};

// 120 clicks per full rotation, matching real Submariner
const CLICK_ANGLE: f64 = TAU / 120.0;

pub struct Star {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub phase: f64,
    pub speed: f64,
}

/// Simple LCG pseudo-random number generator (no external crate needed).
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    /// Returns a f64 in [0.0, 1.0)
    fn next_f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Returns a f64 in [lo, hi)
    fn next_range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next_f64() * (hi - lo)
    }
}

fn generate_stars() -> Vec<Star> {
    let mut rng = SimpleRng::new(0xDEAD_BEEF_CAFE);
    let mut stars = Vec::new();
    let bounds = 100.0;
    let min_radius = 99.0; // must be outside the watch face

    while stars.len() < 50 {
        let x = rng.next_range(-bounds, bounds);
        let y = rng.next_range(-bounds, bounds);
        let dist = (x * x + y * y).sqrt();
        if dist > min_radius {
            stars.push(Star {
                x,
                y,
                size: rng.next_range(0.3, 1.2),
                phase: rng.next_range(0.0, TAU),
                speed: rng.next_range(0.5, 2.0),
            });
        }
    }
    stars
}

pub struct App {
    pub running: bool,
    pub theme: Box<dyn WatchTheme>,
    pub bezel_offset: f64, // radians, added to all bezel element angles
    pub stars: Vec<Star>,
    pub stars_enabled: bool,
    pub lume_mode: bool,
    pub smooth_seconds: bool,
    pub start_time: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            theme: Box::new(SubmarinerTheme),
            bezel_offset: 0.0,
            stars: generate_stars(),
            stars_enabled: false,
            lume_mode: false,
            smooth_seconds: false,
            start_time: Instant::now(),
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_stars(&mut self) {
        self.stars_enabled = !self.stars_enabled;
    }

    pub fn toggle_lume(&mut self) {
        self.lume_mode = !self.lume_mode;
    }

    pub fn toggle_smooth(&mut self) {
        self.smooth_seconds = !self.smooth_seconds;
    }

    /// Rotate the bezel by the given number of clicks (positive = clockwise).
    pub fn rotate_bezel(&mut self, clicks: i32) {
        self.bezel_offset += clicks as f64 * CLICK_ANGLE;
        // Normalize to [0, TAU)
        self.bezel_offset = self.bezel_offset.rem_euclid(TAU);
    }
}
