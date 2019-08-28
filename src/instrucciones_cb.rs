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
use crate::cpu::CPU;
use super::constantes::*;

pub fn mete_funciones_cb(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones_cb[0x00 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x01 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x02 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x03 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x04 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x05 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x06 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 15);
    cpu.funciones_cb[0x07 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x08 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x09 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x0A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x0B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x0C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x0D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x0E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 15);
    cpu.funciones_cb[0x0F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);

    // *************************** 1 ***********************************
    cpu.funciones_cb[0x10 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x11 as usize].set_punt_y_val_a_fn(rl_c, rl_c_txt, 2, 8);
    cpu.funciones_cb[0x12 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x13 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x14 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x15 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x16 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x17 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x18 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x19 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x1F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);

    // *************************** 2 ***********************************
    cpu.funciones_cb[0x20 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x21 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x22 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x23 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x24 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x25 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x26 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x27 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x28 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x29 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x2F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** 3 ***********************************
    cpu.funciones_cb[0x30 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x31 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x32 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x33 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x34 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x35 as usize].set_punt_y_val_a_fn(sll_l, sll_l_txt, 2, 8);
    cpu.funciones_cb[0x36 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x37 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x38 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x39 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x3F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** 4 ***********************************
    cpu.funciones_cb[0x40 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x41 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x42 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x43 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x44 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x45 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x46 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x47 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x48 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x49 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x4F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** 5 ***********************************
    cpu.funciones_cb[0x50 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x51 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x52 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x53 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x54 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x55 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x56 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x57 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x58 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x59 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x5F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** 6 ***********************************
    cpu.funciones_cb[0x60 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x61 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x62 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x63 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x64 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x65 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x66 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x67 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x68 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x69 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x6F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** 7 ***********************************
    cpu.funciones_cb[0x70 as usize].set_punt_y_val_a_fn(bit_6_b, bit_6_b_txt, 2, 8);
    cpu.funciones_cb[0x71 as usize].set_punt_y_val_a_fn(bit_6_c, bit_6_c_txt, 2, 8);
    cpu.funciones_cb[0x72 as usize].set_punt_y_val_a_fn(bit_6_d, bit_6_d_txt, 2, 8);
    cpu.funciones_cb[0x73 as usize].set_punt_y_val_a_fn(bit_6_e, bit_6_e_txt, 2, 8);
    cpu.funciones_cb[0x74 as usize].set_punt_y_val_a_fn(bit_6_h, bit_6_h_txt, 2, 8);
    cpu.funciones_cb[0x75 as usize].set_punt_y_val_a_fn(bit_6_l, bit_6_l_txt, 2, 8);
    cpu.funciones_cb[0x76 as usize].set_punt_y_val_a_fn(bit_6_OhlO, bit_6_OhlO_txt, 2, 12);
    cpu.funciones_cb[0x77 as usize].set_punt_y_val_a_fn(bit_6_a, bit_6_a_txt, 2, 8);
    cpu.funciones_cb[0x78 as usize].set_punt_y_val_a_fn(bit_7_b, bit_7_b_txt, 2, 8);
    cpu.funciones_cb[0x79 as usize].set_punt_y_val_a_fn(bit_7_c, bit_7_c_txt, 2, 8);
    cpu.funciones_cb[0x7A as usize].set_punt_y_val_a_fn(bit_7_d, bit_7_d_txt, 2, 8);
    cpu.funciones_cb[0x7B as usize].set_punt_y_val_a_fn(bit_7_e, bit_7_e_txt, 2, 8);
    cpu.funciones_cb[0x7C as usize].set_punt_y_val_a_fn(bit_7_h, bit_7_h_txt, 2, 8);
    cpu.funciones_cb[0x7D as usize].set_punt_y_val_a_fn(bit_7_l, bit_7_l_txt, 2, 8);
    cpu.funciones_cb[0x7E as usize].set_punt_y_val_a_fn(bit_7_OhlO, bit_7_OhlO_txt, 2, 12);
    cpu.funciones_cb[0x7F as usize].set_punt_y_val_a_fn(bit_7_a, bit_7_a_txt, 2, 8);

    // *************************** 8 ***********************************
    cpu.funciones_cb[0x80 as usize].set_punt_y_val_a_fn(res_0_b, res_0_b_txt, 2, 8);
    cpu.funciones_cb[0x81 as usize].set_punt_y_val_a_fn(res_0_c, res_0_c_txt, 2, 8);
    cpu.funciones_cb[0x82 as usize].set_punt_y_val_a_fn(res_0_d, res_0_d_txt, 2, 8);
    cpu.funciones_cb[0x83 as usize].set_punt_y_val_a_fn(res_0_e, res_0_e_txt, 2, 8);
    cpu.funciones_cb[0x84 as usize].set_punt_y_val_a_fn(res_0_h, res_0_h_txt, 2, 8);
    cpu.funciones_cb[0x85 as usize].set_punt_y_val_a_fn(res_0_l, res_0_l_txt, 2, 8);
    cpu.funciones_cb[0x86 as usize].set_punt_y_val_a_fn(res_0_OhlO, res_0_OhlO_txt, 2, 15);
    cpu.funciones_cb[0x87 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x88 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x89 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x8A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x8B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x8C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x8D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    cpu.funciones_cb[0x8E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 15);
    cpu.funciones_cb[0x8F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 2, 8);
    // *************************** 9 ***********************************
    cpu.funciones_cb[0x90 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x91 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x92 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x93 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x94 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x95 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x96 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x97 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x98 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x99 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9A as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9B as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9C as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9D as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9E as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0x9F as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** A ***********************************
    cpu.funciones_cb[0xA0 as usize].set_punt_y_val_a_fn(res_4_b, res_4_b_txt, 2, 8);
    cpu.funciones_cb[0xA1 as usize].set_punt_y_val_a_fn(res_4_c, res_4_c_txt, 2, 8);
    cpu.funciones_cb[0xA2 as usize].set_punt_y_val_a_fn(res_4_d, res_4_d_txt, 2, 8);
    cpu.funciones_cb[0xA3 as usize].set_punt_y_val_a_fn(res_4_e, res_4_e_txt, 2, 8);
    cpu.funciones_cb[0xA4 as usize].set_punt_y_val_a_fn(res_4_h, res_4_h_txt, 2, 8);
    cpu.funciones_cb[0xA5 as usize].set_punt_y_val_a_fn(res_4_l, res_4_l_txt, 2, 8);
    cpu.funciones_cb[0xA6 as usize].set_punt_y_val_a_fn(res_4_OhlO, res_4_OhlO_txt, 2, 15);
    cpu.funciones_cb[0xA7 as usize].set_punt_y_val_a_fn(res_4_a, res_4_a_txt, 2, 8);
    cpu.funciones_cb[0xA8 as usize].set_punt_y_val_a_fn(res_5_b, res_5_b_txt, 2, 8);
    cpu.funciones_cb[0xA9 as usize].set_punt_y_val_a_fn(res_5_c, res_5_c_txt, 2, 8);
    cpu.funciones_cb[0xAA as usize].set_punt_y_val_a_fn(res_5_d, res_5_d_txt, 2, 8);
    cpu.funciones_cb[0xAB as usize].set_punt_y_val_a_fn(res_5_e, res_5_e_txt, 2, 8);
    cpu.funciones_cb[0xAC as usize].set_punt_y_val_a_fn(res_5_h, res_5_h_txt, 2, 8);
    cpu.funciones_cb[0xAD as usize].set_punt_y_val_a_fn(res_5_l, res_5_l_txt, 2, 8);
    cpu.funciones_cb[0xAE as usize].set_punt_y_val_a_fn(res_5_OhlO, res_5_OhlO_txt, 2, 15);
    cpu.funciones_cb[0xAF as usize].set_punt_y_val_a_fn(res_5_a, res_5_a_txt, 2, 8);
    // *************************** B ***********************************
    cpu.funciones_cb[0xB0 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB1 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB2 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB3 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB4 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB5 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB6 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB7 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB8 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xB9 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBA as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBB as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBC as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBD as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBE as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xBF as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** C ***********************************
    cpu.funciones_cb[0xC0 as usize].set_punt_y_val_a_fn(set_0_b, set_0_b_txt, 2, 8);
    cpu.funciones_cb[0xC1 as usize].set_punt_y_val_a_fn(set_0_c, set_0_c_txt, 2, 8);
    cpu.funciones_cb[0xC2 as usize].set_punt_y_val_a_fn(set_0_d, set_0_d_txt, 2, 8);
    cpu.funciones_cb[0xC3 as usize].set_punt_y_val_a_fn(set_0_e, set_0_e_txt, 2, 8);
    cpu.funciones_cb[0xC4 as usize].set_punt_y_val_a_fn(set_0_h, set_0_h_txt, 2, 8);
    cpu.funciones_cb[0xC5 as usize].set_punt_y_val_a_fn(set_0_l, set_0_l_txt, 2, 8);
    cpu.funciones_cb[0xC6 as usize].set_punt_y_val_a_fn(set_0_OhlO, set_0_OhlO_txt, 2, 15);
    cpu.funciones_cb[0xC7 as usize].set_punt_y_val_a_fn(set_0_a, set_0_a_txt, 2, 8);
    cpu.funciones_cb[0xC8 as usize].set_punt_y_val_a_fn(set_1_b, set_1_b_txt, 2, 8);
    cpu.funciones_cb[0xC9 as usize].set_punt_y_val_a_fn(set_1_c, set_1_c_txt, 2, 8);
    cpu.funciones_cb[0xCA as usize].set_punt_y_val_a_fn(set_1_d, set_1_d_txt, 2, 8);
    cpu.funciones_cb[0xCB as usize].set_punt_y_val_a_fn(set_1_e, set_1_e_txt, 2, 8);
    cpu.funciones_cb[0xCC as usize].set_punt_y_val_a_fn(set_1_h, set_1_h_txt, 2, 8);
    cpu.funciones_cb[0xCD as usize].set_punt_y_val_a_fn(set_1_l, set_1_l_txt, 2, 8);
    cpu.funciones_cb[0xCE as usize].set_punt_y_val_a_fn(set_1_OhlO, set_1_OhlO_txt, 2, 15);
    cpu.funciones_cb[0xCF as usize].set_punt_y_val_a_fn(set_1_a, set_1_a_txt, 2, 8);
    // *************************** D ***********************************
    cpu.funciones_cb[0xD0 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD1 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD2 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD3 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD4 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD5 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD6 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD7 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD8 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xD9 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDA as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDB as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDC as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDD as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDE as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xDF as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** E ***********************************
    cpu.funciones_cb[0xE0 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE1 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE2 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE3 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE4 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE5 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE6 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE7 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE8 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xE9 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xEA as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xEB as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xEC as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xED as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xEE as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xEF as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    // *************************** F ***********************************
    cpu.funciones_cb[0xF0 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF1 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF2 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF3 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF4 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF5 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF6 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF7 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF8 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xF9 as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFA as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFB as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFC as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFD as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFE as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
    cpu.funciones_cb[0xFF as usize].set_punt_y_val_a_fn(fnCB_no_impl, fnCB_no_impl, 0, 0);
}

pub fn fnCB_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion normal no implementada\n\
    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
    r3 = #{:02X}\n",
                   cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3));
}


// funcion metida por defecto en el arreglo de punteros a funciones cb
pub fn nopCB(cpu: &mut CPU) {
    panic!("Instrucción CB no implementada");
}

pub fn nopCB_txt(cpu: &mut CPU) {
    panic!("Instrucción CB no implementada");
}

// XXXXXXXXXXXXXXXXXXX Funciones comunes en instrucciones cb XXXXXXXXXXXXXXXXXXXX
// https://wikiti.brandonw.net/index.php?title=Z80_Instruction_Set
// Cuando varias funciones en los arreglos de punteros, tienen opciones comunes
// usan estas funciones, solo se tocaran los flags en estas funciones


// bit B,R 	11001011 01bbbrrr 	8 	+ 	+ 	+ 	1 	+ 	P 	0 	- 	tmp := R AND [1 << B]
pub fn bas_bit_B_R(cpu: &mut CPU, valor: u8, bit: u8) {
//    if valor & (1 << bit) == 0 {
//        cpu.set_z_flag();
//    } else {
//        cpu.reset_z_flag();
//    }
    cpu.set_flag(FLAG_Z, valor & (1 << bit) == 0);

    //cpu.reset_n_flag();
    cpu.set_flag(FLAG_N, false);
    //cpu.set_h_flag();
    cpu.set_flag(FLAG_H, true);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}


// res B,R 	11001011 10bbbrrr 	8 	- 	- 	- 	- 	- 	- 	- 	- 	R := R AND ~[1 << B]
pub fn bas_res_B_R(cpu: &mut CPU, valor: u8, bit: u8) -> u8 {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();

    valor & !(1u8 << bit)
}

// set B,R 	11001011 11bbbrrr 	8 	- 	- 	- 	- 	- 	- 	- 	- 	R := R OR [1 << B]
pub fn bas_set_B_R(cpu: &mut CPU, valor: u8, bit: u8) -> u8 {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();

    valor | (1u8 << bit)
}

// O = ()  p = '
// *************************** 0 ***********************************
/// 0xCB00   "rlc b"
/// The contents of register B are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0x00
/// =================================
/// T-States: 8
pub fn rlc_b(cpu: &mut CPU) {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rlc_b_txt(cpu: &mut CPU) { cpu.texto(&format!("opcode = #CB{:02X}", cpu.r1)); }

/// 0xCB01   "rlc c"
/// The contents of register C are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0x01
/// =================================
/// T-States:8
pub fn rlc_c(cpu: &mut CPU) {}

pub fn rlc_c_txt(cpu: &mut CPU) {}

/// 0xCB02   "rlc d"
/// The contents of register D are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0x02
/// =================================
/// T-States: 8
pub fn rlc_d(cpu: &mut CPU) {}

pub fn rlc_d_txt(cpu: &mut CPU) {}


/// 0xCB03   "rlc e"
/// The contents of register E are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0x03
/// =================================
/// T-States: 8
pub fn rlc_e(cpu: &mut CPU) {}

pub fn rlc_e_txt(cpu: &mut CPU) {}


/// 0xCB04   "rlc h"
/// The contents of register H are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0x04
/// =================================
/// T-States: 8
pub fn rlc_h(cpu: &mut CPU) {}

pub fn rlc_h_txt(cpu: &mut CPU) {}

/// 0xCB05   "rlc l"
/// The contents of register L are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0x05
/// =================================
/// T-States: 8
pub fn rlc_l(cpu: &mut CPU) {}

pub fn rlc_l_txt(cpu: &mut CPU) {}

/// 0xCB06   "rlc (hl)"
/// The contents of the memory address specified by the contents
/// of HL are rotated left 1 bit position.The contents of bit 7
/// are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the source byte.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0x06
/// =================================
/// T-States: 15
pub fn rlc_OhlO(cpu: &mut CPU) {}

pub fn rlc_OhlO_txt(cpu: &mut CPU) {}

/// 0xCB07   "rlc a"
/// The contents of register A are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag and also to bit 0.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 0x07
/// =================================
/// T-States: 8
pub fn rlc_a(cpu: &mut CPU) {}

pub fn rlc_a_txt(cpu: &mut CPU) {}

/// 0xCB08   "rrc b"
/// The contents of register B are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0x08
/// =================================
/// T-States: 8
pub fn rrc_b(cpu: &mut CPU) {}

pub fn rrc_b_txt(cpu: &mut CPU) {}

/// 0xCB09   "rrc c"
/// The contents of register C are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0x09
/// =================================
/// T-States: 8
pub fn rrc_c(cpu: &mut CPU) {}

pub fn rrc_c_txt(cpu: &mut CPU) {}

/// 0xCD0A   "rrc d" operation
/// The contents of register D are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0x0A
/// =================================
/// T-States: 4, 4 (8)
pub fn rrc_d(cpu: &mut CPU) {}

pub fn rrc_d_txt(cpu: &mut CPU) {}

/// 0xCB0B   "rrc e" operation
/// The contents of register E are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 0x0B
/// =================================
/// T-States: 8
pub fn rrc_e(cpu: &mut CPU) {}

pub fn rrc_e_txt(cpu: &mut CPU) {}

/// 0xCB0C   "rrc h"
/// The contents of register H are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0x0C
/// =================================
/// T-States: 8
pub fn rrc_h(cpu: &mut CPU) {}

pub fn rrc_h_txt(cpu: &mut CPU) {}

/// 0xCB0D   "rrc l"
/// The contents of register L are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0x0D
/// =================================
/// T-States: 8
pub fn rrc_l(cpu: &mut CPU) {}

pub fn rrc_l_txt(cpu: &mut CPU) {}

/// 0xCB0E   "rrc (hl)"
/// The contents of the memory address specified by the contents
/// of HL are rotated right 1 bit position. The contents of bit 0
/// are copied to the Carry flag and also to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the source byte.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 0 | 0x0E
/// =================================
/// T-States: 15
pub fn rrcOhlO(cpu: &mut CPU) {}

pub fn rrcOhlO_txt(cpu: &mut CPU) {}

/// 0xCB0F   "rrc a"
/// The contents of register A are rotated right 1 bit position.
/// The contents of bit 0 are copied to the Carry flag and also
/// to bit 7.
///
/// S is set if result is negative; otherwise, it is reset.
/// Z is set if result is 0; otherwise, it is reset.
/// P/V is set if parity even; otherwise, it is reset.
/// H, N are reset.
/// C is data from bit 0 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0x0F
/// =================================
/// T-States: 8
pub fn rrc_a(cpu: &mut CPU) {}

pub fn rrc_a_txt(cpu: &mut CPU) {}

// *************************** 1 ***********************************
/// 0xCB10   "rl b"
/// The contents of Register B are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0x10
/// =================================
/// T-States: 8
pub fn rl_b(cpu: &mut CPU) {}

pub fn rl_b_txt(cpu: &mut CPU) {}

/// 0xCB11   "rl c"
/// The contents of Register C are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0x11
/// =================================
/// T-States: 8
pub fn rl_c(cpu: &mut CPU) {
    let viejo_c_flag = cpu.get_c_flag();
    cpu.set_flag(FLAG_C, (0b1000_0000 & cpu.c) != 0);

    // Rotación
    let mut nuevo_valor = cpu.c << 1;
    nuevo_valor = nuevo_valor & 0b1111_1110;
    if viejo_c_flag {
        nuevo_valor |= 0b0000_0001;
    }

    cpu.set_flag(FLAG_Z, nuevo_valor == 0);
    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_H, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rl_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RL C"));
}

/// 0xCB12   "rl d" operation
/// The contents of Register D are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 0x12
/// =================================
/// T-States: 4, 4 (8)
pub fn rl_d(cpu: &mut CPU) {}

pub fn rl_d_txt(cpu: &mut CPU) {}

/// 0xCB13   "rl e"
/// The contents of Register E are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 0x13
/// =================================
/// T-States: 8
pub fn rl_e(cpu: &mut CPU) {}

pub fn rl_e_txt(cpu: &mut CPU) {}

/// 0xCB14   "rl h" operation
/// </summary>
/// <remarks>
///
/// The contents of Register H are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0x14
/// =================================
/// T-States: 8
pub fn rl_h(cpu: &mut CPU) {}

pub fn rl_h_txt(cpu: &mut CPU) {}

/// 0xCB15   "rl l"
/// The contents of Register L are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0x15
/// =================================
/// T-States: 8
pub fn rl_l(cpu: &mut CPU) {}

pub fn rl_l_txt(cpu: &mut CPU) {}

/// 0xCB16   "rl (hl)"
/// The contents of the memory address specified by the contents
/// of HL are rotated left 1 bit position. The contents of bit 7
/// are copied to the Carry flag, and the previous contents of the
/// Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the source byte.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 0x16
/// =================================
/// T-States: 15
pub fn rl_OhlO(cpu: &mut CPU) {}

pub fn rl_OhlO_txt(cpu: &mut CPU) {}

/// 0xCB17   "rl a"
/// The contents of Register A are rotated left 1 bit position. The
/// contents of bit 7 are copied to the Carry flag, and the previous
/// contents of the Carry flag are copied to bit 0.
///
/// S, Z, P/V are not affected.
/// H, N are reset.
/// C is data from bit 7 of the original register value.
///
/// =================================
/// | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0xCB
/// =================================
/// | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0x17
/// =================================
/// T-States: 8
pub fn rl_a(cpu: &mut CPU) {
    let viejo_c_flag = cpu.get_c_flag();

    cpu.set_flag(FLAG_C, (0b1000_0000 & cpu.c) != 0);
    // Rotación
    let mut nuevo_valor = cpu.a << 1;
    nuevo_valor = nuevo_valor & 0b1111_1110;
    if viejo_c_flag {
        nuevo_valor |= 0b0000_0001;
    }

    //maneja flags

    cpu.set_flag(FLAG_Z, nuevo_valor == 0);
    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_H, false);


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rl_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RL A"));
}

// *************************** 2 ***********************************

// *************************** 3 ***********************************
// 0xCB35
pub fn sll_l(cpu: &mut CPU) {
//    if (cpu.l & 0b1000_0000) != 0 {
//        cpu.set_c_flag();
//    } else {
//        cpu.reset_c_flag();
//    }
    cpu.set_flag(FLAG_C, (cpu.l & 0b1000_0000) != 0);
    cpu.l = (cpu.l << 1) | 0b0000_0001;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sll_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SLL L"));
}

// *************************** 4 ***********************************

// *************************** 5 ***********************************

// *************************** 6 ***********************************

// *************************** 7 ***********************************
// 0xCB70
pub fn bit_6_b(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.b, 6); }

pub fn bit_6_b_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,B")); }

// 0xCB71
pub fn bit_6_c(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.c, 6); }

pub fn bit_6_c_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,C")); }

// 0xCB72
pub fn bit_6_d(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.d, 6); }

pub fn bit_6_d_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,D")); }

// 0xCB73
pub fn bit_6_e(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.e, 6); }

pub fn bit_6_e_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,E")); }

// 0xCB74
pub fn bit_6_h(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.h, 6); }

pub fn bit_6_h_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,H")); }

// 0xCB75
pub fn bit_6_l(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.l, 6); }

pub fn bit_6_l_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,L")); }

// 0xCB76
pub fn bit_6_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let dato = cpu.mem.lee_byte_de_mem(hl);
    bas_bit_B_R(cpu, dato, 6);
}

pub fn bit_6_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,(HL)")); }

// 0xCB77
pub fn bit_6_a(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.a, 6); }

