use crate::cart::{Cart, ROM_START , ROM_STOP};
use crate::ppu::{Ppu, VRAM_START , VRAM_END , PpuUpdateResult};

pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    ram: [u8; 0x6000],
}

impl Bus {
    pub fn new() -> Self{
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            ram: [0; 0x6000],
        }
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.read_cart(addr)
            },
            VRAM_START..=VRAM_END => {
                self.ppu.read_vram(addr)
            }
            _ => {
                let offset = addr - ROM_STOP - 1;
                self.ram[offset as usize]
            }
        }
    }

    pub fn write_ram(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, value);
            },
            VRAM_START..=VRAM_END => {
                self.ppu.write_vram(addr, value);
            }
            _ => {
                let offset = addr - ROM_STOP - 1;
                self.ram[offset as usize] = value;
        }
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
    }

    pub fn update_ppu(&mut self, cycles: u8) -> PpuUpdateResult {
        self.ppu.update(cycles)
    }
}