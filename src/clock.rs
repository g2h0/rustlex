use chrono::{Local, Timelike, Datelike};
use std::f64::consts::TAU;

pub struct ClockHands {
    pub hour_angle: f64,
    pub minute_angle: f64,
    pub second_angle: f64,
    pub date_day: u32,
}

impl ClockHands {
    pub fn now(smooth: bool) -> Self {
        let now = Local::now();
        let h = (now.hour() % 12) as f64;
        let m = now.minute() as f64;
        let s = now.second() as f64;

        // Angles in radians: 0 = 12 o'clock, increasing clockwise
        let second_angle = if smooth {
            let nanos = now.nanosecond() as f64;
            (s + nanos / 1_000_000_000.0) * TAU / 60.0
        } else {
            s * TAU / 60.0
        };
        let minute_angle = (m + s / 60.0) * TAU / 60.0;
        let hour_angle = (h + m / 60.0) * TAU / 12.0;

        Self {
            hour_angle,
            minute_angle,
            second_angle,
            date_day: now.day(),
        }
    }
}

/// Convert a clock angle (0 = 12 o'clock, clockwise) and length to canvas (x, y).
/// In canvas coords: +x is right, +y is up.
pub fn hand_endpoint(angle: f64, length: f64) -> (f64, f64) {
    let x = length * angle.sin();
    let y = length * angle.cos();
    (x, y)
}
