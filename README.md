# rpi_lcd_project

Rust SPI LCD driver MVP for Raspberry Pi + ILI9486-class panel.

## Current known-good parameters

- spi: `/dev/spidev0.0`
- dc: `24`
- rst: `25`
- width: `480`
- height: `320`
- madctl: `0x48`
- pixel-format: `0x55`

## Example

```bash
sudo ~/.cargo/bin/cargo run --release -- --pattern xo
```

Patterns:
- `red`
- `green`
- `blue`
- `white`
- `black`
- `bars`
- `quad`
- `xo`
