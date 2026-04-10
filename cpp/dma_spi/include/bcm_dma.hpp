#pragma once

#include <cstdint>

namespace dma_spi {

struct alignas(32) DmaControlBlock {
    uint32_t transfer_info = 0;
    uint32_t src_addr = 0;
    uint32_t dst_addr = 0;
    uint32_t transfer_len = 0;
    uint32_t stride = 0;
    uint32_t next_cb = 0;
    uint32_t reserved1 = 0;
    uint32_t reserved2 = 0;
};

struct DmaChannelRegisters {
    uint32_t cs;
    uint32_t conblk_ad;
    uint32_t ti;
    uint32_t source_ad;
    uint32_t dest_ad;
    uint32_t txfr_len;
    uint32_t stride;
    uint32_t nextconbk;
    uint32_t debug;
};

constexpr uint32_t DMA_BASE_OFFSET = 0x007000;
constexpr uint32_t DMA_CHANNEL_OFFSET = 0x100;
constexpr uint32_t DMA_CS_RESET = 1u << 31;
constexpr uint32_t DMA_CS_ABORT = 1u << 30;
constexpr uint32_t DMA_CS_DISDEBUG = 1u << 29;
constexpr uint32_t DMA_CS_WAIT_FOR_OUTSTANDING_WRITES = 1u << 28;
constexpr uint32_t DMA_CS_PANIC_PRIORITY(unsigned x) { return (x & 0xf) << 20; }
constexpr uint32_t DMA_CS_PRIORITY(unsigned x) { return (x & 0xf) << 16; }
constexpr uint32_t DMA_CS_ACTIVE = 1u << 0;

constexpr uint32_t DMA_TI_NO_WIDE_BURSTS = 1u << 26;
constexpr uint32_t DMA_TI_WAIT_RESP = 1u << 3;
constexpr uint32_t DMA_TI_DEST_DREQ = 1u << 6;
constexpr uint32_t DMA_TI_PERMAP(unsigned x) { return (x & 0x1f) << 16; }
constexpr uint32_t DMA_TI_SRC_INC = 1u << 8;

constexpr uint32_t PERI_PHYS_BASE_PI3 = 0x3F000000;
constexpr uint32_t PERI_BUS_BASE = 0x7E000000;
constexpr uint32_t SPI0_BASE_OFFSET = 0x204000;
constexpr uint32_t SPI0_FIFO_OFFSET = 0x04;
constexpr uint32_t SPI0_CS_OFFSET = 0x00;
constexpr uint32_t SPI0_DLEN_OFFSET = 0x0C;
constexpr uint32_t SPI0_DC_OFFSET = 0x14;
constexpr uint32_t DMA_DREQ_SPI_TX = 6;

} // namespace dma_spi
