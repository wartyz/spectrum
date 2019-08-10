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
//    cpu.funciones_cb[0x00 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x00 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x01 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x01 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x02 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x02 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x03 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x03 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x04 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x04 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x05 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x05 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x06 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x06 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x07 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x07 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x08 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x08 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x09 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x09 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x0F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x0F as usize] = funcionCB_no_implementada;
    // *************************** 1 ***********************************
//    cpu.funciones_cb[0x10 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x10 as usize] = funcionCB_no_implementada;
    cpu.funciones_cb[0x11 as usize] = rl_c;
    cpu.funciones_cb_txt[0x11 as usize] = rl_c_txt;
//    cpu.funciones_cb[0x12 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x12 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x13 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x13 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x14 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x14 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x15 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x15 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x16 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x16 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x17 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x17 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x18 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x18 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x19 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x19 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x1F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x1F as usize] = funcionCB_no_implementada;
    // *************************** 2 ***********************************
//    cpu.funciones_cb[0x20 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x20 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x21 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x21 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x22 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x22 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x23 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x23 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x24 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x24 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x25 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x25 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x26 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x26 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x27 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x27 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x28 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x28 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x29 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x29 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x2F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x2F as usize] = funcionCB_no_implementada;
    // *************************** 3 ***********************************
//    cpu.funciones_cb[0x30 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x30 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x31 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x31 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x32 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x32 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x33 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x33 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x34 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x34 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x35 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x35 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x36 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x36 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x37 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x37 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x38 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x38 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x39 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x39 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x3F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x3F as usize] = funcionCB_no_implementada;
    // *************************** 4 ***********************************
//    cpu.funciones_cb[0x40 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x40 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x41 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x41 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x42 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x42 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x43 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x43 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x44 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x44 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x45 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x45 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x46 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x46 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x47 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x47 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x48 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x48 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x49 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x49 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x4F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x4F as usize] = funcionCB_no_implementada;
    // *************************** 5 ***********************************
//    cpu.funciones_cb[0x50 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x50 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x51 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x51 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x52 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x52 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x53 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x53 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x54 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x54 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x55 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x55 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x56 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x56 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x57 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x57 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x58 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x58 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x59 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x59 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x5F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x5F as usize] = funcionCB_no_implementada;
    // *************************** 6 ***********************************
//    cpu.funciones_cb[0x60 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x60 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x61 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x61 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x62 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x62 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x63 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x63 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x64 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x64 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x65 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x65 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x66 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x66 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x67 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x67 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x68 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x68 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x69 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x69 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x6F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x6F as usize] = funcionCB_no_implementada;
    // *************************** 7 ***********************************
//    cpu.funciones_cb[0x70 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x70 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x71 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x71 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x72 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x72 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x73 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x73 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x74 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x74 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x75 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x75 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x76 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x76 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x77 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x77 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x78 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x78 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x79 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x79 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x7A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x7A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x7B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x7B as usize] = funcionCB_no_implementada;
    cpu.funciones_cb[0x7C as usize] = bit7h;
    cpu.funciones_cb_txt[0x7C as usize] = bit7h_txt;
//    cpu.funciones_cb[0x7D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x7D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x7E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x7E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x7F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x7F as usize] = funcionCB_no_implementada;
    // *************************** 8 ***********************************
//    cpu.funciones_cb[0x80 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x80 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x81 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x81 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x82 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x82 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x83 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x83 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x84 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x84 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x85 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x85 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x86 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x86 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x87 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x87 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x88 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x88 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x89 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x89 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x8F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x8F as usize] = funcionCB_no_implementada;
    // *************************** 9 ***********************************
//    cpu.funciones_cb[0x90 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x90 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x91 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x91 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x92 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x92 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x93 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x93 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x94 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x94 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x95 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x95 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x96 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x96 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x97 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x97 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x98 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x98 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x99 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x99 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9A as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9B as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9C as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9D as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9E as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0x9F as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0x9F as usize] = funcionCB_no_implementada;
    // *************************** A ***********************************
//    cpu.funciones_cb[0xA0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xA9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xA9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xAF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xAF as usize] = funcionCB_no_implementada;
    // *************************** B ***********************************
//    cpu.funciones_cb[0xB0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xB9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xB9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xBF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xBF as usize] = funcionCB_no_implementada;
    // *************************** C ***********************************
//    cpu.funciones_cb[0xC0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xC9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xC9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xCF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xCF as usize] = funcionCB_no_implementada;
    // *************************** D ***********************************
//    cpu.funciones_cb[0xD0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xD9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xD9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xDF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xDF as usize] = funcionCB_no_implementada;
    // *************************** E ***********************************
//    cpu.funciones_cb[0xE0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xE9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xE9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xEA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xEA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xEB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xEB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xEC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xEC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xED as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xED as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xEE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xEE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xEF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xEF as usize] = funcionCB_no_implementada;
    // *************************** F ***********************************
//    cpu.funciones_cb[0xF0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF0 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF1 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF2 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF3 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF4 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF5 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF6 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF7 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF8 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xF9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xF9 as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFA as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFB as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFC as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFD as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFE as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb[0xFF as usize] = funcionCB_no_implementada;
//    cpu.funciones_cb_txt[0xFF as usize] = funcionCB_no_implementada;
}

pub fn funcionCB_no_implementada(cpu: &mut CPU) {
    panic!(format!("Funcion #CB{:02X} no implementada", cpu.r1));
}

// funcion metida por defecto en el arreglo de punteros a funciones cb
pub fn nopCB(cpu: &mut CPU) {
    panic!("Instrucción CB no implementada");
}

pub fn nopCB_txt(cpu: &mut CPU) {
    panic!("Instrucción CB no implementada");
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
    let viejo_c_flag = cpu.get_c_flag();
    let c_flag: bool = (0b1000_0000 & cpu.c) != 0;
    if c_flag {
        cpu.set_c_flag();
    } else {
        cpu.reset_c_flag();
    }

    // Rotación
    let mut nuevo_valor = cpu.c << 1;
    nuevo_valor = nuevo_valor & 0b1111_1110;
    if viejo_c_flag {
        nuevo_valor |= 0b0000_0001;
    }

    //maneja flags
    if nuevo_valor == 0 {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }
    cpu.reset_n_flag();
    cpu.reset_h_flag();

    cpu.pc += 2;
    cpu.t += 8;
}

pub fn rl_c_txt(cpu: &mut CPU) {
    let txt = format!("RL C");
    cpu.texto(&txt);
}

// 0xCB17     RL A
pub fn rl_a(cpu: &mut CPU) {
    let viejo_c_flag = cpu.get_c_flag();
    let c_flag: bool = (0b1000_0000 & cpu.a) != 0;
    if c_flag {
        cpu.set_c_flag();
    } else {
        cpu.reset_c_flag();
    }

    // Rotación
    let mut nuevo_valor = cpu.a << 1;
    nuevo_valor = nuevo_valor & 0b1111_1110;
    if viejo_c_flag {
        nuevo_valor |= 0b0000_0001;
    }

    //maneja flags
    if nuevo_valor == 0 {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }
    cpu.reset_n_flag();
    cpu.reset_h_flag();

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

