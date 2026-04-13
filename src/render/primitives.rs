use crate::framebuffer::{FrameBuffer, Rgb565};
use crate::render::font5x7::glyph5x7;

pub fn rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
    let r5 = (r as u16 >> 3) & 0x1f;
    let g6 = (g as u16 >> 2) & 0x3f;
    let b5 = (b as u16 >> 3) & 0x1f;
    Rgb565((r5 << 11) | (g6 << 5) | b5)
}

pub fn fill_rect_i32(fb: &mut FrameBuffer, x: i32, y: i32, w: i32, h: i32, color: Rgb565) {
    if w <= 0 || h <= 0 {
        return;
    }
    let x0 = x.max(0) as u16;
    let y0 = y.max(0) as u16;
    let x1 = (x + w).max(0).min(fb.width() as i32) as u16;
    let y1 = (y + h).max(0).min(fb.height() as i32) as u16;
    if x1 <= x0 || y1 <= y0 {
        return;
    }
    fill_rect(fb, x0, y0, x1 - x0, y1 - y0, color);
}

pub fn draw_round_rect_filled(
    fb: &mut FrameBuffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: i32,
    color: Rgb565,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let r = radius.max(0).min(w / 2).min(h / 2);
    if r == 0 {
        fill_rect_i32(fb, x, y, w, h, color);
        return;
    }

    fill_rect_i32(fb, x + r, y, w - 2 * r, h, color);
    fill_rect_i32(fb, x, y + r, r, h - 2 * r, color);
    fill_rect_i32(fb, x + w - r, y + r, r, h - 2 * r, color);

    let rr = r * r;
    for oy in 0..r {
        for ox in 0..r {
            let dx = r - ox;
            let dy = r - oy;
            if dx * dx + dy * dy <= rr {
                let px_l = x + ox;
                let px_r = x + w - 1 - ox;
                let py_t = y + oy;
                let py_b = y + h - 1 - oy;
                if px_l >= 0 && py_t >= 0 {
                    fb.set_pixel(px_l as u16, py_t as u16, color);
                }
                if px_r >= 0 && py_t >= 0 {
                    fb.set_pixel(px_r as u16, py_t as u16, color);
                }
                if px_l >= 0 && py_b >= 0 {
                    fb.set_pixel(px_l as u16, py_b as u16, color);
                }
                if px_r >= 0 && py_b >= 0 {
                    fb.set_pixel(px_r as u16, py_b as u16, color);
                }
            }
        }
    }
}

pub fn draw_vertical_gradient(
    fb: &mut FrameBuffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    top: Rgb565,
    bottom: Rgb565,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let (tr, tg, tb) = rgb565_to_rgb888(top);
    let (br, bg, bb) = rgb565_to_rgb888(bottom);
    for row in 0..h {
        let t = if h <= 1 { 0.0 } else { row as f32 / (h - 1) as f32 };
        let r = lerp_u8(tr, br, t);
        let g = lerp_u8(tg, bg, t);
        let b = lerp_u8(tb, bb, t);
        fill_rect_i32(fb, x, y + row, w, 1, rgb565(r, g, b));
    }
}

fn rgb565_to_rgb888(color: Rgb565) -> (u8, u8, u8) {
    let v = color.0;
    let r5 = ((v >> 11) & 0x1f) as u8;
    let g6 = ((v >> 5) & 0x3f) as u8;
    let b5 = (v & 0x1f) as u8;
    (
        (r5 << 3) | (r5 >> 2),
        (g6 << 2) | (g6 >> 4),
        (b5 << 3) | (b5 >> 2),
    )
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    ((a as f32) + ((b as f32) - (a as f32)) * t).round() as u8
}

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
