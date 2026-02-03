use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    //    0x00,  0x01,    0x02,   0x03,   0x04,    0x05,   0x06,   0x07,   0x08,   0x09,   0x0A,   0x0B,   0x0C,   0x0D,   0x0E,   0x0F
          nop ,  ld_01,  ld_02, inc_03, inc_04,  dec_05,  ld_06,   todo,  ld_08, add_09,  ld_0a, dec_0b, inc_0c, dec_0d,  ld_0e,   todo, //0x00
          todo,  ld_11,  ld_12, inc_13, inc_14,  dec_15,  ld_16,   todo,   todo, add_19,  ld_1a, dec_1b, inc_1c, dec_1d,  ld_1e,   todo, //0x01
          todo,  ld_21,  ld_22, inc_23, inc_24,  dec_25,  ld_26,   todo,   todo, add_29,  ld_2a, dec_2b, inc_2c, dec_2d,  ld_2e,   todo, //0x02
          todo,  ld_31,  ld_32, inc_33, inc_34,  dec_35,  ld_36,   todo,   todo, add_39,  ld_3a, dec_3b, inc_3c, dec_3d,  ld_3e,   todo, //0x03
         ld_40,  ld_41,  ld_42,  ld_43,  ld_44,   ld_45,  ld_46,  ld_47,  ld_48,  ld_49,  ld_4a,  ld_4b,  ld_4c,  ld_4d,  ld_4e,  ld_4f, //0x04
         ld_50,  ld_51,  ld_52,  ld_53,  ld_54,   ld_55,  ld_56,  ld_57,  ld_58,  ld_59,  ld_5a,  ld_5b,  ld_5c,  ld_5d,  ld_5e,  ld_5f, //0x05
         ld_60,  ld_61,  ld_62,  ld_63,  ld_64,   ld_65,  ld_66,  ld_67,  ld_68,  ld_69,  ld_6a,  ld_6b,  ld_6c,  ld_6d,  ld_6e,  ld_6f, //0x06
         ld_70,  ld_71,  ld_72,  ld_73,  ld_74,   ld_75,   todo,  ld_77,  ld_78,  ld_79,  ld_7a,  ld_7b,  ld_7c,  ld_7d,  ld_7e,  ld_7f, //0x07
        add_80, add_81, add_82, add_83, add_84,  add_85, add_86, add_87, adc_88, adc_89, adc_8a, adc_8b, adc_8c, adc_8d, adc_8e, adc_8f, //0x08
        sub_90, sub_91, sub_92, sub_93, sub_94,  sub_95, sub_96, sub_97, sbc_98, sbc_99, sbc_9a, sbc_9b, sbc_9c, sbc_9d, sbc_9e, sbc_9f, //0x09
        and_a0, and_a1, and_a2, and_a3, and_a4,  and_a5, and_a6, and_a7, xor_a8, xor_a9, xor_aa, xor_ab, xor_ac, xor_ad, xor_ae, xor_af, //0x0A
         or_b0,  or_b1,  or_b2,  or_b3,  or_b4,   or_b5,  or_b6,  or_b7,  cp_b8,  cp_b9,  cp_ba,  cp_bb,  cp_bc,  cp_bd,  cp_be,  cp_bf, //0x0B
          todo, pop_c1,   todo,   todo,   todo, push_c5, add_c6,   todo,   todo,   todo,   todo,   todo,   todo,   todo, adc_ce,   todo, //0x0C
          todo, pop_d1,   todo,   todo,   todo, push_d5, sub_d6,   todo,   todo,   todo,   todo,   todo,   todo,   todo, sbc_de,   todo, //0x0D
         ld_e0, pop_e1,  ld_e2,   todo,   todo, push_e5, and_e6,   todo, add_e8,   todo,  ld_ea,   todo,   todo,   todo, xor_ee,   todo, //0x0E
         ld_f0, pop_f1,  ld_f2,   todo,   todo, push_f5,  or_f6,   todo,  ld_f8,  ld_f9,  ld_fa,   todo,   todo,   todo,  cp_fe,   todo, //0x0F
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

//LD (BC), A
fn ld_02(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.get_r16(Registers16::BC);
    cpu.write_ram(addr, value);
    2
}
//LD (DE), A
fn ld_12(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.get_r16(Registers16::DE);
    cpu.write_ram(addr, value);
    2
}
//LD (HL+), A
fn ld_22(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    cpu.set_r16(Registers16::HL, addr.wrapping_add(1));
    2
}
//LD (HL-), A
fn ld_32(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    cpu.set_r16(Registers16::HL, addr.wrapping_sub(1));
    2
}

//LD (HL), B
fn ld_70(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), C
fn ld_71(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), D
fn ld_72(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), E
fn ld_73(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), H
fn ld_74(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), L
fn ld_75(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}
//LD (HL), A
fn ld_77(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    let addr = cpu.get_r16(Registers16::HL);
    cpu.write_ram(addr, value);
    2
}

//LD B, (HL)
fn ld_46(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::B, value);
    2
}
//LD D, (HL)
fn ld_56(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::D, value);
    2
}
//LD H, (HL)
fn ld_66(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::H, value);
    2
}

