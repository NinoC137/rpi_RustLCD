use std::io::Write;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};

pub trait SpiBus {
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;

    fn write_chunks(&mut self, chunks: &[&[u8]]) -> Result<(), Box<dyn std::error::Error>> {
        for chunk in chunks {
            self.write(chunk)?;
        }
        Ok(())
    }
}

pub struct LinuxSpiBus {
    inner: Spidev,
}

impl LinuxSpiBus {
    pub fn open(path: &str, speed_hz: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut spi = Spidev::open(path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(speed_hz)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        Ok(Self { inner: spi })
    }
}

impl SpiBus for LinuxSpiBus {
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.inner.write_all(data)?;
        Ok(())
    }
}
