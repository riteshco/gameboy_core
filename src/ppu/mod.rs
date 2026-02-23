pub mod tile;
pub mod modes;

use tile::Tile;
use modes::{Lcd, LcdResults, LcdModeType};
use crate::utils::*;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;

const TILE_SET_START: u16 = 0x8000;
const TILE_SET_END: u16 = 0x97FF;
const TILE_MAP_START: u16 = 0x9800;
const TILE_MAP_END: u16 = 0x9FFF;
const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize = 384;
const TILE_MAP_SIZE: usize = (TILE_MAP_END - TILE_MAP_START + 1) as usize;

const TILE_SIZE: usize = 8;
const LAYER_SIZE: usize = 32;
const MAP_PIXELS: usize = TILE_SIZE * LAYER_SIZE;

pub struct PpuUpdateResult {
    pub lcd_result: LcdResults,
    pub irq: bool,
}

pub struct Ppu {
    mode: Lcd,
    tiles: [Tile; NUM_TILES],
    maps: [u8; TILE_MAP_SIZE],
    lcd_regs: [u8; LCD_REG_SIZE],
}

const LY: u16 = 0xFF44;
const STAT: u16 = 0xFF41;

const STAT_OAM_IRQ_BIT: u8          = 5;
const STAT_VBLANK_IRQ_BIT: u8       = 4;
const STAT_HBLANK_IRQ_BIT: u8       = 3;

impl Ppu {
    pub fn new() -> Self {
        Self{
            mode: Lcd::new(),
            tiles: [Tile::new(); NUM_TILES],
            maps: [0; TILE_MAP_SIZE],
            lcd_regs: [0; LCD_REG_SIZE],
        }
    }

    pub fn update(&mut self, cycles: u8) -> PpuUpdateResult {
        let old_mode = self.mode.get_mode();
        let lcd_result = self.mode.step(cycles);
        let mut stat = self.read_lcd_reg(STAT);
        let mut irq = false;

        let scanline = self.mode.get_line();
        self.write_lcd_reg(LY, scanline);

        let mode = self.mode.get_mode();
        if old_mode != mode {
            match mode {
                LcdModeType::HBLANK => {
                    irq |= stat.get_bit(STAT_HBLANK_IRQ_BIT);
                },
                LcdModeType::VBLANK => {
                    irq |= stat.get_bit(STAT_VBLANK_IRQ_BIT);
                },
                LcdModeType::OAMReadMode => {
                    irq |= stat.get_bit(STAT_OAM_IRQ_BIT);
                },
                _ => {},
            }
        }

        stat &= 0b1111_1100;
        stat |= mode.get_idx();
        self.write_lcd_reg(STAT, stat);

        PpuUpdateResult { lcd_result, irq }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        match addr {
            TILE_SET_START..=TILE_SET_END => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr/BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize].read_u8(offset)
            },
            TILE_MAP_START..=TILE_MAP_END => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize]
            },
            _ => unreachable!(),
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        match addr {
            TILE_SET_START..=TILE_SET_END => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr/BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize].write_u8(offset, value)
            },
            TILE_MAP_START..=TILE_MAP_END => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize] = value;
            },
            _ => unreachable!(),
        }
    }
}

pub const LCD_REG_START: u16 = 0xFF40;
pub const LCD_REG_END: u16 = 0xFF4B;
const LCD_REG_SIZE: usize = (LCD_REG_END - LCD_REG_START + 1) as usize;

impl Ppu {
    pub fn read_lcd_reg(&self, addr: u16) -> u8 {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize]
    }

    pub fn write_lcd_reg(&mut self, addr: u16, value: u8) {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize] = value;
    }

    fn is_lcd_enabled(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_LCD_ENABLED_BIT)
    }

    fn get_wndw_tile_map_idx(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_WNDW_MAP_BIT) { 1 } else { 0 }
    }

    fn get_bg_window_tile_set_idx(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_WNDW_TILE_BIT) { 1 } else { 0 }
    }

    fn get_bg_tile_map_idx(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_MAP_BIT) { 1 } else { 0 }
    }

    fn are_sprites_8x16(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_SIZE_BIT)
    }

    fn is_sprite_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_ENABLED_BIT)
    }

    fn is_bg_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_BG_WNDW_ENABLED_BIT)
    }

    fn is_window_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_BG_WNDW_ENABLED_BIT) && lcdc.get_bit(LCDC_WNDW_ENABLED_BIT)
    }
}

const LCDC: u16 = 0xFF40;
const LCDC_LCD_ENABLED_BIT: u8      = 7;
const LCDC_WNDW_MAP_BIT: u8         = 6;
const LCDC_WNDW_ENABLED_BIT: u8     = 5;
const LCDC_BG_WNDW_TILE_BIT: u8     = 4;
const LCDC_BG_MAP_BIT: u8           = 3;
const LCDC_SPR_SIZE_BIT: u8         = 2;
const LCDC_SPR_ENABLED_BIT: u8      = 1;
const LCDC_BG_WNDW_ENABLED_BIT: u8  = 0;


