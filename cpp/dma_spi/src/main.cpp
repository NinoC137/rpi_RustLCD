#include "demo_pattern.hpp"
#include "dma_backend.hpp"

#include <fstream>
#include <iostream>
#include <string>

using namespace dma_spi;

static bool write_demo_file(const std::string& path, unsigned w, unsigned h) {
    auto bytes = make_rgb565_bars_demo(w, h);
    std::ofstream out(path, std::ios::binary);
    out.write(reinterpret_cast<const char*>(bytes.data()), static_cast<std::streamsize>(bytes.size()));
    return out.good();
}

int main(int argc, char** argv) {
    FlushRequest req;
    unsigned demo_w = 320;
    unsigned demo_h = 480;
    std::string demo_out;
    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];
        if (arg == "--input" && i + 1 < argc) {
            req.input_path = argv[++i];
        } else if (arg == "--spi" && i + 1 < argc) {
            req.spi.device = argv[++i];
        } else if (arg == "--spi-hz" && i + 1 < argc) {
            req.spi.speed_hz = static_cast<uint32_t>(std::stoul(argv[++i]));
        } else if (arg == "--dma") {
            req.use_dma = true;
        } else if (arg == "--demo-bars" && i + 1 < argc) {
            demo_out = argv[++i];
        } else if (arg == "--width" && i + 1 < argc) {
            demo_w = static_cast<unsigned>(std::stoul(argv[++i]));
        } else if (arg == "--height" && i + 1 < argc) {
            demo_h = static_cast<unsigned>(std::stoul(argv[++i]));
        }
    }

    if (!demo_out.empty()) {
        if (!write_demo_file(demo_out, demo_w, demo_h)) {
            std::cerr << "error: failed to write demo file\n";
            return 3;
        }
        std::cout << "demo_written\n";
        return 0;
    }

    if (req.input_path.empty()) {
        std::cerr << "error: --input is required\n";
        return 2;
    }

    DmaSpiBackend backend;
    if (!backend.flush_file(req)) {
        std::cerr << "error: " << backend.last_error() << "\n";
        return 1;
    }

    std::cout << "ok\n";
    return 0;
}
