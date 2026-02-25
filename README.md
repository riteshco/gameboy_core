# gameboy_core

A DMG-01 Game Boy emulator core written entirely in Rust. This repository acts as a standalone backend library, handling the internal hardware logic, memory mapping, and CPU execution of the original Game Boy.

It is designed to be decoupled from any specific frontend, meaning it only handles state mutations, returning frame buffers and audio/state data to be consumed by a display wrapper.

## Features

* **CPU**: Implementation of the Sharp SM83 instruction set, complete with accurate flag mutations and hardware bug replication (e.g., HALT bugs, POP AF masking).
* **PPU**: Scanline-based rendering pipeline supporting the Background, Window, and Sprite (OAM) layers, including 8x16 sprite mode and priority flags.
* **Memory Bus**: Strict memory mapping including Work RAM, High RAM, and correct ECHO RAM mirroring.
* **Cartridge/MBCs**: Supports ROM-only games, as well as Memory Bank Controllers including MBC1, MBC2, MBC3 (with RTC), and MBC5.
* **Timers**: Hardware-accurate DIV and TIMA registers.
* **Interrupts**: Full support for VBlank, STAT, Timer, and Joypad interrupts.

## Project Structure

* `src/cpu/`: The SM83 processor, registers, and opcode execution table.
* `src/ppu/`: Pixel Processing Unit, maintaining the LCD state machine (HBlank, VBlank, OAM/VRAM reading) and pixel output.
* `src/cart/`: ROM parsing, RAM banking, and MBC logic.
* `src/bus.rs`: The memory controller routing reads/writes to the correct hardware components.
* `src/timer.rs`: System clock and timer interrupts.
* `src/io.rs`: Joypad state and IO register handling.

## Usage

This crate is meant to be imported as a library. You must provide your own frontend to handle the game loop, input polling, and frame rendering.

```toml
[dependencies]
gb_core = { path = "../gameboy_core" }
