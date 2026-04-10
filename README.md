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

## Note:

当旋转屏幕方向时，可能会出现“屏幕内容呈现两列显示”，这个情况主要是因为宽高逻辑与显示刷新方向不匹配导致的。

例：旋转方向设定为`0x88` ，此时需要交换宽高数值。