pub fn bit_6_a_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 6,A")); }

// 0xCB78
pub fn bit_7_b(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.b, 7); }

pub fn bit_7_b_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,B")); }

// 0xCB79
pub fn bit_7_c(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.c, 7); }

pub fn bit_7_c_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,C")); }

// 0xCB7A
pub fn bit_7_d(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.d, 7); }

pub fn bit_7_d_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,D")); }

// 0xCB7B
pub fn bit_7_e(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.e, 7); }

pub fn bit_7_e_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,E")); }


// 0xCB7C     BIT 7, H
pub fn bit_7_h(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.h, 7); }


pub fn bit_7_h_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,H")); }
// 0xCB7D

pub fn bit_7_l(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.l, 7); }

pub fn bit_7_l_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,L")); }

// 0xCB7E
pub fn bit_7_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let dato = cpu.mem.lee_byte_de_mem(hl);
    bas_bit_B_R(cpu, dato, 7);
}

pub fn bit_7_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,(HL)")); }

// 0xCB7F
pub fn bit_7_a(cpu: &mut CPU) { bas_bit_B_R(cpu, cpu.a, 7); }

pub fn bit_7_a_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 7,A")); }

// *************************** 8 ***********************************
//0xCB80
pub fn res_0_b(cpu: &mut CPU) { cpu.b = bas_res_B_R(cpu, cpu.b, 0); }

