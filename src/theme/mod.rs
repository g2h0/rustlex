pub mod submariner;

use ratatui::style::Color;

/// Defines the visual properties of a watch face.
#[allow(dead_code)]
pub trait WatchTheme {
    fn name(&self) -> &str;

    // Colors
    fn bezel_color(&self) -> Color;
    fn hour_hand_color(&self) -> Color;
    fn minute_hand_color(&self) -> Color;
    fn second_hand_color(&self) -> Color;
    fn marker_color(&self) -> Color;
    fn logo_color(&self) -> Color;
    fn date_color(&self) -> Color;

    // Hand lengths as fractions of marker-inner radius
    fn hour_hand_length(&self) -> f64 { 0.50 }
    fn minute_hand_length(&self) -> f64 { 0.85 }
    fn second_hand_length(&self) -> f64 { 0.95 }

    // Features
    fn has_date_window(&self) -> bool { false }
}
