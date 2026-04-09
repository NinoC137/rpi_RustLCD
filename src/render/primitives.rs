use crate::framebuffer::{FrameBuffer, Rgb565};
use crate::render::font5x7::glyph5x7;

pub fn fill_rect(fb: &mut FrameBuffer, x: u16, y: u16, w: u16, h: u16, color: Rgb565) {
    let x_end = x.saturating_add(w).min(fb.width());
    let y_end = y.saturating_add(h).min(fb.height());
    for yy in y..y_end {
        for xx in x..x_end {
            fb.set_pixel(xx, yy, color);
        }
    }
}

pub fn draw_rect_outline(fb: &mut FrameBuffer, x: u16, y: u16, w: u16, h: u16, color: Rgb565, thickness: u16) {
    fill_rect(fb, x, y, w, thickness, color);
    fill_rect(fb, x, y.saturating_add(h.saturating_sub(thickness)), w, thickness, color);
    fill_rect(fb, x, y, thickness, h, color);
    fill_rect(fb, x.saturating_add(w.saturating_sub(thickness)), y, thickness, h, color);
}

pub fn draw_line(fb: &mut FrameBuffer, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Rgb565, thickness: u16) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let t = (thickness as i32).max(1);

    loop {
        for oy in -(t / 2)..=(t / 2) {
            for ox in -(t / 2)..=(t / 2) {
                let xx = x0 + ox;
                let yy = y0 + oy;
                if xx >= 0 && yy >= 0 {
                    fb.set_pixel(xx as u16, yy as u16, color);
                }
            }
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            if x0 == x1 { break; }
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 { break; }
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_circle_ring(fb: &mut FrameBuffer, cx: i32, cy: i32, radius: i32, thickness: i32, color: Rgb565) {
    let r_out2 = radius * radius;
    let r_in = (radius - thickness).max(0);
    let r_in2 = r_in * r_in;
    for y in (cy - radius - 1)..=(cy + radius + 1) {
        for x in (cx - radius - 1)..=(cx + radius + 1) {
            let dx = x - cx;
            let dy = y - cy;
            let d2 = dx * dx + dy * dy;
            if d2 <= r_out2 && d2 >= r_in2 && x >= 0 && y >= 0 {
                fb.set_pixel(x as u16, y as u16, color);
            }
        }
    }
}

pub fn draw_char_5x7(fb: &mut FrameBuffer, x: u16, y: u16, ch: char, fg: Rgb565, scale: u16) {
    if let Some(rows) = glyph5x7(ch.to_ascii_uppercase()) {
        let s = scale.max(1);
        for (row, bits) in rows.iter().enumerate() {
            for col in 0..5u16 {
                let mask = 1 << (4 - col);
                if (*bits & mask) != 0 {
                    fill_rect(fb, x + col * s, y + row as u16 * s, s, s, fg);
                }
            }
        }
    }
}

pub fn draw_text_5x7(fb: &mut FrameBuffer, mut x: u16, y: u16, text: &str, fg: Rgb565, scale: u16, spacing: u16) {
    let step = 5 * scale.max(1) + spacing;
    for ch in text.chars() {
        draw_char_5x7(fb, x, y, ch, fg, scale);
        x = x.saturating_add(step);
    }
}
