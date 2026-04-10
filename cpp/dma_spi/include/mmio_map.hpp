#pragma once

#include <cstddef>
#include <cstdint>
#include <string>

namespace dma_spi {

class MmioMap {
public:
    MmioMap() = default;
    ~MmioMap();

    bool map(uint32_t phys_addr, size_t size);
    void unmap();

    template <typename T>
    volatile T* as() { return reinterpret_cast<volatile T*>(virt_); }

    volatile uint8_t* bytes() { return reinterpret_cast<volatile uint8_t*>(virt_); }
    bool is_mapped() const { return virt_ != nullptr; }
    std::string last_error() const { return last_error_; }

private:
    void* virt_ = nullptr;
    size_t size_ = 0;
    std::string last_error_;
};

} // namespace dma_spi
