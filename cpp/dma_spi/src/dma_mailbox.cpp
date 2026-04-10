#include "dma_mailbox.hpp"
#include "rpi_mailbox_ioctl.hpp"

#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>

#include <array>
#include <cerrno>
#include <cstring>

namespace dma_spi {

namespace {

struct MboxAllocMsg {
    uint32_t size;
    uint32_t req_resp;
    uint32_t tag;
    uint32_t buf_size;
    uint32_t data_len;
    uint32_t alloc_size;
    uint32_t align;
    uint32_t flags;
    uint32_t end_tag;
};

struct MboxHandleMsg {
    uint32_t size;
    uint32_t req_resp;
    uint32_t tag;
    uint32_t buf_size;
    uint32_t data_len;
    uint32_t handle;
    uint32_t end_tag;
};

static void* map_physical(uint32_t phys_addr, size_t size, std::string& err) {
    int mem_fd = ::open("/dev/mem", O_RDWR | O_SYNC);
    if (mem_fd < 0) {
        err = std::strerror(errno);
        return nullptr;
    }
    void* ptr = ::mmap(nullptr, size, PROT_READ | PROT_WRITE, MAP_SHARED, mem_fd, phys_addr);
    ::close(mem_fd);
    if (ptr == MAP_FAILED) {
        err = std::strerror(errno);
        return nullptr;
    }
    return ptr;
}

static bool mbox_property(int fd, void* msg) {
    return ::ioctl(fd, IOCTL_MBOX_PROPERTY, msg) >= 0;
}

} // namespace

MailboxAllocator::MailboxAllocator() = default;
MailboxAllocator::~MailboxAllocator() { close(); }

bool MailboxAllocator::open() {
    close();
    fd_ = ::open(DEVICE_FILE_NAME, 0);
    if (fd_ < 0) {
        last_error_ = std::string("open ") + DEVICE_FILE_NAME + ": " + std::strerror(errno);
        return false;
    }
    return true;
}

void MailboxAllocator::close() {
    if (fd_ >= 0) {
        ::close(fd_);
        fd_ = -1;
    }
}

DmaBuffer MailboxAllocator::alloc(size_t size, size_t align) {
    DmaBuffer buf;
    if (fd_ < 0 && !open()) {
        return buf;
    }

    MboxAllocMsg alloc_msg{};
    alloc_msg.size = sizeof(alloc_msg);
    alloc_msg.req_resp = MBOX_REQUEST;
    alloc_msg.tag = TAG_ALLOCATE_MEMORY;
    alloc_msg.buf_size = 12;
    alloc_msg.data_len = 12;
    alloc_msg.alloc_size = static_cast<uint32_t>(size);
    alloc_msg.align = static_cast<uint32_t>(align);
    alloc_msg.flags = MEM_FLAG_L1_NONALLOCATING | MEM_FLAG_ZERO;
    alloc_msg.end_tag = MBOX_TAG_LAST;

    if (!mbox_property(fd_, &alloc_msg) || alloc_msg.alloc_size == 0) {
        last_error_ = std::string("mailbox allocate-memory ioctl failed") +
                      " size=" + std::to_string(size) +
                      " align=" + std::to_string(align) +
                      " flags=" + std::to_string(MEM_FLAG_L1_NONALLOCATING | MEM_FLAG_ZERO) +
                      ": " + std::strerror(errno);
        return buf;
    }

    buf.mailbox_handle = alloc_msg.alloc_size;

    MboxHandleMsg lock_msg{};
    lock_msg.size = sizeof(lock_msg);
    lock_msg.req_resp = MBOX_REQUEST;
    lock_msg.tag = TAG_LOCK_MEMORY;
    lock_msg.buf_size = 4;
    lock_msg.data_len = 4;
    lock_msg.handle = buf.mailbox_handle;
    lock_msg.end_tag = MBOX_TAG_LAST;

    if (!mbox_property(fd_, &lock_msg) || lock_msg.handle == 0) {
        last_error_ = std::string("mailbox lock-memory ioctl failed: ") + std::strerror(errno);
        return {};
    }

    buf.bus_addr = lock_msg.handle;
    buf.size = size;
    buf.virt = map_physical(BUS_TO_PHYS(buf.bus_addr), size, last_error_);
    if (!buf.virt) {
        return {};
    }
    return buf;
}

void MailboxAllocator::free(DmaBuffer& buffer) {
    if (buffer.virt && buffer.size) {
        ::munmap(buffer.virt, buffer.size);
    }

    if (fd_ >= 0 && buffer.mailbox_handle) {
        MboxHandleMsg unlock_msg{};
        unlock_msg.size = sizeof(unlock_msg);
        unlock_msg.req_resp = MBOX_REQUEST;
        unlock_msg.tag = TAG_UNLOCK_MEMORY;
        unlock_msg.buf_size = 4;
        unlock_msg.data_len = 4;
        unlock_msg.handle = buffer.mailbox_handle;
        unlock_msg.end_tag = MBOX_TAG_LAST;
        (void)mbox_property(fd_, &unlock_msg);

        MboxHandleMsg free_msg{};
        free_msg.size = sizeof(free_msg);
        free_msg.req_resp = MBOX_REQUEST;
        free_msg.tag = TAG_RELEASE_MEMORY;
        free_msg.buf_size = 4;
        free_msg.data_len = 4;
        free_msg.handle = buffer.mailbox_handle;
        free_msg.end_tag = MBOX_TAG_LAST;
        (void)mbox_property(fd_, &free_msg);
    }

    buffer = {};
}

} // namespace dma_spi
