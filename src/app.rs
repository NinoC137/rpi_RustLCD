use std::env;

use crate::framebuffer::{DirtyRegion, FlushOrder, FrameBuffer, PageBuffer, Rgb565};
use crate::panel::{ili9486::Ili9486, Panel, PanelConfig};
use crate::render::patterns;

#[derive(Debug, Clone, Copy)]
enum Pattern {
    Red,
    Green,
    Blue,
    White,
    Black,
    Bars,
    Quad,
    Xo,
    Status,
    DebugMap,
}

impl Pattern {
    fn parse(s: &str) -> Self {
        match s {
            "green" => Self::Green,
            "blue" => Self::Blue,
            "white" => Self::White,
            "black" => Self::Black,
            "bars" => Self::Bars,
            "quad" => Self::Quad,
            "xo" => Self::Xo,
            "status" => Self::Status,
            "debugmap" => Self::DebugMap,
            _ => Self::Red,
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut spi = "/dev/spidev0.0".to_string();
    let mut spi_hz: u32 = 24_000_000;
    let mut dc: u8 = 24;
    let mut rst: u8 = 25;
    let mut width: u16 = 480;
    let mut height: u16 = 320;
    let mut madctl: u8 = 0x88;
    let mut pixel_format: u8 = 0x66;
    let mut invert = true;
    let mut pattern = Pattern::Red;
    let mut use_page_flush = false;
    let mut page_height: u16 = 40;
    let flush_order = FlushOrder::RowMajor;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--spi" => {
                i += 1;
                spi = args[i].clone();
            }
            "--spi-hz" => {
                i += 1;
                spi_hz = args[i].parse()?;
            }
            "--dc" => {
                i += 1;
                dc = args[i].parse()?;
            }
            "--rst" => {
                i += 1;
                rst = args[i].parse()?;
            }
            "--width" => {
                i += 1;
                width = args[i].parse()?;
            }
            "--height" => {
                i += 1;
                height = args[i].parse()?;
            }
            "--madctl" => {
                i += 1;
                madctl = u8::from_str_radix(args[i].trim_start_matches("0x"), 16)?;
            }
            "--pixel-format" => {
                i += 1;
                pixel_format = u8::from_str_radix(args[i].trim_start_matches("0x"), 16)?;
            }
            "--invert" => {
                invert = true;
            }
            "--pattern" => {
                i += 1;
                pattern = Pattern::parse(&args[i]);
            }
            "--page-flush" => {
                use_page_flush = true;
            }
            "--page-height" => {
                i += 1;
                page_height = args[i].parse()?;
            }
            _ => {}
        }
        i += 1;
    }

    let (panel_width, panel_height) = if madctl == 0x88 {
        (height, width)
    } else {
        (width, height)
    };

    let mut fb = FrameBuffer::new(panel_width, panel_height);
    match pattern {
        Pattern::Red => fb.clear(Rgb565::RED),
        Pattern::Green => fb.clear(Rgb565::GREEN),
        Pattern::Blue => fb.clear(Rgb565::BLUE),
        Pattern::White => fb.clear(Rgb565::WHITE),
        Pattern::Black => fb.clear(Rgb565::BLACK),
        Pattern::Bars => patterns::color_bars(&mut fb),
        Pattern::Quad => patterns::quad(&mut fb),
        Pattern::Xo => patterns::xo_center_demo(&mut fb),
        Pattern::Status => patterns::status_page_demo(&mut fb),
        Pattern::DebugMap => patterns::debug_map(&mut fb),
    }

    let cfg = PanelConfig {
        width: panel_width,
        height: panel_height,
        madctl,
        pixel_format,
        invert,
        flush_order,
        spi_path: spi,
        spi_hz,
        dc_pin: dc,
        rst_pin: rst,
    };
    let mut panel = Ili9486::new(cfg)?;
    panel.init()?;

    if use_page_flush {
        let mut page = PageBuffer::new(panel_width, page_height);
        let mut y = 0u16;
        while y < panel_height {
            let h = page_height.min(panel_height - y);
            let region = DirtyRegion {
                x: 0,
                y,
                width: panel_width,
                height: h,
            };
            fb.copy_region_to_page(region, &mut page);
            panel.flush_region(region, &page)?;
            y = y.saturating_add(page_height);
        }
        println!("ok: region page flush complete");
    } else {
        panel.flush(&fb)?;
        println!("ok: full frame flush complete");
    }

    Ok(())
}
