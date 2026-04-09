# ILI9486 init review for `rpi-lcd-project`

## Saved datasheet

- `docs/ILI9486_Datasheet.pdf`

Primary downloaded source used for this review:
- Waveshare mirror: `https://files.waveshare.com/upload/7/78/ILI9486_Datasheet.pdf`

Secondary code references used for cross-checking typical init sequences:
- Linux fbtft `fb_ili9486.c`
- Bodmer `TFT_eSPI/TFT_Drivers/ILI9486_Init.h`
- `fbcp-ili9341/ili9486.cpp`

## Current project init sequence

From `src/panel/ili9486.rs`:

1. reset()
2. `0x01` Software Reset
3. delay 120ms
4. `0x11` Sleep Out
5. delay 120ms
6. `0x3A` Interface Pixel Format = `cfg.pixel_format`
7. `0x36` MADCTL = `cfg.madctl`
8. `0xC2` Power Control 3 = `0x55`
9. `0xC5` VCOM Control = `00 00 00 00`
10. `0xE0` Positive Gamma
11. `0xE1` Negative Gamma
12. `0x20` or `0x21` inversion off/on
13. `0x29` Display ON
14. delay 50ms

## What seems OK

These items match common minimal sequences seen in Linux/TFT_eSPI examples:

- `0x01` + delay
- `0x11` + delay
- `0x3A` pixel format
- `0xC2`
- `0xC5`
- `0xE0` / `0xE1`
- `0x20` or `0x21`
- `0x29`

In particular, Linux fbtft for a PiScreen-compatible ILI9486 uses a very similar minimal sequence:
- `0xB0 0x00`
- `0x11`
- `0x3A 0x55`
- `0xC2 0x44`
- `0xC5 00 00 00 00`
- `0xE0 ...`
- `0xE1 ...`
- `0xE2 ...`
- `0x11`
- `0x29`

So the current driver is not obviously broken, but it is minimalist.

## Likely omissions / weak spots

### 1. Missing `0xB0` Interface Mode Control

This is the most notable omission.

Typical Linux init sends:
- `0xB0, 0x00`

`fbcp-ili9341` comments this register as Interface Mode Control and documents the bits as related to:
- DE polarity
- PCLK polarity
- HSYNC polarity
- VSYNC polarity

For pure SPI/DBI write mode, this register may not always be critical, but for some modules it influences internal interface behavior or default state after reset.

**Assessment:** not guaranteed fatal, but worth adding because it is common in known-good Raspberry Pi ILI9486 init sequences.

### 2. Missing `0xE2` Digital Gamma Control 1

Linux fbtft sends:
- `0xE2` with the same 15-byte table as `0xE1`

Current project sends only:
- `0xE0`
- `0xE1`

This usually affects image quality more than geometry, but if the panel powers up in an odd default gamma state, behavior can be inconsistent across modules.

**Assessment:** low-to-medium importance. Not the first suspect for column split artifacts, but a real omission versus known-good Linux init.

### 3. Missing `0xC0` / `0xC1` power control programming

Current project programs:
- `0xC2`
- `0xC5`

But some common init sequences also program:
- `0xC0` Power Control 1
- `0xC1` Power Control 2

Examples:
- TFT_eSPI uses `0xC0 0x0E 0x0E`, `0xC1 0x41 0x00`
- fbcp-ili9341 uses `0xC0 0x09 0x09`, `0xC1 0x41 0x00`

These are mostly analog/power/drive related, so they are more likely to influence stability and contrast than exact x-addressing. Still, on marginal modules they can matter.

**Assessment:** medium importance for robustness, not strongest explanation for a stable “one column looks like two columns” fault.

### 4. Missing `0xB6` Display Function Control

`fbcp-ili9341` configures:
- `0xB6, 0x00, 0x02, 0x3B`

The comment explains height coding for 480 rows.

This register is more directly tied to display function/scanning behavior than gamma or power control. Depending on module defaults, omitting it could theoretically cause scan/display mismatch issues.

**Assessment:** medium-to-high importance if there are persistent scan/geometry anomalies on a specific module.

### 5. No explicit normal-display / idle-mode commands

Some sequences also send:
- `0x13` Normal Display Mode ON
- `0x38` Idle Mode OFF

Not usually required on every module, but helps force a known post-reset state.

**Assessment:** low importance, but nice for determinism.

### 6. Pixel format default is risky for SPI modules

Project default in `src/app.rs` is:
- `pixel_format = 0x66`

That means DBI/SPI transfers are sent as 18-bit pixel data by default.

Known-good Raspberry Pi/Linux style init commonly uses:
- `0x3A = 0x55` for 16-bit/pixel over SPI

This is not an omission in `init()`, but it is the strongest configuration risk in the whole driver.

**Assessment:** highest practical risk. If the symptom is stable vertical double-edge/column split artifacts, this is more suspicious than missing TE/VSYNC logic.

### 7. MADCTL default differs from common references

Project default in `src/app.rs`:
- `madctl = 0x88`

A common TFT_eSPI default is:
- `0x48`

Different values may still be valid depending on rotation/BGR choice, but if the module expects a different memory scan orientation, bad defaults can cause address interpretation surprises.

**Assessment:** medium risk. More likely to show as rotation/mirroring/BGR issues, but still worth validating.

## Is there a frame-sync rigor problem?

Short answer: **not in the classic VSYNC/HSYNC sense.**

This driver is SPI-to-GRAM. It does not stream pixels with external RGB frame timing. Therefore:
- no explicit VSYNC state machine exists in the code,
- no front porch/back porch timing exists in the code,
- no framebuffer swap synchronized to VSYNC exists in the code.

So the init review suggests:
- the code is **minimal**,
- it is **missing some determinism/compatibility registers**,
- but the more likely source of stable column-splitting artifacts is still **pixel format / addressing mode / module-specific init compatibility**, not “frame sync not rigorous”.

## Recommended stronger init sequence

A more conservative SPI init for this project would likely include at least:

1. `0x01` Software Reset
2. wait 120ms
3. `0xB0 0x00` Interface Mode Control
4. `0x11` Sleep Out
5. wait 120ms
6. `0x3A 0x55` (prefer 16-bit SPI first during bring-up)
7. `0x36 <validated madctl>`
8. `0xC0 ...`
9. `0xC1 ...`
10. `0xC2 ...`
11. `0xC5 ...`
12. `0xE0 ...`
13. `0xE1 ...`
14. `0xE2 ...` (optional but recommended)
15. `0x20` or `0x21`
16. `0xB6 0x00 0x02 0x3B` (module-dependent but recommended to test)
17. `0x38` Idle Mode OFF
18. `0x13` Normal Display Mode ON
19. `0x29` Display ON
20. wait 120ms

## Practical recommendation order

If debugging the current artifact, test in this order:

1. Change default pixel format from `0x66` to `0x55`
2. Lower SPI clock
3. Validate MADCTL against the actual module orientation/BGR wiring
4. Add `0xB0 0x00`
5. Add `0xC0` / `0xC1`
6. Add `0xB6`
7. Add `0xE2`
8. Optionally add `0x38` and `0x13`

## Bottom line

- **Yes, the current init is incomplete/minimal relative to common ILI9486 reference sequences.**
- **The most meaningful omissions are `0xB0`, `0xB6`, `0xC0`, `0xC1`, and `0xE2`.**
- **But the most suspicious real-world cause of “one column appears split into two columns” remains default `0x66` SPI pixel format rather than frame-sync rigor.**
