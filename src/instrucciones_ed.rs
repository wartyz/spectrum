#![allow(non_snake_case)]

use crate::cpu::CPU;
use crate::instrucciones_basicas::fn_no_impl;


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
    cpu.funciones_ed[0x57 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 9);
    cpu.funciones_ed[0x58 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x59 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 12);
    cpu.funciones_ed[0x5A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 15);
    cpu.funciones_ed[0x5B as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 4, 20);
    cpu.funciones_ed[0x5C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x5D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 14);
    cpu.funciones_ed[0x5E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 8);
    cpu.funciones_ed[0x5F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 2, 9);

    // *************************** 6 ***********************************
    cpu.funciones_ed[0x60 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x61 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x62 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x63 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x64 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x65 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x66 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x67 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x68 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x69 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6B as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6C as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6D as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6E as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x6F as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);

    // *************************** 7 ***********************************
    cpu.funciones_ed[0x70 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x71 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x72 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x73 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x74 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x75 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x76 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x77 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x78 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x79 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7A as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0x7B as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
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
    cpu.funciones_ed[0xB0 as usize].set_punt_y_val_a_fn(ldir, ldir_txt, 2, 18);
    cpu.funciones_ed[0xB1 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB2 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB3 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB4 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB5 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB6 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB7 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xB8 as usize].set_punt_y_val_a_fn(lddr, lddr_txt, 2, 20);
    cpu.funciones_ed[0xB9 as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBA as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBB as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBC as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBD as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBE as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
    cpu.funciones_ed[0xBF as usize].set_punt_y_val_a_fn(fnED_no_impl, fnED_no_impl, 0, 0);
}

pub fn fnED_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion especial ED   #ED{:02X} no implementada", cpu.r1));
}


// *************************** 4 ***********************************
// 0xED40
fn b_OcO(cpu: &mut CPU) { panic!("0x40 funcion ED no implementada"); }

fn b_OcO_txt(cpu: &mut CPU) { panic!("0x40 funcion ED no implementada"); }

// 0xED41
fn out_OcO_b(cpu: &mut CPU) { panic!("0x41 funcion ED no implementada"); }

fn out_OcO_b_txt(cpu: &mut CPU) { panic!("0x41 funcion ED no implementada"); }

// 0xED42
fn sbc_hl_bc(cpu: &mut CPU) { panic!("0x42 funcion ED no implementada"); }

fn sbc_hl_bc_txt(cpu: &mut CPU) { panic!("0x42 funcion ED no implementada"); }

// 0xED43
fn ld_OnnO_bc(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.c);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.b);

    cpu.t += 20;
    cpu.pc += 4;
}

fn ld_OnnO_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),BC", cpu.r2r3)); }

// 0xED44
fn neg(cpu: &mut CPU) { panic!("0x44 funcion ED no implementada"); }

fn neg_txt(cpu: &mut CPU) { panic!("0x44 funcion ED no implementada"); }

// 0xED45
fn retn(cpu: &mut CPU) { panic!("0x45 funcion ED no implementada"); }

fn retn_txt(cpu: &mut CPU) { panic!("0x45 funcion ED no implementada"); }

// 0xED46
fn im_0(cpu: &mut CPU) { panic!("0x46 funcion ED no implementada"); }

fn im_0_txt(cpu: &mut CPU) { panic!("0x46 funcion ED no implementada"); }

// 0xED47
fn ld_i_a(cpu: &mut CPU) {
    cpu.i = cpu.a;

    cpu.t += 9;
    cpu.pc += 2;
}

fn ld_i_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD I,A")); }


fn in_c_OcO(cpu: &mut CPU) { fn_no_impl(cpu); }

fn outOcO_c(cpu: &mut CPU) { fn_no_impl(cpu); }

fn adc_hl_bc(cpu: &mut CPU) { fn_no_impl(cpu); }

