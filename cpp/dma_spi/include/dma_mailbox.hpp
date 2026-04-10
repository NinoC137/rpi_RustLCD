#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace dma_spi {

struct DmaBuffer {
    void* virt = nullptr;
    uint32_t bus_addr = 0;
    uint32_t mailbox_handle = 0;
    size_t size = 0;
};

class MailboxAllocator {
public:
    MailboxAllocator();
    ~MailboxAllocator();

    bool open();
    void close();

    DmaBuffer alloc(size_t size, size_t align = 4096);
    void free(DmaBuffer& buffer);

    bool is_open() const { return fd_ >= 0; }
    std::string last_error() const { return last_error_; }

private:
    int fd_ = -1;
    std::string last_error_;
};

} // namespace dma_spi
