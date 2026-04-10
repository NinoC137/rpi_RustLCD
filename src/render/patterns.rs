use crate::framebuffer::{FrameBuffer, Rgb565};
use crate::render::primitives::{
    draw_circle_ring, draw_line, draw_rect_outline, draw_text_5x7, fill_rect,
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
