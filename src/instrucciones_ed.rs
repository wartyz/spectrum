#![allow(non_snake_case)]

use crate::cpu::{CPU, PROCESADOR};


pub fn mete_funciones_ed(cpu: &mut CPU) {

    // *************************** 4 ***********************************
    cpu.funciones_ed[0x43 as usize] = ld_OnnO_bc;
    cpu.funciones_ed_txt[0x43 as usize] = ld_OnnO_bc_txt;
    cpu.funciones_ed[0x47 as usize] = ld_i_a;
    cpu.funciones_ed_txt[0x47 as usize] = ld_i_a_txt;
    // *************************** 5 ***********************************
    cpu.funciones_ed[0x52 as usize] = sbc_hl_de;
    cpu.funciones_ed_txt[0x52 as usize] = sbc_hl_de_txt;
    cpu.funciones_ed[0x53 as usize] = ld_OnnO_de;
    cpu.funciones_ed_txt[0x53 as usize] = ld_OnnO_de_txt;
    // *************************** 6 ***********************************
    // *************************** 7 ***********************************
    // *************************** A ***********************************
    // *************************** B ***********************************
    cpu.funciones_ed[0xB8 as usize] = lddr;
    cpu.funciones_ed_txt[0xB8 as usize] = lddr_txt;
}

pub fn funcionED_no_implementada(cpu: &mut CPU) {
    panic!(format!("Funcion normal ED#{:02X} no implementada", cpu.r1));
}


// *************************** 4 ***********************************
// 0x40
fn b_OcO(cpu: &mut CPU) { panic!("0x40 funcion ED no implementada"); }

fn b_OcO_txt(cpu: &mut CPU) { panic!("0x40 funcion ED no implementada"); }

// 0x41
fn out_OcO_b(cpu: &mut CPU) { panic!("0x41 funcion ED no implementada"); }

fn out_OcO_b_txt(cpu: &mut CPU) { panic!("0x41 funcion ED no implementada"); }

// 0x42
fn sbc_hl_bc(cpu: &mut CPU) { panic!("0x42 funcion ED no implementada"); }

fn sbc_hl_bc_txt(cpu: &mut CPU) { panic!("0x42 funcion ED no implementada"); }

// 0x43
fn ld_OnnO_bc(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.c);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.b);

    cpu.t += 20;
    cpu.pc += 4;
}

fn ld_OnnO_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),BC", cpu.r2r3)); }

// 0x44
fn neg(cpu: &mut CPU) { panic!("0x44 funcion ED no implementada"); }

fn neg_txt(cpu: &mut CPU) { panic!("0x44 funcion ED no implementada"); }

// 0x45
fn retn(cpu: &mut CPU) { panic!("0x45 funcion ED no implementada"); }

fn retn_txt(cpu: &mut CPU) { panic!("0x45 funcion ED no implementada"); }

// 0x46
fn im_0(cpu: &mut CPU) { panic!("0x46 funcion ED no implementada"); }

fn im_0_txt(cpu: &mut CPU) { panic!("0x46 funcion ED no implementada"); }

// 0x47
fn ld_i_a(cpu: &mut CPU) {
    cpu.i = cpu.a;

    cpu.t += 9;
    cpu.pc += 2;
}

fn ld_i_a_txt(cpu: &mut CPU) {
    let txt = format!("LD I,A");
    cpu.texto(&txt);
}


fn in_c_OcO() { panic!("0x48 funcion ED no implementada"); }

fn outOcO_c() { panic!("0x49 funcion ED no implementada"); }

fn adc_hl_bc() { panic!("0x4A funcion ED no implementada"); }

fn ld_bcOnnO() { panic!("0x4B funcion ED no implementada"); }

//fn neg(){}
fn reti() { panic!("0x4D funcion ED no implementada"); }

fn im_0_1() { panic!("0x4E funcion ED no implementada"); }

fn ld_r_a() { panic!("0x4F funcion ED no implementada"); }


