use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Circle, Line, Context};
use std::f64::consts::{PI, TAU};

use crate::app::App;
use crate::clock::{ClockHands, hand_endpoint};

const BOUNDS: f64 = 100.0;

// ── Radius layout (outside → inside) ──
const CASE_EDGE: f64 = 99.0;
const BEZEL_OUTER: f64 = 97.0;
const BEZEL_NUM_R: f64 = 88.0;   // center radius for bezel numbers
const BEZEL_INNER: f64 = 80.0;
const CHAPTER_OUTER: f64 = 80.0;
const CHAPTER_INNER: f64 = 77.0;
const MARKER_OUTER: f64 = 74.0;
const MARKER_INNER: f64 = 65.0;
const MARKER_CENTER: f64 = 69.5;  // center of circle dot markers

// ── Letter definitions for RUSTLEX logo ──
type Segs = &'static [(f64, f64, f64, f64)];

const LETTER_R: Segs = &[
    (0.0, 0.0, 0.0, 10.0), (0.0, 10.0, 6.0, 10.0),
    (6.0, 10.0, 6.0, 5.0), (6.0, 5.0, 0.0, 5.0),
    (0.0, 5.0, 6.0, 0.0),
];
const LETTER_U: Segs = &[
    (0.0, 10.0, 0.0, 0.0), (0.0, 0.0, 6.0, 0.0), (6.0, 0.0, 6.0, 10.0),
];
const LETTER_S: Segs = &[
    (6.0, 10.0, 0.0, 10.0), (0.0, 10.0, 0.0, 5.0),
    (0.0, 5.0, 6.0, 5.0), (6.0, 5.0, 6.0, 0.0), (6.0, 0.0, 0.0, 0.0),
];
const LETTER_T: Segs = &[
    (0.0, 10.0, 6.0, 10.0), (3.0, 10.0, 3.0, 0.0),
];
const LETTER_L: Segs = &[
    (0.0, 10.0, 0.0, 0.0), (0.0, 0.0, 6.0, 0.0),
];
const LETTER_E: Segs = &[
    (0.0, 0.0, 0.0, 10.0), (0.0, 10.0, 6.0, 10.0),
    (0.0, 5.0, 4.0, 5.0), (0.0, 0.0, 6.0, 0.0),
];
const LETTER_X: Segs = &[
    (0.0, 10.0, 6.0, 0.0), (0.0, 0.0, 6.0, 10.0),
];
const LOGO_LETTERS: [Segs; 7] = [
    LETTER_R, LETTER_U, LETTER_S, LETTER_T, LETTER_L, LETTER_E, LETTER_X,
];

// ── Crown shape (centered at origin, 12 wide × 10 tall) ──
const CROWN_SEGS: &[(f64, f64, f64, f64)] = &[
    // Base
    (-6.0, 0.0, 6.0, 0.0),
    // Left wall
    (-6.0, 0.0, -6.0, 2.0),
    // Right wall
    (6.0, 0.0, 6.0, 2.0),
    // Prongs (zigzag from left to right)
    (-6.0, 2.0, -5.0, 8.0),   // up to prong 1
    (-5.0, 8.0, -3.5, 3.0),   // down to valley 1
    (-3.5, 3.0, -2.0, 7.0),   // up to prong 2
    (-2.0, 7.0, -0.5, 3.0),   // down to valley 2
    (-0.5, 3.0, 0.0, 9.0),    // up to prong 3 (center, tallest)
    (0.0, 9.0, 0.5, 3.0),     // down to valley 3
    (0.5, 3.0, 2.0, 7.0),     // up to prong 4
    (2.0, 7.0, 3.5, 3.0),     // down to valley 4
    (3.5, 3.0, 5.0, 8.0),     // up to prong 5
    (5.0, 8.0, 6.0, 2.0),     // down to right wall
];
// Crown dot positions (tips of prongs 1, 3, 5)
const CROWN_DOTS: &[(f64, f64)] = &[(-5.0, 8.0), (0.0, 9.0), (5.0, 8.0)];

