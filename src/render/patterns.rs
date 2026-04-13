use crate::delta::load_passwords;
use crate::framebuffer::{FrameBuffer, Rgb565};
use crate::sysinfo::read_system_status;
use crate::render::primitives::{
    draw_circle_ring, draw_line, draw_rect_outline, draw_round_rect_filled, draw_text_5x7,
    draw_vertical_gradient, fill_rect, rgb565,
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
    if h > w {
        apple_delta_dashboard_portrait(fb, w, h);
    } else {
        apple_delta_dashboard_landscape(fb, w, h);
    }
}

fn apple_delta_dashboard_landscape(fb: &mut FrameBuffer, w: i32, h: i32) {
    draw_vertical_gradient(fb, 0, 0, w, h, rgb565(7, 7, 10), rgb565(0, 0, 0));
    draw_round_rect_filled(fb, 8, 8, w - 16, h - 16, 20, rgb565(10, 10, 14));

    let margin = 20;
    let gap = 10;
    let top_h = 44;
    let mid_h = 116;
    let top_y = margin;
    let mid_y = top_y + top_h + gap;
    let bottom_y = mid_y + mid_h + gap;
    let left_w = 222;
    let right_w = w - margin * 2 - gap - left_w;

    draw_watch_card(fb, margin, top_y, 138, top_h, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + 138 + gap, top_y, 106, top_h, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + 138 + gap + 106 + gap, top_y, w - margin - (margin + 138 + gap + 106 + gap), top_h, rgb565(0, 113, 227));
    draw_watch_card(fb, margin, mid_y, left_w, mid_h, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + left_w + gap, mid_y, right_w, mid_h, rgb565(22, 22, 28));
    draw_watch_card(fb, margin, bottom_y, left_w, 108, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + left_w + gap, bottom_y, right_w, 49, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + left_w + gap, bottom_y + 59, right_w, 49, rgb565(22, 22, 28));

    draw_text_5x7(fb, 32, 33, "MON", rgb565(228, 228, 232), 2, 1);
    draw_text_5x7(fb, 172, 33, &day_progress_pct(), rgb565(228, 228, 232), 2, 1);
    draw_text_5x7(fb, 300, 33, "LIVE", Rgb565::WHITE, 2, 1);

    let (hh, mm) = current_hhmm_local();
    let sys = read_system_status();
    draw_text_5x7(fb, 36, 88, "TIME", rgb565(130, 130, 140), 2, 1);
    draw_big_digits(fb, 36, 118, &format!("{}:{}", hh, mm), rgb565(248, 248, 250), 6, 4, 8);
    draw_text_5x7(fb, 38, 168, &time_period_label(), rgb565(120, 194, 255), 2, 1);

    draw_text_5x7(fb, 266, 88, "SYSTEM", rgb565(130, 130, 140), 2, 1);
    draw_text_5x7(fb, 266, 116, &truncate_label(&sys.top_label, 10), Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, 266, 140, &format!("CPU {} MEM {}", sys.top_cpu, sys.top_mem), Rgb565::WHITE, 1, 1);
    draw_text_5x7(fb, 266, 166, &format!("SYS {} {}", sys.cpu_percent, sys.mem_percent), rgb565(0, 113, 227), 1, 1);

    draw_text_5x7(fb, 36, 222, "DELTA", rgb565(130, 130, 140), 2, 1);
    draw_text_5x7(fb, 36, 242, "PASSWORDS", Rgb565::WHITE, 2, 1);
    let passwords = load_passwords();
    let shown: Vec<_> = passwords.into_iter().take(3).collect();
    let mut row_y = 264;
    for item in shown.iter() {
        draw_password_row_480(fb, 34, row_y, left_w - 24, &item.location, &item.password);
        row_y += 16;
    }

    draw_text_5x7(fb, 266, 220, "DAY", rgb565(130, 130, 140), 1, 1);
    draw_text_5x7(fb, 266, 235, &day_progress_label(), Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, 266, 279, "WEEK", rgb565(130, 130, 140), 1, 1);
    draw_text_5x7(fb, 266, 294, &weekday_short(), Rgb565::WHITE, 2, 1);
}

