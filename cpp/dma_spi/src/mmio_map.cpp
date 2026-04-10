#include "mmio_map.hpp"

#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>

#include <cerrno>
#include <cstring>

namespace dma_spi {

MmioMap::~MmioMap() { unmap(); }

bool MmioMap::map(uint32_t phys_addr, size_t size) {
    unmap();
    int fd = ::open("/dev/mem", O_RDWR | O_SYNC);
    if (fd < 0) {
        last_error_ = std::strerror(errno);
        return false;
    }
    virt_ = ::mmap(nullptr, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, phys_addr);
    ::close(fd);
    if (virt_ == MAP_FAILED) {
        virt_ = nullptr;
        last_error_ = std::strerror(errno);
        return false;
    }
    size_ = size;
    return true;
}

void MmioMap::unmap() {
    if (virt_) {
        ::munmap(virt_, size_);
        virt_ = nullptr;
        size_ = 0;
    }
}

} // namespace dma_spi