const SCY: u16 = 0xFF42;
const SCX: u16 = 0xFF43;

const WY: u16 = 0xFF4A;
const WX: u16 = 0xFF4B;

impl Ppu {
    fn get_viewport_coord(&self) -> Point {
        let x = self.read_lcd_reg(SCX);
        let y = self.read_lcd_reg(SCY);
        Point::new(x, y)
    }

    fn get_window_coord(&self) -> Point {
        let x = self.read_lcd_reg(WX);
        let y = self.read_lcd_reg(WY);
        Point::new(x.saturating_sub(7), y)
    }
}

const BGP: u16 = 0xFF47;

impl Ppu {
    fn get_bg_palette(&self) -> [u8; 4] {
        unpack_u8(self.read_lcd_reg(BGP))
    }
}

const OBP0: u16 = 0xFF48;
const OBP1: u16 = 0xFF49;

const NUM_TILE_COLS: usize = SCREEN_WIDTH / 8;
const NUM_TILE_ROWS: usize = SCREEN_HEIGHT / 8;
const LAYER_WIDTH: usize = 32;
const TILE_MAP_TABLE_SIZE: usize = TILE_MAP_SIZE / 2;

impl Ppu {
    fn get_sprite_palette(&self, index: u8) -> [u8; 4] {
        match index {
            0 => unpack_u8(self.read_lcd_reg(OBP0)) ,
            1 => unpack_u8(self.read_lcd_reg(OBP1)) ,
            _ => { unreachable!(); },
        }
    }

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        let mut result = [0xFF; DISPLAY_BUFFER];

        if self.is_bg_layer_displayed() {
            self.render_bg(&mut result);
        }

        if self.is_window_layer_displayed() {
            self.render_window(&mut result);
        }
        result
    }

    pub fn render_bg(&self, buffer: &mut [u8]) {
        let map_offset = self.get_bg_tile_map_idx() as usize * TILE_MAP_TABLE_SIZE;
        let palette = self.get_bg_palette();
        let viewport = self.get_viewport_coord();
        for py in 0..SCREEN_HEIGHT {
            let current_y = viewport.y as usize + py as usize;
            let y = current_y % MAP_PIXELS;
            let row = current_y % TILE_SIZE;
            for px in 0..SCREEN_WIDTH {
                let current_x = viewport.x as usize + px as usize;
                let x = current_x % MAP_PIXELS;
                let col = current_x % TILE_SIZE;
                let map_num = (y / TILE_SIZE) * LAYER_SIZE + (x / TILE_SIZE);
                let tile_index = self.maps[map_offset + map_num] as usize;
                let adjusted_tile_index = if self.get_bg_window_tile_set_idx() == 1 {
                    tile_index as usize
                } else {
                    (256 + tile_index as i8 as isize) as usize
                };
                let tile = self.tiles[adjusted_tile_index];
                let data = tile.get_row(row as u8);
                let cell = data[col];
                let color_index = palette[cell as usize];
                let color = GB_PALETTE[color_index as usize];
                let buffer_index = 4 * (py * SCREEN_WIDTH + px);
                for i in 0..4 {
                    buffer[buffer_index + i] = color[i];
                }
            }
        }
    }

    fn render_window(&self, buffer: &mut [u8]) {
        let map_offset = self.get_wndw_tile_map_idx() as usize * TILE_MAP_TABLE_SIZE;
        let palette = self.get_bg_palette();
        let coords = self.get_window_coord();
        if (coords.x as usize > SCREEN_WIDTH) || (coords.y as usize > SCREEN_HEIGHT) {
            return;
        }
        for y in (coords.y as usize)..SCREEN_HEIGHT {
            let row = y % TILE_SIZE;
            for x in (coords.x as usize)..SCREEN_WIDTH {
                let col = x % TILE_SIZE;
                let map_num = (y / TILE_SIZE) * LAYER_SIZE + (x / TILE_SIZE);
                let tile_index = self.maps[map_offset + map_num] as usize;
                let adjusted_tile_index = if self.get_bg_window_tile_set_idx() == 1 {
                    tile_index as usize
                } else {
                    (256 + tile_index as i8 as isize) as usize
                };
                let tile = self.tiles[adjusted_tile_index];
                let data = tile.get_row(row as u8);
                let cell = data[col];
                let color_idx = palette[cell as usize];
                let color = GB_PALETTE[color_idx as usize];
                let buffer_idx = 4 * (y * SCREEN_WIDTH + x);
                for i in 0..4 {
                    buffer[buffer_idx + i] = color[i];
                }
            }
        }
    }
}