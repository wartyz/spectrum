#![allow(non_snake_case)]

/*
Las instrucciones especiales de GameBoy acaban en GB
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


pub fn mete_funciones_normales(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones[0x00 as usize] = nop;
    cpu.funciones_txt[0x00 as usize] = nop_txt;
    cpu.funciones[0x01 as usize] = ld_bc_nn;
    cpu.funciones_txt[0x01 as usize] = ld_bc_nn_txt;
    cpu.funciones[0x02 as usize] = ldObcO_a;
    cpu.funciones_txt[0x02 as usize] = ldObcO_a_txt;
    cpu.funciones[0x03 as usize] = inc_bc;
    cpu.funciones_txt[0x03 as usize] = inc_bc_txt;
    cpu.funciones[0x04 as usize] = inc_b;
    cpu.funciones_txt[0x04 as usize] = inc_b_txt;
    cpu.funciones[0x05 as usize] = dec_b;
    cpu.funciones_txt[0x05 as usize] = dec_b_txt;
    cpu.funciones[0x06 as usize] = ld_b_n;
    cpu.funciones_txt[0x06 as usize] = ld_b_n_txt;
    cpu.funciones[0x07 as usize] = rlca;
    cpu.funciones_txt[0x07 as usize] = rlca_txt;
    // 08 LR35902->LD   (nn),SP         Z-80->EX  AF,AF'
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x08 as usize] = ldOnnO_spGB;
            cpu.funciones_txt[0x08 as usize] = ldOnnO_spGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x08 as usize] = ex_af_afp;
            cpu.funciones_txt[0x08 as usize] = ex_af_afp_txt;
        }
    };
    cpu.funciones[0x09 as usize] = add_hl_bc;
    cpu.funciones_txt[0x09 as usize] = add_hl_bc_txt;
    cpu.funciones[0x0A as usize] = ld_aObcO;
    cpu.funciones_txt[0x0A as usize] = ld_aObcO_txt;
    cpu.funciones[0x0B as usize] = dec_bc;
    cpu.funciones_txt[0x0B as usize] = dec_bc_txt;
    cpu.funciones[0x0C as usize] = inc_c;
    cpu.funciones_txt[0x0C as usize] = inc_c_txt;
    cpu.funciones[0x0D as usize] = dec_c;
    cpu.funciones_txt[0x0D as usize] = dec_c_txt;
    cpu.funciones[0x0E as usize] = ld_c_n;
    cpu.funciones_txt[0x0E as usize] = ld_c_n_txt;
    cpu.funciones[0x0F as usize] = rrca;
    cpu.funciones_txt[0x0F as usize] = rrca_txt;
    // *************************** 1 ***********************************
    cpu.funciones[0x10 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x10 as usize] = funcion_no_implementada;
    cpu.funciones[0x11 as usize] = ld_de_nn;
    cpu.funciones_txt[0x11 as usize] = ld_de_nn_txt;
    cpu.funciones[0x12 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x12 as usize] = funcion_no_implementada;
    cpu.funciones[0x13 as usize] = inc_de;
    cpu.funciones_txt[0x13 as usize] = inc_de_txt;
    cpu.funciones[0x14 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x14 as usize] = funcion_no_implementada;
    cpu.funciones[0x15 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x15 as usize] = funcion_no_implementada;
    cpu.funciones[0x16 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x16 as usize] = funcion_no_implementada;
    cpu.funciones[0x17 as usize] = rla;
    cpu.funciones_txt[0x17 as usize] = rla_txt;
    cpu.funciones[0x18 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x18 as usize] = funcion_no_implementada;
    cpu.funciones[0x19 as usize] = add_hl_de;
    cpu.funciones_txt[0x19 as usize] = add_hl_de_txt;
    cpu.funciones[0x1A as usize] = ld_aOdeO;
    cpu.funciones_txt[0x1A as usize] = ld_aOdeO_txt;
    cpu.funciones[0x1B as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x1B as usize] = funcion_no_implementada;
    cpu.funciones[0x1C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x1C as usize] = funcion_no_implementada;
    cpu.funciones[0x1D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x1D as usize] = funcion_no_implementada;
    cpu.funciones[0x1E as usize] = ld_d_n;
    cpu.funciones_txt[0x1E as usize] = ld_d_n_txt;
    cpu.funciones[0x1F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x1F as usize] = funcion_no_implementada;
    // *************************** 2 ***********************************
    cpu.funciones[0x20 as usize] = jr_nz_n;
    cpu.funciones_txt[0x20 as usize] = jr_nz_n_txt;
    cpu.funciones[0x21 as usize] = ld_hl_nn;
    cpu.funciones_txt[0x21 as usize] = ld_hl_nn_txt;
    // LR35902->LDI  (HL),A        Z-80->LD  (nn),HL
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x22 as usize] = ldiOhlO_aGB;
            cpu.funciones_txt[0x22 as usize] = ldiOhlO_aGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x22 as usize] = ldOnnO_hl;
            cpu.funciones_txt[0x22 as usize] = ldOnnO_hl_txt;
        }
    };
    cpu.funciones[0x23 as usize] = inc_hl;
    cpu.funciones_txt[0x23 as usize] = inc_hl_txt;
    cpu.funciones[0x24 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x24 as usize] = funcion_no_implementada;
    cpu.funciones[0x25 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x25 as usize] = funcion_no_implementada;
    cpu.funciones[0x26 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x26 as usize] = funcion_no_implementada;
    cpu.funciones[0x27 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x27 as usize] = funcion_no_implementada;
    cpu.funciones[0x28 as usize] = jr_z_n;
    cpu.funciones_txt[0x28 as usize] = jr_z_n_txt;
    cpu.funciones[0x29 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x29 as usize] = funcion_no_implementada;
    cpu.funciones[0x2A as usize] = ld_hlOnnO;
    cpu.funciones_txt[0x2A as usize] = ld_hlOnnO_txt;
    cpu.funciones[0x2B as usize] = dec_hl;
    cpu.funciones_txt[0x2B as usize] = dec_hl_txt;
    cpu.funciones[0x2C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x2C as usize] = funcion_no_implementada;
    cpu.funciones[0x2D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x2D as usize] = funcion_no_implementada;
    cpu.funciones[0x2E as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x2E as usize] = funcion_no_implementada;
    cpu.funciones[0x2F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x2F as usize] = funcion_no_implementada;
    // *************************** 3 ***********************************

    cpu.funciones[0x30 as usize] = jr_nc_n;
    cpu.funciones_txt[0x30 as usize] = jr_nc_n_txt;
    cpu.funciones[0x31 as usize] = ld_sp_nn;
    cpu.funciones_txt[0x31 as usize] = ld_sp_nn_txt;
    // LR35902->LDD  (HL),A    Z-80->LD  (nn),A
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x32 as usize] = lddOhlO_aGB;
            cpu.funciones_txt[0x32 as usize] = lddOhlO_aGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x32 as usize] = ldOnnO_a;
            cpu.funciones_txt[0x32 as usize] = ldOnnO_a_txt;
        }
    };


    cpu.funciones[0x33 as usize] = inc_sp;
    cpu.funciones_txt[0x33 as usize] = inc_sp_txt;
    cpu.funciones[0x34 as usize] = inc_OhlO;
    cpu.funciones_txt[0x34 as usize] = inc_OhlO_txt;
    cpu.funciones[0x35 as usize] = dec_OhlO;
    cpu.funciones_txt[0x35 as usize] = dec_OhlO_txt;
    cpu.funciones[0x36 as usize] = ld_OhlO_n;
    cpu.funciones_txt[0x36 as usize] = ld_OhlO_n_txt;
    cpu.funciones[0x37 as usize] = scf;
    cpu.funciones_txt[0x37 as usize] = scf_txt;
    cpu.funciones[0x38 as usize] = jr_c_n;
    cpu.funciones_txt[0x38 as usize] = jr_c_n_txt;
    cpu.funciones[0x39 as usize] = add_hl_sp;
    cpu.funciones_txt[0x39 as usize] = add_hl_sp_txt;

    // LR35902->LDD  A,(HL)     Z-80->LD  A,(nn)
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x3A as usize] = ldd_a_OhlOGB;
            cpu.funciones_txt[0x3A as usize] = ldd_a_OhlOGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x3A as usize] = ld_a_OnnO;
            cpu.funciones_txt[0x3A as usize] = ld_a_OnnO_txt;
        }
    };

    cpu.funciones[0x3B as usize] = dec_sp;
    cpu.funciones_txt[0x3B as usize] = dec_sp_txt;
    cpu.funciones[0x3C as usize] = inc_a;
    cpu.funciones_txt[0x3C as usize] = inc_a_txt;
    cpu.funciones[0x3D as usize] = dec_a;
    cpu.funciones_txt[0x3D as usize] = dec_a_txt;
    cpu.funciones[0x3E as usize] = ld_a_n;
    cpu.funciones_txt[0x3E as usize] = ld_a_n_txt;
    cpu.funciones[0x3F as usize] = ccf;
    cpu.funciones_txt[0x3F as usize] = ccf_txt;

    // *************************** 4 ***********************************
    cpu.funciones[0x40 as usize] = ld_b_b;
    cpu.funciones_txt[0x40 as usize] = ld_b_b_txt;
    cpu.funciones[0x41 as usize] = ld_b_c;
    cpu.funciones_txt[0x41 as usize] = ld_b_c_txt;
    cpu.funciones[0x42 as usize] = ld_b_d;
    cpu.funciones_txt[0x42 as usize] = ld_b_d_txt;
    cpu.funciones[0x43 as usize] = ld_b_e;
    cpu.funciones_txt[0x43 as usize] = ld_b_e_txt;
    cpu.funciones[0x44 as usize] = ld_b_h;
    cpu.funciones_txt[0x44 as usize] = ld_b_h_txt;
    cpu.funciones[0x45 as usize] = ld_b_l;
    cpu.funciones_txt[0x45 as usize] = ld_b_l_txt;
    cpu.funciones[0x46 as usize] = ld_b_OhlO;
    cpu.funciones_txt[0x46 as usize] = ld_b_OhlO_txt;
    cpu.funciones[0x47 as usize] = ld_b_a;
    cpu.funciones_txt[0x47 as usize] = ld_b_a_txt;
    cpu.funciones[0x48 as usize] = ld_c_b;
    cpu.funciones_txt[0x48 as usize] = ld_c_b_txt;
    cpu.funciones[0x49 as usize] = ld_c_c;
    cpu.funciones_txt[0x49 as usize] = ld_c_c_txt;
    cpu.funciones[0x4A as usize] = ld_c_d;
    cpu.funciones_txt[0x4A as usize] = ld_c_d_txt;
    cpu.funciones[0x4B as usize] = ld_c_e;
    cpu.funciones_txt[0x4B as usize] = ld_c_e_txt;
    cpu.funciones[0x4C as usize] = ld_c_h;
    cpu.funciones_txt[0x4C as usize] = ld_c_h_txt;
    cpu.funciones[0x4D as usize] = ld_c_l;
    cpu.funciones_txt[0x4D as usize] = ld_c_l_txt;
    cpu.funciones[0x4E as usize] = ld_c_OhlO;
    cpu.funciones_txt[0x4E as usize] = ld_c_OhlO_txt;
    cpu.funciones[0x4F as usize] = ld_c_a;
    cpu.funciones_txt[0x4F as usize] = ld_c_a_txt;
    // *************************** 5 ***********************************
    cpu.funciones[0x50 as usize] = ld_d_b;
    cpu.funciones_txt[0x50 as usize] = ld_d_b_txt;
    cpu.funciones[0x51 as usize] = ld_d_c;
    cpu.funciones_txt[0x51 as usize] = ld_d_c_txt;
    cpu.funciones[0x52 as usize] = ld_d_d;
    cpu.funciones_txt[0x52 as usize] = ld_d_d_txt;
    cpu.funciones[0x53 as usize] = ld_d_e;
    cpu.funciones_txt[0x53 as usize] = ld_d_e_txt;
    cpu.funciones[0x54 as usize] = ld_d_h;
    cpu.funciones_txt[0x54 as usize] = ld_d_h_txt;
    cpu.funciones[0x55 as usize] = ld_d_l;
    cpu.funciones_txt[0x55 as usize] = ld_d_l_txt;
    cpu.funciones[0x56 as usize] = ld_d_OhlO;
    cpu.funciones_txt[0x56 as usize] = ld_d_OhlO_txt;
    cpu.funciones[0x57 as usize] = ld_d_a;
    cpu.funciones_txt[0x57 as usize] = ld_d_a_txt;
    cpu.funciones[0x58 as usize] = ld_e_b;
    cpu.funciones_txt[0x58 as usize] = ld_e_b_txt;
    cpu.funciones[0x59 as usize] = ld_e_c;
    cpu.funciones_txt[0x59 as usize] = ld_e_c_txt;
    cpu.funciones[0x5A as usize] = ld_e_d;
    cpu.funciones_txt[0x5A as usize] = ld_e_d_txt;
    cpu.funciones[0x5B as usize] = ld_e_e;
    cpu.funciones_txt[0x5B as usize] = ld_e_e_txt;
    cpu.funciones[0x5C as usize] = ld_e_h;
    cpu.funciones_txt[0x5C as usize] = ld_e_h_txt;
    cpu.funciones[0x5D as usize] = ld_e_l;
    cpu.funciones_txt[0x5D as usize] = ld_e_l_txt;
    cpu.funciones[0x5E as usize] = ld_e_OhlO;
    cpu.funciones_txt[0x5E as usize] = ld_eOhlO_txt;
    cpu.funciones[0x5F as usize] = ld_e_a;
    cpu.funciones_txt[0x5F as usize] = ld_e_a_txt;

    // *************************** 6 ***********************************
    cpu.funciones[0x60 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x60 as usize] = funcion_no_implementada;
    cpu.funciones[0x61 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x61 as usize] = funcion_no_implementada;
    cpu.funciones[0x62 as usize] = ld_h_d;
    cpu.funciones_txt[0x62 as usize] = ld_h_d_txt;
    cpu.funciones[0x63 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x63 as usize] = funcion_no_implementada;
    cpu.funciones[0x64 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x64 as usize] = funcion_no_implementada;
    cpu.funciones[0x65 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x65 as usize] = funcion_no_implementada;
    cpu.funciones[0x66 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x66 as usize] = funcion_no_implementada;
    cpu.funciones[0x67 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x67 as usize] = funcion_no_implementada;
    cpu.funciones[0x68 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x68 as usize] = funcion_no_implementada;
    cpu.funciones[0x69 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x69 as usize] = funcion_no_implementada;
    cpu.funciones[0x6A as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x6A as usize] = funcion_no_implementada;
    cpu.funciones[0x6B as usize] = ld_l_e;
    cpu.funciones_txt[0x6B as usize] = ld_l_e_txt;
    cpu.funciones[0x6C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x6C as usize] = funcion_no_implementada;
    cpu.funciones[0x6D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x6D as usize] = funcion_no_implementada;
    cpu.funciones[0x6E as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x6E as usize] = funcion_no_implementada;
    cpu.funciones[0x6F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x6F as usize] = funcion_no_implementada;
    // *************************** 7 ***********************************
    cpu.funciones[0x70 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x70 as usize] = funcion_no_implementada;
    cpu.funciones[0x71 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x71 as usize] = funcion_no_implementada;
    cpu.funciones[0x72 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x72 as usize] = funcion_no_implementada;
    cpu.funciones[0x73 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x73 as usize] = funcion_no_implementada;
    cpu.funciones[0x74 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x74 as usize] = funcion_no_implementada;
    cpu.funciones[0x75 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x75 as usize] = funcion_no_implementada;
    cpu.funciones[0x76 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x76 as usize] = funcion_no_implementada;
    cpu.funciones[0x77 as usize] = ldOhlO_a;
    cpu.funciones_txt[0x77 as usize] = ldOhlO_a_txt;
    cpu.funciones[0x78 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x78 as usize] = funcion_no_implementada;
    cpu.funciones[0x79 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x79 as usize] = funcion_no_implementada;
    cpu.funciones[0x7A as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x7A as usize] = funcion_no_implementada;
    cpu.funciones[0x7B as usize] = ld_a_e;
    cpu.funciones_txt[0x7B as usize] = ld_a_e_txt;
    cpu.funciones[0x7C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x7C as usize] = funcion_no_implementada;
    cpu.funciones[0x7D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x7D as usize] = funcion_no_implementada;
    cpu.funciones[0x7E as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x7E as usize] = funcion_no_implementada;
    cpu.funciones[0x7F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x7F as usize] = funcion_no_implementada;

    // *************************** 8 ***********************************
    cpu.funciones[0x80 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x80 as usize] = funcion_no_implementada;
    cpu.funciones[0x81 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x81 as usize] = funcion_no_implementada;
    cpu.funciones[0x82 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x82 as usize] = funcion_no_implementada;
    cpu.funciones[0x83 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x83 as usize] = funcion_no_implementada;
    cpu.funciones[0x84 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x84 as usize] = funcion_no_implementada;
    cpu.funciones[0x85 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x85 as usize] = funcion_no_implementada;
    cpu.funciones[0x86 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x86 as usize] = funcion_no_implementada;
    cpu.funciones[0x87 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x87 as usize] = funcion_no_implementada;
    cpu.funciones[0x88 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x88 as usize] = funcion_no_implementada;
    cpu.funciones[0x89 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x89 as usize] = funcion_no_implementada;
    cpu.funciones[0x8A as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8A as usize] = funcion_no_implementada;
    cpu.funciones[0x8B as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8B as usize] = funcion_no_implementada;
    cpu.funciones[0x8C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8C as usize] = funcion_no_implementada;
    cpu.funciones[0x8D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8D as usize] = funcion_no_implementada;
    cpu.funciones[0x8E as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8E as usize] = funcion_no_implementada;
    cpu.funciones[0x8F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x8F as usize] = funcion_no_implementada;
    // *************************** 9 ***********************************
    cpu.funciones[0x90 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x90 as usize] = funcion_no_implementada;
    cpu.funciones[0x91 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x91 as usize] = funcion_no_implementada;
    cpu.funciones[0x92 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x92 as usize] = funcion_no_implementada;
    cpu.funciones[0x93 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x93 as usize] = funcion_no_implementada;
    cpu.funciones[0x94 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x94 as usize] = funcion_no_implementada;
    cpu.funciones[0x95 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x95 as usize] = funcion_no_implementada;
    cpu.funciones[0x96 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x96 as usize] = funcion_no_implementada;
    cpu.funciones[0x97 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x97 as usize] = funcion_no_implementada;
    cpu.funciones[0x98 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x98 as usize] = funcion_no_implementada;
    cpu.funciones[0x99 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x99 as usize] = funcion_no_implementada;
    cpu.funciones[0x9A as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9A as usize] = funcion_no_implementada;
    cpu.funciones[0x9B as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9B as usize] = funcion_no_implementada;
    cpu.funciones[0x9C as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9C as usize] = funcion_no_implementada;
    cpu.funciones[0x9D as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9D as usize] = funcion_no_implementada;
    cpu.funciones[0x9E as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9E as usize] = funcion_no_implementada;
    cpu.funciones[0x9F as usize] = funcion_no_implementada;
    cpu.funciones_txt[0x9F as usize] = funcion_no_implementada;
    // *************************** A ***********************************
    cpu.funciones[0xA0 as usize] = and_b;
    cpu.funciones_txt[0xA0 as usize] = and_b_txt;
    cpu.funciones[0xA1 as usize] = and_c;
    cpu.funciones_txt[0xA1 as usize] = and_c_txt;
    cpu.funciones[0xA2 as usize] = and_d;
    cpu.funciones_txt[0xA2 as usize] = and_d_txt;
    cpu.funciones[0xA3 as usize] = and_e;
    cpu.funciones_txt[0xA3 as usize] = and_e_txt;
    cpu.funciones[0xA4 as usize] = and_h;
    cpu.funciones_txt[0xA4 as usize] = and_h_txt;
    cpu.funciones[0xA5 as usize] = and_l;
    cpu.funciones_txt[0xA5 as usize] = and_l_txt;
    cpu.funciones[0xA6 as usize] = and_OhlO;
    cpu.funciones_txt[0xA6 as usize] = and_OhlO_txt;
    cpu.funciones[0xA7 as usize] = and_a;
    cpu.funciones_txt[0xA7 as usize] = and_a_txt;
    cpu.funciones[0xA8 as usize] = xor_b;
    cpu.funciones_txt[0xA8 as usize] = xor_b_txt;
    cpu.funciones[0xA9 as usize] = xor_c;
    cpu.funciones_txt[0xA9 as usize] = xor_c_txt;
    cpu.funciones[0xAA as usize] = xor_d;
    cpu.funciones_txt[0xAA as usize] = xor_d_txt;
    cpu.funciones[0xAB as usize] = xor_e;
    cpu.funciones_txt[0xAB as usize] = xor_e_txt;
    cpu.funciones[0xAC as usize] = xor_h;
    cpu.funciones_txt[0xAC as usize] = xor_h_txt;
    cpu.funciones[0xAD as usize] = xor_l;
    cpu.funciones_txt[0xAD as usize] = xor_l_txt;
    cpu.funciones[0xAE as usize] = xor_OhlO;
    cpu.funciones_txt[0xAE as usize] = xor_OhlO_txt;
    cpu.funciones[0xAF as usize] = xor_a;
    cpu.funciones_txt[0xAF as usize] = xor_a_txt;
    // *************************** B ***********************************
    cpu.funciones[0xB0 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB0 as usize] = funcion_no_implementada;
    cpu.funciones[0xB1 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB1 as usize] = funcion_no_implementada;
    cpu.funciones[0xB2 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB2 as usize] = funcion_no_implementada;
    cpu.funciones[0xB3 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB3 as usize] = funcion_no_implementada;
    cpu.funciones[0xB4 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB4 as usize] = funcion_no_implementada;
    cpu.funciones[0xB5 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB5 as usize] = funcion_no_implementada;
    cpu.funciones[0xB6 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB6 as usize] = funcion_no_implementada;
    cpu.funciones[0xB7 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB7 as usize] = funcion_no_implementada;
    cpu.funciones[0xB8 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB8 as usize] = funcion_no_implementada;
    cpu.funciones[0xB9 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xB9 as usize] = funcion_no_implementada;
    cpu.funciones[0xBA as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xBA as usize] = funcion_no_implementada;
    cpu.funciones[0xBB as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xBB as usize] = funcion_no_implementada;
    cpu.funciones[0xBC as usize] = cp_h;
    cpu.funciones_txt[0xBC as usize] = cp_h_txt;
    cpu.funciones[0xBD as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xBD as usize] = funcion_no_implementada;
    cpu.funciones[0xBE as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xBE as usize] = funcion_no_implementada;
    cpu.funciones[0xBF as usize] = cp_h;
    cpu.funciones_txt[0xBF as usize] = cp_h_txt;

    // *************************** C ***********************************
    cpu.funciones[0xC0 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC0 as usize] = funcion_no_implementada;
    cpu.funciones[0xC1 as usize] = pop_bc;
    cpu.funciones_txt[0xC1 as usize] = pop_bc_txt;
    cpu.funciones[0xC2 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC2 as usize] = funcion_no_implementada;
    cpu.funciones[0xC3 as usize] = jp_nn;
    cpu.funciones_txt[0xC3 as usize] = jp_nn_txt;
    cpu.funciones[0xC4 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC4 as usize] = funcion_no_implementada;
    cpu.funciones[0xC5 as usize] = push_bc;
    cpu.funciones_txt[0xC5 as usize] = push_bc_txt;
    cpu.funciones[0xC6 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC6 as usize] = funcion_no_implementada;
    cpu.funciones[0xC7 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC7 as usize] = funcion_no_implementada;
    cpu.funciones[0xC8 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xC8 as usize] = funcion_no_implementada;
    cpu.funciones[0xC9 as usize] = ret;
    cpu.funciones_txt[0xC9 as usize] = ret_txt;
    cpu.funciones[0xCA as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xCA as usize] = funcion_no_implementada;
    cpu.funciones[0xCB as usize] = CB; // Extensión
    cpu.funciones_txt[0xCB as usize] = CB_txt;
    cpu.funciones[0xCC as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xCC as usize] = funcion_no_implementada;
    cpu.funciones[0xCD as usize] = call_nn;
    cpu.funciones_txt[0xCD as usize] = call_nn_txt;
    cpu.funciones[0xCE as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xCE as usize] = funcion_no_implementada;
    cpu.funciones[0xCF as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xCF as usize] = funcion_no_implementada;
    // *************************** D ***********************************
    cpu.funciones[0xD0 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD0 as usize] = funcion_no_implementada;
    cpu.funciones[0xD1 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD1 as usize] = funcion_no_implementada;
    cpu.funciones[0xD2 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD2 as usize] = funcion_no_implementada;
    cpu.funciones[0xD3 as usize] = out_OnO_a;
    cpu.funciones_txt[0xD3 as usize] = out_OnO_a_txt;
    cpu.funciones[0xD4 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD4 as usize] = funcion_no_implementada;
    cpu.funciones[0xD5 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD5 as usize] = funcion_no_implementada;
    cpu.funciones[0xD6 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD6 as usize] = funcion_no_implementada;
    cpu.funciones[0xD7 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD7 as usize] = funcion_no_implementada;
    cpu.funciones[0xD8 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xD8 as usize] = funcion_no_implementada;


    // D9 LR35902->RETI        Z-80->EXX
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xD9 as usize] = retiGB;
            cpu.funciones_txt[0xD9 as usize] = retiGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xD9 as usize] = exx;
            cpu.funciones_txt[0xD9 as usize] = exx_txt;
        }
    };

    cpu.funciones[0xDA as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDA as usize] = funcion_no_implementada;
    cpu.funciones[0xDB as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDB as usize] = funcion_no_implementada;
    cpu.funciones[0xDC as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDC as usize] = funcion_no_implementada;
    cpu.funciones[0xDD as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDD as usize] = funcion_no_implementada;
    cpu.funciones[0xDE as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDE as usize] = funcion_no_implementada;
    cpu.funciones[0xDF as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xDF as usize] = funcion_no_implementada;


    // *************************** E ***********************************
    // LR35902->LD (FF00+u8),A         Z-80->RET NV
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE0 as usize] = ldOff00_m_nO_aGB;
            cpu.funciones_txt[0xE0 as usize] = ldOff00_m_nO_aGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE0 as usize] = ret_po;
            cpu.funciones_txt[0xE0 as usize] = ret_po_txt;
        }
    }
    cpu.funciones[0xE1 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE1 as usize] = funcion_no_implementada;
    // LR35902->LD(C),A         Z-80->JP NV,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE2 as usize] = ldOff00_m_cO_aGB;
            cpu.funciones_txt[0xE2 as usize] = ldOff00_m_cO_aGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE2 as usize] = jp_po_nn;
            cpu.funciones_txt[0xE2 as usize] = jp_po_nn_txt;
        }
    }
    cpu.funciones[0xE3 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE3 as usize] = funcion_no_implementada;
    cpu.funciones[0xE4 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE4 as usize] = funcion_no_implementada;
    cpu.funciones[0xE5 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE5 as usize] = funcion_no_implementada;
    cpu.funciones[0xE6 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE6 as usize] = funcion_no_implementada;
    cpu.funciones[0xE7 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE7 as usize] = funcion_no_implementada;
    cpu.funciones[0xE8 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE8 as usize] = funcion_no_implementada;
    cpu.funciones[0xE9 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xE9 as usize] = funcion_no_implementada;
    // LR35902->LD (nn),A         Z80->JP  V,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xEA as usize] = ldOnnO_aGB;
            cpu.funciones_txt[0xEA as usize] = ldOnnO_aGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xEA as usize] = jp_pe_nn;
            cpu.funciones_txt[0xEA as usize] = jp_pe_nn_txt;
        }
    }
    cpu.funciones[0xEB as usize] = ex_de_hl;
    cpu.funciones_txt[0xEB as usize] = ex_de_hl_txt;
    cpu.funciones[0xEC as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xEC as usize] = funcion_no_implementada;
    cpu.funciones[0xED as usize] = ED; // Extensión
    cpu.funciones_txt[0xED as usize] = ED_txt;


    // *************************** F ***********************************
    // LR35902->LDH  A,(n)           Z-80->RET P
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xF0 as usize] = ld_a_Off00_m_nOGB;
            cpu.funciones_txt[0xF0 as usize] = ld_a_Off00_m_nOGB_txt;
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xF0 as usize] = ret_p;
            cpu.funciones_txt[0xF0 as usize] = ret_p_txt;
        }
    }
    cpu.funciones[0xF1 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF1 as usize] = funcion_no_implementada;
    cpu.funciones[0xF2 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF2 as usize] = funcion_no_implementada;
    cpu.funciones[0xF3 as usize] = di;
    cpu.funciones_txt[0xF3 as usize] = di_txt;
    cpu.funciones[0xF4 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF4 as usize] = funcion_no_implementada;
    cpu.funciones[0xF5 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF5 as usize] = funcion_no_implementada;
    cpu.funciones[0xF6 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF6 as usize] = funcion_no_implementada;
    cpu.funciones[0xF7 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF7 as usize] = funcion_no_implementada;
    cpu.funciones[0xF8 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF8 as usize] = funcion_no_implementada;
    cpu.funciones[0xF9 as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xF9 as usize] = funcion_no_implementada;
    cpu.funciones[0xFA as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xFA as usize] = funcion_no_implementada;
    cpu.funciones[0xFB as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xFB as usize] = funcion_no_implementada;
    cpu.funciones[0xFC as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xFC as usize] = funcion_no_implementada;
    cpu.funciones[0xFD as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xFD as usize] = funcion_no_implementada;
    cpu.funciones[0xFE as usize] = cp_n;
    cpu.funciones_txt[0xFE as usize] = cp_n_txt;
    cpu.funciones[0xFF as usize] = funcion_no_implementada;
    cpu.funciones_txt[0xFF as usize] = funcion_no_implementada;
}

pub fn funcion_no_implementada(cpu: &mut CPU) {
    panic!(format!("Funcion normal no implementada\n PC=#{:04X} r0=#{:02X},r1=#{:02X},r2#{:02X},\
    r3=#{:02X}\n",
                   cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3));
}

// O = ()     p = '    m = +       n = valor hex de 8 bits
// *************************** 0 ***********************************
// 0x00
pub fn nop(cpu: &mut CPU) {
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn nop_txt(cpu: &mut CPU) {
    cpu.texto(&format!("NOP opcode = #{:02X}", cpu.r0));
}

// 0x01 NN NN
pub fn ld_bc_nn(cpu: &mut CPU) {
    cpu.c = cpu.r1;
    cpu.b = cpu.r2;

    cpu.t += 10;
    cpu.pc += 3;
}

pub fn ld_bc_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD BC,#{:04X}", cpu.r1r2)); }

// 0x02
pub fn ldObcO_a(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.b, cpu.c);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += 7;
    cpu.pc += 1;
}

pub fn ldObcO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (BC),A"));
}

// 0x03
pub fn inc_bc(cpu: &mut CPU) {
    panic!("0x03 inc_bc: funcion no implementada");
}

pub fn inc_bc_txt(cpu: &mut CPU) {
    panic!("0x03 inc_bc_txt: funcion no implementada");
}

// 0x04
pub fn inc_b(cpu: &mut CPU) {
    cpu.b = cpu.inc_8bits(cpu.b);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn inc_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC B"));
}

// 0x05
pub fn dec_b(cpu: &mut CPU) {
    cpu.b = cpu.dec_8bits(cpu.b);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn dec_b_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC B")); }

// 0x06
pub fn ld_b_n(cpu: &mut CPU) {
    cpu.b = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_b_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,#{:02X}", cpu.r1)); }

// 0x07
pub fn rlca(cpu: &mut CPU) {
    panic!("0x07 rlca: funcion no implementada");
}

pub fn rlca_txt(cpu: &mut CPU) {
    panic!("0x07 rlca_txt: funcion no implementada");
}

// 0x08 Difiere según procesador (LR35902->LD(NN),SP)
pub fn ldOnnO_spGB(cpu: &mut CPU) {
    cpu.mem.escribe_2bytes_en_mem(cpu.r1r2, cpu.sp);
}

pub fn ldOnnO_spGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),SP", cpu.r1r2));
}

// 0x08 Difiere según procesador (Z80->EX AF,AF')
pub fn ex_af_afp(cpu: &mut CPU) {
    panic!("0x08 ex_af_afp: funcion no implementada");
}

pub fn ex_af_afp_txt(cpu: &mut CPU) {
    panic!("0x08 ex_af_afp_txt: funcion no implementada");
}

// 0x09
pub fn add_hl_bc(cpu: &mut CPU) {
    panic!("0x09 add_hl_bc: funcion no implementada");
}

pub fn add_hl_bc_txt(cpu: &mut CPU) {
    panic!("0x09 add_hl_bc_txt: funcion no implementada");
}

// 0x0A
pub fn ld_aObcO(cpu: &mut CPU) { panic!("0x0A ld_aObcO: funcion no implementada"); }

pub fn ld_aObcO_txt(cpu: &mut CPU) { panic!("0x0A ld_aObcO_txt: funcion no implementada"); }

// 0x0B
pub fn dec_bc(cpu: &mut CPU) { panic!("0x0B dec_bc: funcion no implementada"); }

pub fn dec_bc_txt(cpu: &mut CPU) { panic!("0x0B dec_bc_txt: funcion no implementada"); }

// 0x0C
pub fn inc_c(cpu: &mut CPU) {
    cpu.c = cpu.inc_8bits(cpu.c);

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn inc_c_txt(cpu: &mut CPU) { cpu.texto(&format!("INC C")); }

// 0x0D
pub fn dec_c(cpu: &mut CPU) {
    cpu.c = cpu.dec_8bits(cpu.c);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn dec_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC C"));
}

// 0x0E
pub fn ld_c_n(cpu: &mut CPU) {
    cpu.c = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_c_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,#{:02X}", cpu.r1)); }

// 0x0F
pub fn rrca(cpu: &mut CPU) {
    let bit0: bool = (0b0000_0001 & cpu.a) != 0;
    if bit0 {
        cpu.set_c_flag();
    } else {
        cpu.reset_c_flag();
    }

    // Rotación
    let mut nuevo_valor = cpu.a >> 1;
    nuevo_valor = nuevo_valor & 0b0111_1111;
    if bit0 {
        nuevo_valor |= 0b1000_0000;
    }

    //maneja flags
    if nuevo_valor == 0 {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }
    cpu.reset_n_flag();
    cpu.reset_h_flag();

    cpu.pc += 1;
    cpu.t += 4;
}

pub fn rrca_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RRCA"));
}
// *************************** 1 ***********************************

// 0x10
pub fn djnz_n(cpu: &mut CPU) { panic!("0x10 djnz_n: funcion no implementada"); }

// 0x11 NN NN
pub fn ld_de_nn(cpu: &mut CPU) {
    cpu.d = cpu.r2;
    cpu.e = cpu.r1;

    cpu.t += 10;
    cpu.pc += 3;
}

pub fn ld_de_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD DE,#{:04X}", cpu.r1r2)); }

// 0x12
pub fn ldOdeO_a(cpu: &mut CPU) { panic!("0x12 ldOdeO_a: funcion no implementada"); }

// 0x13
pub fn inc_de(cpu: &mut CPU) {
    let mut de = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);

    let resultado = cpu.desconcatena_un_u16_en_dos_u8(de.wrapping_add(1));
    cpu.d = resultado.0;
    cpu.e = resultado.1;

    cpu.pc += 1;
    cpu.t += 13;
}

pub fn inc_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC DE"));
}

// 0x14
pub fn inc_d(cpu: &mut CPU) { panic!("0x14 inc_d: funcion no implementada"); }

// 0x15
pub fn dec_d(cpu: &mut CPU) { panic!("0x15 dec_d: funcion no implementada"); }

// 0x16
pub fn ld_d_n(cpu: &mut CPU) {
    cpu.d = cpu.r1;
    cpu.pc += 2;
    cpu.t += 7;
}

pub fn ld_d_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD D #{:02X}", cpu.r1));
}


// 0x17 C<-76543210<-
//      |___________|
pub fn rla(cpu: &mut CPU) {
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

    cpu.pc += 1;
    cpu.t += 4;
}

pub fn rla_txt(cpu: &mut CPU) { cpu.texto(&format!("RLA")); }

// 0x18
pub fn jr_n(cpu: &mut CPU) { panic!("0x18 jr_n: funcion no implementada"); }

// 0x19
pub fn add_hl_de(cpu: &mut CPU) {
    // TODO: Faltan flags
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let de16 = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    let hl16 = hl16.wrapping_add(de16);

    if hl16 == 0 {
        cpu.set_z_flag();
    } else {
        cpu.reset_z_flag();
    }
    cpu.pc += 1;

    cpu.t += 11;
}

pub fn add_hl_de_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,DE")); }

// 0x1A
pub fn ld_aOdeO(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);
    cpu.pc += 1;

    cpu.t += 7;
}

pub fn ld_aOdeO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A(DE)")); }

// 0x1B
pub fn dec_de(cpu: &mut CPU) { panic!("0x1B dec_de: funcion no implementada"); }

// 0x1C
pub fn inc_e(cpu: &mut CPU) { panic!("0x1C inc_e: funcion no implementada"); }

// 0x1D
pub fn dec_e(cpu: &mut CPU) { panic!("0x1D dec_e: funcion no implementada"); }

// 0x1E
pub fn ld_e_n(cpu: &mut CPU) { panic!("0x1E ld_e_n: funcion no implementada"); }

// 0x1F
pub fn rra(cpu: &mut CPU) { panic!("0x1F rra: funcion no implementada"); }

// *************************** 2 ***********************************
// 0x20 NN
fn jr_nz_n(cpu: &mut CPU) {
    //cpu.pc = cpu.pc + 2; // Parece ser que es necesario para salto relativo
    let salto = cpu.pc.wrapping_add(2);
    if !cpu.get_z_flag() {
        //cpu.pc = cpu.pc.wrapping_add(cpu.r1 as u16);
        cpu.pc = salto.wrapping_add((cpu.r1 as i8) as u16);

        cpu.t += 13;
    } else {
        cpu.pc += 2;
        cpu.t += 8;
    }
}

fn jr_nz_n_txt(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    cpu.texto(&format!("JR NZ #{:04X}", salto.wrapping_add((cpu.r1 as i8) as u16)));
}

// 0x21 NN NN
pub fn ld_hl_nn(cpu: &mut CPU) {
    cpu.h = cpu.r2;
    cpu.l = cpu.r1;

    cpu.pc += 3;
    cpu.t += 10;
}

pub fn ld_hl_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL#{:04X}", cpu.r1r2)); }


// 0x22 Difiere según procesador (LR35902->LDI  (HL),A)
pub fn ldiOhlO_aGB(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.mem.escribe_byte_en_mem(hl16, cpu.a);

    let resultado = cpu.desconcatena_un_u16_en_dos_u8(hl16.wrapping_add(1));
    cpu.h = resultado.0;
    cpu.l = resultado.1;

    cpu.pc += 1;

    cpu.t += 8;
}

pub fn ldiOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDI(HL),A")); }

// 0x22 Difiere según procesador (Z80->LD  (nn),HL)
pub fn ldOnnO_hl(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.l);
    cpu.mem.escribe_byte_en_mem(cpu.r1r2 + 1, cpu.h);

    cpu.pc += 3;
    cpu.t += 16;
}

pub fn ldOnnO_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),HL", cpu.r1r2));
}

// 0x23
pub fn inc_hl(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);

    let resultado = cpu.desconcatena_un_u16_en_dos_u8(hl16.wrapping_add(1));
    cpu.h = resultado.0;
    cpu.l = resultado.1;

    cpu.pc += 1;
    cpu.t += 6;
}

pub fn inc_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("INC HL")); }

// 0x24
pub fn inc_h(cpu: &mut CPU) { panic!("0x24 inc_h: funcion no implementada"); }

pub fn inc_h_txt(cpu: &mut CPU) { panic!("0x24 inc_h_txt: funcion no implementada"); }

// 0x25
pub fn dec_h(cpu: &mut CPU) { panic!("0x25 dec_h: funcion no implementada"); }

pub fn dec_h_txt(cpu: &mut CPU) { panic!("0x25 dec_h_txt: funcion no implementada"); }


// 0x26
pub fn ld_h_n(cpu: &mut CPU) { panic!("0x26 ld_h_n: funcion no implementada"); }

pub fn ld_h_n_txt(cpu: &mut CPU) { panic!("0x26 ld_h_n_txt: funcion no implementada"); }

// 0x27
pub fn daa(cpu: &mut CPU) { panic!("0x27 daa: funcion no implementada"); }

pub fn daa_txt(cpu: &mut CPU) { panic!("0x27_txt daa: funcion no implementada"); }

// 0x28
pub fn jr_z_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    if cpu.get_z_flag() {
        cpu.pc = salto.wrapping_add((cpu.r1 as i8) as u16);
        cpu.t += 12;
    } else {
        cpu.pc += 2;
        cpu.t += 7;
    }
}

pub fn jr_z_n_txt(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    cpu.texto(&format!("JR Z#{:02X})", salto.wrapping_add((cpu.r1 as i8) as u16)));
}

// 0x29
pub fn add_hl_hl(cpu: &mut CPU) { panic!("0x29 add_hl_hl: funcion no implementada"); }

pub fn add_hl_hl_txt(cpu: &mut CPU) { panic!("0x29 add_hl_hl_txt: funcion no implementada"); }


// 0x2A NN NN
pub fn ld_hlOnnO(cpu: &mut CPU) {
    cpu.l = cpu.mem.lee_byte_de_mem(cpu.r1r2);
    cpu.h = cpu.mem.lee_byte_de_mem(cpu.r1r2 + 1);

    cpu.t += 16;
    cpu.pc += 3;
}

pub fn ld_hlOnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL(#{:04X})", cpu.r1r2)); }

// 0x2B
pub fn dec_hl(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);

    let resultado = cpu.desconcatena_un_u16_en_dos_u8(hl16.wrapping_sub(1));
    cpu.h = resultado.0;
    cpu.l = resultado.1;


    cpu.pc += 1;
    cpu.t += 6;
}

pub fn dec_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC HL")); }

// 0x2C
pub fn inc_l(cpu: &mut CPU) { panic!("0x2C inc_l: funcion no implementada"); }

// 0x2D
pub fn dec_l(cpu: &mut CPU) { panic!("0x2D dec_l: funcion no implementada"); }

// 0x2E
pub fn ld_l_n(cpu: &mut CPU) { panic!("0x2E ld_l_n: funcion no implementada"); }

// 0x2F
pub fn cpl(cpu: &mut CPU) { panic!("0x2F cpl: funcion no implementada"); }

// *************************** 3 ***********************************

// 0x30
pub fn jr_nc_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    if !cpu.get_c_flag() {
        cpu.pc = salto.wrapping_add((cpu.r1 as i8) as u16);

        cpu.t += 12;
    } else {
        cpu.pc += 2;
        cpu.t += 7;
    }
}

pub fn jr_nc_n_txt(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    cpu.texto(&format!("JR NC #{:04X}", salto.wrapping_add((cpu.r1 as i8) as u16)));
}

// 0x31
pub fn ld_sp_nn(cpu: &mut CPU) {
    cpu.sp = cpu.r1r2; // LD SP,d16

    cpu.pc += 3;
    cpu.t += 10;
}

pub fn ld_sp_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP#{:04X}", cpu.r1r2));
}

// 0x32 Difiere según procesador (LR35902->LDD  (HL),A)
pub fn lddOhlO_aGB(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.mem.escribe_byte_en_mem(hl16, cpu.a);

    let resultado = cpu.desconcatena_un_u16_en_dos_u8(hl16.wrapping_sub(1));
    cpu.h = resultado.0;
    cpu.l = resultado.1;

    cpu.pc += 1;
    cpu.t += 8;
}

pub fn lddOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDD (HL),A")); }

// 0x32 Difiere según procesador (Z80->LD  (nn),A)
pub fn ldOnnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.pc += 3;
    cpu.t += 13;
}

pub fn ldOnnO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}

// 0x33
pub fn inc_sp(cpu: &mut CPU) {
    cpu.sp = cpu.sp.wrapping_add(1);

    cpu.t += 6;
    cpu.pc += 1;
}

pub fn inc_sp_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC SP"));
}

// 0x34
pub fn inc_OhlO(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);

    let mut valor = cpu.mem.lee_byte_de_mem(hl16);
    valor = valor.wrapping_add(1);
    cpu.mem.escribe_byte_en_mem(hl16, valor);

    cpu.t += 11;
    cpu.pc += 1;
}

pub fn inc_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC(HL)"));
}

// 0x35
pub fn dec_OhlO(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);

    let mut valor = cpu.mem.lee_byte_de_mem(hl16);
    valor = valor.wrapping_sub(1);
    cpu.mem.escribe_byte_en_mem(hl16, valor);

    cpu.t += 11;
    cpu.pc += 1;
}

pub fn dec_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC(HL)"));
}

// 0x36 NN
pub fn ld_OhlO_n(cpu: &mut CPU) {
    let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.mem.escribe_byte_en_mem(hl16, cpu.r1);

    cpu.t += 10;
    cpu.pc += 2;
}

pub fn ld_OhlO_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL)#{:02X}", cpu.r1)); }

// 0x37
pub fn scf(cpu: &mut CPU) {
    cpu.set_c_flag();
    cpu.reset_h_flag();
    cpu.reset_n_flag();

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn scf_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SCF"));
}


// 0x38
pub fn jr_c_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(2);
    if cpu.get_c_flag() {
        cpu.pc = salto.wrapping_add((cpu.r1 as i8) as u16);
        cpu.t += 12;
    } else {
        cpu.pc += 2;
        cpu.t += 7;
    }
}

pub fn jr_c_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("JR C #{:02X}", cpu.r1));
}


// 0x39
pub fn add_hl_sp(cpu: &mut CPU) {
    panic!("0x39 add_hl_sp funcion no implementada");
}

pub fn add_hl_sp_txt(cpu: &mut CPU) { panic!("0x39 add_hl_sp_txt funcion no implementada"); }

// 0x3A Difiere según procesador (LR35902->LDD A(HL))
pub fn ldd_a_OhlOGB(cpu: &mut CPU) {
    panic!("0x3A ldd_a_OhlOGB funcion no implementada");
}

pub fn ldd_a_OhlOGB_txt(cpu: &mut CPU) {
    panic!("0x3A ldd_a_OhlOGB_txt funcion no implementada");
}

// 0x3A Difiere según procesador (Z80->LD a(NN))
pub fn ld_a_OnnO(cpu: &mut CPU) {
    panic!("0x3A ld_a_OnnO funcion no implementada");
}

pub fn ld_a_OnnO_txt(cpu: &mut CPU) {
    //cpu.texto(&format!("LD A(#{:04X})",cpu.r1r2));
    panic!("0x3A ld_a_OnnO_txt funcion no implementada");
}

// 0x3B
pub fn dec_sp(cpu: &mut CPU) {
    panic!("0x3B dec_sp funcion no implementada");
}

pub fn dec_sp_txt(cpu: &mut CPU) {
    //cpu.texto(&format!("DEC SP"));
    panic!("0x3B dec_sp_txt funcion no implementada");
}

// 0x3C
pub fn inc_a(cpu: &mut CPU) {
    panic!("0x3C inc_a funcion no implementada");
}

pub fn inc_a_txt(cpu: &mut CPU) {
    //cpu.texto(&format!("INC A"));
    panic!("0x3C inc_a_txt funcion no implementada");
}

// 0x3D
pub fn dec_a(cpu: &mut CPU) {
    cpu.a = cpu.dec_8bits(cpu.a);

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn dec_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC A"));
}

// 0x3E
pub fn ld_a_n(cpu: &mut CPU) {
    cpu.a = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_a_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,#{:02X}", cpu.r1));
}

// 0x3F
pub fn ccf(cpu: &mut CPU) {
    panic!("0x3F ccf funcion no implementada");
}

pub fn ccf_txt(cpu: &mut CPU) {
    //cpu.texto(&format!("CCF"));
    panic!("0x3F ccf_txt funcion no implementada");
}

// *************************** 4 ***********************************
// 0x40
pub fn ld_b_b(cpu: &mut CPU) {
    cpu.b = cpu.b;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,B"));
}

// 0x41
pub fn ld_b_c(cpu: &mut CPU) {
    cpu.b = cpu.c;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,C"));
}

// 0x42
pub fn ld_b_d(cpu: &mut CPU) {
    cpu.b = cpu.d;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,D"));
}

// 0x43
pub fn ld_b_e(cpu: &mut CPU) {
    cpu.b = cpu.e;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,E"));
}

// 0x44
pub fn ld_b_h(cpu: &mut CPU) {
    cpu.b = cpu.h;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,H"));
}

// 0x45
pub fn ld_b_l(cpu: &mut CPU) {
    cpu.b = cpu.l;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,L"));
}

// 0x46
pub fn ld_b_OhlO(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.b = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += 7;
    cpu.pc += 1;
}

pub fn ld_b_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,(HL)"));
}


// 0x47
pub fn ld_b_a(cpu: &mut CPU) {
    cpu.b = cpu.a;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,A"));
}

// 0x48
pub fn ld_c_b(cpu: &mut CPU) {
    cpu.c = cpu.b;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,B"));
}

// 0x49
pub fn ld_c_c(cpu: &mut CPU) {
    cpu.c = cpu.c;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,C"));
}

// 0x4A
pub fn ld_c_d(cpu: &mut CPU) {
    cpu.c = cpu.d;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,D"));
}

// 0x4B
pub fn ld_c_e(cpu: &mut CPU) {
    cpu.c = cpu.e;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,E"));
}

// 0x4C
pub fn ld_c_h(cpu: &mut CPU) {
    cpu.c = cpu.h;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,H"));
}

// 0x4D
pub fn ld_c_l(cpu: &mut CPU) {
    cpu.c = cpu.l;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,E"));
}

// 0x4E
pub fn ld_c_OhlO(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.c = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += 7;
    cpu.pc += 1;
}

pub fn ld_c_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,(HL)"));
}

// 0x4F
pub fn ld_c_a(cpu: &mut CPU) {
    cpu.c = cpu.a;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,A"));
}

// *************************** 5 ***********************************
// 0x50
pub fn ld_d_b(cpu: &mut CPU) { panic!("0x50 funcion no implementada"); }

pub fn ld_d_b_txt(cpu: &mut CPU) { panic!("0x50 funcion no implementada"); }

// 0x51
pub fn ld_d_c(cpu: &mut CPU) { panic!("0x51 funcion no implementada"); }

pub fn ld_d_c_txt(cpu: &mut CPU) { panic!("0x51 funcion no implementada"); }

// 0x52
pub fn ld_d_d(cpu: &mut CPU) { panic!("0x52 funcion no implementada"); }

pub fn ld_d_d_txt(cpu: &mut CPU) { panic!("0x52 funcion no implementada"); }

// 0x53
pub fn ld_d_e(cpu: &mut CPU) {
    cpu.d = cpu.e;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_d_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,E")); }

// 0x54
pub fn ld_d_h(cpu: &mut CPU) { panic!("0x54 funcion no implementada"); }

pub fn ld_d_h_txt(cpu: &mut CPU) { panic!("0x54 funcion no implementada"); }

// 0x55
pub fn ld_d_l(cpu: &mut CPU) { panic!("0x55 funcion no implementada"); }

pub fn ld_d_l_txt(cpu: &mut CPU) { panic!("0x55 funcion no implementada"); }

// 0x56
pub fn ld_d_OhlO(cpu: &mut CPU) { panic!("0x56 funcion no implementada"); }

pub fn ld_d_OhlO_txt(cpu: &mut CPU) { panic!("0x56 funcion no implementada"); }

// 0x57
pub fn ld_d_a(cpu: &mut CPU) { panic!("0x57 funcion no implementada"); }

pub fn ld_d_a_txt(cpu: &mut CPU) { panic!("0x57 funcion no implementada"); }

// 0x58
pub fn ld_e_b(cpu: &mut CPU) { panic!("0x58 funcion no implementada"); }

pub fn ld_e_b_txt(cpu: &mut CPU) { panic!("0x58 funcion no implementada"); }

// 0x59
pub fn ld_e_c(cpu: &mut CPU) { panic!("0x59 funcion no implementada"); }

pub fn ld_e_c_txt(cpu: &mut CPU) { panic!("0x59 funcion no implementada"); }

// 0x5A
pub fn ld_e_d(cpu: &mut CPU) { panic!("0x5A funcion no implementada"); }

pub fn ld_e_d_txt(cpu: &mut CPU) { panic!("0x5A funcion no implementada"); }

// 0x5B
pub fn ld_e_e(cpu: &mut CPU) { panic!("0x5B funcion no implementada"); }

pub fn ld_e_e_txt(cpu: &mut CPU) { panic!("0x5B funcion no implementada"); }

// 0x5C
pub fn ld_e_h(cpu: &mut CPU) {
    cpu.e = cpu.h;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_e_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,H")); }

// 0x5D
pub fn ld_e_l(cpu: &mut CPU) { panic!("0x5D funcion no implementada"); }

pub fn ld_e_l_txt(cpu: &mut CPU) { panic!("0x5D funcion no implementada"); }

// 0x5E
pub fn ld_e_OhlO(cpu: &mut CPU) { panic!("0x5E funcion no implementada"); }

pub fn ld_eOhlO_txt(cpu: &mut CPU) { panic!("0x5E funcion no implementada"); }

// 0x5F
pub fn ld_e_a(cpu: &mut CPU) { panic!("0x5F funcion no implementada"); }

pub fn ld_e_a_txt(cpu: &mut CPU) { panic!("0x5F funcion no implementada"); }

// *************************** 6 ***********************************
// 0x62
pub fn ld_h_d(cpu: &mut CPU) {
    cpu.h = cpu.d;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_h_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,D"));
}

// 0x6B
pub fn ld_l_e(cpu: &mut CPU) {
    cpu.l = cpu.e;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_l_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,E"));
}

// *************************** 7 ***********************************
// 0x77
pub fn ldOhlO_a(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.pc += 1;
    cpu.t += 7;
}

pub fn ldOhlO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(HL),A"));
}

// 0x7B
pub fn ld_a_e(cpu: &mut CPU) {
    cpu.a = cpu.e;
    cpu.pc += 1;

    cpu.t += 4;
}

pub fn ld_a_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,E"));
}


// *************************** 8 ***********************************
// *************************** 9 ***********************************
// *************************** A ***********************************
// 0xAF
pub fn and_b(cpu: &mut CPU) { panic!("0xA0 funcion no implementada"); }

pub fn and_b_txt(cpu: &mut CPU) { panic!("0xA0 funcion no implementada"); }

pub fn and_c(cpu: &mut CPU) { panic!("0xA1 funcion no implementada"); }

pub fn and_c_txt(cpu: &mut CPU) { panic!("0xA1 funcion no implementada"); }

pub fn and_d(cpu: &mut CPU) { panic!("0xA2 funcion no implementada"); }

pub fn and_d_txt(cpu: &mut CPU) { panic!("0xA2 funcion no implementada"); }

pub fn and_e(cpu: &mut CPU) { panic!("0xA3 funcion no implementada"); }

pub fn and_e_txt(cpu: &mut CPU) { panic!("0xA3 funcion no implementada"); }

pub fn and_h(cpu: &mut CPU) { panic!("0xA4 funcion no implementada"); }

pub fn and_h_txt(cpu: &mut CPU) { panic!("0xA4 funcion no implementada"); }

pub fn and_l(cpu: &mut CPU) { panic!("0xA5 funcion no implementada"); }

pub fn and_l_txt(cpu: &mut CPU) { panic!("0xA5 funcion no implementada"); }

pub fn and_OhlO(cpu: &mut CPU) { panic!("0xA6 funcion no implementada"); }

pub fn and_OhlO_txt(cpu: &mut CPU) { panic!("0xA6 funcion no implementada"); }

pub fn and_a(cpu: &mut CPU) {
    cpu.a = cpu.and_u8_con_u8(cpu.a, cpu.a);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn and_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("AND A"));
}

pub fn xor_b(cpu: &mut CPU) { panic!("0xA8 funcion no implementada"); }

pub fn xor_b_txt(cpu: &mut CPU) { panic!("0xA8 funcion no implementada"); }

pub fn xor_c(cpu: &mut CPU) { panic!("0xA9 funcion no implementada"); }

pub fn xor_c_txt(cpu: &mut CPU) { panic!("0xA9 funcion no implementada"); }

pub fn xor_d(cpu: &mut CPU) { panic!("0xAA funcion no implementada"); }

pub fn xor_d_txt(cpu: &mut CPU) { panic!("0xAA funcion no implementada"); }

pub fn xor_e(cpu: &mut CPU) { panic!("0xAB funcion no implementada"); }

pub fn xor_e_txt(cpu: &mut CPU) { panic!("0xAB funcion no implementada"); }

pub fn xor_h(cpu: &mut CPU) { panic!("0xAC funcion no implementada"); }

pub fn xor_h_txt(cpu: &mut CPU) { panic!("0xAC funcion no implementada"); }

pub fn xor_l(cpu: &mut CPU) { panic!("0xAD funcion no implementada"); }

pub fn xor_l_txt(cpu: &mut CPU) { panic!("0xAD funcion no implementada"); }

pub fn xor_OhlO(cpu: &mut CPU) { panic!("0xAE funcion no implementada"); }

pub fn xor_OhlO_txt(cpu: &mut CPU) { panic!("0xAE funcion no implementada"); }


pub fn xor_a(cpu: &mut CPU) {
    cpu.a ^= cpu.a;

    cpu.reset_c_flag();
    cpu.reset_n_flag();
    cpu.reset_h_flag();

    if cpu.a == 0 { cpu.set_z_flag(); } else { cpu.reset_z_flag(); }
    if cpu.a < 0 { cpu.set_s_flag(); } else { cpu.reset_s_flag(); }
    if (cpu.a & 0x0000_0001) != 0 { cpu.reset_pv_flag(); } else { cpu.set_pv_flag(); }

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn xor_a_txt(cpu: &mut CPU) {
    //let txt = format!("XOR A");
    cpu.texto(&format!("XOR A"));
}

// *************************** B ***********************************
// 0xBC
pub fn cp_h(cpu: &mut CPU) {
    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.h);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn cp_h_txt(cpu: &mut CPU) {
    //let txt = format!("CP H");
    cpu.texto(&format!("CP H"));
}

// *************************** C ***********************************
// 0xC1
pub fn pop_bc(cpu: &mut CPU) {
    let addr: u16 = cpu.pop();
    cpu.b = ((addr & 0xFF00) >> 8) as u8;
    cpu.c = (addr & 0x00FF) as u8;

    cpu.pc += 1;

    cpu.t += 10;
}

pub fn pop_bc_txt(cpu: &mut CPU) {
    //let txt = format!("POP BC");
    cpu.texto(&format!("POP BC"));
}


// 0xC3 NN NN
pub fn jp_nn(cpu: &mut CPU) {
    cpu.t += 10;
    cpu.pc = cpu.r1r2;
}

pub fn jp_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("JP #{:04X}", cpu.r1r2));
}

// 0xC5
pub fn push_bc(cpu: &mut CPU) {
    let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.b, cpu.c);
    cpu.push(direccion);

    cpu.pc += 1;
    cpu.t += 11;
}

pub fn push_bc_txt(cpu: &mut CPU) {
    cpu.texto(&format!("PUSH BC"));
}

// 0xC9
pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.pop();

    cpu.t += 10;
}

pub fn ret_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RET"));
}

// 0xCB   -----EXTENSION--------------------------------------------------------
pub fn CB(cpu: &mut CPU) {
    // Ejecuta instruccion
    cpu.funciones_cb[cpu.r1 as usize](cpu);
}

pub fn CB_txt(cpu: &mut CPU) {
    // Ejecuta instruccion
    cpu.funciones_cb_txt[cpu.r1 as usize](cpu);
}


// 0xCD
pub fn call_nn(cpu: &mut CPU) {
    cpu.pc += 3;
    cpu.push(cpu.pc);
    cpu.pc = cpu.r1r2;

    cpu.t += 17;
}

pub fn call_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CALL #{:04X}", cpu.r1r2));
}

// *************************** D ***********************************
// 0xD3
pub fn out_OnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_port(cpu.r1, cpu.a);

    cpu.t += 11;
    cpu.pc += 2;
}

pub fn out_OnO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("OUT(#{:02X}),A", cpu.r1));
}

// 0xD9 Difiere según procesador (LR35902->RETI)
pub fn retiGB(cpu: &mut CPU) { panic!("0xD9: funcion no implementada"); }

pub fn retiGB_txt(cpu: &mut CPU) { panic!("0xD9: funcion no implementada"); }

// 0xD9 Difiere según procesador (Z80->EXX)
pub fn exx(cpu: &mut CPU) {
    let btemp = cpu.b;
    let ctemp = cpu.c;
    let dtemp = cpu.d;
    let etemp = cpu.e;
    let htemp = cpu.h;
    let ltemp = cpu.l;

    cpu.b = cpu.bp;
    cpu.c = cpu.cp;
    cpu.d = cpu.dp;
    cpu.e = cpu.ep;
    cpu.h = cpu.hp;
    cpu.l = cpu.lp;

    cpu.bp = btemp;
    cpu.cp = ctemp;
    cpu.dp = dtemp;
    cpu.ep = etemp;
    cpu.hp = htemp;
    cpu.lp = ltemp;

    cpu.pc += 1;
    cpu.t += 4;
}

pub fn exx_txt(cpu: &mut CPU) { cpu.texto(&format!("EXX")); }

// *************************** E ***********************************
// 0xE0 Difiere según procesador (LR35902->LD(#FF00+N),A)
pub fn ldOff00_m_nO_aGB(cpu: &mut CPU) {
    let addr: u16 = 0xFF00 + cpu.r1 as u16;
    cpu.mem.escribe_byte_en_mem(addr, cpu.a);

    cpu.pc += 2;
    cpu.t += 12;
}

pub fn ldOff00_m_nO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD ($FF00+#{:02X}),A", cpu.r1));
}

// 0xE0 Difiere según procesador (Z80->RET NV)
pub fn ret_po(cpu: &mut CPU) {
    panic!("0xE0 ret_po: funcion no implementada");
}

pub fn ret_po_txt(cpu: &mut CPU) {
    panic!("0xE0 ret_po_txt: funcion no implementada");
}

// 0xE2 Difiere según procesador (LR35902->LD(#FF00+C),A)
pub fn ldOff00_m_cO_aGB(cpu: &mut CPU) {
    let addr: u16 = 0xFF00 + cpu.c as u16;
    cpu.mem.escribe_byte_en_mem(addr, cpu.a);

    cpu.pc += 1;
    cpu.t += 8;
}

pub fn ldOff00_m_cO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD ($FF00+C),A"));
}

// 0xE2 Difiere según procesador (Z80->JP PO,NN)
pub fn jp_po_nn(cpu: &mut CPU) {
    panic!("0xE2 jp_po_nn: funcion no implementada");
}

pub fn jp_po_nn_txt(cpu: &mut CPU) {
    panic!("0xE2 jp_po_nn_txt: funcion no implementada");
}

// 0xEA  Difiere según procesador (LR35902->LD (nn),A)
pub fn ldOnnO_aGB(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.pc += 3;
    cpu.t += 16;
}

pub fn ldOnnO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}

// 0xEA  Difiere según procesador (Z80->JP  V,nn)
pub fn jp_pe_nn(cpu: &mut CPU) {
    panic!("0xEA jp_pe_nn: funcion no implementada");
}

pub fn jp_pe_nn_txt(cpu: &mut CPU) {
    panic!("0xEA jp_pe_nn_txt: funcion no implementada");
}

// 0xEB
pub fn ex_de_hl(cpu: &mut CPU) {
    let dtemp = cpu.d;
    let etemp = cpu.e;
    cpu.d = cpu.h;
    cpu.e = cpu.l;
    cpu.h = dtemp;
    cpu.l = etemp;

    cpu.pc += 1;
    cpu.t += 4;
}

pub fn ex_de_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("EX DE,HL"));
}

// 0xED   -----EXTENSION--------------------------------------------------------
pub fn ED(cpu: &mut CPU) {
// Ejecuta instruccion
    cpu.funciones_ed[cpu.r1 as usize](cpu);
}

pub fn ED_txt(cpu: &mut CPU) {
// Ejecuta instruccion
    cpu.funciones_ed_txt[cpu.r1 as usize](cpu);
}

// *************************** F ***********************************
// 0xF0 Difiere según procesador (LR35902->LDH  A,(n))
pub fn ld_a_Off00_m_nOGB(cpu: &mut CPU) {
    let addr: u16 = 0xFF00 + cpu.r1 as u16;
    cpu.a = cpu.mem.lee_byte_de_mem(addr);

    cpu.t += 12;
    cpu.pc += 2;
}

pub fn ld_a_Off00_m_nOGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,($FF00+#{:02X})", cpu.r1));
}

// 0xF0 Difiere según procesador (Z80->RET P)
pub fn ret_p(cpu: &mut CPU) {
    if cpu.get_p_flag() {
        cpu.pc = cpu.pop();
        cpu.t += 11;
    } else {
        cpu.t += 5;
        cpu.pc += 1;
    }
}

pub fn ret_p_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RET P"));
}

// 0xF3 TODO: Debe deshablitar la interrupción enmascarada
pub fn di(cpu: &mut CPU) {
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn di_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DI"));
}

// 0xFE
pub fn cp_n(cpu: &mut CPU) {
    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.r1);

    cpu.pc += 2;
    cpu.t += 7;
}

pub fn cp_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP #{:02X}", cpu.r1));
}


/*3 	jr nc,* 	ld sp,** 	ld (**),a 	inc sp 	inc (hl) 	dec (hl) 	ld (hl),* 	scf
jr c,* 	add hl,sp 	ld a,(**) 	dec sp 	inc a 	dec a 	ld a,* 	ccf
4 	ld b,b 	ld b,c 	ld b,d 	ld b,e 	ld b,h 	ld b,l 	ld b,(hl) 	ld b,a 	ld c,b 	ld c,c 	ld c,d 	ld c,e 	ld c,h 	ld c,l 	ld c,(hl) 	ld c,a
5 	ld d,b 	ld d,c 	ld d,d 	ld d,e 	ld d,h 	ld d,l 	ld d,(hl) 	ld d,a 	ld e,b 	ld e,c 	ld e,d 	ld e,e 	ld e,h 	ld e,l 	ld e,(hl) 	ld e,a
6 	ld h,b 	ld h,c 	ld h,d 	ld h,e 	ld h,h 	ld h,l 	ld h,(hl) 	ld h,a 	ld l,b 	ld l,c 	ld l,d 	ld l,e 	ld l,h 	ld l,l 	ld l,(hl) 	ld l,a
7 	ld (hl),b 	ld (hl),c 	ld (hl),d 	ld (hl),e 	ld (hl),h 	ld (hl),l 	halt 	ld (hl),a 	ld a,b 	ld a,c 	ld a,d 	ld a,e 	ld a,h 	ld a,l 	ld a,(hl) 	ld a,a
8 	add a,b 	add a,c 	add a,d 	add a,e 	add a,h 	add a,l 	add a,(hl) 	add a,a 	adc a,b 	adc a,c 	adc a,d 	adc a,e 	adc a,h 	adc a,l 	adc a,(hl) 	adc a,a
9 	sub b 	sub c 	sub d 	sub e 	sub h 	sub l 	sub (hl) 	sub a 	sbc a,b 	sbc a,c 	sbc a,d 	sbc a,e 	sbc a,h 	sbc a,l 	sbc a,(hl) 	sbc a,a
A 	and b 	and c 	and d 	and e 	and h 	and l 	and (hl) 	and a 	xor b 	xor c 	xor d 	xor e 	xor h 	xor l 	xor (hl) 	xor a
B 	or b 	or c 	or d 	or e 	or h 	or l 	or (hl) 	or a 	cp b 	cp c 	cp d 	cp e 	cp h 	cp l 	cp (hl) 	cp a
C 	ret nz 	pop bc 	jp nz,** 	jp ** 	call nz,** 	push bc 	add a,* 	rst 00h 	ret z 	ret 	jp z,**
BITS
	call z,** 	call ** 	adc a,* 	rst 08h
D 	ret nc 	pop de 	jp nc,** 	out (*),a 	call nc,** 	push de 	sub * 	rst 10h 	ret c 	exx 	jp c,** 	in a,(*) 	call c,**
IX
	sbc a,* 	rst 18h
E 	ret po 	pop hl 	jp po,** 	ex (sp),hl 	call po,** 	push hl 	and * 	rst 20h 	ret pe 	jp (hl) 	jp pe,** 	ex de,hl 	call pe,**
EXTD
	xor * 	rst 28h
F 	ret p 	pop af 	jp p,** 	di 	call p,** 	push af 	or * 	rst 30h 	ret m 	ld sp,hl 	jp m,** 	ei 	call m,**
IY
	cp * 	rst 38h

*/