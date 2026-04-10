#include "dma_mailbox.hpp"

namespace dma_spi {

MailboxAllocator::MailboxAllocator() = default;
MailboxAllocator::~MailboxAllocator() { close(); }

bool MailboxAllocator::open() {
    last_error_ = "mailbox allocator not implemented yet";
    return false;
}

void MailboxAllocator::close() {
    fd_ = -1;
}

DmaBuffer MailboxAllocator::alloc(size_t size, size_t) {
    DmaBuffer buf;
    buf.size = size;
    last_error_ = "dma mailbox allocation not implemented yet";
    return buf;
}

void MailboxAllocator::free(DmaBuffer& buffer) {
    buffer = {};
}

} // namespace dma_spi
