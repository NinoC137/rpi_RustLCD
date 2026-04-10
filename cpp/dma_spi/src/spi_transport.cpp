#include "spi_transport.hpp"

#include <fcntl.h>
#include <linux/spi/spidev.h>
#include <sys/ioctl.h>
#include <unistd.h>

#include <cerrno>
#include <cstring>

namespace dma_spi {

SpiTransport::~SpiTransport() { close(); }

bool SpiTransport::open(const SpiConfig& cfg) {
    close();
    fd_ = ::open(cfg.device.c_str(), O_RDWR);
    if (fd_ < 0) {
        last_error_ = std::strerror(errno);
        return false;
    }

    if (::ioctl(fd_, SPI_IOC_WR_MODE, &cfg.mode) < 0 ||
        ::ioctl(fd_, SPI_IOC_WR_BITS_PER_WORD, &cfg.bits_per_word) < 0 ||
        ::ioctl(fd_, SPI_IOC_WR_MAX_SPEED_HZ, &cfg.speed_hz) < 0) {
        last_error_ = std::strerror(errno);
        close();
        return false;
    }

    return true;
}

void SpiTransport::close() {
    if (fd_ >= 0) {
        ::close(fd_);
        fd_ = -1;
    }
}

bool SpiTransport::write(const uint8_t* data, size_t size) {
    if (fd_ < 0) {
        last_error_ = "spi device not open";
        return false;
    }
    if (::write(fd_, data, size) < 0) {
        last_error_ = std::strerror(errno);
        return false;
    }
    return true;
}

} // namespace dma_spi