// ── 7-segment digit definitions ──
// [top, top-left, top-right, middle, bottom-left, bottom-right, bottom]
const DIGITS: [[bool; 7]; 10] = [
    [true,  true,  true,  false, true,  true,  true],   // 0
    [false, false, true,  false, false, true,  false],   // 1
    [true,  false, true,  true,  true,  false, true],    // 2
    [true,  false, true,  true,  false, true,  true],    // 3
    [false, true,  true,  true,  false, true,  false],   // 4
    [true,  true,  false, true,  false, true,  true],    // 5
    [true,  true,  false, true,  true,  true,  true],    // 6
    [true,  false, true,  false, false, true,  false],   // 7
    [true,  true,  true,  true,  true,  true,  true],    // 8
    [true,  true,  true,  true,  false, true,  true],    // 9
];

// ── Star snapshot (owned copy for closure capture) ──
struct StarData {
    x: f64,
    y: f64,
    size: f64,
    phase: f64,
    speed: f64,
}

// ── Theme data + app state (owned, for closure capture) ──
struct ThemeData {
    bezel_color: Color,
    hour_hand_color: Color,
    minute_hand_color: Color,
    second_hand_color: Color,
    marker_color: Color,
    logo_color: Color,
    date_color: Color,
    hour_hand_length: f64,
    minute_hand_length: f64,
    second_hand_length: f64,
    has_date_window: bool,
    bezel_offset: f64,
    lume_mode: bool,
    smooth_seconds: bool,
    elapsed: f64,
    stars_enabled: bool,
    stars: Vec<StarData>,
}

impl ThemeData {
    fn from_app(app: &App) -> Self {
        let theme = app.theme.as_ref();
        Self {
            bezel_color: theme.bezel_color(),
            hour_hand_color: theme.hour_hand_color(),
            minute_hand_color: theme.minute_hand_color(),
            second_hand_color: theme.second_hand_color(),
            marker_color: theme.marker_color(),
            logo_color: theme.logo_color(),
            date_color: theme.date_color(),
            hour_hand_length: theme.hour_hand_length(),
            minute_hand_length: theme.minute_hand_length(),
            second_hand_length: theme.second_hand_length(),
            has_date_window: theme.has_date_window(),
            bezel_offset: app.bezel_offset,
            lume_mode: app.lume_mode,
            smooth_seconds: app.smooth_seconds,
            elapsed: app.elapsed_secs(),
            stars_enabled: app.stars_enabled,
            stars: app.stars.iter().map(|s| StarData {
                x: s.x, y: s.y, size: s.size, phase: s.phase, speed: s.speed,
            }).collect(),
        }
    }
}

// ── Rotation helper ──
// Rotates a point around the origin for a given clock angle.
// Clock angle: 0 = 12 o'clock, increasing clockwise.
// Maps local "up" (+y) to the radially-outward direction at that angle.
fn rotate_for_clock(x: f64, y: f64, clock_angle: f64) -> (f64, f64) {
    let cos_a = clock_angle.cos();
    let sin_a = clock_angle.sin();
    (x * cos_a + y * sin_a, -x * sin_a + y * cos_a)
}

// ── Main draw ──
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let td = ThemeData::from_app(app);
    let clock = ClockHands::now(td.smooth_seconds);

    let marker = if area.width < 40 {
        Marker::HalfBlock
    } else {
        Marker::Braille
    };

    let canvas = Canvas::default()
        .x_bounds([-BOUNDS, BOUNDS])
        .y_bounds([-BOUNDS, BOUNDS])
        .marker(marker)
        .background_color(Color::Reset)
        .paint(move |ctx| {
            paint_stars(ctx, &td);
            paint_bezel(ctx, &td);
            paint_chapter_ring(ctx, &td);
            paint_hour_markers(ctx, &td);
            paint_crown(ctx, &td);
            paint_logo(ctx, &td);
            paint_date_window(ctx, &td, clock.date_day);
            paint_hands(ctx, &td, &clock);
            paint_center_dot(ctx, &td);
        });

    frame.render_widget(canvas, area);
}

