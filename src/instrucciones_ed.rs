#![allow(non_snake_case)]

use crate::cpu::{CPU, PROCESADOR};


pub fn mete_funciones_ed(cpu: &mut CPU) {

    // *************************** 4 ***********************************
    cpu.funciones_ed[0x47 as usize] = ld_i_a;
    cpu.funciones_ed_txt[0x47 as usize] = ld_i_a_txt;
    // *************************** 5 ***********************************
    cpu.funciones_ed[0x52 as usize] = sbc_hl_de;
    cpu.funciones_ed_txt[0x52 as usize] = sbc_hl_de_txt;
}


// funcion metida por defecto en el arreglo de punteros a funciones ed
pub fn nopED(cpu: &mut CPU) {
    panic!("Instrucción ED no implementada");
}

pub fn nopED_txt(cpu: &mut CPU) {
    panic!("Instrucción ED no implementada");
}

// *************************** 4 ***********************************
// 0x40
fn b_OcO() { panic!("0x40 funcion ED no implementada"); }

fn b_OcO_txt() { panic!("0x40 funcion ED no implementada"); }

// 0x41
fn out_OcO_b() { panic!("0x41 funcion ED no implementada"); }

fn out_OcO_b_txt() { panic!("0x41 funcion ED no implementada"); }

// 0x42
fn sbc_hl_bc() { panic!("0x42 funcion ED no implementada"); }

fn sbc_hl_bc_txt() { panic!("0x42 funcion ED no implementada"); }

// 0x43
fn ld_OnnO_bc() { panic!("0x43 funcion ED no implementada"); }

fn ld_OnnO_bc_txt() { panic!("0x43 funcion ED no implementada"); }

// 0x44
fn neg() { panic!("0x44 funcion ED no implementada"); }

fn neg_txt() { panic!("0x44 funcion ED no implementada"); }

// 0x45
fn retn() { panic!("0x45 funcion ED no implementada"); }

fn retn_txt() { panic!("0x45 funcion ED no implementada"); }

// 0x46
fn im_0() { panic!("0x46 funcion ED no implementada"); }

fn im_0_txt() { panic!("0x46 funcion ED no implementada"); }

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
fn ld_OnnO_de() { panic!("0x53 funcion ED no implementada"); }

fn ld_OnnO_de_txt() { panic!("0x53 funcion ED no implementada"); }

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

