use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    //  0x00, 0x01, 0x02,   0x03,   0x04,   0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,   0x0B,   0x0C,   0x0D, 0x0E, 0x0F
        nop , ld_01, todo, inc_03, inc_04, dec_05, ld_06, todo, todo, todo, ld_0a, dec_0b, inc_0c, dec_0d, todo, todo, //0x00
        todo, ld_11, todo, inc_13, inc_14, dec_15, ld_16, todo, todo, todo, ld_1a, dec_1b, inc_1c, dec_1d, todo, todo, //0x01
        todo, ld_21, todo, inc_23, inc_24, dec_25, ld_26, todo, todo, todo, ld_2a, dec_2b, inc_2c, dec_2d, todo, todo, //0x02
        todo, ld_31, todo, inc_33, inc_34, dec_35, ld_36, todo, todo, todo, ld_3a, dec_3b, inc_3c, dec_3d, todo, todo, //0x03
        ld_40, ld_41, ld_42, ld_43, ld_44, ld_45, todo, todo, ld_48, ld_49, ld_4a, ld_4b, ld_4c, ld_4d, todo, todo, //0x04
        ld_50, ld_51, ld_52, ld_53, ld_54, ld_55, todo, todo, ld_58, ld_59, ld_5a, ld_5b, ld_5c, ld_5d, todo, todo, //0x05
        ld_60, ld_61, ld_62, ld_63, ld_64, ld_65, todo, todo, ld_68, ld_69, ld_6a, ld_6b, ld_6c, ld_6d, todo, todo, //0x06
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x07
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x08
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x09
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0A
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0B
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0C
        todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,   todo,   todo,   todo, todo, todo, //0x0D
        ld_e0, todo, ld_e2,   todo,   todo,   todo, todo, todo, todo, todo, ld_ea,   todo,   todo,   todo, todo, todo, //0x0E
        ld_f0, todo, ld_f2,   todo,   todo,   todo, todo, todo, ld_f8, ld_f9, ld_fa,   todo,   todo,   todo, todo, todo, //0x0F
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
//LD B, D
fn ld_42(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::B , value);
    1
}
//LD B, E
fn ld_43(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::B, value);
    1
}
//LD B, H
fn ld_44(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::B, value);
    1
}
//LD B, L
fn ld_45(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::B, value);
    1
}


//LD D,B
fn ld_50(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::D, value);
    1
}
//LD D, C
fn ld_51(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::D , value);
    1
}
//LD D, D
fn ld_52(cpu: &mut Cpu) -> u8 {
    1
}
//LD D, E
fn ld_53(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::D, value);
    1
}
//LD D, H
fn ld_54(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::D, value);
    1
}
//LD D, L
fn ld_55(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::D, value);
    1
}

//LD H,B
fn ld_60(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::H, value);
    1
}
//LD H, C
fn ld_61(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::H , value);
    1
}
//LD H, D
fn ld_62(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::H, value);
    1
}
//LD H, E
fn ld_63(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::H, value);
    1
}
//LD H, H
fn ld_64(cpu: &mut Cpu) -> u8 {
    1
}
//LD H, L
fn ld_65(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::H, value);
    1
}

//LD B, A
fn ld_47(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::B, value);
    1
}
//LD D, A
fn ld_57(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::D, value);
    1
}
//LD H,A
fn ld_67(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::H, value);
    1
}

//LD C,B
fn ld_48(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::C, value);
    1
}
//LD C, C
fn ld_49(cpu: &mut Cpu) -> u8 {
    1
}
//LD C, D
fn ld_4a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::C , value);
    1
}
//LD C, E
fn ld_4b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::C, value);
    1
}
//LD C, H
fn ld_4c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::C, value);
    1
}
//LD C, L
fn ld_4d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::C, value);
    1
}

//LD E,B
fn ld_58(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::E, value);
    1
}
//LD E, C
fn ld_59(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::E, value);
    1
}
//LD E, D
fn ld_5a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::E , value);
    1
}
//LD E, E
fn ld_5b(cpu: &mut Cpu) -> u8 {
    1
}
//LD E, H
fn ld_5c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::E, value);
    1
}
//LD E, L
fn ld_5d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::E, value);
    1
}

