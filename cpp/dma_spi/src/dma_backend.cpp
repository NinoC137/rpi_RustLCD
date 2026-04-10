#include "dma_backend.hpp"

#include <fstream>
#include <iterator>

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
    (void)req;
    (void)bytes;
    MailboxAllocator alloc;
    if (!alloc.open()) {
        last_error_ = alloc.last_error();
        return false;
    }
    last_error_ = "dma candidate path not implemented yet";
    return false;
}

} // namespace dma_spi
