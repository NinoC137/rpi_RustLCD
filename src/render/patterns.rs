use crate::delta::load_passwords;
use crate::framebuffer::{FrameBuffer, Rgb565};
use crate::render::primitives::{
    draw_circle_ring, draw_line, draw_rect_outline, draw_round_rect_filled, draw_text_5x7,
    draw_vertical_gradient, fill_rect, fill_rect_i32, rgb565,
};

pub fn debug_map(fb: &mut FrameBuffer) {
    let w = fb.width();
    let h = fb.height();
    let w2 = w / 2;
    let h2 = h / 2;

    // Quadrants: easy to see left/right mirror or top/bottom folding.
    fill_rect(fb, 0, 0, w2, h2, Rgb565::RED);
    fill_rect(fb, w2, 0, w - w2, h2, Rgb565::GREEN);
    fill_rect(fb, 0, h2, w2, h - h2, Rgb565::BLUE);
    fill_rect(fb, w2, h2, w - w2, h - h2, Rgb565::YELLOW);

    // Thin center axes: easy to see if one axis is duplicated or mirrored.
    let vline_x = w / 2;
    let hline_y = h / 2;
    fill_rect(fb, vline_x.saturating_sub(1), 0, 3, h, Rgb565::WHITE);
    fill_rect(fb, 0, hline_y.saturating_sub(1), w, 3, Rgb565::WHITE);

    // Corner markers: each corner has a distinct color block.
    let m = 24u16;
    fill_rect(fb, 0, 0, m, m, Rgb565::WHITE);
    fill_rect(fb, w.saturating_sub(m), 0, m, m, Rgb565::CYAN);
    fill_rect(fb, 0, h.saturating_sub(m), m, m, Rgb565::MAGENTA);
    fill_rect(
        fb,
        w.saturating_sub(m),
        h.saturating_sub(m),
        m,
        m,
        Rgb565::BLACK,
    );

    // Border: helps reveal clipping and wrap-around.
    draw_rect_outline(fb, 0, 0, w, h, Rgb565::WHITE, 2);

    // A few labels for orientation. Even if text is distorted, rough placement still helps.
    draw_text_5x7(fb, 12, 30, "TL", Rgb565::BLACK, 3, 2);
    draw_text_5x7(fb, w.saturating_sub(48), 30, "TR", Rgb565::BLACK, 3, 2);
    draw_text_5x7(fb, 12, h.saturating_sub(50), "BL", Rgb565::WHITE, 3, 2);
    draw_text_5x7(
        fb,
        w.saturating_sub(48),
        h.saturating_sub(50),
        "BR",
        Rgb565::BLACK,
        3,
        2,
    );
    draw_text_5x7(
        fb,
        w2.saturating_sub(45),
        h2.saturating_sub(10),
        "C",
        Rgb565::BLACK,
        3,
        2,
    );
}

pub fn color_bars(fb: &mut FrameBuffer) {
    let colors = [
        Rgb565::RED,
        Rgb565::GREEN,
        Rgb565::BLUE,
        Rgb565::WHITE,
        Rgb565::YELLOW,
        Rgb565::CYAN,
        Rgb565::MAGENTA,
        Rgb565::BLACK,
    ];
    let band_h = std::cmp::max(1, fb.height() as usize / colors.len());
    for (i, color) in colors.iter().enumerate() {
        let y0 = i * band_h;
        let y1 = std::cmp::min((i + 1) * band_h, fb.height() as usize);
        for y in y0..y1 {
            for x in 0..fb.width() as usize {
                fb.set_pixel(x as u16, y as u16, *color);
            }
        }
    }
}

pub fn quad(fb: &mut FrameBuffer) {
    let w2 = fb.width() / 2;
    let h2 = fb.height() / 2;
    fill_rect(fb, 0, 0, w2, h2, Rgb565::RED);
    fill_rect(fb, w2, 0, fb.width() - w2, h2, Rgb565::GREEN);
    fill_rect(fb, 0, h2, w2, fb.height() - h2, Rgb565::BLUE);
    fill_rect(
        fb,
        w2,
        h2,
        fb.width() - w2,
        fb.height() - h2,
        Rgb565::YELLOW,
    );
}

