pub mod opcodes;

use crate::utils::*;
use crate::bus::Bus;
use crate::io::Button;
use crate::ppu::modes::LcdResults;

pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    irq_enabled: bool,
    halted: bool,
    bus: Bus,
    last_read: Option<u16>,
    last_write: Option<u16>,
}

const IRQ_PRIORITIES: [Interrupts; 5] = [
    Interrupts::Vblank,
    Interrupts::Stat,
    Interrupts::Timer,
    Interrupts::Serial,
    Interrupts::Joypad,
];

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            pc: 0x0100,
            sp: 0xFFFE,
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            irq_enabled: false,
            halted: false,
            bus: Bus::new(),
            last_read: None,
            last_write: None,
        };

        cpu.write_ram(0xFF10, 0x80);
        cpu.write_ram(0xFF11, 0xBF);
        cpu.write_ram(0xFF12, 0xF3);
        cpu.write_ram(0xFF14, 0xBF);
        cpu.write_ram(0xFF16, 0x3F);
        cpu.write_ram(0xFF19, 0xBF);
        cpu.write_ram(0xFF1A, 0x7F);
        cpu.write_ram(0xFF1B, 0xFF);
        cpu.write_ram(0xFF1C, 0x9F);
        cpu.write_ram(0xFF1E, 0xBF);
        cpu.write_ram(0xFF20, 0xFF);
        cpu.write_ram(0xFF23, 0xBF);
        cpu.write_ram(0xFF24, 0x77);
        cpu.write_ram(0xFF25, 0xF3);
        cpu.write_ram(0xFF26, 0xF1); // 0xF0 for SGB
        cpu.write_ram(0xFF40, 0x91);
        cpu.write_ram(0xFF47, 0xFC);
        cpu.write_ram(0xFF48, 0xFF);
        cpu.write_ram(0xFF49, 0xFF);

        cpu
    }

    pub fn get_r8(&self, r: Registers) -> u8 {
        match r {
            Registers::A => self.a,
            Registers::B => self.b,
            Registers::C => self.c,
            Registers::D => self.d,
            Registers::E => self.e,
            Registers::F => self.f,
            Registers::H => self.h,
            Registers::L => self.l,
            Registers::HL => {
                let addr = self.get_r16(Registers16::HL);
                self.read_ram(addr)
            }
        }
    }

    pub fn set_r8(&mut self, r: Registers, value: u8) {
        match r {
            Registers::A => self.a = value,
            Registers::B => self.b = value,
            Registers::C => self.c = value,
            Registers::D => self.d = value,
            Registers::E => self.e = value,
            Registers::F => self.f = value & 0xF0,
            Registers::H => self.h = value,
            Registers::L => self.l = value,
            Registers::HL => {
                let addr = self.get_r16(Registers16::HL);
                self.write_ram(addr, value);
            }
        }
    }

    pub fn get_r16(&self, r: Registers16) -> u16 {
        match r {
            Registers16::AF => merge_bytes(self.a, self.f),
            Registers16::BC => merge_bytes(self.b, self.c),
            Registers16::DE => merge_bytes(self.d, self.e),
            Registers16::HL => merge_bytes(self.h, self.l),
            Registers16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, r: Registers16, value: u16) {
        let high = value.high_byte();
        let low = value.low_byte();
        match r {
            Registers16::AF => {
                self.set_r8(Registers::A, high);
                self.set_r8(Registers::F, low);
            }
            Registers16::BC => {
                self.set_r8(Registers::B, high);
                self.set_r8(Registers::C, low);
            }
            Registers16::DE => {
                self.set_r8(Registers::D, high);
                self.set_r8(Registers::E, low);
            }
            Registers16::HL => {
                self.set_r8(Registers::H, low);
                self.set_r8(Registers::L, high);
            }
            Registers16::SP => self.sp = value,
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Z => (self.f & 0b1000_0000) != 0,
            Flags::S => (self.f & 0b0100_0000) != 0,
            Flags::HC => (self.f & 0b0010_0000) != 0,
            Flags::C => (self.f & 0b0001_0000) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            match flag {
                Flags::Z => self.f |= 0b1000_0000,
                Flags::S => self.f |= 0b0100_0000,
                Flags::HC => self.f |= 0b0010_0000,
                Flags::C => self.f |= 0b0001_0000,
            }
        } else {
            match flag {
                Flags::Z => self.f &= 0b0111_0000,
                Flags::S => self.f &= 0b1011_0000,
                Flags::HC => self.f &= 0b1101_0000,
                Flags::C => self.f &= 0b1110_0000,
            }
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.read_ram(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let value = merge_bytes(high, low);
        value
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.bus.read_ram(address)
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.bus.write_ram(address, value);
    }

    pub fn inc_r16(&mut self, r: Registers16) {
        let value = self.get_r16(r);
        let inc = value.wrapping_add(1);
        self.set_r16(r, inc);
    }

    pub fn dec_r16(&mut self, r: Registers16) {
        let value = self.get_r16(r);
        let dec = value.wrapping_sub(1);
        self.set_r16(r, dec);
    }

    pub fn dec_r8(&mut self, r: Registers) {
        let value = self.get_r8(r);
        let dec = value.wrapping_sub(1);
        let set_h = check_h_borrow_u8(value, 1);

        self.set_r8(r, dec);
        self.set_flag(Flags::S, true);
        self.set_flag(Flags::Z, dec == 0);
        self.set_flag(Flags::HC, set_h);
    }

    pub fn inc_r8(&mut self, r: Registers) {
        let value = self.get_r8(r);
        let inc = value.wrapping_add(1);
        let set_h = check_h_carry_u8(value, 1);

        self.set_r8(r, inc);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::Z, inc == 0);
        self.set_flag(Flags::HC, set_h);
    }

    pub fn and_a_u8(&mut self, value: u8) {
        let mut a = self.get_r8(Registers::A);
        a &= value;

        self.set_r8(Registers::A, a);
        self.set_flag(Flags::Z, a==0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, true);
        self.set_flag(Flags::C, false);
    }

    pub fn or_a_u8(&mut self, value: u8) {
        let mut a = self.get_r8(Registers::A);
        a |= value;

        self.set_r8(Registers::A, a);
        self.set_flag(Flags::Z, a==0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, false);
    }

    pub fn xor_a_u8(&mut self, value: u8) {
        let mut a = self.get_r8(Registers::A);
        a ^= value;

        self.set_r8(Registers::A, a);
        self.set_flag(Flags::Z, a==0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, false);
    }

    pub fn add_a_u8(&mut self, value: u8, adc: bool) {
        let mut carry = 0;
        if adc && self.get_flag(Flags::C) {
            carry = 1;
        }
        let a = self.get_r8(Registers::A);
        let result1= a.overflowing_add(value);
        let h_check1 = check_h_carry_u8(a, value);
        let result2 = result1.0.overflowing_add(carry);
        let h_check2 = check_h_carry_u8(result1.0, carry);
        let set_h = h_check1 || h_check2;
        let set_c = result1.1 || result2.1;

        self.set_flag(Flags::S, false);
        self.set_flag(Flags::C, set_c);
        self.set_flag(Flags::HC, set_h);
        self.set_flag(Flags::Z, result2.0 == 0);
        self.set_r8(Registers::A, result2.0);
    }

    pub fn sub_a_u8(&mut self, value: u8, sbc: bool) {
        let mut carry = 0;
        if sbc && self.get_flag(Flags::C) {
            carry = 1;
        }
        let a = self.get_r8(Registers::A);
        let result1= a.overflowing_sub(value);
        let check_h1 = check_h_borrow_u8(a, value);
        let result2 = result1.0.overflowing_sub(carry);
        let check_h2 = check_h_borrow_u8(result1.0, carry);
        let set_h = check_h1 || check_h2;

        self.set_flag(Flags::S, true);
        self.set_flag(Flags::Z, result2.0 == 0);
        self.set_flag(Flags::HC, set_h);
        self.set_flag(Flags::C, result1.1 || result2.1);
        self.set_r8(Registers::A, result2.0);
    }

    pub fn cp_a_u8(&mut self, value: u8) {
        let a = self.get_r8(Registers::A);
        let set_h = check_h_borrow_u8(a, value);

        self.set_flag(Flags::Z, a == value);
        self.set_flag(Flags::S, true);
        self.set_flag(Flags::HC , set_h);
        self.set_flag(Flags::C, a < value);
    }

    pub fn add_r16(&mut self, dst_r: Registers16, src_r: Registers16) {
        let dst = self.get_r16(dst_r);
        let src = self.get_r16(src_r);
        let res = dst.overflowing_add(src);
        let set_h = check_h_carry_u16(dst, src);

        self.set_r16(dst_r, res.0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, set_h);
        self.set_flag(Flags::C, res.1);
    }

    pub fn pop(&mut self) -> u16 {
        assert_ne!(self.sp, 0xFFFE, "Stack mustn't be empty when trying to pop!");
        let low = self.read_ram(self.sp);
        let high = self.read_ram(self.sp.wrapping_add(1));
        let value = merge_bytes(high, low);
        self.sp = self.sp.wrapping_add(2);
        value
    }

    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_ram(self.sp, value.low_byte());
        self.write_ram(self.sp.wrapping_add(1), value.high_byte());
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn rotate_left(&mut self, reg: Registers, carry: bool) {
        let value = self.get_r8(reg);
        let msb = value.get_bit(7);
        let mut new = value.rotate_left(1);
        if carry {
            new.set_bit(0, self.get_flag(Flags::C));
        }
        self.set_r8(reg, new);
        self.set_flag(Flags::Z, new == 0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, msb);
    }

    pub fn rotate_right(&mut self, reg: Registers, carry: bool) {
        let value = self.get_r8(reg);
        let lsb = value.get_bit(0);
        let mut new = value.rotate_right(1);
        if carry {
            new.set_bit(7, self.get_flag(Flags::C));
        }
        self.set_r8(reg, new);
        self.set_flag(Flags::Z, new == 0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, lsb);
    }

    pub fn shift_left(&mut self, reg: Registers) {
        let value = self.get_r8(reg);
        let msb = value.get_bit(7);
        let res = value.wrapping_shl(1);

        self.set_r8(reg, res);
        self.set_flag(Flags::Z, res == 0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, msb);
    }

    pub fn shift_right(&mut self, reg: Registers, arithmetic: bool) {
        let value = self.get_r8(reg);
        let lsb = value.get_bit(0);
        let msb = value.get_bit(7);
        let mut res = value.wrapping_shr(1);
        if arithmetic {
            res.set_bit(7, msb);
        }

        self.set_r8(reg, res);
        self.set_flag(Flags::Z, res == 0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, lsb);
    }

    pub fn swap_bits(&mut self, reg: Registers) {
        let value = self.get_r8(reg);
        let low = value & 0xF;
        let high = (value & 0xF0) >> 4;
        let res = (low << 4) | high;

        self.set_r8(reg, res);
        self.set_flag(Flags::Z, res == 0);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, false);
        self.set_flag(Flags::C, false);
    }

    pub fn test_bit(&mut self, reg: Registers, bit: u8) {
        let byte = self.get_r8(reg);
        let value = byte.get_bit(bit);

        self.set_flag(Flags::Z, !value);
        self.set_flag(Flags::S, false);
        self.set_flag(Flags::HC, true);
    }

    pub fn write_bit(&mut self, reg: Registers, bit: u8, set: bool){
        let mut byte = self.get_r8(reg);
        byte.set_bit(bit, set);
        self.set_r8(reg, byte);
    }

    pub fn set_irq(&mut self, enabled: bool) {
        self.irq_enabled = enabled;
    }

    pub fn set_halted(&mut self, halted: bool){
        self.halted = halted;
    }

    pub fn tick(&mut self) -> bool {
        self.last_read = None;
        self.last_write = None;
        let mut draw_time = false;
        let cycles = if self.halted { 1 } else { opcodes::execute(self) };
        let ppu_result = self.bus.update_ppu(cycles);
        if ppu_result.irq {
            self.enable_irq_type(Interrupts::Stat, true);
        }
        match ppu_result.lcd_result {
            LcdResults::RenderFrame => {
                self.bus.render_scanline();
                self.enable_irq_type(Interrupts::Vblank, true);
                draw_time = true;
            },
            LcdResults::RenderLine => {
                self.bus.render_scanline();
            },
            _ => {}
        }
        let timer_irq = self.bus.update_timer(cycles);
        if timer_irq {
            self.enable_irq_type(Interrupts::Timer, true);
        }

        if let Some(irq) = self.check_irq() {
            self.trigger_irq(irq);
        }
        draw_time
    }

    fn check_irq(&mut self) -> Option<Interrupts> {
        if !self.irq_enabled && !self.halted {
            return None;
        }

        let if_reg = self.read_ram(IF);
        let ie_reg = self.read_ram(IE);
        let irq_flags = if_reg & ie_reg;
        for (i, irq) in IRQ_PRIORITIES.iter().enumerate() {
            if irq_flags.get_bit(i as u8) {
                return Some(*irq);
            }
        }
        None
    }

    fn enable_irq_type(&mut self, irq: Interrupts, enabled: bool) {
        let mut if_reg = self.read_ram(IF);
        match irq {
            Interrupts::Vblank => { if_reg.set_bit(0, enabled) },
            Interrupts::Stat =>   { if_reg.set_bit(1, enabled) },
            Interrupts::Timer =>  { if_reg.set_bit(2, enabled) },
            Interrupts::Serial => { if_reg.set_bit(3, enabled) },
            Interrupts::Joypad => { if_reg.set_bit(4, enabled) },
        }
        self.write_ram(IF, if_reg);
    }

    fn trigger_irq(&mut self, irq: Interrupts) {
        self.halted = false;

        if self.irq_enabled {
            self.irq_enabled = false;

            let vector = irq.get_vector();
            self.push(self.pc);
            self.set_pc(vector);

            self.enable_irq_type(irq, false);
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.bus.load_rom(rom);
    }

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        self.bus.render()
    }

    pub fn press_button(&mut self, button: Button, pressed: bool) {
        self.bus.press_button(button, pressed);
        self.enable_irq_type(Interrupts::Joypad, true);
    }
}

#[derive(Copy, Clone)]
pub enum Registers {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL
}

#[derive(Copy, Clone)]
pub enum Registers16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum Flags {
    Z,
    S,
    C,
    HC,
}

const IF: u16 = 0xFF0F;
const IE: u16 = 0xFFFF;

#[derive(Copy, Clone)]
pub enum Interrupts {
    Vblank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupts {
    pub fn get_vector(&self) -> u16 {
        match *self {
            Interrupts::Vblank => 0x0040,
            Interrupts::Stat => 0x0048,
            Interrupts::Timer => 0x0050,
            Interrupts::Serial => 0x0058,
            Interrupts::Joypad => 0x0060,
        }
    }
}
