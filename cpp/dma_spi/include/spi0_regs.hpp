#pragma once

#include <cstdint>

namespace dma_spi {

struct Spi0Registers {
    uint32_t cs;
    uint32_t fifo;
    uint32_t clk;
    uint32_t dlen;
    uint32_t ltoh;
    uint32_t dc;
};

constexpr uint32_t SPI0_CS_LEN_LONG = 1u << 25;
constexpr uint32_t SPI0_CS_DMA_LEN = 1u << 24;
constexpr uint32_t SPI0_CS_CSPOL2 = 1u << 23;
constexpr uint32_t SPI0_CS_CSPOL1 = 1u << 22;
constexpr uint32_t SPI0_CS_CSPOL0 = 1u << 21;
constexpr uint32_t SPI0_CS_RXF = 1u << 20;
constexpr uint32_t SPI0_CS_RXR = 1u << 19;
constexpr uint32_t SPI0_CS_TXD = 1u << 18;
constexpr uint32_t SPI0_CS_RXD = 1u << 17;
constexpr uint32_t SPI0_CS_DONE = 1u << 16;
constexpr uint32_t SPI0_CS_TE_EN = 1u << 15;
constexpr uint32_t SPI0_CS_LMONO = 1u << 14;
constexpr uint32_t SPI0_CS_LEN = 1u << 13;
constexpr uint32_t SPI0_CS_REN = 1u << 12;
constexpr uint32_t SPI0_CS_ADCS = 1u << 11;
constexpr uint32_t SPI0_CS_INTR = 1u << 10;
constexpr uint32_t SPI0_CS_INTD = 1u << 9;
constexpr uint32_t SPI0_CS_DMAEN = 1u << 8;
constexpr uint32_t SPI0_CS_TA = 1u << 7;
constexpr uint32_t SPI0_CS_CLEAR_RX = 1u << 5;
constexpr uint32_t SPI0_CS_CLEAR_TX = 1u << 4;
constexpr uint32_t SPI0_CS_CPOL = 1u << 3;
constexpr uint32_t SPI0_CS_CPHA = 1u << 2;
constexpr uint32_t SPI0_CS_CS(unsigned x) { return x & 0x3; }

} // namespace dma_spi