pub fn xo_center_demo(fb: &mut FrameBuffer) {
    quad(fb);
    let cx = (fb.width() / 2) as i32;
    let cy = (fb.height() / 2) as i32;
    let radius = (fb.height().min(fb.width()) as i32) / 4;

    draw_circle_ring(fb, cx, cy, radius, 10, Rgb565::WHITE);

    let x0 = cx - radius + 12;
    let y0 = cy - radius + 12;
    let x1 = cx + radius - 12;
    let y1 = cy + radius - 12;
    draw_line(fb, x0, y0, x1, y1, Rgb565::BLACK, 12);
    draw_line(fb, x0, y1, x1, y0, Rgb565::BLACK, 12);
}

pub fn status_page_demo(fb: &mut FrameBuffer) {
    fill_rect(fb, 0, 0, fb.width(), fb.height(), Rgb565::BLACK);
    draw_rect_outline(
        fb,
        8,
        8,
        fb.width() - 16,
        fb.height() - 16,
        Rgb565::WHITE,
        2,
    );
    draw_text_5x7(fb, 20, 20, "RPI LCD", Rgb565::WHITE, 3, 2);
    draw_text_5x7(fb, 20, 60, "RUST DRIVER", Rgb565::CYAN, 2, 2);
    draw_text_5x7(fb, 20, 90, "SPI OK", Rgb565::GREEN, 2, 2);
    draw_text_5x7(fb, 20, 115, "MADCTL 48", Rgb565::YELLOW, 2, 2);
    draw_text_5x7(fb, 20, 140, "RGB565", Rgb565::MAGENTA, 2, 2);
}

pub fn apple_delta_dashboard_demo(fb: &mut FrameBuffer) {
    let w = fb.width() as i32;
    let h = fb.height() as i32;

    let bg_top = rgb565(10, 10, 14);
    let bg_bottom = rgb565(0, 0, 0);
    draw_vertical_gradient(fb, 0, 0, w, h, bg_top, bg_bottom);

    draw_round_rect_filled(fb, 8, 8, w - 16, h - 16, 18, rgb565(14, 14, 18));
    draw_round_rect_filled(fb, 12, 12, w - 24, h - 24, 16, rgb565(22, 22, 28));

    let left_x = 24;
    let left_y = 24;
    let left_w = 260;
    let left_h = h - 48;
    let right_x = left_x + left_w + 12;
    let right_y = left_y;
    let right_w = w - right_x - 24;
    let top_h = 126;
    let bottom_y = right_y + top_h + 12;
    let bottom_h = left_h - top_h - 12;

    draw_round_rect_filled(fb, left_x, left_y, left_w, left_h, 18, rgb565(28, 28, 34));
    draw_round_rect_filled(fb, right_x, right_y, right_w, top_h, 18, rgb565(28, 28, 34));
    draw_round_rect_filled(fb, right_x, bottom_y, right_w, bottom_h, 18, rgb565(28, 28, 34));

    fill_rect_i32(fb, 28, 28, w - 56, 1, rgb565(80, 80, 92));

    draw_text_5x7(fb, 36, 38, "NINO LCD", rgb565(172, 172, 182), 2, 1);
    draw_text_5x7(fb, 36, 58, "APPLE STYLE", rgb565(172, 172, 182), 2, 1);
    draw_text_5x7(fb, 36, 88, "TIME NOW", Rgb565::WHITE, 3, 2);

    let (hh, mm) = current_hhmm_local();
    draw_big_digits(fb, 38, 126, &format!("{}:{}", hh, mm), rgb565(245, 245, 247), 7, 6, 12);

    draw_text_5x7(fb, 40, 222, &time_period_label(), rgb565(120, 194, 255), 2, 1);
    draw_text_5x7(fb, 40, 244, &day_progress_label(), rgb565(186, 186, 196), 2, 1);
    draw_chip(fb, 38, 268, 86, 24, rgb565(0, 113, 227), "LIVE", Rgb565::WHITE);
    draw_chip(fb, 132, 268, 142, 24, rgb565(38, 38, 46), "DELTA PASS", rgb565(220, 220, 228));

    draw_text_5x7(fb, (right_x + 14) as u16, (right_y + 16) as u16, "DELTA DAILY", Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, (right_x + 14) as u16, (right_y + 34) as u16, "PASSWORDS", Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, (right_x + 14) as u16, (right_y + 56) as u16, "ONLINE OR FALLBACK", rgb565(150, 150, 160), 1, 1);

    let passwords = load_passwords();
    let mut row_y = right_y + 72;
    for item in passwords.iter().take(2) {
        draw_round_rect_filled(fb, right_x + 10, row_y, right_w - 20, 20, 8, rgb565(36, 37, 44));
        draw_text_5x7(
            fb,
            (right_x + 18) as u16,
            (row_y + 6) as u16,
            &item.location,
            rgb565(214, 214, 220),
            2,
            1,
        );
        draw_small_code_box(fb, right_x + right_w - 72, row_y + 3, &item.password);
        row_y += 24;
    }

    draw_text_5x7(fb, (right_x + 14) as u16, (bottom_y + 16) as u16, "FUN BLOCKS", Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, (right_x + 14) as u16, (bottom_y + 34) as u16, "QUIET BUT USEFUL", rgb565(150, 150, 160), 1, 1);

    let mini_gap = 8;
    let mini_w = (right_w - 10 - 10 - mini_gap) / 2;
    let mini_h = 52;
    let mini1_x = right_x + 10;
    let mini2_x = mini1_x + mini_w + mini_gap;
    let mini1_y = bottom_y + 52;
    let mini2_y = mini1_y + mini_h + mini_gap;

    draw_mini_card(fb, mini1_x, mini1_y, mini_w, mini_h, "DAY", &day_progress_pct());
    draw_mini_card(fb, mini2_x, mini1_y, mini_w, mini_h, "WEEK", &weekday_short());
    draw_mini_card(fb, mini1_x, mini2_y, mini_w, mini_h, "LUCKY", &lucky_number());
    draw_mini_card(fb, mini2_x, mini2_y, mini_w, mini_h, "MODE", &mode_label());
}