//LD C, (HL)
fn ld_4e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::C, value);
    2
}
//LD E, (HL)
fn ld_5e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::E, value);
    2
}
//LD L, (HL)
fn ld_6e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::L, value);
    2
}
//LD A, (HL)
fn ld_7e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Registers16::HL);
    let value = cpu.read_ram(addr);
    cpu.set_r8(Registers::A, value);
    2
}

//LD C, A
fn ld_4f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::C, value);
    1
}
//LD E, A
fn ld_5f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::E, value);
    1
}
//LD L, A
fn ld_6f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.set_r8(Registers::L, value);
    1
}
//LD A, A
fn ld_7f(cpu: &mut Cpu) -> u8 {
    1
}

//LD C, u8
fn ld_0e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::C, value);
    2
}
//LD E, u8
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::E, value);
    2
}
//LD L, u8
fn ld_2e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::L, value);
    2
}
//LD A, u8
fn ld_3e(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.set_r8(Registers::A, value);
    2
}

//ADD HL, BC (-0HC)
fn add_09(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Registers16::HL, Registers16::BC);
    2
}
//ADD HL, DE (-0HC)
fn add_19(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Registers16::HL, Registers16::DE);
    2
}
//ADD HL, HL (-0HC)
fn add_29(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Registers16::HL, Registers16::HL);
    2
}
//ADD HL, SP (-0HC)
fn add_39(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Registers16::HL, Registers16::SP);
    2
}

//ADD A, B (Z0HC)
fn add_80(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.add_a_u8(value , false);
    1
}
//ADD A, C (Z0HC)
fn add_81(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.add_a_u8(value , false);
    1
}
//ADD A, D (Z0HC)
fn add_82(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.add_a_u8(value, false);
    1
}
//ADD A, E (Z0HC)
fn add_83(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.add_a_u8(value, false);
    1
}
//ADD A, H (Z0HC)
fn add_84(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.add_a_u8(value, false);
    1
}
//ADD A, L (Z0HC)
fn add_85(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.add_a_u8(value, false);
    1
}
//ADD A, (HL) (Z0HC)
fn add_86(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.add_a_u8(value, false);
    2
}
//ADD A, A (Z0HC)
fn add_87(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.add_a_u8(value, false);
    1
}

//SUB A, B (Z1HC)
fn sub_90(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, C (Z1HC)
fn sub_91(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, D (Z1HC)
fn sub_92(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, E (Z1HC)
fn sub_93(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, H (Z1HC)
fn sub_94(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, L (Z1HC)
fn sub_95(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.sub_a_u8(value, false);
    1
}
//SUB A, (HL) (Z1HC)
fn sub_96(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.sub_a_u8(value, false);
    2
}
//SUB A, A (Z1HC)
fn sub_97(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.sub_a_u8(value, false);
    1
}

//SBC A, B (ZIHC)
fn sbc_98(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A, C (ZIHC)
fn sbc_99(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A, D (ZIHC)
fn sbc_9a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A, E (ZIHC)
fn sbc_9b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A, H (ZIHC)
fn sbc_9c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A, L (ZIHC)
fn sbc_9d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.sub_a_u8(value, true);
    1
}
//SBC A,(HL) (Z1HC)
fn sbc_9e(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.sub_a_u8(value , true);
    2
}
//SBC A, A (ZIHC)
fn sbc_9f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.sub_a_u8(value, true);
    1
}

//CP A, u8 (Z1HC)
fn cpu_fe(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.cp_a_u8(value);
    2
}

//ADC A, B (Z0HC)
fn adc_88(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.add_a_u8(value , true);
    1
}
//ADC A, C (Z0HC)
fn adc_89(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.add_a_u8(value , true);
    1
}
//ADC A, D (Z0HC)
fn adc_8a(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.add_a_u8(value, true);
    1
}
//ADC A, E (Z0HC)
fn adc_8b(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.add_a_u8(value, true);
    1
}
//ADC A, H (Z0HC)
fn adc_8c(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.add_a_u8(value, true);
    1
}
//ADC A, L (Z0HC)
fn adc_8d(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.add_a_u8(value, true);
    1
}
//ADC A, (HL) (Z0HC)
fn adc_8e(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.add_a_u8(value, true);
    2
}
//ADC A, A (Z0HC)
fn adc_8f(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.add_a_u8(value, true);
    1
}

//AND A, B (Z010)
fn and_a0(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.and_a_u8(value);
    1
}
//AND A, C (Z010)
fn and_a1(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.and_a_u8(value);
    1
}
//AND A, D (Z010)
fn and_a2(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.and_a_u8(value);
    1
}
//AND A, E (Z010)
fn and_a3(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.and_a_u8(value);
    1
}
//AND A, H (Z010)
fn and_a4(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.and_a_u8(value);
    1
}
//AND A, L (Z010)
fn and_a5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.and_a_u8(value);
    1
}
//AND A, (HL) (Z010)
fn and_a6(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.and_a_u8(value);
    2
}
//AND A, A (Z010)
fn and_a7(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.and_a_u8(value);
    1
}

//XOR A, B (Z000)
fn xor_a8(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.xor_a_u8(value);
    1
}
//XOR A, C (Z000)
fn xor_a9(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.xor_a_u8(value);
    1
}
//XOR A, D (Z000)
fn xor_aa(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.xor_a_u8(value);
    1
}
//XOR A, E (Z000)
fn xor_ab(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.xor_a_u8(value);
    1
}
//XOR A, H (Z000)
fn xor_ac(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.xor_a_u8(value);
    1
}
//XOR A, L (Z000)
fn xor_ad(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.xor_a_u8(value);
    1
}
//XOR A, (HL) (Z000)
fn xor_ae(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.xor_a_u8(value);
    2
}
//XOR A, A (Z000)
fn xor_af(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.xor_a_u8(value);
    1
}

//OR A, B (Z000)
fn or_b0(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.or_a_u8(value);
    1
}
//OR A, C (Z000)
fn or_b1(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.or_a_u8(value);
    1
}
//OR A, D (Z000)
fn or_b2(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.or_a_u8(value);
    1
}
//OR A, E (Z000)
fn or_b3(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.or_a_u8(value);
    1
}
//OR A, H (Z000)
fn or_b4(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.or_a_u8(value);
    1
}
//OR A, L (Z000)
fn or_b5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.or_a_u8(value);
    1
}
//OR A, (HL) (Z000)
fn or_b6(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.or_a_u8(value);
    2
}
//OR A, A (Z000)
fn or_b7(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.or_a_u8(value);
    1
}

//CP A, B (ZIHC)
fn cp_b8(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::B);
    cpu.cp_a_u8(value);
    1
}
//CP A, C (ZIHC)
fn cp_b9(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::C);
    cpu.cp_a_u8(value);
    1
}
//CP A, D (ZIHC)
fn cp_ba(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::D);
    cpu.cp_a_u8(value);
    1
}
//CP A, E (ZIHC)
fn cp_bb(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::E);
    cpu.cp_a_u8(value);
    1
}
//CP A, H (ZIHC)
fn cp_bc(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::H);
    cpu.cp_a_u8(value);
    1
}
//CP A, L (ZIHC)
fn cp_bd(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::L);
    cpu.cp_a_u8(value);
    1
}
//CP A, (HL) (ZIHC)
fn cp_be(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::HL);
    cpu.cp_a_u8(value);
    2
}
//CP A, A (ZIHC)
fn cp_bf(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r8(Registers::A);
    cpu.cp_a_u8(value);
    1
}

