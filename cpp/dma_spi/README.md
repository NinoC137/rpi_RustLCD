# dma_spi backend (WIP)

This directory hosts a C++ backend prototype for Raspberry Pi SPI LCD transfer with a future DMA path.

Current goal:
- provide a standalone, testable C++ transport layer
- prepare mailbox / coherent memory / MMIO abstractions
- define a stable command-line interface that Rust can call later

Planned executable:
- `dma_spi_flush`

Planned usage:
- Rust prepares a contiguous transfer buffer
- Rust invokes the C++ backend with SPI config + framebuffer path/shared-memory handle
- C++ backend performs transport through either:
  - blocking spidev fallback
  - future DMA path using mailbox-allocated memory and DMA control blocks

Status:
- interface skeleton only
- not yet wired to a real DMA submission path
