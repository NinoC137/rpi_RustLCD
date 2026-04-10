use std::{thread, time::Duration};

use crate::bus::spi::{LinuxSpiBus, SpiBus};
use crate::framebuffer::{DirtyRegion, FrameBuffer, PageBuffer, TransferBuffer};
use crate::gpio::{OutputPin, SysfsGpioPin};
use crate::panel::{Panel, PanelConfig};

pub struct Ili9486 {
    cfg: PanelConfig,
    spi: LinuxSpiBus,
    dc: SysfsGpioPin,
    rst: SysfsGpioPin,
}

impl Ili9486 {
    pub fn new(cfg: PanelConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let spi = LinuxSpiBus::open(&cfg.spi_path, cfg.spi_hz)?;
        let dc = SysfsGpioPin::new(cfg.dc_pin)?;
        let rst = SysfsGpioPin::new(cfg.rst_pin)?;
        Ok(Self { cfg, spi, dc, rst })
    }

    fn set_window(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cmd(
            0x2A,
            &[(x0 >> 8) as u8, x0 as u8, (x1 >> 8) as u8, x1 as u8],
        )?;
        self.cmd(
            0x2B,
            &[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8],
        )?;
        self.cmd(0x2C, &[])?;
        Ok(())
    }

    fn cmd(&mut self, cmd: u8, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.dc.set_low()?;
        self.spi.write(&[cmd])?;
        if !data.is_empty() {
            self.dc.set_high()?;
            for chunk in data.chunks(4096) {
                self.spi.write(chunk)?;
            }
        }
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.rst.set_high()?;
        thread::sleep(Duration::from_millis(50));
        self.rst.set_low()?;
        thread::sleep(Duration::from_millis(80));
        self.rst.set_high()?;
        thread::sleep(Duration::from_millis(150));
        Ok(())
    }
}

impl Panel for Ili9486 {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.reset()?;
        self.cmd(0x01, &[])?;
        thread::sleep(Duration::from_millis(120));
        self.cmd(0x11, &[])?;
        thread::sleep(Duration::from_millis(120));
        self.cmd(0x3A, &[self.cfg.pixel_format])?;
        self.cmd(0x36, &[self.cfg.madctl])?;
        self.cmd(0xC2, &[0x55])?;
        self.cmd(0xC5, &[0x00, 0x00, 0x00, 0x00])?;
        self.cmd(
            0xE0,
            &[
                0x0F, 0x1F, 0x1C, 0x0C, 0x0F, 0x08, 0x48, 0x98, 0x37, 0x0A, 0x13, 0x04, 0x11, 0x0D,
                0x00,
            ],
        )?;
        self.cmd(
            0xE1,
            &[
                0x0F, 0x32, 0x2E, 0x0B, 0x0D, 0x05, 0x47, 0x75, 0x37, 0x06, 0x10, 0x03, 0x24, 0x20,
                0x00,
            ],
        )?;
        if self.cfg.invert {
            self.cmd(0x21, &[])?;
        } else {
            self.cmd(0x20, &[])?;
        }
        self.cmd(0x29, &[])?;
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }

    fn make_transfer_buffer(&self, fb: &FrameBuffer) -> TransferBuffer {
        fb.to_transfer_buffer(self.cfg.pixel_format, self.cfg.flush_order)
    }

    fn flush_transfer_buffer(
        &mut self,
        tx: &TransferBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_window(0, 0, self.cfg.width - 1, self.cfg.height - 1)?;
        self.dc.set_high()?;
        self.spi
            .write_buffer(tx.as_slice(), self.cfg.transfer_mode)?;
        Ok(())
    }

    fn flush(&mut self, fb: &FrameBuffer) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.make_transfer_buffer(fb);
        self.flush_transfer_buffer(&tx)
    }

    fn flush_region(
        &mut self,
        region: DirtyRegion,
        page: &PageBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let x1 = region.x + region.width.saturating_sub(1);
        let y1 = region.y + region.height.saturating_sub(1);
        self.set_window(region.x, region.y, x1, y1)?;
        self.dc.set_high()?;
        let bytes = page.as_bytes_be();
        let valid_len = region.width as usize * region.height as usize * 2;
        for chunk in bytes[..valid_len.min(bytes.len())].chunks(4096) {
            self.spi.write(chunk)?;
        }
        Ok(())
    }
}