// ══════════════════════════════════════════════════════════════
// STARS — twinkling background particles in the corner negative space
// ══════════════════════════════════════════════════════════════
fn paint_stars(ctx: &mut Context, td: &ThemeData) {
    if !td.stars_enabled { return; }
    let elapsed = td.elapsed;
    for star in &td.stars {
        let val = (elapsed * star.speed + star.phase).sin();
        let color = if val > 0.5 {
            Color::White
        } else if val > -0.3 {
            Color::Gray
        } else {
            Color::DarkGray
        };
        ctx.draw(&Circle { x: star.x, y: star.y, radius: star.size, color });
    }
}

// ══════════════════════════════════════════════════════════════
// BEZEL — outer rotating dive bezel with triangle, numbers, ticks
// ══════════════════════════════════════════════════════════════
fn paint_bezel(ctx: &mut Context, td: &ThemeData) {
    if td.lume_mode { return; }
    let bc = td.bezel_color;
    let bo = td.bezel_offset; // angular offset from scrollwheel

    // Outer and inner bezel rings (circles don't rotate)
    ctx.draw(&Circle { x: 0.0, y: 0.0, radius: CASE_EDGE, color: bc });
    ctx.draw(&Circle { x: 0.0, y: 0.0, radius: BEZEL_OUTER, color: bc });
    ctx.draw(&Circle { x: 0.0, y: 0.0, radius: BEZEL_INNER, color: bc });

    // ── Bezel triangle at 12 o'clock (the pip / zero marker) ──
    let tri_inner = 83.0;
    let tri_outer = 94.0;
    let tri_spread = 0.04;
    let angle_12 = bo; // rotates with bezel
    let (tx, ty) = hand_endpoint(angle_12, tri_outer);
    let (lx, ly) = hand_endpoint(angle_12 - tri_spread, tri_inner);
    let (rx, ry) = hand_endpoint(angle_12 + tri_spread, tri_inner);
    ctx.draw(&Line { x1: lx, y1: ly, x2: tx, y2: ty, color: td.marker_color });
    ctx.draw(&Line { x1: rx, y1: ry, x2: tx, y2: ty, color: td.marker_color });
    ctx.draw(&Line { x1: lx, y1: ly, x2: rx, y2: ry, color: td.marker_color });
    // Luminous pip circle inside the triangle
    let (px, py) = hand_endpoint(angle_12, 88.0);
    ctx.draw(&Circle { x: px, y: py, radius: 1.5, color: td.marker_color });

    // ── Bezel tick marks ──
    // Minutes 0-15: individual minute ticks (fine graduation)
    // Minutes 15-60: ticks only at every 5 minutes
    // Skip minute 0 (that's the triangle)
    for i in 1..60 {
        let is_five = i % 5 == 0;
        let in_first_quarter = i <= 15;

        if !is_five && !in_first_quarter {
            continue;
        }

        let angle = bo + (i as f64) * TAU / 60.0; // offset applied

        if is_five {
            // Bold rectangular tick at 5-minute positions
            let tick_h = BEZEL_OUTER - 82.0;
            let center_r = (BEZEL_OUTER + 82.0) / 2.0;
            draw_rotated_rect(ctx, angle, center_r, 2.5, tick_h, td.marker_color);
        } else {
            // Fine line tick (minutes 1-15 only)
            let (x1, y1) = hand_endpoint(angle, 84.0);
            let (x2, y2) = hand_endpoint(angle, BEZEL_OUTER);
            ctx.draw(&Line { x1, y1, x2, y2, color: bc });
        }
    }

    // ── Bezel numbers: 10, 20, 30, 40, 50 ──
    let bezel_numbers: [(u32, f64); 5] = [
        (10, bo + 10.0 / 60.0 * TAU),
        (20, bo + 20.0 / 60.0 * TAU),
        (30, bo + 30.0 / 60.0 * TAU),
        (40, bo + 40.0 / 60.0 * TAU),
        (50, bo + 50.0 / 60.0 * TAU),
    ];
    for &(number, clock_angle) in &bezel_numbers {
        draw_bezel_number(ctx, number, clock_angle, BEZEL_NUM_R, td.marker_color);
    }
}

