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
// O = ()     p = '    m = +       n = valor hex de 8 bits
// *************************** 0 ***********************************

use crate::cpu::CPU;
//use crate::instrucciones_normales::di;


pub fn mete_funciones_fdcb(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones_fdcb[0x00 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x01 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x02 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x03 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x04 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x05 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x06 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x07 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x08 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x09 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x0F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);

    // *************************** 1 ***********************************
    cpu.funciones_fdcb[0x10 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x11 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x12 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x13 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x14 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x15 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x16 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x17 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x18 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x19 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x1F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);

    // *************************** 2 ***********************************
    cpu.funciones_fdcb[0x20 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x21 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x22 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x23 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x24 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x25 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x26 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x27 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x28 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x29 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x2A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x2B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x2C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 8);
    cpu.funciones_fdcb[0x2D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x2E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x2F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** 3 ***********************************
    cpu.funciones_fdcb[0x30 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x31 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x32 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x33 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x34 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x35 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x36 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x37 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x38 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x39 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x3F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** 4 ***********************************
    cpu.funciones_fdcb[0x40 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x41 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x42 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x43 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x44 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x45 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x46 as usize].set_punt_y_val_a_fn(bit_0_OiymnO, bit_0_OiymnO_txt, 4, 20);
    cpu.funciones_fdcb[0x47 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x48 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x49 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x4A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x4B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x4C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x4D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    cpu.funciones_fdcb[0x4E as usize].set_punt_y_val_a_fn(bit_1_OiymnO, bit_1_OiymnO_txt, 4, 20);
    cpu.funciones_fdcb[0x4F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 4, 20);
    // *************************** 5 ***********************************
    cpu.funciones_fdcb[0x50 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x51 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x52 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x53 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x54 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x55 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x56 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x57 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x58 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x59 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x5F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** 6 ***********************************
    cpu.funciones_fdcb[0x60 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x61 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x62 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x63 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x64 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x65 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x66 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x67 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x68 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x69 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x6F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** 7 ***********************************

    cpu.funciones_fdcb[0x70 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x71 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x72 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x73 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x74 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x75 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x76 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x77 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x78 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x79 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x7F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);

    // *************************** 8 ***********************************
    cpu.funciones_fdcb[0x80 as usize].set_punt_y_val_a_fn(res_0_OiymnO_b, res_0_OiymnO_b_txt, 4, 23);
    cpu.funciones_fdcb[0x81 as usize].set_punt_y_val_a_fn(res_0_OiymnO_c, res_0_OiymnO_c_txt, 4, 23);
    cpu.funciones_fdcb[0x82 as usize].set_punt_y_val_a_fn(res_0_OiymnO_d, res_0_OiymnO_d_txt, 4, 23);
    cpu.funciones_fdcb[0x83 as usize].set_punt_y_val_a_fn(res_0_OiymnO_e, res_0_OiymnO_e_txt, 4, 23);
    cpu.funciones_fdcb[0x84 as usize].set_punt_y_val_a_fn(res_0_OiymnO_h, res_0_OiymnO_h_txt, 4, 23);
    cpu.funciones_fdcb[0x85 as usize].set_punt_y_val_a_fn(res_0_OiymnO_l, res_0_OiymnO_l_txt, 4, 23);
    cpu.funciones_fdcb[0x86 as usize].set_punt_y_val_a_fn(res_0_OiymnO, res_0_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0x87 as usize].set_punt_y_val_a_fn(res_0_OiymnO_a, res_0_OiymnO_a_txt, 4, 23);
    cpu.funciones_fdcb[0x88 as usize].set_punt_y_val_a_fn(res_1_OiymnO_b, res_1_OiymnO_b_txt, 4, 23);
    cpu.funciones_fdcb[0x89 as usize].set_punt_y_val_a_fn(res_1_OiymnO_c, res_1_OiymnO_c_txt, 4, 23);
    cpu.funciones_fdcb[0x8A as usize].set_punt_y_val_a_fn(res_1_OiymnO_d, res_1_OiymnO_d_txt, 4, 23);
    cpu.funciones_fdcb[0x8B as usize].set_punt_y_val_a_fn(res_1_OiymnO_e, res_1_OiymnO_e_txt, 4, 23);
    cpu.funciones_fdcb[0x8C as usize].set_punt_y_val_a_fn(res_1_OiymnO_h, res_1_OiymnO_h_txt, 4, 23);
    cpu.funciones_fdcb[0x8D as usize].set_punt_y_val_a_fn(res_1_OiymnO_l, res_1_OiymnO_l_txt, 4, 23);
    cpu.funciones_fdcb[0x8E as usize].set_punt_y_val_a_fn(res_1_OiymnO, res_1_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0x8F as usize].set_punt_y_val_a_fn(res_1_OiymnO_a, res_1_OiymnO_a_txt, 4, 23);
    // *************************** 9 ***********************************
    cpu.funciones_fdcb[0x90 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x91 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x92 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x93 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x94 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x95 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x96 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x97 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x98 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x99 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9A as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9B as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9C as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9D as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9E as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0x9F as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** A ***********************************
    cpu.funciones_fdcb[0xA0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA6 as usize].set_punt_y_val_a_fn(res_4_OiymnO, res_4_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0xA7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xA9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xAA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xAB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xAC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xAD as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xAE as usize].set_punt_y_val_a_fn(res_5_OiymnO, res_5_OiymnO, 4, 23);
    cpu.funciones_fdcb[0xAF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** B ***********************************
    cpu.funciones_fdcb[0xB0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB6 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xB9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBD as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBE as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xBF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** C ***********************************
    cpu.funciones_fdcb[0xC0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC6 as usize].set_punt_y_val_a_fn(set0_OiymnO, set0_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0xC7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xC9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xCA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xCB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xCC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xCD as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xCE as usize].set_punt_y_val_a_fn(set1_OiymnO, set1_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0xCF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** D ***********************************
    cpu.funciones_fdcb[0xD0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD6 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xD9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDD as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDE as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xDF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** E ***********************************
    cpu.funciones_fdcb[0xE0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE6 as usize].set_punt_y_val_a_fn(set4_OiymnO, set4_OiymnO_txt, 4, 23);
    cpu.funciones_fdcb[0xE7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xE9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xEA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xEB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xEC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xED as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xEE as usize].set_punt_y_val_a_fn(set5_OiymnO, set5_OiymnO, 4, 23);
    cpu.funciones_fdcb[0xEF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    // *************************** F ***********************************
    cpu.funciones_fdcb[0xF0 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF1 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF2 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF3 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF4 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF5 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF6 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF7 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF8 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xF9 as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFA as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFB as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFC as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFD as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFE as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
    cpu.funciones_fdcb[0xFF as usize].set_punt_y_val_a_fn(fnFDCB_no_impl, fnFDCB_no_impl, 0, 0);
}

pub fn fnFDCB_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion #FDCB{:02X}{:02X} no implementada PC = {:04X}", cpu.r2, cpu.r3, cpu.pc));
}

// O = ()  p = '
// *************************** 0 ***********************************
// *************************** 1 ***********************************
// *************************** 2 ***********************************
// *************************** 3 ***********************************
// *************************** 4 ***********************************


// 0xFDCBNN46  TODO:hacer una funcion global
pub fn bit_0_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    if cpu.get_bitu8(dato, 0) {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }

    cpu.set_h_flag();
    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn bit_0_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("BIT 0,(IY+#{:02X})", cpu.r2)); }


// 0xFDCBNN4E  TODO:hacer una funcion global
pub fn bit_1_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    if cpu.get_bitu8(dato, 1) {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }

    cpu.set_h_flag();
    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn bit_1_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("Bit 1,(IY+#{:02X})", cpu.r2)); }

// *************************** 5 ***********************************
// *************************** 6 ***********************************
// *************************** 7 ***********************************


// *************************** 8 ***********************************
// 0xFDCBNN80
pub fn res_0_OiymnO_b(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_b_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN81
pub fn res_0_OiymnO_c(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_c_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN82
pub fn res_0_OiymnO_d(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_d_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN83
pub fn res_0_OiymnO_e(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_e_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN84
pub fn res_0_OiymnO_h(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_h_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN85
pub fn res_0_OiymnO_l(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_l_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN86  TODO:hacer una funcion global
pub fn res_0_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    let dato = cpu.reset_bitu8(dato, 0);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn res_0_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RES 0,(IY+#{:02X})", cpu.r2)); }

// 0xFDCBNN87
pub fn res_0_OiymnO_a(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_0_OiymnO_a_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN88
pub fn res_1_OiymnO_b(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_b_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN89
pub fn res_1_OiymnO_c(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_c_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN8A
pub fn res_1_OiymnO_d(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_d_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN8B
pub fn res_1_OiymnO_e(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_e_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN8C
pub fn res_1_OiymnO_h(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_h_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

// 0xFDCBNN8D
pub fn res_1_OiymnO_l(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }

pub fn res_1_OiymnO_l_txt(cpu: &mut CPU) { fnFDCB_no_impl(cpu); }


// 0xFDCBNN8E  TODO:hacer una funcion global
pub fn res_1_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.reset_bitu8(dato, 1);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn res_1_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RESET 1(IY+#{:02X})", cpu.r2)); }

// 0xFDCBNN8F
pub fn res_1_OiymnO_a(cpu: &mut CPU) {}

pub fn res_1_OiymnO_a_txt(cpu: &mut CPU) {}

// *************************** 9 ***********************************
// *************************** A ***********************************
//0xFDCBNNA6   TODO:hacer una funcion global
pub fn res_4_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    let dato = cpu.reset_bitu8(dato, 4);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn res_4_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RESET 4(IY+#{:02X})", cpu.r2)); }

//0xFDCBNNAE   TODO:hacer una funcion global
pub fn res_5_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    let dato = cpu.reset_bitu8(dato, 5);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn res_5_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RESET 5(IY+#{:02X})", cpu.r2)); }

// *************************** B ***********************************
//0xFDCBNNB6   TODO:hacer una funcion global
pub fn res_6_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    let dato = cpu.reset_bitu8(dato, 6);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn res_6_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RESET 6(IY+#{:02X})", cpu.r2)); }

//0xFDCBNNBE   TODO:hacer una funcion global
pub fn res_7_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);

    let dato = cpu.mem.lee_byte_de_mem(direccion);

    let dato = cpu.reset_bitu8(dato, 7);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn res_7_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("RESET 7(IY+#{:02X})", cpu.r2)); }

// *************************** C ***********************************
// 0xFDCBNNC6 TODO:hacer una funcion global
pub fn set0_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 0);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set0_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 0(IY+#{:02X})", cpu.r2));
}

// 0xFDCBNNCE  TODO:hacer una funcion global
pub fn set1_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 1);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set1_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 1(IY+#{:02X})", cpu.r2));
}

// *************************** D ***********************************
// *************************** E ***********************************
// 0xFDCBNNE6 TODO:hacer una funcion global
pub fn set4_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 4);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set4_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 4(IY+#{:02X})", cpu.r2));
}

// 0xFDCBNNCE  TODO:hacer una funcion global
pub fn set5_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 5);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set5_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 5(IY+#{:02X})", cpu.r2));
}

// *************************** F ***********************************
// 0xFDCBNNF6 TODO:hacer una funcion global
pub fn set6_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 6);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set6_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 6(IY+#{:02X})", cpu.r2));
}

// 0xFDCBNNFE  TODO:hacer una funcion global
pub fn set7_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let mut dato = cpu.mem.lee_byte_de_mem(direccion);
    dato = cpu.set_bitu8(dato, 7);
    cpu.mem.escribe_byte_en_mem(direccion, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn set7_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SET 7(IY+#{:02X})", cpu.r2));
}
