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

use crate::cpu::{CPU, Funcion};
//use crate::instrucciones_normales::di;


pub fn mete_funciones_fd(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones_fd[0x00 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x01 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x02 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x03 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x04 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x05 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x06 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x07 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x08 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x09 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 15);
    cpu.funciones_fd[0x0A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x0B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x0C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x0D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x0E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x0F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);

    // *************************** 1 ***********************************
    cpu.funciones_fd[0x10 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x11 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x12 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x13 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x14 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x15 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x16 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x17 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x18 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x19 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 15);
    cpu.funciones_fd[0x1A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x1B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x1C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x1D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x1E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x1F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);

    // *************************** 2 ***********************************
    cpu.funciones_fd[0x20 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x21 as usize].set_punt_y_val_a_fn(ld_iy_nn, ld_iy_nn_txt, 4, 14);
    cpu.funciones_fd[0x22 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 4, 20);
    cpu.funciones_fd[0x23 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 10);
    cpu.funciones_fd[0x24 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 8);
    cpu.funciones_fd[0x25 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 8);
    cpu.funciones_fd[0x26 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 3, 11);
    cpu.funciones_fd[0x27 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x28 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x29 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 15);
    cpu.funciones_fd[0x2A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 4, 20);
    cpu.funciones_fd[0x2B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 10);
    cpu.funciones_fd[0x2C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 8);
    cpu.funciones_fd[0x2D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 8);
    cpu.funciones_fd[0x2E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 3, 11);
    cpu.funciones_fd[0x2F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 3 ***********************************
    cpu.funciones_fd[0x30 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x31 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x32 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x33 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x34 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 3, 23);
    cpu.funciones_fd[0x35 as usize].set_punt_y_val_a_fn(decOiymnO, decOiymnO_txt, 3, 23);
    cpu.funciones_fd[0x36 as usize].set_punt_y_val_a_fn(ldOiymn1O_n2, ldOiymn1O_n2_txt, 4, 19);
    cpu.funciones_fd[0x37 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x38 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x39 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 2, 15);
    cpu.funciones_fd[0x3A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x3B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x3C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x3D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x3E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x3F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 4 ***********************************
    cpu.funciones_fd[0x40 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x41 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x42 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x43 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x44 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x45 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x46 as usize].set_punt_y_val_a_fn(ld_b_OiymnO, ld_b_OiymnO_txt, 3, 19);
    cpu.funciones_fd[0x47 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x48 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x49 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x4A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x4B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x4C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x4D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x4E as usize].set_punt_y_val_a_fn(ld_c_OiymnO, ld_c_OiymnO_txt, 3, 19);
    cpu.funciones_fd[0x4F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 5 ***********************************
    cpu.funciones_fd[0x50 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x51 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x52 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x53 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x54 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x55 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x56 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x57 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x58 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x59 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x5A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x5B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x5C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x5D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x5E as usize].set_punt_y_val_a_fn(ld_e_OiymnO, ld_e_OiymnO_txt, 3, 19);
    cpu.funciones_fd[0x5F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 6 ***********************************
    cpu.funciones_fd[0x60 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x61 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x62 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x63 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x64 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x65 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x66 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x67 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x68 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x69 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x6A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x6B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x6C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x6D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x6E as usize].set_punt_y_val_a_fn(ld_l_OiymnO, ld_l_OiymnO_txt, 3, 19);
    cpu.funciones_fd[0x6F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 7 ***********************************
    cpu.funciones_fd[0x70 as usize].set_punt_y_val_a_fn(ldOiymnO_b, ldOiymnO_b_txt, 3, 19);
    cpu.funciones_fd[0x71 as usize].set_punt_y_val_a_fn(ldOiymnO_c, ldOiymnO_c_txt, 3, 19);
    cpu.funciones_fd[0x72 as usize].set_punt_y_val_a_fn(ldOiymnO_d, ldOiymnO_d_txt, 3, 19);
    cpu.funciones_fd[0x73 as usize].set_punt_y_val_a_fn(ldOiymnO_e, ldOiymnO_e_txt, 3, 19);
    cpu.funciones_fd[0x74 as usize].set_punt_y_val_a_fn(ldOiymnO_h, ldOiymnO_h_txt, 3, 19);
    cpu.funciones_fd[0x75 as usize].set_punt_y_val_a_fn(ldOiymnO_l, ldOiymnO_l_txt, 3, 19);
    cpu.funciones_fd[0x76 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x77 as usize].set_punt_y_val_a_fn(ldOiymnO_a, ldOiymnO_a_txt, 0, 0);
    cpu.funciones_fd[0x78 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x79 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x7A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x7B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x7C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x7D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x7E as usize].set_punt_y_val_a_fn(ld_a_OiymnO, ld_a_OiymnO_txt, 3, 19);
    cpu.funciones_fd[0x7F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);

    // *************************** 8 ***********************************
    cpu.funciones_fd[0x80 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x81 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x82 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x83 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x84 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x85 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x86 as usize].set_punt_y_val_a_fn(add_aOiymnO, add_aOiymnO_txt, 3, 19);
    cpu.funciones_fd[0x87 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x88 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x89 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x8F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** 9 ***********************************
    cpu.funciones_fd[0x90 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x91 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x92 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x93 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x94 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x95 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x96 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x97 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x98 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x99 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9A as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9B as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9C as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9D as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9E as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0x9F as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** A ***********************************
    cpu.funciones_fd[0xA0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xA9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAB as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAD as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xAF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** B ***********************************
    cpu.funciones_fd[0xB0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xB9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBB as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBD as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xBF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** C ***********************************
    cpu.funciones_fd[0xC0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xC9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xCA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xCB as usize].set_punt_y_val_a_fn(FDCB, FDCB_txt, 0, 0);
    cpu.funciones_fd[0xCC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xCD as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xCE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xCF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** D ***********************************
    cpu.funciones_fd[0xD0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xD9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDB as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDD as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xDF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** E ***********************************
    cpu.funciones_fd[0xE0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xE9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xEA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xEB as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xEC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xED as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xEE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xEF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    // *************************** F ***********************************
    cpu.funciones_fd[0xF0 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF1 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF2 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF3 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF4 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF5 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF6 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF7 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF8 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xF9 as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFA as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFB as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFC as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFD as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFE as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
    cpu.funciones_fd[0xFF as usize].set_punt_y_val_a_fn(fnFD_no_impl, fnFD_no_impl, 0, 0);
}

pub fn fnFD_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion normal no implementada\n\
    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
    r3 = #{:02X}\n",
                   cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3));
}


// XXXXXXXXXXXXXXXXXXX Funciones comunes en instrucciones fd XXXXXXXXXXXXXXXXXXXX
// https://wikiti.brandonw.net/index.php?title=Z80_Instruction_Set
// Cuando varias funciones en los arreglos de punteros, tienen opciones comunes
// usan estas funciones, solo se tocaran los flags en estas funciones

// O = ()  p = '
// *************************** 0 ***********************************
// *************************** 1 ***********************************
// *************************** 2 ***********************************
// 0xFD21
pub fn ld_iy_nn(cpu: &mut CPU) {
    cpu.iyl = cpu.r2;
    cpu.iyh = cpu.r3;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_iy_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD IY,#{:04X}", cpu.r2r3));
}

// *************************** 3 ***********************************
// 0xFD35 Ojo N es complemento a 2 (tiene signo)
// dec (I+D) 	11i11101 00110101 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	(I+D) -= 1
pub fn decOiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let datodec = cpu.dec_8bits(cpu.mem.lee_byte_de_mem(direccion));
    cpu.mem.escribe_byte_en_mem(direccion, datodec);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn decOiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC(IY+#{:02X})", cpu.r2));
}

// OxFD36    (IY+dd)<-nn   FD36ddnn
// ld (I+D),N 	11i11101 00110110 dddddddd nnnnnnnn 	19 	- 	- 	- 	- 	- 	- 	- 	- 	(I+D) := N
pub fn ldOiymn1O_n2(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.r3);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOiymn1O_n2_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (IY+#{:02X}),#{:02X}", cpu.r2, cpu.r3));
}

// *************************** 4 ***********************************
// OxFD46
pub fn ld_b_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.b = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,(IY+#{:02X})", cpu.r2)); }

// 0xFD4E
pub fn ld_c_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.c = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,(IY+#{:02X})", cpu.r2)); }

// *************************** 5 ***********************************
// OxFD56
pub fn ld_d_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.d = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_d_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,(IY+#{:02X})", cpu.r2)); }

// 0xFD5E
pub fn ld_e_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.e = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_e_OiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,(IY+#{:02X})", cpu.r2)); }

// *************************** 6 ***********************************
// 0xFD60
pub fn ld_iyh_b(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_b_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD61
pub fn ld_iyh_c(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_c_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD62
pub fn ld_iyh_d(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_d_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD63
pub fn ld_iyh_e(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_e_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD64
pub fn ld_iyh_iyh(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_iyh_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD65
pub fn ld_iyh_iyl(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ld_iyh_iyl_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// OxFD66
pub fn ld_h_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.h = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,(IY+#{:02X})", cpu.r2));
}

// 0xFD6E
pub fn ld_l_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.l = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,(IY+#{:02X})", cpu.r2));
}

// *************************** 7 ***********************************
// 0xFD70
pub fn ldOiymnO_b(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ldOiymnO_b_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD71
pub fn ldOiymnO_c(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.c);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOiymnO_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(IY+#{:02X}),C", cpu.r2)); }

// 0xFD72
pub fn ldOiymnO_d(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ldOiymnO_d_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD73
pub fn ldOiymnO_e(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ldOiymnO_e_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD74
pub fn ldOiymnO_h(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ldOiymnO_h_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD75
pub fn ldOiymnO_l(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);


    cpu.mem.escribe_byte_en_mem(direccion, cpu.l);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOiymnO_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(IY+#{:02X}),L", cpu.r2)); }

// 0xFD77
pub fn ldOiymnO_a(cpu: &mut CPU) { fnFD_no_impl(cpu); }

pub fn ldOiymnO_a_txt(cpu: &mut CPU) { fnFD_no_impl(cpu); }

// 0xFD7E
pub fn ld_a_OiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_OiymnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,(IY+#{:02X})", cpu.r2));
}

// *************************** 8 ***********************************
// 0xFD86
pub fn add_aOiymnO(cpu: &mut CPU) {
    let iy = cpu.lee_iy();
    let direccion = cpu.suma_compl2_a_un_u16(iy, cpu.r2);
    let dato = cpu.mem.lee_byte_de_mem(direccion);
    let resultado = cpu.suma_u8_mas_u8(cpu.a, dato);
    cpu.mem.escribe_byte_en_mem(direccion, resultado);


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn add_aOiymnO_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A(IY+#{:02X})", cpu.r2)); }

// *************************** 9 ***********************************
// *************************** A ***********************************
// *************************** B ***********************************
// *************************** C ***********************************
// 0xFDCB -----EXTENSION (2a vez)  ojo la funcion la dice el cuarto byte
pub fn FDCB(cpu: &mut CPU) {
// Ejecuta instruccion
    let f: Funcion = cpu.funciones_fdcb[cpu.r3 as usize];
    let ff = f.get_puntero_a_funcion();
    ff(cpu);
}

pub fn FDCB_txt(cpu: &mut CPU) {
    let f: Funcion = cpu.funciones_fdcb[cpu.r3 as usize];
    let ff = f.get_puntero_txt_a_funcion();
    ff(cpu);
    ;
}
// *************************** D ***********************************
// *************************** E ***********************************
// *************************** F ***********************************
// 0xFD00

//pub fn rlc_b(cpu: &mut CPU) {
//    cpu.t += 8;
//    cpu.pc += 2;
//}
//
//pub fn rlc_b_txt(cpu: &mut CPU) {
////    let txt = format!("NOP opcode = #{:02X}", cpu.r0);
////    cpu.texto(&txt);
//    cpu.texto(&format!("opcode = #CB{:02X}", cpu.r1));
//}
