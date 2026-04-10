#pragma once

#include <cstdint>

#ifndef MAJOR_NUM
#define MAJOR_NUM 100
#endif

#ifndef DEVICE_FILE_NAME
#define DEVICE_FILE_NAME "/dev/vcio"
#endif

#ifndef IOCTL_MBOX_PROPERTY
#define IOCTL_MBOX_PROPERTY _IOWR(MAJOR_NUM, 0, char *)
#endif

namespace dma_spi {

constexpr uint32_t MBOX_REQUEST = 0x00000000;
constexpr uint32_t MBOX_RESPONSE = 0x80000000;
constexpr uint32_t MBOX_TAG_LAST = 0;

constexpr uint32_t TAG_ALLOCATE_MEMORY = 0x0003000c;
constexpr uint32_t TAG_LOCK_MEMORY = 0x0003000d;
constexpr uint32_t TAG_UNLOCK_MEMORY = 0x0003000e;
constexpr uint32_t TAG_RELEASE_MEMORY = 0x0003000f;

constexpr uint32_t MEM_FLAG_DISCARDABLE = 1 << 0;
constexpr uint32_t MEM_FLAG_NORMAL = 0 << 2;
constexpr uint32_t MEM_FLAG_DIRECT = 1 << 2;
constexpr uint32_t MEM_FLAG_COHERENT = 2 << 2;
constexpr uint32_t MEM_FLAG_L1_NONALLOCATING = (MEM_FLAG_DIRECT | MEM_FLAG_COHERENT);
constexpr uint32_t MEM_FLAG_ZERO = 1 << 4;
constexpr uint32_t MEM_FLAG_NO_INIT = 1 << 5;
constexpr uint32_t MEM_FLAG_HINT_PERMALOCK = 1 << 6;

constexpr uint32_t BUS_TO_PHYS(uint32_t x) { return x & ~0xC0000000u; }

} // namespace dma_spi
