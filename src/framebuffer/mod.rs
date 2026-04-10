#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb565(pub u16);

impl Rgb565 {
    pub const BLACK: Self = Self(0x0000);
    pub const WHITE: Self = Self(0xFFFF);
    pub const RED: Self = Self(0xF800);
    pub const GREEN: Self = Self(0x07E0);
    pub const BLUE: Self = Self(0x001F);
    pub const YELLOW: Self = Self(0xFFE0);
    pub const CYAN: Self = Self(0x07FF);
    pub const MAGENTA: Self = Self(0xF81F);
    pub fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

pub struct FrameBuffer {
    width: u16,
    height: u16,
    pixels: Vec<Rgb565>,
}

pub struct TransferBuffer {
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushOrder {
    RowMajor,
    ColumnMajor,
}

pub struct PageBuffer {
    width: u16,
    height: u16,
    pixels: Vec<Rgb565>,
}

#[derive(Debug, Clone, Copy)]
pub struct DirtyRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgb565::BLACK; width as usize * height as usize],
        }
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn clear(&mut self, color: Rgb565) {
        self.pixels.fill(color);
    }
    pub fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        if x < self.width && y < self.height {
            let idx = y as usize * self.width as usize + x as usize;
            self.pixels[idx] = color;
        }
    }
    pub fn as_bytes_be(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.pixels.len() * 2);
        for px in &self.pixels {
            out.extend_from_slice(&px.to_be_bytes());
        }
        out
    }

    pub fn as_bytes_666_from_565(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.pixels.len() * 3);
        for px in &self.pixels {
            let v = px.0;
            let r5 = ((v >> 11) & 0x1F) as u8;
            let g6 = ((v >> 5) & 0x3F) as u8;
            let b5 = (v & 0x1F) as u8;
            let r6 = (r5 << 1) | (r5 >> 4);
            let b6 = (b5 << 1) | (b5 >> 4);
            out.push(r6 << 2);
            out.push(g6 << 2);
            out.push(b6 << 2);
        }
        out
    }

    pub fn as_bytes_be_with_order(&self, order: FlushOrder) -> Vec<u8> {
        match order {
            FlushOrder::RowMajor => self.as_bytes_be(),
            FlushOrder::ColumnMajor => {
                let mut out = Vec::with_capacity(self.pixels.len() * 2);
                for x in 0..self.width {
                    for y in 0..self.height {
                        let idx = y as usize * self.width as usize + x as usize;
                        out.extend_from_slice(&self.pixels[idx].to_be_bytes());
                    }
                }
                out
            }
        }
    }

    pub fn as_bytes_666_from_565_with_order(&self, order: FlushOrder) -> Vec<u8> {
        match order {
            FlushOrder::RowMajor => self.as_bytes_666_from_565(),
            FlushOrder::ColumnMajor => {
                let mut out = Vec::with_capacity(self.pixels.len() * 3);
                for x in 0..self.width {
                    for y in 0..self.height {
                        let idx = y as usize * self.width as usize + x as usize;
                        let v = self.pixels[idx].0;
                        let r5 = ((v >> 11) & 0x1F) as u8;
                        let g6 = ((v >> 5) & 0x3F) as u8;
                        let b5 = (v & 0x1F) as u8;
                        let r6 = (r5 << 1) | (r5 >> 4);
                        let b6 = (b5 << 1) | (b5 >> 4);
                        out.push(r6 << 2);
                        out.push(g6 << 2);
                        out.push(b6 << 2);
                    }
                }
                out
            }
        }
    }

    pub fn to_transfer_buffer(&self, pixel_format: u8, order: FlushOrder) -> TransferBuffer {
        let bytes = if pixel_format == 0x66 {
            self.as_bytes_666_from_565_with_order(order)
        } else {
            self.as_bytes_be_with_order(order)
        };
        TransferBuffer { bytes }
    }

    pub fn copy_region_to_page(&self, region: DirtyRegion, page: &mut PageBuffer) {
        let rows = region
            .height
            .min(page.height)
            .min(self.height.saturating_sub(region.y));
        let cols = region
            .width
            .min(page.width)
            .min(self.width.saturating_sub(region.x));
        page.clear(Rgb565::BLACK);
        for row in 0..rows {
            for col in 0..cols {
                let src =
                    (region.y + row) as usize * self.width as usize + (region.x + col) as usize;
                let dst = row as usize * page.width as usize + col as usize;
                page.pixels[dst] = self.pixels[src];
            }
        }
    }
}

impl PageBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgb565::BLACK; width as usize * height as usize],
        }
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn clear(&mut self, color: Rgb565) {
        self.pixels.fill(color);
    }
    pub fn as_bytes_be(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.pixels.len() * 2);
        for px in &self.pixels {
            out.extend_from_slice(&px.to_be_bytes());
        }
        out
    }
}

impl TransferBuffer {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