fn apple_delta_dashboard_portrait(fb: &mut FrameBuffer, w: i32, h: i32) {
    draw_vertical_gradient(fb, 0, 0, w, h, rgb565(7, 7, 10), rgb565(0, 0, 0));
    draw_round_rect_filled(fb, 8, 8, w - 16, h - 16, 22, rgb565(10, 10, 14));

    let margin = 18;
    let gap = 10;
    let top_y = margin;

    draw_watch_card(fb, margin, top_y, 88, 40, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + 98, top_y, 88, 40, rgb565(22, 22, 28));
    draw_watch_card(fb, margin + 196, top_y, w - (margin + 196) - margin, 40, rgb565(0, 113, 227));

    let time_y = top_y + 50;
    draw_watch_card(fb, margin, time_y, w - margin * 2, 118, rgb565(22, 22, 28));

    let mid_y = time_y + 128;
    draw_watch_card(fb, margin, mid_y, w - margin * 2, 72, rgb565(22, 22, 28));

    let list_y = mid_y + 82;
    draw_watch_card(fb, margin, list_y, w - margin * 2, 138, rgb565(22, 22, 28));

    let stat_y = list_y + 148;
    draw_watch_card(fb, margin, stat_y, w - margin * 2, 42, rgb565(22, 22, 28));
    draw_watch_card(fb, margin, stat_y + 52, w - margin * 2, 42, rgb565(22, 22, 28));

    draw_text_5x7(fb, 28, 30, "MON", rgb565(228, 228, 232), 2, 1);
    draw_text_5x7(fb, 128, 30, &day_progress_pct(), rgb565(228, 228, 232), 2, 1);
    draw_text_5x7(fb, 238, 30, "LIVE", Rgb565::WHITE, 2, 1);

    let (hh, mm) = current_hhmm_local();
    let sys = read_system_status();
    draw_text_5x7(fb, 34, 86, "TIME", rgb565(130, 130, 140), 2, 1);
    draw_big_digits(fb, 36, 118, &format!("{}:{}", hh, mm), rgb565(248, 248, 250), 5, 4, 8);
    draw_text_5x7(fb, 34, 166, &time_period_label(), rgb565(120, 194, 255), 2, 1);

    draw_text_5x7(fb, 34, 222, "SYSTEM", rgb565(130, 130, 140), 1, 1);
    draw_text_5x7(fb, 34, 238, &truncate_label(&sys.top_label, 10), Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, 34, 258, &format!("CPU {} MEM {}", sys.top_cpu, sys.top_mem), Rgb565::WHITE, 1, 1);

    draw_text_5x7(fb, 34, 304, "DELTA PASSWORDS", Rgb565::WHITE, 2, 1);
    let passwords = load_passwords();
    let shown: Vec<_> = passwords.into_iter().take(4).collect();
    let mut row_y = 330;
    for item in shown.iter() {
        draw_password_row_480(fb, 30, row_y, w - 60, &item.location, &item.password);
        row_y += 22;
    }

    draw_text_5x7(fb, 34, (stat_y + 14) as u16, "DAY", rgb565(130, 130, 140), 1, 1);
    draw_text_5x7(fb, 110, (stat_y + 12) as u16, &day_progress_label(), Rgb565::WHITE, 2, 1);
    draw_text_5x7(fb, 34, (stat_y + 66) as u16, "WEEK", rgb565(130, 130, 140), 1, 1);
    draw_text_5x7(fb, 110, (stat_y + 64) as u16, &weekday_short(), Rgb565::WHITE, 2, 1);
}

fn draw_watch_card(fb: &mut FrameBuffer, x: i32, y: i32, w: i32, h: i32, bg: Rgb565) {
    draw_round_rect_filled(fb, x, y, w, h, 16, bg);
}

fn draw_password_row_480(
    fb: &mut FrameBuffer,
    x: i32,
    y: i32,
    w: i32,
    location: &str,
    password: &str,
) {
    draw_round_rect_filled(fb, x, y, w, 14, 6, rgb565(36, 37, 44));
    draw_text_5x7(fb, (x + 8) as u16, (y + 4) as u16, location, rgb565(224, 224, 230), 1, 1);
    let code_w = 50;
    draw_round_rect_filled(fb, x + w - code_w - 4, y + 1, code_w, 12, 5, rgb565(0, 113, 227));
    draw_text_5x7(fb, (x + w - code_w + 8) as u16, (y + 4) as u16, password, Rgb565::WHITE, 1, 1);
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

fn truncate_label(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}