/// Draw a two-digit number on the bezel, rotated to face outward.
fn draw_bezel_number(ctx: &mut Context, number: u32, clock_angle: f64, radius: f64, color: Color) {
    let tens = number / 10;
    let ones = number % 10;

    let dw = 4.5;   // digit width
    let dh = 7.0;   // digit height
    let gap = 2.0;  // gap between digits
    let total_w = 2.0 * dw + gap;

    // Center position on bezel
    let (bx, by) = hand_endpoint(clock_angle, radius);

    // Draw tens digit (offset left of center)
    draw_digit_rotated(ctx, tens, -(total_w / 2.0), -dh / 2.0, dw, dh, bx, by, clock_angle, color);
    // Draw ones digit (offset right of center)
    draw_digit_rotated(ctx, ones, gap / 2.0, -dh / 2.0, dw, dh, bx, by, clock_angle, color);
}

/// Draw a 7-segment digit in local coordinates, rotated by clock_angle, translated to (cx, cy).
fn draw_digit_rotated(
    ctx: &mut Context, digit: u32,
    lx: f64, ly: f64, w: f64, h: f64,
    cx: f64, cy: f64, clock_angle: f64, color: Color,
) {
    if digit > 9 { return; }
    let segs = DIGITS[digit as usize];
    let hh = h / 2.0;

    // 7 segments in local coordinates (relative to digit box at lx, ly)
    let local_lines: [(f64, f64, f64, f64); 7] = [
        (lx, ly + h, lx + w, ly + h),           // top
        (lx, ly + h, lx, ly + hh),              // top-left
        (lx + w, ly + h, lx + w, ly + hh),      // top-right
        (lx, ly + hh, lx + w, ly + hh),         // middle
        (lx, ly + hh, lx, ly),                  // bottom-left
        (lx + w, ly + hh, lx + w, ly),          // bottom-right
        (lx, ly, lx + w, ly),                   // bottom
    ];

    for (i, &(x1, y1, x2, y2)) in local_lines.iter().enumerate() {
        if segs[i] {
            let (rx1, ry1) = rotate_for_clock(x1, y1, clock_angle);
            let (rx2, ry2) = rotate_for_clock(x2, y2, clock_angle);
            ctx.draw(&Line {
                x1: cx + rx1, y1: cy + ry1,
                x2: cx + rx2, y2: cy + ry2,
                color,
            });
        }
    }
}

// ══════════════════════════════════════════════════════════════
// CHAPTER RING — fine minute tick track between bezel and dial
// ══════════════════════════════════════════════════════════════
fn paint_chapter_ring(ctx: &mut Context, td: &ThemeData) {
    if td.lume_mode { return; }
    for i in 0..60 {
        let angle = (i as f64) * TAU / 60.0;
        let inner_r = if i % 5 == 0 { CHAPTER_INNER - 2.5 } else { CHAPTER_INNER };
        let (x1, y1) = hand_endpoint(angle, inner_r);
        let (x2, y2) = hand_endpoint(angle, CHAPTER_OUTER);
        ctx.draw(&Line { x1, y1, x2, y2, color: td.bezel_color });
    }
}

