#include <cstdint>
#include <vector>

namespace dma_spi {

std::vector<uint8_t> make_rgb565_bars_demo(unsigned width, unsigned height) {
    std::vector<uint8_t> out;
    out.reserve(width * height * 2);
    const uint16_t colors[] = {0xF800, 0x07E0, 0x001F, 0xFFFF, 0xFFE0, 0x07FF, 0xF81F, 0x0000};
    unsigned band_h = height / 8;
    if (band_h == 0) band_h = 1;
    for (unsigned y = 0; y < height; ++y) {
        uint16_t c = colors[(y / band_h) % 8];
        uint8_t hi = static_cast<uint8_t>(c >> 8);
        uint8_t lo = static_cast<uint8_t>(c & 0xFF);
        for (unsigned x = 0; x < width; ++x) {
            out.push_back(hi);
            out.push_back(lo);
        }
    }
    return out;
}

} // namespace dma_spi
