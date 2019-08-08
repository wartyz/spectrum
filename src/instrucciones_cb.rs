#![allow(non_snake_case)]

/*
Opcode  LR35902            Z-80
------  --------------     ----------
F2      LD   A,(C)         JP  P,nn
E2      LD   (C),A         JP  NV,nn
EA      LD   (nn),A        JP  V,nn
FA      LD   A,(nn)        JP  M,nn
3A      LDD  A,(HL)        LD  A,(nn)
32      LDD  (HL),A        LD  (nn),A
2A      LDI  A,(HL)        LD  HL,(nn)
22      LDI  (HL),A        LD  (nn),HL
08      LD   (nn),SP       EX  AF,AF'
E0      LDH  (n),A         RET NV
F0      LDH  A,(n)         RET P
F8      LD   HL,(SP+e)     RET M
E8      ADD  SP,e          RET V
CB 3x   SWAP r             SL1 r (undocumented)
10      STOP               DJNZ
D9      RETI               EXX


*/
use crate::cpu::{CPU, PROCESADOR};


pub fn mete_funciones_cb(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    // *************************** 1 ***********************************
    cpu.funciones_cb[0x11 as usize] = rl_c;
    cpu.funciones_cb_txt[0x11 as usize] = rl_c_txt;
    // *************************** 2 ***********************************
    // *************************** 3 ***********************************
    // *************************** 4 ***********************************
    // *************************** 5 ***********************************
    // *************************** 6 ***********************************
    // *************************** 7 ***********************************
    cpu.funciones_cb[0x7C as usize] = bit7h;
    cpu.funciones_cb_txt[0x7C as usize] = bit7h_txt;
    // *************************** 8 ***********************************
    // *************************** 9 ***********************************
    // *************************** A ***********************************
    // *************************** B ***********************************
    // *************************** C ***********************************
    // *************************** D ***********************************
    // *************************** E ***********************************
    // *************************** F ***********************************
}


// O = ()  p = '
// *************************** 0 ***********************************
// 0xCB00

pub fn rlc_b(cpu: &mut CPU) {
    cpu.t += 8;
    cpu.pc += 2;
}

pub fn rlc_b_txt(cpu: &mut CPU) {
//    let txt = format!("NOP opcode = #{:02X}", cpu.r0);
//    cpu.texto(&txt);
    cpu.texto(&format!("opcode = #CB{:02X}", cpu.r1));
}

// 0xCB01
pub fn rlc_c(cpu: &mut CPU) {}

// 0xCB02
pub fn rlc_d(cpu: &mut CPU) {}


// 0xCB03
pub fn rlc_e(cpu: &mut CPU) {}


// 0xCB04
pub fn rlc_h(cpu: &mut CPU) {}


// 0xCB05
pub fn rlc_l(cpu: &mut CPU) {}

// 0xCB06
pub fn rlc_OhlO(cpu: &mut CPU) {}

pub fn rlc_a() {}

pub fn rrc_b() {}

pub fn rrc_c() {}

pub fn rrc_d() {}

pub fn rrc_e() {}

pub fn rrc_h() {}

pub fn rrc_l() {}

pub fn rrcOhlO() {}

pub fn rrc_a() {}

// *************************** 1 ***********************************
// 0xCB11     RL C
pub fn rl_c(cpu: &mut CPU) {
    cpu.c = cpu.do_rl_n(cpu.c);

    cpu.pc += 2;

    cpu.t += 8;
}

pub fn rl_c_txt(cpu: &mut CPU) {
    let txt = format!("RL C");
    cpu.texto(&txt);
}

// 0xCB17     RL A
pub fn rl_a(cpu: &mut CPU) {
    cpu.a = cpu.do_rl_n(cpu.a);

    cpu.pc += 2;

    cpu.t += 8;
}

pub fn rl_a_txt(cpu: &mut CPU) {
    let txt = format!("RL A");
    cpu.texto(&txt);
}

// *************************** 2 ***********************************
// *************************** 3 ***********************************
// *************************** 4 ***********************************
// *************************** 5 ***********************************
// *************************** 6 ***********************************
// *************************** 7 ***********************************
// 0xCB7C     BIT 7, H
pub fn bit7h(cpu: &mut CPU) {
    cpu.bit(cpu.h, 7);
    cpu.pc += 2;

    cpu.t += 8;
}

pub fn bit7h_txt(cpu: &mut CPU) {
    let txt = format!("Bit 7,H");
    cpu.texto(&txt);
}

// *************************** 8 ***********************************
// *************************** 9 ***********************************
// *************************** A ***********************************
// *************************** B ***********************************
// *************************** C ***********************************
// *************************** D ***********************************
// *************************** E ***********************************
// *************************** F ***********************************