// ══════════════════════════════════════════════════════════════
// HOUR MARKERS — circle dots, rectangular batons, triangle
// ══════════════════════════════════════════════════════════════
fn paint_hour_markers(ctx: &mut Context, td: &ThemeData) {
    let color = if td.lume_mode { Color::LightGreen } else { td.marker_color };
    for h in 1..=12 {
        let angle = (h as f64) * TAU / 12.0;

        match h {
            12 => {
                // Inverted triangle marker at 12
                let spread = 0.075;
                let (lx, ly) = hand_endpoint(angle - spread, MARKER_INNER);
                let (rx, ry) = hand_endpoint(angle + spread, MARKER_INNER);
                let (tx, ty) = hand_endpoint(angle, MARKER_OUTER);
                ctx.draw(&Line { x1: lx, y1: ly, x2: tx, y2: ty, color });
                ctx.draw(&Line { x1: rx, y1: ry, x2: tx, y2: ty, color });
                ctx.draw(&Line { x1: lx, y1: ly, x2: rx, y2: ry, color });
            }
            3 | 6 | 9 => {
                // Rectangular baton markers — drawn as rotated rectangles
                draw_baton(ctx, angle, color);
            }
            _ => {
                // Circle/dot lume indices
                let (mx, my) = hand_endpoint(angle, MARKER_CENTER);
                ctx.draw(&Circle { x: mx, y: my, radius: 3.8, color });
            }
        }
    }
}

/// Draw a rotated rectangle. `w` = tangential width, `h` = radial length.
/// Centered at canvas position for `clock_angle` at `center_radius`.
fn draw_rotated_rect(
    ctx: &mut Context, clock_angle: f64, center_radius: f64,
    w: f64, h: f64, color: Color,
) {
    let (cx, cy) = hand_endpoint(clock_angle, center_radius);
    let corners = [
        (-w / 2.0, -h / 2.0),
        (w / 2.0, -h / 2.0),
        (w / 2.0, h / 2.0),
        (-w / 2.0, h / 2.0),
    ];
    let pts: Vec<(f64, f64)> = corners.iter().map(|&(x, y)| {
        let (rx, ry) = rotate_for_clock(x, y, clock_angle);
        (cx + rx, cy + ry)
    }).collect();
    for i in 0..4 {
        let j = (i + 1) % 4;
        ctx.draw(&Line {
            x1: pts[i].0, y1: pts[i].1,
            x2: pts[j].0, y2: pts[j].1,
            color,
        });
    }
}

/// Draw a rectangular baton marker at the given clock angle.
fn draw_baton(ctx: &mut Context, clock_angle: f64, color: Color) {
    let w = 4.0;
    let h = (MARKER_OUTER - MARKER_INNER) + 2.0; // taller than default span
    let center_r = (MARKER_OUTER + MARKER_INNER) / 2.0;
    draw_rotated_rect(ctx, clock_angle, center_r, w, h, color);
}

// ══════════════════════════════════════════════════════════════
// CROWN — 5-pronged crown logo above RUSTLEX
// ══════════════════════════════════════════════════════════════
fn paint_crown(ctx: &mut Context, td: &ThemeData) {
    if td.lume_mode { return; }
    let scale = 0.7;
    let ox = 0.0;
    let oy = 54.0; // position above logo

    for &(x1, y1, x2, y2) in CROWN_SEGS {
        ctx.draw(&Line {
            x1: ox + x1 * scale, y1: oy + y1 * scale,
            x2: ox + x2 * scale, y2: oy + y2 * scale,
            color: td.logo_color,
        });
    }

    // Dots at prong tips
    for &(dx, dy) in CROWN_DOTS {
        ctx.draw(&Circle {
            x: ox + dx * scale, y: oy + dy * scale,
            radius: 0.6, color: td.logo_color,
        });
    }
}

