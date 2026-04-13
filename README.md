# rpi_lcd_project

Rust SPI LCD driver for Raspberry Pi + ILI9486-class panel.

## Default behavior

Running without `--pattern` uses:

- `dashboard`

## Current default panel parameters

- spi: `/dev/spidev0.0`
- dc: `24`
- rst: `25`
- width: `480`
- height: `320`
- madctl: `0x88`
- pixel-format: `0x66`
- invert: `false`

## Build

```bash
cargo build --release
```

## Run

```bash
sudo ./target/release/rpi_lcd_project
```

Specify pattern explicitly:

```bash
sudo ./target/release/rpi_lcd_project --pattern dashboard
```

Live refresh:

```bash
sudo ./target/release/rpi_lcd_project --pattern dashboard --live --refresh-ms 1000
```

## Patterns

- `dashboard`
- `status`
- `debugmap`
- `xo`
- `quad`
- `bars`
- `red`
- `green`
- `blue`
- `white`
- `black`

## Parameters

- `--pattern <name>`: select render pattern
- `--spi <path>`: SPI device path
- `--spi-hz <hz>`: SPI clock frequency
- `--dc <pin>`: DC GPIO pin
- `--rst <pin>`: reset GPIO pin
- `--width <px>`: logical width before MADCTL remap
- `--height <px>`: logical height before MADCTL remap
- `--madctl <hex>`: panel MADCTL register value
- `--pixel-format <hex>`: panel pixel format register value
- `--invert`: enable panel inversion
- `--live`: continuously redraw the screen
- `--refresh-ms <ms>`: redraw interval in live mode

## Notes

- With `madctl = 0x88`, the program internally swaps logical width and height for panel rendering.
- Password data is fetched in a background cache thread with a default 100ms timeout.
