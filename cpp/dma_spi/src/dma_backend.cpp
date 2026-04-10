#include "dma_backend.hpp"
#include "spi0_regs.hpp"

#include <chrono>
#include <cstring>
#include <fstream>
#include <iterator>
#include <thread>

namespace dma_spi {

static std::vector<uint8_t> read_all_bytes(const std::string& path) {
    std::ifstream in(path, std::ios::binary);
    return std::vector<uint8_t>(std::istreambuf_iterator<char>(in), std::istreambuf_iterator<char>());
}

bool DmaSpiBackend::flush_file(const FlushRequest& req) {
    auto bytes = read_all_bytes(req.input_path);
    if (bytes.empty()) {
        last_error_ = "input buffer is empty or unreadable";
        return false;
    }
    return req.use_dma ? flush_dma_candidate(req, bytes) : flush_blocking(req, bytes);
}

bool DmaSpiBackend::flush_blocking(const FlushRequest& req, const std::vector<uint8_t>& bytes) {
    SpiTransport spi;
    if (!spi.open(req.spi)) {
        last_error_ = spi.last_error();
        return false;
    }
    if (!spi.write(bytes)) {
        last_error_ = spi.last_error();
        return false;
    }
    return true;
}

bool DmaSpiBackend::flush_dma_candidate(const FlushRequest& req, const std::vector<uint8_t>& bytes) {
    MailboxAllocator alloc;
    if (!alloc.open()) {
        last_error_ = std::string("dma path: mailbox open failed: ") + alloc.last_error();
        return false;
    }

    auto dma_buf = alloc.alloc(bytes.size());
    if (!dma_buf.virt || dma_buf.bus_addr == 0) {
        last_error_ = std::string("dma path: payload buffer alloc failed: ") + alloc.last_error();
        return false;
    }
    std::memcpy(dma_buf.virt, bytes.data(), bytes.size());

    auto cb_buf = alloc.alloc(4096, 4096);
    if (!cb_buf.virt || cb_buf.bus_addr == 0) {
        alloc.free(dma_buf);
        last_error_ = std::string("dma path: control-block alloc failed: ") + alloc.last_error();
        return false;
    }

    MmioMap dma_regs;
    if (!dma_regs.map(PERI_PHYS_BASE_PI3 + DMA_BASE_OFFSET + 5 * DMA_CHANNEL_OFFSET, 0x100)) {
        alloc.free(cb_buf);
        alloc.free(dma_buf);
        last_error_ = std::string("dma path: map dma channel regs failed: ") + dma_regs.last_error();
        return false;
    }

    MmioMap spi_regs;
    if (!spi_regs.map(PERI_PHYS_BASE_PI3 + SPI0_BASE_OFFSET, 0x100)) {
        alloc.free(cb_buf);
        alloc.free(dma_buf);
        last_error_ = std::string("dma path: map spi0 regs failed: ") + spi_regs.last_error();
        return false;
    }

    auto* chan = dma_regs.as<DmaChannelRegisters>();
    auto* spi0 = spi_regs.as<Spi0Registers>();
    auto* cb = reinterpret_cast<DmaControlBlock*>(cb_buf.virt);
    *cb = {};
    cb->transfer_info = DMA_TI_NO_WIDE_BURSTS | DMA_TI_WAIT_RESP | DMA_TI_SRC_INC | DMA_TI_DEST_DREQ |
                        DMA_TI_PERMAP(DMA_DREQ_SPI_TX);
    cb->src_addr = dma_buf.bus_addr;
    cb->dst_addr = PERI_BUS_BASE + SPI0_BASE_OFFSET + SPI0_FIFO_OFFSET;
    cb->transfer_len = static_cast<uint32_t>(bytes.size());
    cb->stride = 0;
    cb->next_cb = 0;

    spi0->cs = SPI0_CS_CLEAR_RX | SPI0_CS_CLEAR_TX;
    spi0->dlen = static_cast<uint32_t>(bytes.size());
    spi0->cs = SPI0_CS_DMAEN | SPI0_CS_TA | SPI0_CS_CLEAR_RX | SPI0_CS_CLEAR_TX | SPI0_CS_CS(0);

    chan->cs = DMA_CS_RESET;
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    chan->conblk_ad = cb_buf.bus_addr;
    chan->cs = DMA_CS_PRIORITY(8) | DMA_CS_PANIC_PRIORITY(8) | DMA_CS_DISDEBUG |
               DMA_CS_WAIT_FOR_OUTSTANDING_WRITES | DMA_CS_ACTIVE;

    auto start = std::chrono::steady_clock::now();
    while (!(spi0->cs & SPI0_CS_DONE)) {
        auto now = std::chrono::steady_clock::now();
        if (std::chrono::duration_cast<std::chrono::milliseconds>(now - start).count() > 100) {
            uint32_t spi_cs = spi0->cs;
            uint32_t dma_cs = chan->cs;
            uint32_t dma_debug = chan->debug;
            uint32_t dma_cb = chan->conblk_ad;
            spi0->cs = 0;
            chan->cs = DMA_CS_ABORT;
            alloc.free(cb_buf);
            alloc.free(dma_buf);
            last_error_ = std::string("dma path: timeout waiting for spi0 dma transfer completion") +
                          " spi_cs=0x" + [&]{ char b[16]; std::snprintf(b, sizeof(b), "%08x", spi_cs); return std::string(b); }() +
                          " dma_cs=0x" + [&]{ char b[16]; std::snprintf(b, sizeof(b), "%08x", dma_cs); return std::string(b); }() +
                          " dma_debug=0x" + [&]{ char b[16]; std::snprintf(b, sizeof(b), "%08x", dma_debug); return std::string(b); }() +
                          " conblk=0x" + [&]{ char b[16]; std::snprintf(b, sizeof(b), "%08x", dma_cb); return std::string(b); }();
            return false;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    spi0->cs = SPI0_CS_CLEAR_RX | SPI0_CS_CLEAR_TX;
    alloc.free(cb_buf);
    alloc.free(dma_buf);
    return true;
}

} // namespace dma_spi