// ══════════════════════════════════════════════════════════════
// LOGO — "RUSTLEX" drawn with canvas lines
// ══════════════════════════════════════════════════════════════
fn paint_logo(ctx: &mut Context, td: &ThemeData) {
    if td.lume_mode { return; }
    let letter_w = 5.0;
    let letter_h_scale = 0.8; // slightly squish height to fit
    let gap = 2.5;
    let total_w = 7.0 * letter_w + 6.0 * gap;
    let start_x = -total_w / 2.0;
    let start_y = 42.0;

    for (i, segments) in LOGO_LETTERS.iter().enumerate() {
        let ox = start_x + (i as f64) * (letter_w + gap);
        let oy = start_y;
        for &(lx1, ly1, lx2, ly2) in *segments {
            // Scale x to letter_w (letters defined as 6-wide)
            let sx = letter_w / 6.0;
            // Scale y to letter_h (letters defined as 10-tall)
            let sy = letter_h_scale;
            ctx.draw(&Line {
                x1: ox + lx1 * sx, y1: oy + ly1 * sy,
                x2: ox + lx2 * sx, y2: oy + ly2 * sy,
                color: td.logo_color,
            });
        }
    }
}

// ══════════════════════════════════════════════════════════════
// DATE WINDOW — at 3 o'clock with 7-segment digits
// ══════════════════════════════════════════════════════════════
fn paint_date_window(ctx: &mut Context, td: &ThemeData, day: u32) {
    if td.lume_mode || !td.has_date_window { return; }

    let cx = 50.0;
    let cy = 0.0;
    let hw = 10.0;
    let hh = 8.0;
    let c = td.date_color;

    // Rectangle border
    ctx.draw(&Line { x1: cx - hw, y1: cy - hh, x2: cx + hw, y2: cy - hh, color: c });
    ctx.draw(&Line { x1: cx + hw, y1: cy - hh, x2: cx + hw, y2: cy + hh, color: c });
    ctx.draw(&Line { x1: cx + hw, y1: cy + hh, x2: cx - hw, y2: cy + hh, color: c });
    ctx.draw(&Line { x1: cx - hw, y1: cy + hh, x2: cx - hw, y2: cy - hh, color: c });

    // Draw day as two 7-segment digits, centered
    let tens = day / 10;
    let ones = day % 10;
    let dw = 6.0;
    let dh = 10.0;
    let dgap = 2.0;
    let tw = dw * 2.0 + dgap;
    let dx = cx - tw / 2.0;
    let dy = cy - dh / 2.0;

    if tens > 0 {
        draw_digit(ctx, tens, dx, dy, dw, dh, c);
    }
    draw_digit(ctx, ones, dx + dw + dgap, dy, dw, dh, c);
}

/// Draw a 7-segment digit at position (ox, oy) — not rotated.
fn draw_digit(ctx: &mut Context, digit: u32, ox: f64, oy: f64, w: f64, h: f64, color: Color) {
    if digit > 9 { return; }
    let segs = DIGITS[digit as usize];
    let hh = h / 2.0;

    let lines: [(f64, f64, f64, f64); 7] = [
        (ox, oy + h, ox + w, oy + h),
        (ox, oy + h, ox, oy + hh),
        (ox + w, oy + h, ox + w, oy + hh),
        (ox, oy + hh, ox + w, oy + hh),
        (ox, oy + hh, ox, oy),
        (ox + w, oy + hh, ox + w, oy),
        (ox, oy, ox + w, oy),
    ];

    for (i, &(x1, y1, x2, y2)) in lines.iter().enumerate() {
        if segs[i] {
            ctx.draw(&Line { x1, y1, x2, y2, color });
        }
    }
}