fn draw_chip(fb: &mut FrameBuffer, x: i32, y: i32, w: i32, h: i32, bg: Rgb565, text: &str, fg: Rgb565) {
    draw_round_rect_filled(fb, x, y, w, h, h / 2, bg);
    draw_text_5x7(fb, (x + 12) as u16, (y + 8) as u16, text, fg, 1, 1);
}

fn draw_mini_card(fb: &mut FrameBuffer, x: i32, y: i32, w: i32, h: i32, title: &str, value: &str) {
    draw_round_rect_filled(fb, x, y, w, h, 10, rgb565(36, 37, 44));
    draw_text_5x7(fb, (x + 10) as u16, (y + 9) as u16, title, rgb565(150, 150, 160), 1, 1);
    draw_text_5x7(fb, (x + 10) as u16, (y + 26) as u16, value, Rgb565::WHITE, 2, 1);
}

fn draw_small_code_box(fb: &mut FrameBuffer, x: i32, y: i32, code: &str) {
    draw_round_rect_filled(fb, x, y, 54, 14, 6, rgb565(0, 113, 227));
    draw_text_5x7(fb, (x + 8) as u16, (y + 4) as u16, code, Rgb565::WHITE, 1, 1);
}

fn draw_big_digits(
    fb: &mut FrameBuffer,
    x: i32,
    y: i32,
    text: &str,
    color: Rgb565,
    scale: i32,
    spacing: i32,
    colon_gap: i32,
) {
    let mut cursor = x;
    for ch in text.chars() {
        match ch {
            '0'..='9' => {
                draw_seven_segment_digit(fb, cursor, y, ch, color, scale);
                cursor += digit_width(scale) + spacing;
            }
            ':' => {
                draw_colon(fb, cursor, y + scale * 2, color, scale);
                cursor += colon_gap;
            }
            _ => {}
        }
    }
}

fn digit_width(scale: i32) -> i32 {
    scale * 8
}

fn digit_height(scale: i32) -> i32 {
    scale * 14
}

fn draw_colon(fb: &mut FrameBuffer, x: i32, y: i32, color: Rgb565, scale: i32) {
    let s = scale.max(1);
    let size = s + 1;
    draw_round_rect_filled(fb, x, y + s * 2, size, size, size / 2, color);
    draw_round_rect_filled(fb, x, y + s * 7, size, size, size / 2, color);
}

