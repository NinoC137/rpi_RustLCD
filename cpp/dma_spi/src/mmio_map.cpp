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

    long page_size = ::sysconf(_SC_PAGESIZE);
    if (page_size <= 0) {
        ::close(fd);
        last_error_ = "invalid page size";
        return false;
    }

    uint32_t aligned_phys = phys_addr & ~static_cast<uint32_t>(page_size - 1);
    size_t offset = phys_addr - aligned_phys;
    mapped_size_ = size + offset;

    map_base_ = ::mmap(nullptr, mapped_size_, PROT_READ | PROT_WRITE, MAP_SHARED, fd, aligned_phys);
    ::close(fd);
    if (map_base_ == MAP_FAILED) {
        map_base_ = nullptr;
        last_error_ = std::strerror(errno);
        return false;
    }

    virt_ = static_cast<uint8_t*>(map_base_) + offset;
    size_ = size;
    return true;
}

void MmioMap::unmap() {
    if (map_base_) {
        ::munmap(map_base_, mapped_size_);
        map_base_ = nullptr;
        virt_ = nullptr;
        size_ = 0;
        mapped_size_ = 0;
    }
}

} // namespace dma_spi
