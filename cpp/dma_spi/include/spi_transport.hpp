#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace dma_spi {

struct SpiConfig {
    std::string device = "/dev/spidev0.0";
    uint32_t speed_hz = 24000000;
    uint8_t mode = 0;
    uint8_t bits_per_word = 8;
};

class SpiTransport {
public:
    SpiTransport() = default;
    ~SpiTransport();

    bool open(const SpiConfig& cfg);
    void close();

    bool write(const uint8_t* data, size_t size);
    bool write(const std::vector<uint8_t>& data) { return write(data.data(), data.size()); }

    std::string last_error() const { return last_error_; }
    bool is_open() const { return fd_ >= 0; }

private:
    int fd_ = -1;
    std::string last_error_;
};

} // namespace dma_spi
