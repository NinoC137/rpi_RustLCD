#pragma once

#include "bcm_dma.hpp"
#include "dma_mailbox.hpp"
#include "mmio_map.hpp"
#include "spi_transport.hpp"

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace dma_spi {

struct FlushRequest {
    SpiConfig spi;
    std::string input_path;
    bool use_dma = false;
};

class DmaSpiBackend {
public:
    bool flush_file(const FlushRequest& req);
    std::string last_error() const { return last_error_; }

private:
    bool flush_blocking(const FlushRequest& req, const std::vector<uint8_t>& bytes);
    bool flush_dma_candidate(const FlushRequest& req, const std::vector<uint8_t>& bytes);

    std::string last_error_;
};

} // namespace dma_spi