// 0xED4B
fn ld_bc_OnnO(cpu: &mut CPU) {
    cpu.c = cpu.mem.lee_byte_de_mem(cpu.r2r3);
    cpu.b = cpu.mem.lee_byte_de_mem(cpu.r2r3 + 1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_bc_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD BC(#{:04X})", cpu.r2r3)); }

//fn neg(){}
fn reti(cpu: &mut CPU) { fn_no_impl(cpu); }

fn im_0_1(cpu: &mut CPU) { fn_no_impl(cpu); }

fn ld_r_a(cpu: &mut CPU) { fn_no_impl(cpu); }


// *************************** 5 ***********************************


// 0xED50
fn in_dOcO() { panic!("0x50 funcion ED no implementada"); }

fn in_dOcO_txt() { panic!("0x50 funcion ED no implementada"); }

// 0xED51
fn out_cOdO() { panic!("0x51 funcion ED no implementada"); }

fn out_cOdO_txt() { panic!("0x51 funcion ED no implementada"); }

// 0xED52
fn sbc_hl_de(cpu: &mut CPU) {
    // TODO: Poner flag H S
    let mut hl = cpu.lee_hl();
    let de = cpu.lee_de();

    if cpu.get_c_flag() {  // C
        hl = hl.wrapping_sub(de.wrapping_add(1));
    } else {
        hl = hl.wrapping_sub(de);
    }
    let hltupla = cpu.desconcatena_un_u16_en_dos_u8(hl);
    cpu.h = hltupla.0;
    cpu.l = hltupla.1;

    // Z
    if hl == 0 {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }

    // C
    if hl < de {
        cpu.set_c_flag();
    } else {
        cpu.reset_c_flag();
    }

    // P/V
    if cpu.overflow_en_resta_u16(hl, de, hl) {
        cpu.set_pv_flag();
    } else {
        cpu.reset_pv_flag();
    }

    // N
    cpu.set_n_flag();

    cpu.t += 15;
    cpu.pc += 2;
}

fn sbc_hl_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SBC HL,DE"));
}

// 0xED53
fn ld_OnnO_de(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.e);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.d);

    cpu.t += 20;
    cpu.pc += 4;
}

fn ld_OnnO_de_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),DE", cpu.r2r3)); }

// 0xED54 REPETIDO
// 0xED55 REPETIDO

// 0xED56
fn im_1(cpu: &mut CPU) {
    cpu.im = 1;

//    cpu.t += 8;
//    cpu.pc += 2;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn im_1_txt(cpu: &mut CPU) { cpu.texto(&format!("IM 1")); }

// 0xED57
fn ld_a_i(cpu: &mut CPU) {
    // TODO: Afecta a flags
    cpu.a = cpu.i;

    cpu.t += 9;
    cpu.pc += 2;
}

fn ld_a_i_txt(cpu: &mut CPU) {
    let txt = format!("LD A,I");
    cpu.texto(&txt);
}

// 0xED58
fn in_e_OcO() { panic!("0x58 funcion ED no implementada"); }

fn in_e_OcO_txt() { panic!("0x58 funcion ED no implementada"); }

// 0x59
fn outOcO_e() { panic!("0x59 funcion ED no implementada"); }

fn outOcO_e_txt() { panic!("0x59 funcion ED no implementada"); }

// 0xED5A
fn adc_hl_de() { panic!("0x5A funcion ED no implementada"); }

fn adc_hl_de_txt() { panic!("0x5A funcion ED no implementada"); }

// 0xED5B
fn ld_de_OnnO(cpu: &mut CPU) {
    cpu.e = cpu.mem.lee_byte_de_mem(cpu.r2r3);
    cpu.d = cpu.mem.lee_byte_de_mem(cpu.r2r3 + 1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

fn ld_de_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD DE(#{:04X})", cpu.r2r3)); }

// 0x5C REPETIDO
// 0x5D REPETIDO

// 0x5E
fn im_2() { panic!("0x5E funcion ED no implementada"); }

fn im_2_txt() { panic!("0x5E funcion ED no implementada"); }

// 0xED5F
fn ld_a_r() { panic!("0x5F funcion ED no implementada"); }

fn ld_a_r_txt() { panic!("0x5F funcion ED no implementada"); }

// *************************** 6 ***********************************
// *************************** 7 ***********************************
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

    let hldec = cpu.inc_16bits(hl);
    let dedec = cpu.inc_16bits(de);
    let bcdec = cpu.dec_16bits(bc);

    let hl_tupla = cpu.desconcatena_un_u16_en_dos_u8(hldec);
    let de_tupla = cpu.desconcatena_un_u16_en_dos_u8(dedec);
    let bc_tupla = cpu.desconcatena_un_u16_en_dos_u8(bcdec);

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


    cpu.reset_n_flag();
    cpu.reset_pv_flag();
    cpu.reset_h_flag();
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

    let hldec = cpu.dec_16bits(hl);
    let dedec = cpu.dec_16bits(de);
    let bcdec = cpu.dec_16bits(bc);

    let hl_tupla = cpu.desconcatena_un_u16_en_dos_u8(hldec);
    let de_tupla = cpu.desconcatena_un_u16_en_dos_u8(dedec);
    let bc_tupla = cpu.desconcatena_un_u16_en_dos_u8(bcdec);

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


    cpu.reset_n_flag();
    cpu.reset_pv_flag();
    cpu.reset_h_flag();
}

fn lddr_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LDDR"));
}
