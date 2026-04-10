use std::io::Write;

use spidev::{SpiModeFlags, Spidev, SpidevOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    Blocking,
    Chunked { chunk_size: usize },
    DmaCandidate,
}

pub trait SpiBus {
    fn write(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;

    fn write_chunks(&mut self, chunks: &[&[u8]]) -> Result<(), Box<dyn std::error::Error>> {
        for chunk in chunks {
            self.write(chunk)?;
        }
        Ok(())
    }

    fn write_buffer(
        &mut self,
        data: &[u8],
        mode: TransferMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match mode {
            TransferMode::Blocking | TransferMode::DmaCandidate => self.write(data),
            TransferMode::Chunked { chunk_size } => {
                for chunk in data.chunks(chunk_size.max(1)) {
                    self.write(chunk)?;
                }
                Ok(())
            }
        }
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