//ADD A, u8 (Z0HC)
fn add_c6(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.add_a_u8(value , false);
    2
}
//SUB A, u8 (Z1HC)
fn sub_d6(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.sub_a_u8(value , false);
    2
}
//AND A, u8 (Z010)
fn and_e6(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.and_a_u8(value);
    2
}
//OR A, u8 (Z000)
fn or_f6(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.or_a_u8(value);
    2
}

//ADD SP, i8 (00HC)
fn add_e8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch() as i8 as u16;
    let sp = cpu.get_r16(Registers16::SP);
    let res = sp.wrapping_add(value);
    let set_c = check_h_carry_u16(sp, value);
    let set_h = check_h_carry_u16(sp, value);

    cpu.set_r16(Registers16::SP, res);
    cpu.set_flag(Flags::Z, false);
    cpu.set_flag(Flags::S, false);
    cpu.set_flag(Flags::HC, set_h);
    cpu.set_flag(Flags::C, set_c);
    2
}

//ADC A, u8 (Z0HC)
fn adc_ce(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.add_a_u8(value, true);
    2
}
//SBC A, u8 (Z1HC)
fn sbc_de(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.sub_a_u8(value, true);
    2
}
//XOR A, u8 (Z000)
fn xor_ee(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.xor_a_u8(value);
    2
}
//CP A, u8 (Z1HC)
fn cp_fe(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch();
    cpu.cp_a_u8(value);
    2
}

fn pop_c1(cpu: &mut Cpu) -> u8 {
    let value = cpu.pop();
    cpu.set_r16(Registers16::BC, value);
    3 // Needs checking
}
fn pop_d1(cpu: &mut Cpu) -> u8 {
    let value = cpu.pop();
    cpu.set_r16(Registers16::DE, value);
    3
}
fn pop_e1(cpu: &mut Cpu) -> u8 {
    let value = cpu.pop();
    cpu.set_r16(Registers16::HL, value);
    3
}
fn pop_f1(cpu: &mut Cpu) -> u8 {
    let value = cpu.pop();
    cpu.set_r16(Registers16::AF, value);
    3
}

fn push_c5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r16(Registers16::BC);
    cpu.push(value);
    4
}
fn push_d5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r16(Registers16::DE);
    cpu.push(value);
    4
}
fn push_e5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r16(Registers16::HL);
    cpu.push(value);
    4
}
fn push_f5(cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r16(Registers16::AF);
    cpu.push(value);
    4
}

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_idx = cpu.fetch();
    OPCODES[op_idx as usize](cpu);
    return 0;
}