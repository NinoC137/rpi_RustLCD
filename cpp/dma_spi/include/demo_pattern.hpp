#pragma once

#include <cstdint>
#include <vector>

namespace dma_spi {

std::vector<uint8_t> make_rgb565_bars_demo(unsigned width, unsigned height);

} // namespace dma_spi