pub fn res_0_b_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,B")); }

//0xCB81
pub fn res_0_c(cpu: &mut CPU) { cpu.c = bas_res_B_R(cpu, cpu.c, 0); }

pub fn res_0_c_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,C")); }

//0xCB82
pub fn res_0_d(cpu: &mut CPU) { cpu.d = bas_res_B_R(cpu, cpu.d, 0); }

pub fn res_0_d_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,D")); }

//0xCB83
pub fn res_0_e(cpu: &mut CPU) { cpu.e = bas_res_B_R(cpu, cpu.e, 0); }

pub fn res_0_e_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,E")); }

//0xCB84
pub fn res_0_h(cpu: &mut CPU) { cpu.h = bas_res_B_R(cpu, cpu.h, 0); }

pub fn res_0_h_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,H")); }

//0xCB85
pub fn res_0_l(cpu: &mut CPU) { cpu.l = bas_res_B_R(cpu, cpu.l, 0); }

pub fn res_0_l_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,L")); }

//0xCB86
pub fn res_0_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let mut dato = cpu.mem.lee_byte_de_mem(hl);

    dato = bas_res_B_R(cpu, dato, 0);
    cpu.mem.escribe_byte_en_mem(hl, dato);
}

pub fn res_0_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,(HL)")); }

