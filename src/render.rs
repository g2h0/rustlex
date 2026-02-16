use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::App;
use crate::watch_face;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let square = compute_square_area(area);
    watch_face::draw(frame, square, app);
}

/// Compute the largest visually-square Rect centered in the available area.
/// Terminal cells are ~2:1 (height:width in pixels), so we need
/// rect_width = rect_height * 2 in cell units for a visual square.
fn compute_square_area(area: Rect) -> Rect {
    let max_w = area.width;
    let max_h = area.height;

    let (w, h) = if max_w >= max_h * 2 {
        (max_h * 2, max_h)
    } else {
        (max_w, max_w / 2)
    };

    let x = area.x + (max_w.saturating_sub(w)) / 2;
    let y = area.y + (max_h.saturating_sub(h)) / 2;

    Rect::new(x, y, w, h)
}