//LD L,B
fn ld_68(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::L, value);
    1
}
//LD L, C
fn ld_69(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::L, value);
    1
}
//LD L, D
fn ld_6a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::L , value);
    1
}
//LD L, E
fn ld_6b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::L, value);
    1
}
//LD L, H
fn ld_6c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::L, value);
    1
}
//LD L, L
fn ld_6d(cpu: &mut Cpu) -> u8 {
    1
}

//LD A,B
fn ld_78(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.set_r8(Registers::A, value);
    1
}
//LD A, C
fn ld_79(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.set_r8(Registers::A, value);
    1
}
//LD A, D
fn ld_7a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.set_r8(Registers::A , value);
    1
}
//LD A, E
fn ld_7b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.set_r8(Registers::A, value);
    1
}
//LD A, H
fn ld_7c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.set_r8(Registers::A, value);
    1
}
//LD A, L
fn ld_7d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.set_r8(Registers::A, value);
    1
}

//LD BC, u16
fn ld_01(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_r16(Registers16::BC, value);
    3
}
//LD DE, u16
fn ld_11(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_r16(Registers16::DE, value);
    3
}
//LD HL, u16
fn ld_21(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_r16(Registers16::HL, value);
    3
}
//LD SP, u16
fn ld_31(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_u16();
    cpu.set_r16(Registers16::SP, value);
    3
}

//LD (u16), SP
fn ld_08(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let value = cpu.get_r16(Registers16::SP);
    cpu.write_ram(addr , value.low_byte());
    cpu.write_ram(addr + 1 , value.high_byte());
    5
}

//LD A, (BC)
fn ld_0a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::BC);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    2
}
//LD A, (DE)
fn ld_1a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::DE);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    2
}
//LD A, (HL+)
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    cpu.set_r16(Registers16::HL, addr.wrapping_add(1));
    2
}
//LD A, (HL-)
fn ld_3a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    cpu.set_r16(Registers16::HL, addr.wrapping_sub(1));
    2
}

//LD B, u8
fn ld_06(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::B, value);
    2
}
//LD D, u8
fn ld_16(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::D, value);
    2
}
//LD H, u8
fn ld_26(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::H, value);
    2
}
//LD (HL), u8
fn ld_36(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    3
}

//LD (FF00+u8), A
fn ld_e0(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    cpu.write_ram(addr, value);
    2
}
//LD (FF00+C), A
fn ld_e2(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let offset = cpu.get_r8(Registers::C) as u16;
    let addr = 0xFF00 + offset;
    cpu.write_ram(addr, value);
    2
}
//LD A, (FF00+u8)
fn ld_f0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    3
}
//LD A, (FF00+C)
fn ld_f2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_r8(Registers::C) as u16;
    let addr = 0xFF00 + offset;
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    2
}

//LD (u16), A
fn ld_ea(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.fetch_u16();
    cpu.write_ram(addr, value);
    4
}
//LD A, (u16)
fn ld_fa(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    4
}

//LD HL, SP+i8
// 00HC
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8 as i16 as u16;
    let sp = cpu.get_r16(Registers16::SP);
    let set_c = check_c_carry_u8(sp.low_byte(), offset.low_byte());
    let set_h = check_h_carry_u8(sp.low_byte(), offset.low_byte());

    cpu.set_r16(Registers16::HL, offset.wrapping_add(sp));
    cpu.set_flag(Flags::Z , false);
    cpu.set_flag(Flags::S, false);
    cpu.set_flag(Flags::HC, set_h);
    cpu.set_flag(Flags::C, set_c);
    3
}
//LD SP, HL
fn ld_f9(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r16(Registers16::HL);
    cpu.set_r16(Registers16::SP, value);
    2
}

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_idx = cpu.fetch();
    OPCODES[op_idx as usize](cpu);
    return 0;
}