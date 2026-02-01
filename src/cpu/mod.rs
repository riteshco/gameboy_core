pub mod opcodes;

use crate::utils::*;

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
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
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
            Flags::S => (self.b & 0b0100_0000) != 0,
            Flags::HC => (self.c & 0b0010_0000) != 0,
            Flags::C => (self.d & 0b0001_0000) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            match flag {
                Flags::Z => self.f |= 0b1000_0000,
                Flags::S => self.b |= 0b0100_0000,
                Flags::HC => self.c |= 0b0010_0000,
                Flags::C => self.d |= 0b0001_0000,
            }
        } else {
            match flag {
                Flags::Z => self.f &= 0b0111_0000,
                Flags::S => self.b &= 0b1011_0000,
                Flags::HC => self.c &= 0b1101_0000,
                Flags::C => self.d &= 0b1110_0000,
            }
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        value
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let value = merge_bytes(low, high);
        value
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        todo!();
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        todo!();
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
