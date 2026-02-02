use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    //  0x00, 0x01, 0x02,   0x03,   0x04,   0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,   0x0B,   0x0C,   0x0D, 0x0E, 0x0F
        nop , todo, todo, inc_03, inc_04, dec_05, todo, todo, todo, todo, todo, dec_0b, inc_0c, dec_0d, todo, todo, //0x00
        todo, todo, todo, inc_13, inc_14, dec_15, todo, todo, todo, todo, todo, dec_1b, inc_1c, dec_1d, todo, todo, //0x01
        todo, todo, todo, inc_23, inc_24, dec_25, todo, todo, todo, todo, todo, dec_2b, inc_2c, dec_2d, todo, todo, //0x02
        todo, todo, todo, inc_33, inc_34, dec_35, todo, todo, todo, todo, todo, dec_3b, inc_3c, dec_3d, todo, todo, //0x03
        ld_40, ld_41, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x04
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x05
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x06
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x07
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x08
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x09
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0A
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0B
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0C
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0D
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0E
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0F
];

fn todo(cpu: &mut Cpu) -> u8 {
    todo!();
}

fn nop(_cpu: &mut Cpu) -> u8 {
    1
}

//INC BC
//----
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Registers16::BC);
    2
}

//INC DE
//----
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Registers16::DE);
    2
}

fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Registers16::HL);
    2
}

fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Registers16::SP);
    2
}

fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::B);
    1
}

fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::D);
    1
}

fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::H);
    1
}

fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::HL);
    3
}

fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::C);
    1
}

fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::E);
    1
}

fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::L);
    1
}

fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Registers::A);
    1
}

//DEC B
//Z1H
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::B);
    1
}

fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::D);
    1
}

fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::H);
    1
}

fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::HL);
    3
}

fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Registers16::BC);
    2
}

fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Registers16::DE);
    2
}

fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Registers16::HL);
    2
}

//DEC SP
//----
fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Registers16::SP);
    2 //Needs checking here
}

fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::C);
    1
}

fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::E);
    1
}

fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::L);
    1
}

fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Registers::A);
    1
}

//LD B,B
fn ld_40(cpu: &mut Cpu) -> u8 {
    1
}
//LD B, C
fn ld_41(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::B , value);
    1
}
//LD



pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_idx = cpu.fetch();
    OPCODES[op_idx as usize](cpu);
    return 0;
}