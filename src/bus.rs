use crate::cart::{Cart, ROM_START , ROM_STOP};
use crate::cpu::Registers16;
use crate::ppu::{Ppu, VRAM_START, VRAM_END, PpuUpdateResult, LCD_REG_START, LCD_REG_END, OAM_START, OAM_STOP};
use crate::utils::DISPLAY_BUFFER;
use crate::io::{IO, Button , IO_START, IO_STOP};

pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    io: IO,
    ram: [u8; 0x6000],
}

const OAM_DMA: u16 = 0xFF46;

impl Bus {
    pub fn new() -> Self{
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            io: IO::new(),
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
            },
            IO_START..=IO_STOP => {
                self.io.read_u8(addr)
            },
            OAM_START..=OAM_STOP => {
              self.ppu.read_oam(addr)
            },
            LCD_REG_START..=LCD_REG_END => {
                self.ppu.read_lcd_reg(addr)
            },
            _ => {
                let offset = addr - VRAM_END - 1;
                self.ram[offset as usize]
            }
        }
    }

    pub fn press_button(&mut self, button: Button, pressed: bool) {
        self.io.set_buttons(button, pressed);
    }


    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if addr == 0xFF40 {
            if (value & 0x80) != 0 {
                println!("LCD IS ON! LCDC: {:02X}", value);
            } else {
                println!("LCD IS OFF! LCDC: {:02X}", value);
            }
        }
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, value);
            },
            VRAM_START..=VRAM_END => {
                self.ppu.write_vram(addr, value);
            },
            IO_START..=IO_STOP => {
                self.io.write_u8(addr, value);
            },
            OAM_START..=OAM_STOP => {
                self.ppu.write_oam(addr, value);
            },
            LCD_REG_START..=LCD_REG_END => {
                if addr == OAM_DMA {
                    self.dma_transfer(value);
                }
                self.ppu.write_lcd_reg(addr, value);
            }
            _ => {
                let offset = addr - VRAM_END - 1;
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

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        self.ppu.render()
    }

    fn dma_transfer(&mut self, value: u8) {
        let src = (value as u16) << 8;
        for i in 0..0xA0 {
            let value = self.read_ram(src + i);
            self.write_ram(OAM_START + i, value);
        }
    }
}