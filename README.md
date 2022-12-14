# 分裂 Bunretsu!

My custom firmware for the Corne keyboard, written in Rust.

🚧 WIP 🚧

## Setup

`TODO`


```bash
# macOS: pre-built bare-metal toolchain for 23-bit arms
$ brew install armmbed/formulae/arm-none-eabi-gcc

# installs the arm target
$ rustup target add thumbv7em-none-eabihf
```

## Building

`TODO`

## Running

`TODO`

## Useful things

`TODO`

```bash
# sanity check
$ arm-none-eabi-readelf -A target/thumbv7em-none-eabihf/release/bunretsu

# how much space is used for what
$ arm-none-eabi-size -Ax target/thumbv7em-none-eabihf/release/bunretsu

# disasm it!
$ arm-none-eabi-objdump -d target/thumbv7em-none-eabihf/release/bunretsu
```

## nice!nano

- Adafruit nRF52 Bootloader (DFU/UF2 flashing)
- nRF52840 chip on board, with 1 MB Flash and 256 KB RAM
- Cortex™-M4 CPU 32-bit, with FPU, 64 Mhz
- Bluetooth 5
- [pinout](https://nicekeyboards.com/static/0281a0132be3ca18b3d8a2f19f05eaf8/3c492/pinout-v2.png)
- [nRF52840 product spec](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.7.pdf)

## Links

- [nice!nano](https://nicekeyboards.com/nice-nano/)
