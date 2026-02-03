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
        let value = merge_bytes(high, low);
        value
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        todo!();
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        todo!();
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
        let high = self.read_ram(self.sp+1);
        let value = merge_bytes(high, low);
        self.sp += 2;
        value
    }

    pub fn push(&mut self, value: u16) {
        self.sp -= 2;
        self.write_ram(self.sp, value.low_byte());
        self.write_ram(self.sp, value.high_byte());
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
