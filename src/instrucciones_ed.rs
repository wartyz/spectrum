#![allow(non_snake_case)]

use crate::cpu::CPU;
use crate::operaciones_binarias::*;
use crate::constantes::*;

pub fn mete_funciones_ed(cpu: &mut CPU) {

    // *************************** 4 ***********************************
    cpu.funciones_ed[0x40 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x41 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x42 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x43 as usize].set_punt_y_val_a_fn(ld_OnnO_bc, ld_OnnO_bc_txt, 4, 20);
    cpu.funciones_ed[0x44 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x45 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x46 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x47 as usize].set_punt_y_val_a_fn(ld_i_a, ld_i_a_txt, 2, 9);
    cpu.funciones_ed[0x48 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x49 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x4A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x4B as usize].set_punt_y_val_a_fn(ld_bc_OnnO, ld_bc_OnnO_txt, 4, 20);
    cpu.funciones_ed[0x4C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x4D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x4E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x4F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 9);

    // *************************** 5 ***********************************
    cpu.funciones_ed[0x50 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x51 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x52 as usize].set_punt_y_val_a_fn(sbc_hl_de, sbc_hl_de_txt, 2, 15);
    cpu.funciones_ed[0x53 as usize].set_punt_y_val_a_fn(ld_OnnO_de, ld_OnnO_de_txt, 4, 20);
    cpu.funciones_ed[0x54 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x55 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x56 as usize].set_punt_y_val_a_fn(im_1, im_1_txt, 2, 8);
    cpu.funciones_ed[0x57 as usize].set_punt_y_val_a_fn(ld_a_i, ld_a_i_txt, 2, 9);
    cpu.funciones_ed[0x58 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x59 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x5A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x5B as usize].set_punt_y_val_a_fn(ld_de_OnnO, ld_de_OnnO_txt, 4, 20);
    cpu.funciones_ed[0x5C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x5D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x5E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x5F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 9);

    // *************************** 6 ***********************************
    cpu.funciones_ed[0x60 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x61 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x62 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x63 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 4, 20);
    cpu.funciones_ed[0x64 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x65 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x66 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x67 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 18);
    cpu.funciones_ed[0x68 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x69 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x6A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x6B as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 4, 20);
    cpu.funciones_ed[0x6C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x6D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x6E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x6F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 18);

    // *************************** 7 ***********************************
    cpu.funciones_ed[0x70 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x71 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x72 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x73 as usize].set_punt_y_val_a_fn(ld_OnnO_sp, ld_OnnO_sp_txt, 4, 20);
    cpu.funciones_ed[0x74 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x75 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x76 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x77 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x78 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x79 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7B as usize].set_punt_y_val_a_fn(ld_sp_OnnO, ld_sp_OnnO_txt, 4, 20);
    cpu.funciones_ed[0x7C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    // *************************** A ***********************************
    cpu.funciones_ed[0xA0 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA1 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA2 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA3 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA4 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA5 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA6 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA7 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA8 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xA9 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAA as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAB as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAC as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAD as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAE as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xAF as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    // *************************** B ***********************************
    cpu.funciones_ed[0xB0 as usize].set_punt_y_val_a_fn(ldir, ldir_txt, 2, 16);
    cpu.funciones_ed[0xB1 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB2 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB3 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB4 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB5 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB6 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB7 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB8 as usize].set_punt_y_val_a_fn(lddr, lddr_txt, 2, 16);
    cpu.funciones_ed[0xB9 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBA as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBB as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBC as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBD as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBE as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBF as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
}

pub fn fnED_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion ED no implementada\n\
    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
    r3 = #{:02X}\n",
                   cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3));
}

