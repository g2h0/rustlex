use ratatui::style::Color;
use super::WatchTheme;

pub struct SubmarinerTheme;

impl WatchTheme for SubmarinerTheme {
    fn name(&self) -> &str { "Submariner" }

    fn bezel_color(&self) -> Color { Color::DarkGray }
    fn hour_hand_color(&self) -> Color { Color::White }
    fn minute_hand_color(&self) -> Color { Color::White }
    fn second_hand_color(&self) -> Color { Color::Red }
    fn marker_color(&self) -> Color { Color::Green }
    fn logo_color(&self) -> Color { Color::DarkGray }
    fn date_color(&self) -> Color { Color::White }

    fn hour_hand_length(&self) -> f64 { 0.50 }
    fn minute_hand_length(&self) -> f64 { 1.0 }
    fn second_hand_length(&self) -> f64 { 0.95 }

    fn has_date_window(&self) -> bool { true }
}
