#include "dma_backend.hpp"

#include <iostream>
#include <string>

using namespace dma_spi;

int main(int argc, char** argv) {
    FlushRequest req;
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
        }
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