// ══════════════════════════════════════════════════════════════
// HANDS — Mercedes hour, sword minute, lollipop second
// ══════════════════════════════════════════════════════════════
fn paint_hands(ctx: &mut Context, td: &ThemeData, clock: &ClockHands) {
    let r = MARKER_INNER;
    let lume = td.lume_mode;
    let hc = if lume { Color::LightGreen } else { td.hour_hand_color };
    let mc = if lume { Color::LightGreen } else { td.minute_hand_color };
    let sc = td.second_hand_color; // second hand has no lume, stays red

    // ── Hour hand: Mercedes style ──
    // Thick rectangular shaft + circle pip near the tip
    let hour_len = td.hour_hand_length * r;
    let hour_w = 3.5;  // wider than minute hand
    let hour_shaft_len = hour_len * 0.65;
    let hour_shaft_center = hour_shaft_len / 2.0;
    // Main shaft (rectangle from center outward, stopping before the pip)
    draw_rotated_rect(ctx, clock.hour_angle, hour_shaft_center, hour_w, hour_shaft_len, hc);
    // Mercedes circle pip near the tip
    let pip_r = hour_len * 0.82;
    let (px, py) = hand_endpoint(clock.hour_angle, pip_r);
    ctx.draw(&Circle { x: px, y: py, radius: 2.5, color: hc });
    // Thin line connecting shaft to pip and pip to tip
    let (shaft_end_x, shaft_end_y) = hand_endpoint(clock.hour_angle, hour_shaft_len);
    ctx.draw(&Line { x1: shaft_end_x, y1: shaft_end_y, x2: px, y2: py, color: hc });
    let (tip_x, tip_y) = hand_endpoint(clock.hour_angle, hour_len);
    ctx.draw(&Line { x1: px, y1: py, x2: tip_x, y2: tip_y, color: hc });
    // Short tail behind center
    draw_rotated_rect(ctx, clock.hour_angle + PI, 4.0, hour_w, 8.0, hc);

    // ── Minute hand: sword style ──
    // Narrower rectangle, longer than hour hand
    let min_len = td.minute_hand_length * r;
    let min_w = 2.2;
    let min_shaft_center = min_len / 2.0;
    draw_rotated_rect(ctx, clock.minute_angle, min_shaft_center, min_w, min_len, mc);
    // Pointed tip (tapers from shaft end to a point)
    let min_tip_len = min_len * 0.15;
    let (ms_x, ms_y) = hand_endpoint(clock.minute_angle, min_len);
    let (mt_x, mt_y) = hand_endpoint(clock.minute_angle, min_len + min_tip_len);
    ctx.draw(&Line { x1: ms_x, y1: ms_y, x2: mt_x, y2: mt_y, color: mc });
    // Short tail behind center
    draw_rotated_rect(ctx, clock.minute_angle + PI, 4.0, min_w, 8.0, mc);

    // ── Second hand: lollipop style ──
    // Thin line + circle "lollipop" near tip + counterbalance circle on tail
    let sec_len = td.second_hand_length * r;
    let (sx, sy) = hand_endpoint(clock.second_angle, sec_len);
    ctx.draw(&Line { x1: 0.0, y1: 0.0, x2: sx, y2: sy, color: sc });
    // Lollipop circle near the tip
    let lollipop_r = sec_len * 0.85;
    let (lx, ly) = hand_endpoint(clock.second_angle, lollipop_r);
    ctx.draw(&Circle { x: lx, y: ly, radius: 1.8, color: sc });
    // Counterweight tail with circle
    let tail_len = 0.20 * r;
    let (tx, ty) = hand_endpoint(clock.second_angle + PI, tail_len);
    ctx.draw(&Line { x1: 0.0, y1: 0.0, x2: tx, y2: ty, color: sc });
    let (cx, cy) = hand_endpoint(clock.second_angle + PI, tail_len * 0.7);
    ctx.draw(&Circle { x: cx, y: cy, radius: 1.2, color: sc });
}

// ══════════════════════════════════════════════════════════════
// CENTER DOT — pivot point
// ══════════════════════════════════════════════════════════════
fn paint_center_dot(ctx: &mut Context, td: &ThemeData) {
    let color = if td.lume_mode { Color::LightGreen } else { td.hour_hand_color };
    ctx.draw(&Circle { x: 0.0, y: 0.0, radius: 2.5, color });
}