// Funciones auxiliares bas
// ld Q,(A) 	11101101 01qq1011 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	-
// Q := (A) [ld hl,(A) has a faster non-prefixed duplicate, see below.]
pub fn bas_ld_Q_OAO(cpu: &mut CPU) {
    let dato1 = cpu.mem.lee_byte_de_mem(cpu.r2r3);
    let dato2 = cpu.mem.lee_byte_de_mem(cpu.r2r3 + 1);
    match cpu.r1 & 0b00_11_0000 {
        0b00_00_0000 => {
            cpu.c = dato1;
            cpu.b = dato2;
        }
        0b00_01_0000 => {
            cpu.e = dato1;
            cpu.d = dato2;
        }
        0b00_10_0000 => {
            cpu.l = dato1;
            cpu.h = dato2;
        }
        0b00_11_0000 => {
            cpu.sp = concatena_dos_u8_en_un_u16(dato2, dato1);
        }
        _ => panic!("Instruccion en bas_ld_Q_OAO no reconocida"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// ld (A),Q 	11101101 01qq0011 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	-
// (A) := Q [ld (A),hl has a faster non-prefixed duplicate, see below.]
pub fn bas_ld_OAO_Q(cpu: &mut CPU) {
    match cpu.r1 & 0b00_11_0000 {
        0b00_00_0000 => {
            cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.c);
            cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.b);
        }
        0b00_01_0000 => {
            cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.e);
            cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.d);
        }
        0b00_10_0000 => {
            cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.l);
            cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.h);
        }
        0b00_11_0000 => {
            let sp = desconcatena_un_u16_en_dos_u8(cpu.sp);
            cpu.mem.escribe_byte_en_mem(cpu.r2r3, sp.1);
            cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, sp.0);
        }
        _ => panic!("Instruccion en bas_ld_OAO_Q no reconocida"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// *************************** 4 ***********************************
// 0xED40
fn b_OcO(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn b_OcO_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED41
fn out_OcO_b(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn out_OcO_b_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED42
fn sbc_hl_bc(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn sbc_hl_bc_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED43
fn ld_OnnO_bc(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.c);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.b);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_OnnO_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),BC", cpu.r2r3)); }

// 0xED44
fn neg(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn neg_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED45
fn retn(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn retn_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED46
fn im_0(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn im_0_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED47
fn ld_i_a(cpu: &mut CPU) {
//    let t = cpu.get_t_instruccion();
//    let by = cpu.get_bytes_instruccion();
//    panic!("PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}   r3 = #{:02X}\
//    t={} bytes={}",
//           cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3,
//           t, by
//    );


    cpu.i = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_i_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD I,A")); }


fn in_c_OcO(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn outOcO_c(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn adc_hl_bc(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED4B
fn ld_bc_OnnO(cpu: &mut CPU) {
    cpu.c = cpu.mem.lee_byte_de_mem(cpu.r2r3);
    cpu.b = cpu.mem.lee_byte_de_mem(cpu.r2r3 + 1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_bc_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD BC(#{:04X})", cpu.r2r3)); }

//fn neg(){}
fn reti(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn im_0_1(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn ld_r_a(cpu: &mut CPU) { fnED_no_impl(cpu); }


// *************************** 5 ***********************************


// 0xED50
fn in_dOcO(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn in_dOcO_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED51
fn out_cOdO(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn out_cOdO_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED52
// The contents of the register pair DE and the Carry Flag (C flag in the F Register) are
// subtracted from the contents of register pair HL, and the result is stored in HL.
fn sbc_hl_de(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let de = cpu.lee_de();
    let mut resultado: u16 = 0;

    if cpu.get_c_flag() {
        resultado = hl.wrapping_sub(de.wrapping_add(1));
    } else {
        resultado = hl.wrapping_sub(de);
    }

    cpu.set_flag(FLAG_S, (resultado & 0x8000) != 0);
    cpu.set_flag(FLAG_Z, resultado == 0); // Z
    cpu.set_flag(FLAG_C, hl < de); // C
    cpu.set_flag(FLAG_PV, overflow_en_resta_u16(hl, de, resultado));
    cpu.set_flag(FLAG_N, true);


    // FLAG H en resta **********************************************
    // H como es resta lo hago con resultado y sustraendo
    //cpu.set_flag(FLAG_H, (resultado & 0x0FFF) + (de & 0x0FFF) > 0x0FFF);
    // var cfVal = _registers.CFlag ? 1 : 0;
    // flags |= (byte)((((_registers.HL & 0x0FFF) - (_registers[qq] & 0x0FFF) - cfVal) >> 8) & FlagsSetMask.H);

//    let hl_0fff = hl & 0x0FFF;
////    let de_0fff = de & 0x0FFF;
////    let hl_de_resta = hl_0fff.wrapping_sub(de_0fff);
////
////    if cpu.get_c_flag() {
////        cpu.set_flag(FLAG_H, (hl_de_resta.wrapping_sub(1)) > 0x0FFF);
////    } else {
////        cpu.set_flag(FLAG_H, (hl_de_resta > 0x0FFF));
////    }

    // EXPERIMENTO: en 8 bits: bit 4 de  X(antes)  XOR  bit 4 de resultado
    // lo hago con el bit 12

    cpu.set_flag(FLAG_H, ((hl ^ resultado) & 0x800) != 0);
    // FIN FLAG H en resta *********************************************


    let hltupla = desconcatena_un_u16_en_dos_u8(resultado);
    cpu.h = hltupla.0;
    cpu.l = hltupla.1;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn sbc_hl_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SBC HL,DE"));
}

// 0xED53
fn ld_OnnO_de(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.e);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.d);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_OnnO_de_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),DE", cpu.r2r3)); }

// 0xED54 REPETIDO
// 0xED55 REPETIDO

// 0xED56
fn im_1(cpu: &mut CPU) {
    cpu.im = 1;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn im_1_txt(cpu: &mut CPU) { cpu.texto(&format!("IM 1")); }

// 0xED57
fn ld_a_i(cpu: &mut CPU) {
    // TODO: Afecta a flags
    cpu.set_flag(FLAG_S, (cpu.i & 0x80) != 0);
    cpu.set_flag(FLAG_Z, cpu.i == 0);
    cpu.set_flag(FLAG_H, false);
    // TODO P/V contains contents of IFF2. ???????
    //cpu.set_flag(FLAG_PV, );
    cpu.set_flag(FLAG_N, false);

    cpu.a = cpu.i;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_a_i_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,I")); }

// 0xED58
fn in_e_OcO(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn in_e_OcO_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0x59
fn outOcO_e(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn outOcO_e_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED5A
fn adc_hl_de(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn adc_hl_de_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED5B
fn ld_de_OnnO(cpu: &mut CPU) { bas_ld_Q_OAO(cpu); }

fn ld_de_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD DE(#{:04X})", cpu.r2r3)); }

// 0x5C REPETIDO
// 0x5D REPETIDO

// 0x5E
fn im_2(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn im_2_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// 0xED5F
fn ld_a_r(cpu: &mut CPU) { fnED_no_impl(cpu); }

fn ld_a_r_txt(cpu: &mut CPU) { fnED_no_impl(cpu); }

// *************************** 6 ***********************************
// *************************** 7 ***********************************
// 0xED73
pub fn ld_OnnO_sp(cpu: &mut CPU) { bas_ld_OAO_Q(cpu); }

pub fn ld_OnnO_sp_txt(cpu: &mut CPU) { cpu.texto(&format!("LD (#{:04X}),SP", cpu.r2r3)); }

// 0xED7B
pub fn ld_sp_OnnO(cpu: &mut CPU) {
    let sp = cpu.mem.lee_2bytes_de_mem(cpu.r2r3);
    cpu.sp = sp;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_sp_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD SP(#{:04X})", cpu.r2r3)); }

// *************************** A ***********************************
// *************************** B ***********************************
// 0xEDB0
pub fn ldir(cpu: &mut CPU) {
    cpu.pc += 2;

    let hl = cpu.lee_hl();
    let de = cpu.lee_de();
    let bc = cpu.lee_bc();

    let dato = cpu.mem.lee_byte_de_mem(hl);

    cpu.mem.escribe_byte_en_mem(de, dato);

    let hlinc = hl.wrapping_add(1);
    let deinc = de.wrapping_add(1);
    let bcdec = bc.wrapping_sub(1);

    let hl_tupla = desconcatena_un_u16_en_dos_u8(hlinc);
    let de_tupla = desconcatena_un_u16_en_dos_u8(deinc);
    let bc_tupla = desconcatena_un_u16_en_dos_u8(bcdec);

    cpu.h = hl_tupla.0;
    cpu.l = hl_tupla.1;
    cpu.d = de_tupla.0;
    cpu.e = de_tupla.1;
    cpu.b = bc_tupla.0;
    cpu.c = bc_tupla.1;


    if bcdec == 0 {
        cpu.t += 16;
    } else {
        cpu.pc -= 2;
        cpu.t += 21;
    }

    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_PV, false);
    cpu.set_flag(FLAG_H, false);
}

pub fn ldir_txt(cpu: &mut CPU) { cpu.texto(&format!("LDIR")); }

// 0xEDB8
pub fn lddr(cpu: &mut CPU) {
    cpu.pc += 2;

    let hl = cpu.lee_hl();
    let de = cpu.lee_de();
    let bc = cpu.lee_bc();

    let dato = cpu.mem.lee_byte_de_mem(hl);

    cpu.mem.escribe_byte_en_mem(de, dato);

    let hldec = hl.wrapping_sub(1);
    let dedec = de.wrapping_sub(1);
    let bcdec = bc.wrapping_sub(1);

    let hl_tupla = desconcatena_un_u16_en_dos_u8(hldec);
    let de_tupla = desconcatena_un_u16_en_dos_u8(dedec);
    let bc_tupla = desconcatena_un_u16_en_dos_u8(bcdec);

    cpu.h = hl_tupla.0;
    cpu.l = hl_tupla.1;
    cpu.d = de_tupla.0;
    cpu.e = de_tupla.1;
    cpu.b = bc_tupla.0;
    cpu.c = bc_tupla.1;


    if bcdec == 0 {
        cpu.t += 16;
    } else {
        cpu.pc -= 2;
        cpu.t += 21;
    }

    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_PV, false);
    cpu.set_flag(FLAG_H, false);
}

fn lddr_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LDDR"));
}