// *************************** 5 ***********************************


// 0x50
fn in_dOcO() { panic!("0x50 funcion ED no implementada"); }

fn in_dOcO_txt() { panic!("0x50 funcion ED no implementada"); }

// 0x51
fn out_cOdO() { panic!("0x51 funcion ED no implementada"); }

fn out_cOdO_txt() { panic!("0x51 funcion ED no implementada"); }

// 0x52
fn sbc_hl_de(cpu: &mut CPU) {
    // TODO: Poner todos los flags
    let mut hl = ((cpu.h as u16) << 8) | cpu.l as u16;
    let de = ((cpu.d as u16) << 8) | cpu.e as u16;

    if cpu.get_c_flag() {
        hl = hl.wrapping_sub(de.wrapping_add(1));
    } else {
        hl = hl.wrapping_sub(de);
    }

    cpu.t += 15;
    cpu.pc += 2;
}

fn sbc_hl_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SBC HL,DE"));
}

// 0x53
fn ld_OnnO_de(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r2r3, cpu.e);
    cpu.mem.escribe_byte_en_mem(cpu.r2r3 + 1, cpu.d);

    cpu.t += 20;
    cpu.pc += 4;
}

fn ld_OnnO_de_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(#{:04X}),DE", cpu.r2r3)); }

// 0x54 REPETIDO
// 0x55 REPETIDO

// 0x56
fn im_1() { panic!("0x56 funcion ED no implementada"); }

fn im_1_txt() { panic!("0x56 funcion ED no implementada"); }

// 0x57
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

// 0x58
fn in_e_OcO() { panic!("0x58 funcion ED no implementada"); }

fn in_e_OcO_txt() { panic!("0x58 funcion ED no implementada"); }

// 0x59
fn outOcO_e() { panic!("0x59 funcion ED no implementada"); }

fn outOcO_e_txt() { panic!("0x59 funcion ED no implementada"); }

// 0x5A
fn adc_hl_de() { panic!("0x5A funcion ED no implementada"); }

fn adc_hl_de_txt() { panic!("0x5A funcion ED no implementada"); }

// 0x5B
fn ld_deOnnO() { panic!("0x5B funcion ED no implementada"); }

fn ld_deOnnO_txt() { panic!("0x5B funcion ED no implementada"); }

// 0x5C REPETIDO
// 0x5D REPETIDO

// 0x5E
fn im_2() { panic!("0x5E funcion ED no implementada"); }

fn im_2_txt() { panic!("0x5E funcion ED no implementada"); }

// 0x5F
fn ld_a_r() { panic!("0x5F funcion ED no implementada"); }

fn ld_a_r_txt() { panic!("0x5F funcion ED no implementada"); }

// *************************** 6 ***********************************
// *************************** 7 ***********************************
// *************************** A ***********************************
// *************************** B ***********************************
// 0xB8
fn lddr(cpu: &mut CPU) {
    cpu.pc += 2;
    let hldir = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let origen = cpu.mem.lee_byte_de_mem(hldir);

    let dedir = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    cpu.mem.escribe_byte_en_mem(dedir, origen);

    let hl = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let de = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    let bc = cpu.concatena_dos_u8_en_un_u16(cpu.b, cpu.c);

    let hldec = cpu.dec_16bits(hl);
    let dedec = cpu.dec_16bits(de);
    let bcdec = cpu.dec_16bits(bc);

    let hl = cpu.desconcatena_un_u16_en_dos_u8(hldec);
    let de = cpu.desconcatena_un_u16_en_dos_u8(dedec);
    let bc = cpu.desconcatena_un_u16_en_dos_u8(bcdec);

    cpu.h = hl.0;
    cpu.l = hl.1;
    cpu.d = de.0;
    cpu.e = de.1;
    cpu.b = bc.0;
    cpu.c = bc.1;


    if (cpu.b == 0) & (cpu.c == 0) {
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