fn draw_seven_segment_digit(fb: &mut FrameBuffer, x: i32, y: i32, ch: char, color: Rgb565, scale: i32) {
    let s = scale.max(1);
    let t = (s / 2).max(2);
    let w = digit_width(s);
    let h = digit_height(s);

    let top = (x + t, y, w - 2 * t, t);
    let mid = (x + t, y + h / 2 - t / 2, w - 2 * t, t);
    let bot = (x + t, y + h - t, w - 2 * t, t);
    let lt = (x, y + t, t, h / 2 - t);
    let rt = (x + w - t, y + t, t, h / 2 - t);
    let lb = (x, y + h / 2, t, h / 2 - t);
    let rb = (x + w - t, y + h / 2, t, h / 2 - t);

    let segs = match ch {
        '0' => [true, false, true, true, true, true, true],
        '1' => [false, false, false, false, true, false, true],
        '2' => [true, true, true, false, true, true, false],
        '3' => [true, true, true, false, true, false, true],
        '4' => [false, true, false, true, true, false, true],
        '5' => [true, true, true, true, false, false, true],
        '6' => [true, true, true, true, false, true, true],
        '7' => [true, false, false, false, true, false, true],
        '8' => [true, true, true, true, true, true, true],
        '9' => [true, true, true, true, true, false, true],
        _ => [false; 7],
    };

    let segments = [top, mid, bot, lt, rt, lb, rb];
    for (enabled, (sx, sy, sw, sh)) in segs.into_iter().zip(segments.into_iter()) {
        if enabled {
            draw_round_rect_filled(fb, sx, sy, sw, sh, t / 2, color);
        }
    }
}

fn current_hhmm_local() -> (String, String) {
    use std::process::Command;
    if let Ok(output) = Command::new("date").args(["+%H %M"]).output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let parts: Vec<_> = s.split_whitespace().collect();
                if parts.len() >= 2 {
                    return (parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    }
    ("10".to_string(), "47".to_string())
}

fn current_hour_local() -> u32 {
    use std::process::Command;
    if let Ok(output) = Command::new("date").args(["+%H"]).output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(v) = s.trim().parse::<u32>() {
                    return v;
                }
            }
        }
    }
    10
}

fn current_weekday_local() -> u32 {
    use std::process::Command;
    if let Ok(output) = Command::new("date").args(["+%u"]).output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(v) = s.trim().parse::<u32>() {
                    return v;
                }
            }
        }
    }
    1
}

fn current_day_of_month() -> u32 {
    use std::process::Command;
    if let Ok(output) = Command::new("date").args(["+%d"]).output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                if let Ok(v) = s.trim().parse::<u32>() {
                    return v;
                }
            }
        }
    }
    13
}

fn current_hms_local() -> (u32, u32, u32) {
    use std::process::Command;
    if let Ok(output) = Command::new("date").args(["+%H %M %S"]).output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let parts: Vec<_> = s.split_whitespace().collect();
                if parts.len() >= 3 {
                    let h = parts[0].parse().unwrap_or(10);
                    let m = parts[1].parse().unwrap_or(47);
                    let sec = parts[2].parse().unwrap_or(0);
                    return (h, m, sec);
                }
            }
        }
    }
    (10, 47, 0)
}

fn time_period_label() -> String {
    match current_hour_local() {
        0..=5 => "LATE NIGHT".to_string(),
        6..=10 => "GOOD MORNING".to_string(),
        11..=13 => "NOON MODE".to_string(),
        14..=17 => "AFTERNOON".to_string(),
        _ => "EVENING".to_string(),
    }
}

fn day_progress_pct() -> String {
    let (h, m, s) = current_hms_local();
    let total = h * 3600 + m * 60 + s;
    let pct = ((total as f32 / 86400.0) * 100.0).round() as u32;
    format!("{}%", pct.min(100))
}

fn day_progress_label() -> String {
    format!("DAY {}", day_progress_pct())
}

fn weekday_short() -> String {
    match current_weekday_local() {
        1 => "MON".to_string(),
        2 => "TUE".to_string(),
        3 => "WED".to_string(),
        4 => "THU".to_string(),
        5 => "FRI".to_string(),
        6 => "SAT".to_string(),
        7 => "SUN".to_string(),
        _ => "MON".to_string(),
    }
}

fn lucky_number() -> String {
    let d = current_day_of_month();
    format!("{:02}", (d * 17 + 8) % 100)
}

fn mode_label() -> String {
    match current_hour_local() {
        0..=5 => "SLEEP".to_string(),
        6..=10 => "FOCUS".to_string(),
        11..=13 => "RESET".to_string(),
        14..=17 => "BUILD".to_string(),
        _ => "UNWIND".to_string(),
    }
}
