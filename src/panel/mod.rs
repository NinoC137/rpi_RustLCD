pub mod ili9486;

use crate::framebuffer::{DirtyRegion, FlushOrder, FrameBuffer, PageBuffer, TransferBuffer};

#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub width: u16,
    pub height: u16,
    pub madctl: u8,
    pub pixel_format: u8,
    pub invert: bool,
    pub flush_order: FlushOrder,
    pub spi_path: String,
    pub spi_hz: u32,
    pub dc_pin: u8,
    pub rst_pin: u8,
}

pub trait Panel {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn make_transfer_buffer(&self, fb: &FrameBuffer) -> TransferBuffer;
    fn flush_transfer_buffer(
        &mut self,
        tx: &TransferBuffer,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn flush(&mut self, fb: &FrameBuffer) -> Result<(), Box<dyn std::error::Error>>;
    fn flush_region(
        &mut self,
        region: DirtyRegion,
        page: &PageBuffer,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