//0xCB87
pub fn res_0_a(cpu: &mut CPU) { cpu.a = bas_res_B_R(cpu, cpu.a, 0); }

pub fn res_0_a_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,A")); }

// *************************** 9 ***********************************
// *************************** A ***********************************
//0xCBA0
pub fn res_4_b(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_b_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA1
pub fn res_4_c(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_c_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA2
pub fn res_4_d(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_d_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA3
pub fn res_4_e(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_e_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA4
pub fn res_4_h(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_h_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA5
pub fn res_4_l(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_l_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA6
pub fn res_4_OhlO(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_OhlO_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA7
pub fn res_4_a(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_4_a_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA8
pub fn res_5_b(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_b_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBA9
pub fn res_5_c(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_c_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBAA
pub fn res_5_d(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_d_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBAB
pub fn res_5_e(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_e_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBAC
pub fn res_5_h(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_h_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBAD
pub fn res_5_l(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_l_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBAE
pub fn res_5_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let mut dato = cpu.mem.lee_byte_de_mem(hl);

    dato = bas_res_B_R(cpu, dato, 5);

    cpu.mem.escribe_byte_en_mem(hl, dato);
}

pub fn res_5_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 5,(HL)")); }

//0xCBAF
pub fn res_5_a(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn res_5_a_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

// *************************** B ***********************************
// *************************** C ***********************************
//0xCBC0
pub fn set_0_b(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_b_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC1
pub fn set_0_c(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_c_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC2
pub fn set_0_d(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_d_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC3
pub fn set_0_e(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_e_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCB4
pub fn set_0_h(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_h_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC5
pub fn set_0_l(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_l_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC6
pub fn set_0_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    let mut dato = cpu.mem.lee_byte_de_mem(hl);

    dato = bas_set_B_R(cpu, dato, 0);
    cpu.mem.escribe_byte_en_mem(hl, dato);
}

pub fn set_0_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("SET 0,(HL)")); }

//0xCBC7
pub fn set_0_a(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_0_a_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC8
pub fn set_1_b(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_b_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBC9
pub fn set_1_c(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_c_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCA
pub fn set_1_d(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_d_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCB
pub fn set_1_e(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_e_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCC
pub fn set_1_h(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_h_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCD
pub fn set_1_l(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_l_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCE
pub fn set_1_OhlO(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_OhlO_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }

//0xCBCF
pub fn set_1_a(cpu: &mut CPU) { fnCB_no_impl(cpu); }

pub fn set_1_a_txt(cpu: &mut CPU) { fnCB_no_impl(cpu); }
// *************************** D ***********************************
// *************************** E ***********************************
// *************************** F ***********************************

