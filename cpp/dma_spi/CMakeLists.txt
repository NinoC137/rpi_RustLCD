cmake_minimum_required(VERSION 3.16)
project(dma_spi_backend CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(dma_spi_flush
    src/main.cpp
    src/spi_transport.cpp
    src/dma_mailbox.cpp
    src/dma_backend.cpp
)

target_include_directories(dma_spi_flush PRIVATE include)
