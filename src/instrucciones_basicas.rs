#![allow(non_snake_case)]

/*
saco texto de
https://github.com/Dotneteer/spectnetide/blob/master/Core/Spect.Net.SpectrumEmu/Cpu/Z80Operations.cs
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

panic!(format!("0x19 ADD HL DE    hl16 = #{:04X}  de16 = #{:04X}\n", hl16, de16));

*/

use crate::cpu::{CPU, Funcion};
use crate::procesador::PROCESADOR;
use crate::constantes::*;
use crate::operaciones_binarias::*;

// bytes, time   datos sacados de fichero:
// https://github.com/malandrin/gbe/blob/master/gbe/opcodes_info.cpp
// En t que difieren segun el código poner el valor menor y luego en la función sumar lo que haga
// falta

pub fn mete_funciones_normales(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones[0x00 as usize].set_punt_y_val_a_fn(nop, nop_txt, 1, 4);
    cpu.funciones[0x01 as usize].set_punt_y_val_a_fn(ld_bc_nn, ld_bc_nn_txt, 3, 10);
    cpu.funciones[0x02 as usize].set_punt_y_val_a_fn(ldObcO_a, ldObcO_a_txt, 1, 7);
    cpu.funciones[0x03 as usize].set_punt_y_val_a_fn(inc_bc, inc_bc_txt, 1, 6);
    cpu.funciones[0x04 as usize].set_punt_y_val_a_fn(inc_b, inc_b_txt, 1, 4);
    cpu.funciones[0x05 as usize].set_punt_y_val_a_fn(dec_b, dec_b_txt, 1, 4);
    cpu.funciones[0x06 as usize].set_punt_y_val_a_fn(ld_b_n, ld_b_n_txt, 2, 7);
    cpu.funciones[0x07 as usize].set_punt_y_val_a_fn(rlca, rlca_txt, 1, 4);

    // 08 LR35902->LD   (nn),SP         Z-80->EX  AF,AF'
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x08 as usize].set_punt_y_val_a_fn(ldOnnO_spGB,
                                                             ldOnnO_spGB_txt, 3, 20);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x08 as usize].set_punt_y_val_a_fn(ex_af_afp,
                                                             ex_af_afp_txt, 1, 4);
        }
    };
    cpu.funciones[0x09 as usize].set_punt_y_val_a_fn(add_hl_bc, add_hl_bc_txt, 1, 11);
    cpu.funciones[0x0A as usize].set_punt_y_val_a_fn(ld_aObcO, ld_aObcO_txt, 1, 7);
    cpu.funciones[0x0B as usize].set_punt_y_val_a_fn(dec_bc, dec_bc_txt, 1, 6);
    cpu.funciones[0x0C as usize].set_punt_y_val_a_fn(inc_c, inc_c_txt, 1, 4);
    cpu.funciones[0x0D as usize].set_punt_y_val_a_fn(dec_c, dec_c_txt, 1, 4);
    cpu.funciones[0x0E as usize].set_punt_y_val_a_fn(ld_c_n, ld_c_n_txt, 2, 7);
    cpu.funciones[0x0F as usize].set_punt_y_val_a_fn(rrca, rrca_txt, 1, 4);

    // *************************** 1 ***********************************
    // LR35902-> STOP               Z80->DJNZ
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x10 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 4);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x10 as usize].set_punt_y_val_a_fn(djnz_n, djnz_n_txt, 2, 8);
        }
    };

    cpu.funciones[0x11 as usize].set_punt_y_val_a_fn(ld_de_nn, ld_de_nn_txt, 3, 10);
    cpu.funciones[0x12 as usize].set_punt_y_val_a_fn(ldOdeO_a, ldOdeO_a_txt, 1, 7);
    cpu.funciones[0x13 as usize].set_punt_y_val_a_fn(inc_de, inc_de_txt, 1, 6);
    cpu.funciones[0x14 as usize].set_punt_y_val_a_fn(inc_d, inc_d_txt, 1, 4);
    cpu.funciones[0x15 as usize].set_punt_y_val_a_fn(dec_d, dec_d_txt, 1, 4);
    cpu.funciones[0x16 as usize].set_punt_y_val_a_fn(ld_d_n, ld_d_n_txt, 2, 7);
    cpu.funciones[0x17 as usize].set_punt_y_val_a_fn(rla, rla_txt, 1, 4);
    cpu.funciones[0x18 as usize].set_punt_y_val_a_fn(jr_n, jr_n_txt, 2, 12);
    cpu.funciones[0x19 as usize].set_punt_y_val_a_fn(add_hl_de, add_hl_de_txt, 1, 11);
    cpu.funciones[0x1A as usize].set_punt_y_val_a_fn(ld_aOdeO, ld_aOdeO_txt, 1, 7);
    cpu.funciones[0x1B as usize].set_punt_y_val_a_fn(dec_de, dec_de_txt, 1, 6);
    cpu.funciones[0x1C as usize].set_punt_y_val_a_fn(inc_e, inc_e_txt, 1, 4);
    cpu.funciones[0x1D as usize].set_punt_y_val_a_fn(dec_e, dec_e_txt, 1, 4);
    cpu.funciones[0x1E as usize].set_punt_y_val_a_fn(ld_e_n, ld_e_n_txt, 2, 7);
    cpu.funciones[0x1F as usize].set_punt_y_val_a_fn(rra, rra_txt, 1, 4);


    // *************************** 2 ***********************************
    cpu.funciones[0x20 as usize].set_punt_y_val_a_fn(jr_nz_n, jr_nz_n_txt, 2, 7);
    cpu.funciones[0x21 as usize].set_punt_y_val_a_fn(ld_hl_nn, ld_hl_nn_txt, 3, 10);

    // LR35902->LDI  (HL),A        Z-80->LD  (nn),HL
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x22 as usize].set_punt_y_val_a_fn(ldiOhlO_aGB, ldiOhlO_aGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x22 as usize].set_punt_y_val_a_fn(ldOnnO_hl, ldOnnO_hl_txt, 3, 16);
        }
    };
    cpu.funciones[0x23 as usize].set_punt_y_val_a_fn(inc_hl, inc_hl_txt, 1, 6);
    cpu.funciones[0x24 as usize].set_punt_y_val_a_fn(inc_h, inc_h_txt, 1, 4);
    cpu.funciones[0x25 as usize].set_punt_y_val_a_fn(dec_h, dec_h_txt, 1, 4);
    cpu.funciones[0x26 as usize].set_punt_y_val_a_fn(ld_h_n, ld_h_n_txt, 2, 7);
    cpu.funciones[0x27 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x28 as usize].set_punt_y_val_a_fn(jr_z_n, jr_z_n_txt, 2, 7);
    cpu.funciones[0x29 as usize].set_punt_y_val_a_fn(add_hl_hl, add_hl_hl_txt, 1, 11);

    // LR35902->LDI  A,(HL)        Z-80->LD  HL,(nn)
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x2A as usize].set_punt_y_val_a_fn(ldi_aOhlOGB, ldi_aOhlOGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x2A as usize].set_punt_y_val_a_fn(ld_hlOnnO, ld_hlOnnO_txt, 3, 16);
        }
    };

    cpu.funciones[0x2B as usize].set_punt_y_val_a_fn(dec_hl, dec_hl_txt, 1, 6);
    cpu.funciones[0x2C as usize].set_punt_y_val_a_fn(inc_l, inc_l_txt, 1, 4);
    cpu.funciones[0x2D as usize].set_punt_y_val_a_fn(dec_l, dec_l_txt, 1, 4);
    cpu.funciones[0x2E as usize].set_punt_y_val_a_fn(ld_l_n, ld_l_n_txt, 2, 7);
    cpu.funciones[0x2F as usize].set_punt_y_val_a_fn(cpl, cpl_txt, 1, 4);

    // *************************** 3 ***********************************

    cpu.funciones[0x30 as usize].set_punt_y_val_a_fn(jr_nc_n, jr_nc_n_txt, 2, 7);

    cpu.funciones[0x31 as usize].set_punt_y_val_a_fn(ld_sp_nn, ld_sp_nn_txt, 3, 10);

    // LR35902->LDD  (HL),A    Z-80->LD  (nn),A
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x32 as usize].set_punt_y_val_a_fn(lddOhlO_aGB, lddOhlO_aGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x32 as usize].set_punt_y_val_a_fn(ldOnnO_a, ldOnnO_a_txt, 3, 13);
        }
    };

    cpu.funciones[0x33 as usize].set_punt_y_val_a_fn(inc_sp, inc_sp_txt, 1, 6);
    cpu.funciones[0x34 as usize].set_punt_y_val_a_fn(inc_OhlO, inc_OhlO_txt, 1, 11);
    cpu.funciones[0x35 as usize].set_punt_y_val_a_fn(dec_OhlO, dec_OhlO_txt, 1, 11);
    cpu.funciones[0x36 as usize].set_punt_y_val_a_fn(ld_OhlO_n, ld_OhlO_n_txt, 2, 10);
    cpu.funciones[0x37 as usize].set_punt_y_val_a_fn(scf, scf_txt, 1, 4);
    cpu.funciones[0x38 as usize].set_punt_y_val_a_fn(jr_c_n, jr_c_n_txt, 2, 7);
    cpu.funciones[0x39 as usize].set_punt_y_val_a_fn(add_hl_sp, add_hl_sp_txt, 1, 11);

    // LR35902->LDD  A,(HL)     Z-80->LD  A,(nn)
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x3A as usize].set_punt_y_val_a_fn(ldd_a_OhlOGB, ldd_a_OhlOGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x3A as usize].set_punt_y_val_a_fn(ld_a_OnnO, ld_a_OnnO_txt, 3, 13);
        }
    };

    cpu.funciones[0x3B as usize].set_punt_y_val_a_fn(dec_sp, dec_sp_txt, 1, 6);
    cpu.funciones[0x3C as usize].set_punt_y_val_a_fn(inc_a, inc_a_txt, 1, 4);
    cpu.funciones[0x3D as usize].set_punt_y_val_a_fn(dec_a, dec_a_txt, 1, 4);
    cpu.funciones[0x3E as usize].set_punt_y_val_a_fn(ld_a_n, ld_a_n_txt, 2, 7);
    cpu.funciones[0x3F as usize].set_punt_y_val_a_fn(ccf, ccf_txt, 1, 4);

    // *************************** 4 ***********************************
    cpu.funciones[0x40 as usize].set_punt_y_val_a_fn(ld_b_b, ld_b_b_txt, 1, 4);
    cpu.funciones[0x41 as usize].set_punt_y_val_a_fn(ld_b_c, ld_b_c_txt, 1, 4);
    cpu.funciones[0x42 as usize].set_punt_y_val_a_fn(ld_b_d, ld_b_d_txt, 1, 4);
    cpu.funciones[0x43 as usize].set_punt_y_val_a_fn(ld_b_e, ld_b_e_txt, 1, 4);
    cpu.funciones[0x44 as usize].set_punt_y_val_a_fn(ld_b_h, ld_b_h_txt, 1, 4);
    cpu.funciones[0x45 as usize].set_punt_y_val_a_fn(ld_b_l, ld_b_l_txt, 1, 4);
    cpu.funciones[0x46 as usize].set_punt_y_val_a_fn(ld_b_OhlO, ld_b_OhlO_txt, 1, 7);
    cpu.funciones[0x47 as usize].set_punt_y_val_a_fn(ld_b_a, ld_b_a_txt, 1, 4);
    cpu.funciones[0x48 as usize].set_punt_y_val_a_fn(ld_c_b, ld_c_b_txt, 1, 4);
    cpu.funciones[0x49 as usize].set_punt_y_val_a_fn(ld_c_c, ld_c_c_txt, 1, 4);
    cpu.funciones[0x4A as usize].set_punt_y_val_a_fn(ld_c_d, ld_c_d_txt, 1, 4);
    cpu.funciones[0x4B as usize].set_punt_y_val_a_fn(ld_c_e, ld_c_e_txt, 1, 4);
    cpu.funciones[0x4C as usize].set_punt_y_val_a_fn(ld_c_h, ld_c_h_txt, 1, 4);
    cpu.funciones[0x4D as usize].set_punt_y_val_a_fn(ld_c_l, ld_c_l_txt, 1, 4);
    cpu.funciones[0x4E as usize].set_punt_y_val_a_fn(ld_c_OhlO, ld_c_OhlO_txt, 1, 7);
    cpu.funciones[0x4F as usize].set_punt_y_val_a_fn(ld_c_a, ld_c_a_txt, 1, 4);
    // *************************** 5 ***********************************
    cpu.funciones[0x50 as usize].set_punt_y_val_a_fn(ld_d_b, ld_d_b_txt, 1, 4);
    cpu.funciones[0x51 as usize].set_punt_y_val_a_fn(ld_d_c, ld_d_c_txt, 1, 4);
    cpu.funciones[0x52 as usize].set_punt_y_val_a_fn(ld_d_d, ld_d_d_txt, 1, 4);
    cpu.funciones[0x53 as usize].set_punt_y_val_a_fn(ld_d_e, ld_d_e_txt, 1, 4);
    cpu.funciones[0x54 as usize].set_punt_y_val_a_fn(ld_d_h, ld_d_h_txt, 1, 4);
    cpu.funciones[0x55 as usize].set_punt_y_val_a_fn(ld_d_l, ld_d_l_txt, 1, 4);
    cpu.funciones[0x56 as usize].set_punt_y_val_a_fn(ld_d_OhlO, ld_d_OhlO_txt, 1, 7);
    cpu.funciones[0x57 as usize].set_punt_y_val_a_fn(ld_d_a, ld_d_a_txt, 1, 4);
    cpu.funciones[0x58 as usize].set_punt_y_val_a_fn(ld_e_b, ld_e_b_txt, 1, 4);
    cpu.funciones[0x59 as usize].set_punt_y_val_a_fn(ld_e_c, ld_e_c_txt, 1, 4);
    cpu.funciones[0x5A as usize].set_punt_y_val_a_fn(ld_e_d, ld_e_d_txt, 1, 4);
    cpu.funciones[0x5B as usize].set_punt_y_val_a_fn(ld_e_e, ld_e_e_txt, 1, 4);
    cpu.funciones[0x5C as usize].set_punt_y_val_a_fn(ld_e_h, ld_e_h_txt, 1, 4);
    cpu.funciones[0x5D as usize].set_punt_y_val_a_fn(ld_e_l, ld_e_l_txt, 1, 4);
    cpu.funciones[0x5E as usize].set_punt_y_val_a_fn(ld_e_OhlO, ld_e_OhlO_txt, 1, 7);
    cpu.funciones[0x5F as usize].set_punt_y_val_a_fn(ld_e_a, ld_e_a_txt, 1, 4);

    // *************************** 6 ***********************************
    cpu.funciones[0x60 as usize].set_punt_y_val_a_fn(ld_h_b, ld_h_b_txt, 1, 4);
    cpu.funciones[0x61 as usize].set_punt_y_val_a_fn(ld_h_c, ld_h_c_txt, 1, 4);
    cpu.funciones[0x62 as usize].set_punt_y_val_a_fn(ld_h_d, ld_h_d_txt, 1, 4);
    cpu.funciones[0x63 as usize].set_punt_y_val_a_fn(ld_h_e, ld_h_e_txt, 1, 4);
    cpu.funciones[0x64 as usize].set_punt_y_val_a_fn(ld_h_h, ld_h_h_txt, 1, 4);
    cpu.funciones[0x65 as usize].set_punt_y_val_a_fn(ld_h_l, ld_h_l_txt, 1, 4);
    cpu.funciones[0x66 as usize].set_punt_y_val_a_fn(ld_h_OhlO, ld_h_OhlO_txt, 1, 7);
    cpu.funciones[0x67 as usize].set_punt_y_val_a_fn(ld_h_a, ld_h_a_txt, 1, 4);
    cpu.funciones[0x68 as usize].set_punt_y_val_a_fn(ld_l_b, ld_l_b_txt, 1, 4);
    cpu.funciones[0x69 as usize].set_punt_y_val_a_fn(ld_l_c, ld_l_c_txt, 1, 4);
    cpu.funciones[0x6A as usize].set_punt_y_val_a_fn(ld_l_d, ld_l_d_txt, 1, 4);
    cpu.funciones[0x6B as usize].set_punt_y_val_a_fn(ld_l_e, ld_l_e_txt, 1, 4);
    cpu.funciones[0x6C as usize].set_punt_y_val_a_fn(ld_l_h, ld_l_h_txt, 1, 4);
    cpu.funciones[0x6D as usize].set_punt_y_val_a_fn(ld_l_l, ld_l_l_txt, 1, 4);
    cpu.funciones[0x6E as usize].set_punt_y_val_a_fn(ld_l_OhlO, ld_l_OhlO_txt, 1, 7);
    cpu.funciones[0x6F as usize].set_punt_y_val_a_fn(ld_l_a, ld_l_a_txt, 1, 4);
    // *************************** 7 ***********************************
    cpu.funciones[0x70 as usize].set_punt_y_val_a_fn(ldOhlO_b, ldOhlO_b_txt, 1, 7);
    cpu.funciones[0x71 as usize].set_punt_y_val_a_fn(ldOhlO_c, ldOhlO_c_txt, 1, 7);
    cpu.funciones[0x72 as usize].set_punt_y_val_a_fn(ldOhlO_d, ldOhlO_d_txt, 1, 7);
    cpu.funciones[0x73 as usize].set_punt_y_val_a_fn(ldOhlO_e, ldOhlO_e_txt, 1, 7);
    cpu.funciones[0x74 as usize].set_punt_y_val_a_fn(ldOhlO_h, ldOhlO_h_txt, 1, 7);
    cpu.funciones[0x75 as usize].set_punt_y_val_a_fn(ldOhlO_l, ldOhlO_l_txt, 1, 7);
    cpu.funciones[0x76 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x77 as usize].set_punt_y_val_a_fn(ldOhlO_a, ldOhlO_a_txt, 1, 7);
    cpu.funciones[0x78 as usize].set_punt_y_val_a_fn(ld_a_b, ld_a_b_txt, 1, 4);
    cpu.funciones[0x79 as usize].set_punt_y_val_a_fn(ld_a_c, ld_a_c_txt, 1, 4);
    cpu.funciones[0x7A as usize].set_punt_y_val_a_fn(ld_a_d, ld_a_d_txt, 1, 4);
    cpu.funciones[0x7B as usize].set_punt_y_val_a_fn(ld_a_e, ld_a_e_txt, 1, 4);
    cpu.funciones[0x7C as usize].set_punt_y_val_a_fn(ld_a_h, ld_a_h_txt, 1, 4);
    cpu.funciones[0x7D as usize].set_punt_y_val_a_fn(ld_a_l, ld_a_l_txt, 1, 4);
    cpu.funciones[0x7E as usize].set_punt_y_val_a_fn(ld_a_OhlO, ld_a_OhlO_txt, 1, 7);
    cpu.funciones[0x7F as usize].set_punt_y_val_a_fn(ld_a_a, ld_a_a_txt, 1, 4);

    // *************************** 8 ***********************************
    cpu.funciones[0x80 as usize].set_punt_y_val_a_fn(add_a_b, add_a_b_txt, 1, 4);
    cpu.funciones[0x81 as usize].set_punt_y_val_a_fn(add_a_c, add_a_c_txt, 1, 4);
    cpu.funciones[0x82 as usize].set_punt_y_val_a_fn(add_a_d, add_a_d_txt, 1, 4);
    cpu.funciones[0x83 as usize].set_punt_y_val_a_fn(add_a_e, add_a_e_txt, 1, 4);
    cpu.funciones[0x84 as usize].set_punt_y_val_a_fn(add_a_h, add_a_h_txt, 1, 4);
    cpu.funciones[0x85 as usize].set_punt_y_val_a_fn(add_a_l, add_a_l_txt, 1, 4);
    cpu.funciones[0x86 as usize].set_punt_y_val_a_fn(add_a_OhlO, add_a_OhlO_txt, 1, 7);
    cpu.funciones[0x87 as usize].set_punt_y_val_a_fn(add_a_a, add_a_a_txt, 1, 4);
    cpu.funciones[0x88 as usize].set_punt_y_val_a_fn(adc_a_b, adc_a_b_txt, 1, 4);
    cpu.funciones[0x89 as usize].set_punt_y_val_a_fn(adc_a_c, adc_a_c_txt, 1, 4);
    cpu.funciones[0x8A as usize].set_punt_y_val_a_fn(adc_a_d, adc_a_d_txt, 1, 4);
    cpu.funciones[0x8B as usize].set_punt_y_val_a_fn(adc_a_e, adc_a_e_txt, 1, 4);
    cpu.funciones[0x8C as usize].set_punt_y_val_a_fn(adc_a_h, adc_a_h_txt, 1, 4);
    cpu.funciones[0x8D as usize].set_punt_y_val_a_fn(adc_a_l, adc_a_l_txt, 1, 4);
    cpu.funciones[0x8E as usize].set_punt_y_val_a_fn(adc_a_OhlO, adc_a_OhlO_txt, 1, 7);
    cpu.funciones[0x8F as usize].set_punt_y_val_a_fn(adc_a_a, adc_a_a_txt, 1, 4);
    // *************************** 9 ***********************************
    cpu.funciones[0x90 as usize].set_punt_y_val_a_fn(sub_b, sub_b_txt, 1, 4);
    cpu.funciones[0x91 as usize].set_punt_y_val_a_fn(sub_c, sub_c_txt, 1, 4);
    cpu.funciones[0x92 as usize].set_punt_y_val_a_fn(sub_d, sub_d_txt, 1, 4);
    cpu.funciones[0x93 as usize].set_punt_y_val_a_fn(sub_e, sub_e_txt, 1, 4);
    cpu.funciones[0x94 as usize].set_punt_y_val_a_fn(sub_h, sub_h_txt, 1, 4);
    cpu.funciones[0x95 as usize].set_punt_y_val_a_fn(sub_l, sub_l_txt, 1, 4);
    cpu.funciones[0x96 as usize].set_punt_y_val_a_fn(subOhlO, subOhlO, 1, 7);
    cpu.funciones[0x97 as usize].set_punt_y_val_a_fn(sub_a, sub_a_txt, 1, 4);
    cpu.funciones[0x98 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x99 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9A as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9B as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9C as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9D as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9E as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 7);
    cpu.funciones[0x9F as usize].set_punt_y_val_a_fn(sbc_a_a, sbc_a_a_txt, 1, 4);
    // *************************** A ***********************************
    cpu.funciones[0xA0 as usize].set_punt_y_val_a_fn(and_b, and_b_txt, 1, 4);
    cpu.funciones[0xA1 as usize].set_punt_y_val_a_fn(and_c, and_c_txt, 1, 4);
    cpu.funciones[0xA2 as usize].set_punt_y_val_a_fn(and_d, and_d_txt, 1, 4);
    cpu.funciones[0xA3 as usize].set_punt_y_val_a_fn(and_e, and_e_txt, 1, 4);
    cpu.funciones[0xA4 as usize].set_punt_y_val_a_fn(and_h, and_h_txt, 1, 4);
    cpu.funciones[0xA5 as usize].set_punt_y_val_a_fn(and_l, and_l_txt, 1, 4);
    cpu.funciones[0xA6 as usize].set_punt_y_val_a_fn(and_OhlO, and_OhlO_txt, 1, 7);
    cpu.funciones[0xA7 as usize].set_punt_y_val_a_fn(and_a, and_a_txt, 1, 4);
    cpu.funciones[0xA8 as usize].set_punt_y_val_a_fn(xor_b, xor_b_txt, 1, 4);
    cpu.funciones[0xA9 as usize].set_punt_y_val_a_fn(xor_c, xor_c_txt, 1, 4);
    cpu.funciones[0xAA as usize].set_punt_y_val_a_fn(xor_d, xor_d_txt, 1, 4);
    cpu.funciones[0xAB as usize].set_punt_y_val_a_fn(xor_e, xor_e_txt, 1, 4);
    cpu.funciones[0xAC as usize].set_punt_y_val_a_fn(xor_h, xor_h_txt, 1, 4);
    cpu.funciones[0xAD as usize].set_punt_y_val_a_fn(xor_l, xor_l_txt, 1, 4);
    cpu.funciones[0xAE as usize].set_punt_y_val_a_fn(xor_OhlO, xor_OhlO_txt, 1, 7);
    cpu.funciones[0xAF as usize].set_punt_y_val_a_fn(xor_a, xor_a_txt, 1, 4);
    // *************************** B ***********************************
    cpu.funciones[0xB0 as usize].set_punt_y_val_a_fn(or_b, or_b_txt, 1, 4);
    cpu.funciones[0xB1 as usize].set_punt_y_val_a_fn(or_c, or_c_txt, 1, 4);
    cpu.funciones[0xB2 as usize].set_punt_y_val_a_fn(or_d, or_d_txt, 1, 4);
    cpu.funciones[0xB3 as usize].set_punt_y_val_a_fn(or_e, or_e_txt, 1, 4);
    cpu.funciones[0xB4 as usize].set_punt_y_val_a_fn(or_h, or_h_txt, 1, 4);
    cpu.funciones[0xB5 as usize].set_punt_y_val_a_fn(or_l, or_l_txt, 1, 4);
    cpu.funciones[0xB6 as usize].set_punt_y_val_a_fn(or_OhlO, or_OhlO_txt, 1, 7);
    cpu.funciones[0xB7 as usize].set_punt_y_val_a_fn(or_a, or_a, 1, 4);
    cpu.funciones[0xB8 as usize].set_punt_y_val_a_fn(cp_b, cp_b_txt, 1, 4);
    cpu.funciones[0xB9 as usize].set_punt_y_val_a_fn(cp_c, cp_c_txt, 1, 4);
    cpu.funciones[0xBA as usize].set_punt_y_val_a_fn(cp_d, cp_d_txt, 1, 4);
    cpu.funciones[0xBB as usize].set_punt_y_val_a_fn(cp_e, cp_e_txt, 1, 4);
    cpu.funciones[0xBC as usize].set_punt_y_val_a_fn(cp_h, cp_h_txt, 1, 4);
    cpu.funciones[0xBD as usize].set_punt_y_val_a_fn(cp_l, cp_l_txt, 1, 4);
    cpu.funciones[0xBE as usize].set_punt_y_val_a_fn(cpOhlO, cpOhlO_txt, 1, 7);
    cpu.funciones[0xBF as usize].set_punt_y_val_a_fn(cp_a, cp_a_txt, 1, 4);

    // *************************** C ***********************************
    cpu.funciones[0xC0 as usize].set_punt_y_val_a_fn(ret_nz, ret_nz_txt, 1, 5);
    cpu.funciones[0xC1 as usize].set_punt_y_val_a_fn(pop_bc, pop_bc_txt, 1, 10);
    cpu.funciones[0xC2 as usize].set_punt_y_val_a_fn(jp_nz, jp_nz_txt, 3, 10);
    cpu.funciones[0xC3 as usize].set_punt_y_val_a_fn(jp_nn, jp_nn_txt, 3, 10);
    cpu.funciones[0xC4 as usize].set_punt_y_val_a_fn(call_nz_nn, call_nz_nn, 3, 10);
    cpu.funciones[0xC5 as usize].set_punt_y_val_a_fn(push_bc, push_bc_txt, 1, 11);
    cpu.funciones[0xC6 as usize].set_punt_y_val_a_fn(add_a_n, add_a_n_txt, 2, 7);
    cpu.funciones[0xC7 as usize].set_punt_y_val_a_fn(rst_00, rst_00_txt, 1, 11);
    cpu.funciones[0xC8 as usize].set_punt_y_val_a_fn(ret_z, ret_z_txt, 1, 5);
    cpu.funciones[0xC9 as usize].set_punt_y_val_a_fn(ret, ret_txt, 1, 10);
    cpu.funciones[0xCA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xCB as usize].set_punt_y_val_a_fn(CB, CB_txt, 0, 0); // Extensión
    cpu.funciones[0xCC as usize].set_punt_y_val_a_fn(call_z_nn, call_z_nn_txt, 3, 10);
    cpu.funciones[0xCD as usize].set_punt_y_val_a_fn(call_nn, call_nn_txt, 3, 17);
    cpu.funciones[0xCE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 7);
    cpu.funciones[0xCF as usize].set_punt_y_val_a_fn(rst_08, rst_08_txt, 1, 11);
    // *************************** D ***********************************
    cpu.funciones[0xD0 as usize].set_punt_y_val_a_fn(ret_nc, ret_nc_txt, 1, 5);
    cpu.funciones[0xD1 as usize].set_punt_y_val_a_fn(pop_de, pop_de_txt, 1, 10);
    cpu.funciones[0xD2 as usize].set_punt_y_val_a_fn(jp_nc_nn, jp_nc_nn_txt, 3, 10);
    cpu.funciones[0xD3 as usize].set_punt_y_val_a_fn(out_OnO_a, out_OnO_a_txt, 2, 11);
    cpu.funciones[0xD4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xD5 as usize].set_punt_y_val_a_fn(push_de, push_de_txt, 1, 11);
    cpu.funciones[0xD6 as usize].set_punt_y_val_a_fn(sub_n, sub_n_txt, 2, 7);
    cpu.funciones[0xD7 as usize].set_punt_y_val_a_fn(rst_10, rst_10_txt, 1, 11);
    cpu.funciones[0xD8 as usize].set_punt_y_val_a_fn(ret_c, ret_c_txt, 1, 5);

    // D9 LR35902->RETI        Z-80->EXX
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xD9 as usize].set_punt_y_val_a_fn(retiGB, retiGB_txt, 1, 16);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xD9 as usize].set_punt_y_val_a_fn(exx, exx_txt, 1, 4);
        }
    };

    cpu.funciones[0xDA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xDB as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 11);
    cpu.funciones[0xDC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xDD as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xDE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 7);
    cpu.funciones[0xDF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 11);

    // *************************** E ***********************************
    // LR35902->LD (FF00+u8),A         Z-80->RET NV
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE0 as usize].set_punt_y_val_a_fn(ldOff00_m_nO_aGB, ldOff00_m_nO_aGB_txt, 2, 12);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE0 as usize].set_punt_y_val_a_fn(ret_po, ret_po_txt, 1, 5);
        }
    }
    cpu.funciones[0xE1 as usize].set_punt_y_val_a_fn(pop_hl, pop_hl_txt, 1, 10);
    // LR35902->LD(C),A         Z-80->JP NV,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE2 as usize].set_punt_y_val_a_fn(ldOff00_m_cO_aGB, ldOff00_m_cO_aGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE2 as usize].set_punt_y_val_a_fn(jp_po_nn, jp_po_nn_txt, 3, 10);
        }
    }
    cpu.funciones[0xE3 as usize].set_punt_y_val_a_fn(exOspO_hl, exOspO_hl_txt, 1, 19);
    cpu.funciones[0xE4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xE5 as usize].set_punt_y_val_a_fn(push_hl, push_hl_txt, 1, 11);
    cpu.funciones[0xE6 as usize].set_punt_y_val_a_fn(and_n, and_n_txt, 2, 7);
    cpu.funciones[0xE7 as usize].set_punt_y_val_a_fn(rst_20, rst_20_txt, 1, 11);
    ;
    // LR35902->ADD  SP,e          Z80->RET V
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 16);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 5);
        }
    }

    cpu.funciones[0xE9 as usize].set_punt_y_val_a_fn(jpOhlO, jpOhlO_txt, 1, 4);

    // LR35902->LD (nn),A         Z80->JP  V,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xEA as usize].set_punt_y_val_a_fn(ldOnnO_aGB, ldOnnO_aGB_txt, 3, 16);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xEA as usize].set_punt_y_val_a_fn(jp_pe_nn, jp_pe_nn_txt, 3, 10);
        }
    }
    cpu.funciones[0xEB as usize].set_punt_y_val_a_fn(ex_de_hl, ex_de_hl_txt, 1, 4);
    cpu.funciones[0xEC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xED as usize].set_punt_y_val_a_fn(ED, ED_txt, 0, 0); // Extensión
    cpu.funciones[0xEE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 7);
    cpu.funciones[0xEF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 11);

    // *************************** F ***********************************
    // LR35902->LDH  A,(n)           Z-80->RET P
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xF0 as usize].set_punt_y_val_a_fn(ld_a_Off00_m_nOGB, ld_a_Off00_m_nOGB_txt, 2, 12);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xF0 as usize].set_punt_y_val_a_fn(ret_p, ret_p_txt, 1, 5);
        }
    }
    cpu.funciones[0xF1 as usize].set_punt_y_val_a_fn(pop_af, pop_af_txt, 1, 10);

    // LR35902->LD   A,(C)         Z-80->JP  P,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xF2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xF2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
        }
    }

    cpu.funciones[0xF3 as usize].set_punt_y_val_a_fn(di, di_txt, 1, 4);
    cpu.funciones[0xF4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xF5 as usize].set_punt_y_val_a_fn(push_af, push_af_txt, 1, 11);
    cpu.funciones[0xF6 as usize].set_punt_y_val_a_fn(or_n, or_n_txt, 2, 7);
    cpu.funciones[0xF7 as usize].set_punt_y_val_a_fn(rst_30, rst_30_txt, 1, 11);
    ;

    // LR35902->LD   HL,(SP+e)     Z80->RET M
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xF8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 12);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xF8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 5);
        }
    }

    cpu.funciones[0xF9 as usize].set_punt_y_val_a_fn(ld_sp_hl, ld_sp_hl_txt, 1, 6);

    // LR35902->LD   A,(nn)        Z-80->JP  M,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xFA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 16);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xFA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
        }
    }

    cpu.funciones[0xFB as usize].set_punt_y_val_a_fn(ei, ei_txt, 1, 4);
    cpu.funciones[0xFC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 10);
    cpu.funciones[0xFD as usize].set_punt_y_val_a_fn(FD, FD_txt, 0, 0); // Extensión
    cpu.funciones[0xFE as usize].set_punt_y_val_a_fn(cp_n, cp_n_txt, 2, 7);
    cpu.funciones[0xFF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 11);
}

// XXXXXXXXXXXXXXXXXXX Funciones comunes en instrucciones básicas XXXXXXXXXXXXXXXXXXXX
// https://wikiti.brandonw.net/index.php?title=Z80_Instruction_Set
// Cuando varias funciones en los arreglos de punteros, tienen opciones comunes
// usan estas funciones, solo se tocaran los flags en estas funciones

// adc a,R 	10001rrr 	4 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += R + cf
pub fn bas_adc_a_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// adc a,J 	11i11101 1000110b 	8 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += J + cf
pub fn bas_adc_a_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// adc a,N 	11001110 nnnnnnnn 	7 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += N + cf
pub fn bas_adc_a_N(cpu: &mut CPU) { fn_no_impl(cpu); }

// adc a,(hl) 	10001110 	7 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += (hl) + cf
pub fn bas_adc_a_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// adc a,(I+D) 	11i11101 10001110 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += (I+D) + cf
pub fn bas_adc_a_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// adc hl,Q 	11101101 01qq1010 	15 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	hl += Q + cf
pub fn bas_adc_hl_Q(cpu: &mut CPU) { fn_no_impl(cpu); }

// add a,R 	10000rrr 	4 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += R
pub fn bas_add_a_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            resultado = cpu.a.wrapping_add(cpu.a);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.a, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.a as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.a & 0x0f)) > 0x0F);

            cpu.a = resultado;
        }
        0b00000_000 => {
            resultado = cpu.a.wrapping_add(cpu.b);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.b, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.b as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.b & 0x0f)) > 0x0F);

            cpu.b = resultado;
        }
        0b00000_001 => {
            resultado = cpu.a.wrapping_add(cpu.c);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.c, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.c as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.c & 0x0f)) > 0x0F);

            cpu.c = resultado;
        }
        0b00000_010 => {
            resultado = cpu.a.wrapping_add(cpu.d);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.d, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.d as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.d & 0x0f)) > 0x0F);

            cpu.d = resultado;
        }
        0b00000_011 => {
            resultado = cpu.a.wrapping_add(cpu.e);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.e, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.e as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.e & 0x0f)) > 0x0F);

            cpu.e = resultado;
        }
        0b00000_100 => {
            resultado = cpu.a.wrapping_add(cpu.h);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.h, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.h as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.h & 0x0f)) > 0x0F);

            cpu.h = resultado;
        }
        0b00000_101 => {
            resultado = cpu.a.wrapping_add(cpu.l);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.l, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.l as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (cpu.l & 0x0f)) > 0x0F);

            cpu.l = resultado;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = cpu.a.wrapping_add(dato);


            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, dato, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(dato as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + (dato & 0x0f)) > 0x0F);

            cpu.a = resultado;
        }
        _ => panic!("Instruccion en bas_add_a_R no reconocida"),
    }

    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// add a,J 	11i11101 1000010b 	8 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += J
pub fn bas_add_a_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// add a,N 	11000110 nnnnnnnn 	7 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += N
pub fn bas_add_a_N(cpu: &mut CPU) { fn_no_impl(cpu); }

//add a,(hl) 	10000110 	7 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += (hl)
pub fn bas_add_a_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//add a,(I+D) 	11i11101 10000110 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	0 	+ 	a += (I+D)
pub fn bas_add_a_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// add hl,Q 	00qq1001 	11 	- 	- 	+ 	+ 	+ 	- 	0 	+ 	hl += Q
pub fn bas_add_hl_Q(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    match cpu.r0 & 0b00_11_0000 {
        0b00_00_0000 => {
            let bc = cpu.lee_bc();

            cpu.set_flag(FLAG_C, (((hl as u32).wrapping_add(bc as u32)) & 0x10000) != 0);
            cpu.set_flag(FLAG_H, ((hl & 0x0FFF).wrapping_add(bc & 0x0FFF)) > 0x0FFF);
            hl = hl.wrapping_add(bc);
        }
        0b00_01_0000 => {
            let de = cpu.lee_de();

            cpu.set_flag(FLAG_C, (((hl as u32).wrapping_add(de as u32)) & 0x10000) != 0);
            //cpu.set_flag(FLAG_H, ((hl & 0x0FFF).wrapping_add(de & 0x0FFF)) > 0x0FFF);
            cpu.set_flag(FLAG_H, calc_half_carry_on_u16_sum(hl, de));

            hl = hl.wrapping_add(de);
        }
        0b00_10_0000 => {
            cpu.set_flag(FLAG_C, (((hl as u32).wrapping_add(hl as u32)) & 0x10000) != 0);
            cpu.set_flag(FLAG_H, ((hl & 0x0FFF).wrapping_add(hl & 0x0FFF)) > 0x0FFF);

            hl = hl.wrapping_add(hl);
        }
        0b00_11_0000 => {
            cpu.set_flag(FLAG_C, (((hl as u32).wrapping_add(cpu.sp as u32)) & 0x10000) != 0);
            cpu.set_flag(FLAG_H, ((hl & 0x0FFF).wrapping_add(cpu.sp & 0x0FFF)) > 0x0FFF);

            hl = hl.wrapping_add(cpu.sp);
        }
        _ => panic!("Instruccion en add_hl_Q no reconocida"),
    }

    cpu.escribe_hl(hl);

    cpu.set_flag(FLAG_N, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}


// add I,Q 	11i11101 00qq1001 	15 	- 	- 	+ 	+ 	+ 	- 	0 	+ 	I += Q
pub fn bas_add_I_Q(cpu: &mut CPU) { fn_no_impl(cpu); }

// and R 	10100rrr 	4 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND R
pub fn bas_and_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            resultado = cpu.a & cpu.a;
        }
        0b00000_000 => {
            resultado = cpu.a & cpu.b;
        }
        0b00000_001 => {
            resultado = cpu.a & cpu.c;
        }
        0b00000_010 => {
            resultado = cpu.a & cpu.d;
        }
        0b00000_011 => {
            resultado = cpu.a & cpu.e;
        }
        0b00000_100 => {
            resultado = cpu.a & cpu.h;
        }
        0b00000_101 => {
            resultado = cpu.a & cpu.l;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = cpu.a & dato;
        }
        _ => panic!("Instruccion en bas_and_R no reconocida"),
    }

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, paridad_bits_u8(resultado));
    cpu.set_flag(FLAG_H, true);
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_N, false);

    cpu.a = resultado;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// and J 	11i11101 1010010b 	8 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND J
pub fn bas_and_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// and N 	11100110 nnnnnnnn 	7 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND N
pub fn bas_and_N(cpu: &mut CPU) { fn_no_impl(cpu); }

// and (hl) 	10100110 	7 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND (hl)
pub fn bas_and_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// and (I+D) 	11i11101 10100110 dddddddd 	19 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND (I+D)
pub fn bas_and_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }


// bit B,(hl) 	11001011 01bbb110 	12 	+ 	+ 	X 	1 	X 	P 	0 	- 	tmp := (hl) AND [1 << B],
//xf := memptr.13, yf := memptr.11
pub fn bas_bit_B_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// bit B,(I+D) 	11i11101 11001011 dddddddd 01bbb*** 	20 	+ 	+ 	X 	1 	X 	P 	0 	-
// tmp := (I+D) AND [1 << B], xf := [I+D].13, yf := [I+D].11 }
pub fn bas_bit_B_OImDO_ix(cpu: &mut CPU) { fn_no_impl(cpu); }

// call A 	11001101 alalalal ahahahah 	17 	- 	- 	- 	- 	- 	- 	- 	- 	sp -= 2, (sp) := pc, pc := A
pub fn bas_call_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//call C,A 	11ccc100 alalalal ahahahah 	17/10 	- 	- 	- 	- 	- 	- 	- 	- 	if C then sp -= 2, (sp) := pc, pc := A
pub fn bas_call_C_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//ccf 	00111111 	4 	- 	- 	A 	X 	A 	- 	0 	X 	hf := cf, cf := ~cf
pub fn bas_ccf(cpu: &mut CPU) { fn_no_impl(cpu); }

// cp R 	10111rrr 	4 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - R, xf := R.5, yf = R.3
pub fn bas_cp_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            resultado = cpu.a.wrapping_sub(cpu.a);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.a, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.a as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.a & 0x0F));
        }
        0b00000_000 => {
            resultado = cpu.a.wrapping_sub(cpu.b);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.b, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.b as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.b & 0x0F));
        }
        0b00000_001 => {
            resultado = cpu.a.wrapping_sub(cpu.c);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.c, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.c as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.c & 0x0F));
        }
        0b00000_010 => {
            resultado = cpu.a.wrapping_sub(cpu.d);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.d, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.d as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.d & 0x0F));
        }
        0b00000_011 => {
            resultado = cpu.a.wrapping_sub(cpu.e);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.e, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.e as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.e & 0x0F));
        }
        0b00000_100 => {
            resultado = cpu.a.wrapping_sub(cpu.h);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.h, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.h as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.h & 0x0F));
        }
        0b00000_101 => {
            resultado = cpu.a.wrapping_sub(cpu.l);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.l, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.l as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (cpu.l & 0x0F));
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = cpu.a.wrapping_sub(dato);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, dato, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(dato as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, (cpu.a & 0x0F) < (dato & 0x0F));
        }
        _ => panic!("Instruccion en bas_cp_R no reconocida"),
    }

    cpu.set_flag(FLAG_N, true);
    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// cp J 	11i11101 1011110b 	8 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - J, xf := J.5, yf = J.3
pub fn bas_cp_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// cp N 	11111110 nnnnnnnn 	7 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - N, xf := N.5, yf =
// N.3
pub fn bas_cp_N(cpu: &mut CPU) { fn_no_impl(cpu); }

// cp (hl) 	10111110 	7 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - (hl), xf := (hl).5, yf =  (hl).3
pub fn bas_cp_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//cp (I+D) 	11i11101 10111110 dddddddd 	19 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - (I+D), xf := (I+D).5, yf = (I+D).3
pub fn bas_cp_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// cpd 	11101101 10101001 	16 	+ 	+ 	X 	+ 	X 	C 	1 	- 	tmp := a - (hl) => flags, bc -=
// 1, hl -= 1, xf := [tmp - hf].1, yf = [tmp - hf].3
pub fn bas_cpd(cpu: &mut CPU) { fn_no_impl(cpu); }

// cpdr 	11101101 10111001 	21/16 	+ 	+ 	X 	+ 	X 	C 	1 	- 	cpd, if bc <> 0 and nz
// then pc -= 2
pub fn bas_cpdr(cpu: &mut CPU) { fn_no_impl(cpu); }

// cpi 	11101101 10100001 	16 	+ 	+ 	X 	+ 	X 	C 	1 	- 	tmp := a - (hl) => flags, bc -=
// 1, hl += 1, xf := [tmp - hf].1, yf = [tmp - hf].3
pub fn bas_cpi(cpu: &mut CPU) { fn_no_impl(cpu); }

// cpir 	11101101 10110001 	21/16 	+ 	+ 	X 	+ 	X 	C 	1 	- 	cpi, if bc <> 0 and nz
// then pc -= 2
pub fn bas_cpir(cpu: &mut CPU) { fn_no_impl(cpu); }

//cpl 	00101111 	4 	- 	- 	+ 	1 	+ 	- 	1 	- 	a := ~a
pub fn bas_cpl(cpu: &mut CPU) { fn_no_impl(cpu); }


// aa 	00100111 	4 	+ 	+ 	+ 	X 	+ 	P 	- 	X 	tmp := a,
//if nf then
// if hf or [a AND 0x0f > 9] then tmp -= 0x06
// if cf or [a > 0x99] then tmp -= 0x60
//else
// if hf or [a AND 0x0f > 9] then tmp += 0x06
// if cf or [a > 0x99] then tmp += 0x60
//endif,
//tmp => flags, cf := cf OR [a > 0x99],
//hf := a.4 XOR tmp.4, a := tmp
pub fn bas_daa(cpu: &mut CPU) { fn_no_impl(cpu); }

// dec R 	00rrr101 	4 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	R -= 1
pub fn bas_dec_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00_111_000 {
        0b00_111_000 => {
            resultado = cpu.a.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.a == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, 1));

            cpu.a = resultado;
        }
        0b00_000_000 => {
            resultado = cpu.b.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.b, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.b == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.b, 1));

            cpu.b = resultado;
        }
        0b00_001_000 => {
            resultado = cpu.c.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.c, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.c == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.c, 1));

            cpu.c = resultado;
        }
        0b00_010_000 => {
            resultado = cpu.d.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.d, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.d == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.d, 1));

            cpu.d = resultado;
        }
        0b00_011_000 => {
            resultado = cpu.e.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.e, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.e == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.e, 1));

            cpu.e = resultado;
        }
        0b00_100_000 => {
            resultado = cpu.h.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.h, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.h == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.h, 1));

            cpu.h = resultado;
        }
        0b00_101_000 => {
            resultado = cpu.l.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.l, 1, resultado));
            cpu.set_flag(FLAG_PV, cpu.l == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.l, 1));

            cpu.l = resultado;
        }
        0b00_110_000 => {
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = dato.wrapping_sub(1);

            //cpu.set_flag(FLAG_PV, overflow_en_resta_u8(dato, 1, resultado));
            cpu.set_flag(FLAG_PV, dato == 0x80);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(dato, 1));

            cpu.mem.escribe_byte_en_mem(hl, resultado);
        }
        _ => panic!("Instruccion en bas_dec_R no reconocida"),
    }

    cpu.set_flag(FLAG_N, true);
    cpu.set_flag(FLAG_S, (resultado & FLAG_S) != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}


// dec J 	11i11101 0010b101 	8 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	J -= 1
pub fn bas_dec_J(cpu: &mut CPU) { fn_no_impl(cpu); }

//dec (hl) 	00110101 	11 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	(hl) -= 1
pub fn bas_dec_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//dec (I+D) 	11i11101 00110101 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	(I+D) -= 1
pub fn bas_dec_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// dec Q 	00qq1011 	6 	- 	- 	- 	- 	- 	- 	- 	- 	Q -= 1
pub fn bas_dec_Q(cpu: &mut CPU) {
    match cpu.r0 & 0b00_11_0000 {
        0b00_00_0000 => {
            let mut bc = cpu.lee_bc();
            bc = bc.wrapping_sub(1);

            cpu.escribe_bc(bc);
        }
        0b00_01_0000 => {
            let mut de = cpu.lee_de();
            de = de.wrapping_sub(1);

            cpu.escribe_de(de);
        }
        0b00_10_0000 => {
            let mut hl = cpu.lee_hl();
            hl = hl.wrapping_sub(1);

            cpu.escribe_hl(hl);
        }
        0b00_11_0000 => {
            cpu.sp = cpu.sp.wrapping_sub(1);
        }
        _ => panic!("Instruccion en bas_dec_Q no reconocida"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}


// dec I 	11i11101 00101011 	10 	- 	- 	- 	- 	- 	- 	- 	- 	I -= 1
pub fn bas_dec_I(cpu: &mut CPU) { fn_no_impl(cpu); }

//di 	11110011 	4 	- 	- 	- 	- 	- 	- 	- 	- 	iff1 := 0, iff2 := 0
pub fn bas_di(cpu: &mut CPU) { fn_no_impl(cpu); }

//djnz E 	00010000 dddddddd 	13/8 	- 	- 	- 	- 	- 	- 	- 	- 	b -= 1, if b <> 0 then pc := E
pub fn bas_djnz_E(cpu: &mut CPU) { fn_no_impl(cpu); }

//ei 	11111011 	4 	- 	- 	- 	- 	- 	- 	- 	- 	iff1 := 1, iff2 := 1 after the next instruction
pub fn bas_ei(cpu: &mut CPU) { fn_no_impl(cpu); }

//ex (sp),hl 	11100011 	19 	- 	- 	- 	- 	- 	- 	- 	- 	(sp) <=> hl
pub fn bas_ex_OspO_hl(cpu: &mut CPU) { fn_no_impl(cpu); }

//ex (sp),I 	11i11101 11100011 	23 	- 	- 	- 	- 	- 	- 	- 	- 	(sp) <=> I
pub fn bas_ex_OspO_I(cpu: &mut CPU) { fn_no_impl(cpu); }

//ex af,af' 	00001000 	4 	X 	X 	X 	X 	X 	X 	X 	X 	af <=> af'
pub fn bas_exaf_afp(cpu: &mut CPU) { fn_no_impl(cpu); }

//ex de,hl 	11101011 	4 	- 	- 	- 	- 	- 	- 	- 	- 	de <=> hl
pub fn bas_es_de_hl(cpu: &mut CPU) { fn_no_impl(cpu); }

//exx 	11011001 	4 	- 	- 	- 	- 	- 	- 	- 	- 	bc, de, hl <=> bc', de', hl'
pub fn bas_exx(cpu: &mut CPU) { fn_no_impl(cpu); }

//halt 	01110110 	4 	- 	- 	- 	- 	- 	- 	- 	- 	wait for interrupt
pub fn bas_halt(cpu: &mut CPU) { fn_no_impl(cpu); }

//im 0 	11101101 01*0*110 	8 	- 	- 	- 	- 	- 	- 	- 	- 	mode 0: execute instruction on bus
pub fn bas_im_0(cpu: &mut CPU) { fn_no_impl(cpu); }

//im 1 	11101101 01*10110 	8 	- 	- 	- 	- 	- 	- 	- 	- 	mode 1: execute rst $38
pub fn bas_im_1(cpu: &mut CPU) { fn_no_impl(cpu); }

//im 2 	11101101 01*11110 	8 	- 	- 	- 	- 	- 	- 	- 	- 	mode 2: call (i * 256 + byte on bus)
pub fn bas_im_2(cpu: &mut CPU) { fn_no_impl(cpu); }

// in a,(N) 	11011011 nnnnnnnn 	11 	- 	- 	- 	- 	- 	- 	- 	- 	a := ((N))
pub fn bas_in_a_ONO(cpu: &mut CPU) { fn_no_impl(cpu); }

// in R,(c) 	11101101 01rrr000 	12 	+ 	+ 	+ 	0 	+ 	P 	0 	- 	R := ((c))
pub fn bas_in_R_OcO(cpu: &mut CPU) { fn_no_impl(cpu); }

//in f,(c) 	11101101 01110000 	12 	+ 	+ 	+ 	0 	+ 	P 	0 	- 	tmp := ((c))
pub fn bas_in_f_OcO(cpu: &mut CPU) { fn_no_impl(cpu); }

// inc R 	00rrr100 	4 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	R += 1
pub fn bas_inc_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00_111_000 {
        0b00_111_000 => {
            resultado = cpu.a.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.a & 0x0f) + 1) > 0x0F);

            cpu.a = resultado;
        }
        0b00_000_000 => {
            resultado = cpu.b.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.b, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.b & 0x0f) + 1) > 0x0F);

            cpu.b = resultado;
        }
        0b00_001_000 => {
            resultado = cpu.c.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.c, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.c & 0x0f) + 1) > 0x0F);

            cpu.c = resultado;
        }
        0b00_010_000 => {
            resultado = cpu.d.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.d, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.d & 0x0f) + 1) > 0x0F);

            cpu.d = resultado;
        }
        0b00_011_000 => {
            resultado = cpu.e.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.e, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.e & 0x0f) + 1) > 0x0F);

            cpu.e = resultado;
        }
        0b00_100_000 => {
            resultado = cpu.h.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.h, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.h & 0x0f) + 1) > 0x0F);

            cpu.h = resultado;
        }
        0b00_101_000 => {
            resultado = cpu.l.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.l, 1, resultado));
            cpu.set_flag(FLAG_H, ((cpu.l & 0x0f) + 1) > 0x0F);

            cpu.l = resultado;
        }
        0b00_110_000 => { //(hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = dato.wrapping_add(1);

            cpu.set_flag(FLAG_PV, overflow_en_suma_u8(dato, 1, resultado));
            cpu.set_flag(FLAG_H, ((dato & 0x0f) + 1) > 0x0F);

            cpu.mem.escribe_byte_en_mem(hl, resultado);
        }
        _ => panic!("Instruccion en bas_inc_R no reconocida"),
    }

    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// inc J 	11i11101 0010b100 	8 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	J += 1
pub fn bas_inc_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// inc (hl) 	00110100 	11 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	(hl) += 1
pub fn bas_inc_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// inc (I+D) 	11i11101 00110100 dddddddd 	23 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	(I+D) += 1
pub fn bas_inc_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// inc Q 	00qq0011 	6 	- 	- 	- 	- 	- 	- 	- 	- 	Q += 1
pub fn bas_inc_Q(cpu: &mut CPU) {
    match cpu.r0 & 0b00_11_0000 {
        0b00_00_0000 => {
            let mut bc = cpu.lee_bc();
            bc = bc.wrapping_add(1);

            cpu.escribe_bc(bc);
        }
        0b00_01_0000 => {
            let mut de = cpu.lee_de();
            de = de.wrapping_add(1);

            cpu.escribe_de(de);
        }
        0b00_10_0000 => {
            let mut hl = cpu.lee_hl();
            hl = hl.wrapping_add(1);

            cpu.escribe_hl(hl);
        }
        0b00_11_0000 => {
            cpu.sp = cpu.sp.wrapping_add(1);
        }
        _ => panic!("Instruccion en bas_inc_Q no reconocida"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// inc I 	11i11101 00100011 	10 	- 	- 	- 	- 	- 	- 	- 	- 	I += 1
pub fn bas_inc_I(cpu: &mut CPU) { fn_no_impl(cpu); }

// ind 	11101101 10101010 	16 	+ 	+ 	+ 	X 	+ 	X 	X 	X
// tmp := ((c)), (hl) := tmp, hl -= 1,
//b -= 1 => flags, nf := tmp.7,
//tmp2 = tmp + [[c - 1] AND 0xff],
//pf := parity of [[tmp2 AND 0x07] XOR b],
//hf := cf := tmp2 > 255
pub fn bas_ind(cpu: &mut CPU) { fn_no_impl(cpu); }

// indr 	11101101 10111010 	21/16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	ind, if b <> 0 then pc -= 2
pub fn bas_indr(cpu: &mut CPU) { fn_no_impl(cpu); }

//ini 	11101101 10100010 	16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	tmp := ((c)), (hl) := tmp, hl += 1,
//b -= 1 => flags, nf := tmp.7,
//tmp2 := tmp + [[c + 1] AND 0xff],
//pf := parity of [[tmp2 AND 0x07] XOR b],
//hf := cf := tmp2 > 255
pub fn bas_ini(cpu: &mut CPU) { fn_no_impl(cpu); }

//inir 	11101101 10110010 	21/16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	ini, if b <> 0 then pc -= 2
pub fn bas_inir(cpu: &mut CPU) { fn_no_impl(cpu); }

//jp A 	11000011 alalalal ahahahah 	10 	- 	- 	- 	- 	- 	- 	- 	- 	pc := A
pub fn bas_jp_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//jp (hl) 	11101001 	4 	- 	- 	- 	- 	- 	- 	- 	- 	pc := hl
pub fn bas_jp_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//jp (I) 	11i11101 11101001 	8 	- 	- 	- 	- 	- 	- 	- 	- 	pc := I
pub fn bas_jp_OIO(cpu: &mut CPU) { fn_no_impl(cpu); }

// jp C,A 	11ccc010 alalalal ahahahah 	10 	- 	- 	- 	- 	- 	- 	- 	- 	if C then pc := A
pub fn bas_jp_C_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//jr E 	00011000 dddddddd 	12 	- 	- 	- 	- 	- 	- 	- 	- 	pc := E
pub fn bas_jr_E(cpu: &mut CPU) { fn_no_impl(cpu); }

//jr nz,E 	00100000 dddddddd 	12/7 	- 	- 	- 	- 	- 	- 	- 	- 	if nz then pc := E
pub fn bas_jr_nz_E(cpu: &mut CPU) { fn_no_impl(cpu); }

//jr z,E 	00101000 dddddddd 	12/7 	- 	- 	- 	- 	- 	- 	- 	- 	if zf then pc := E
pub fn bas_jr_z_E(cpu: &mut CPU) { fn_no_impl(cpu); }

//jr nc,E 	00110000 dddddddd 	12/7 	- 	- 	- 	- 	- 	- 	- 	- 	if nc then pc := E
pub fn bas_jr_nc_E(cpu: &mut CPU) { fn_no_impl(cpu); }

//jr c,E 	00111000 dddddddd 	12/7 	- 	- 	- 	- 	- 	- 	- 	- 	if cf then pc := E
pub fn bas_jr_c_E(cpu: &mut CPU) { fn_no_impl(cpu); }


// ld R1,R2 	01rrrsss 	4 	- 	- 	- 	- 	- 	- 	- 	- 	R1 := R2
pub fn bas_ld_R1_R2(cpu: &mut CPU) {
    match (cpu.r0 & 0b00111111) {
        0b000_000 => cpu.b = cpu.b,
        0b000_001 => cpu.b = cpu.c,
        0b000_010 => cpu.b = cpu.d,
        0b000_011 => cpu.b = cpu.e,
        0b000_100 => cpu.b = cpu.h,
        0b000_101 => cpu.b = cpu.l,
        0b000_110 => {
            let hl = cpu.lee_hl();
            cpu.b = cpu.mem.lee_byte_de_mem(hl);
        }
        0b000_111 => cpu.b = cpu.a,

        0b001_000 => cpu.c = cpu.b,
        0b001_001 => cpu.c = cpu.c,
        0b001_010 => cpu.c = cpu.d,
        0b001_011 => cpu.c = cpu.e,
        0b001_100 => cpu.c = cpu.h,
        0b001_101 => cpu.c = cpu.l,
        0b001_110 => {
            let hl = cpu.lee_hl();
            cpu.c = cpu.mem.lee_byte_de_mem(hl);
        }
        0b001_111 => cpu.c = cpu.a,

        0b010_000 => cpu.d = cpu.b,
        0b010_001 => cpu.d = cpu.c,
        0b010_010 => cpu.d = cpu.d,
        0b010_011 => cpu.d = cpu.e,
        0b010_100 => cpu.d = cpu.h,
        0b010_101 => cpu.d = cpu.l,
        0b010_110 => {
            let hl = cpu.lee_hl();
            cpu.d = cpu.mem.lee_byte_de_mem(hl);
        }
        0b010_111 => cpu.d = cpu.a,

        0b011_000 => cpu.e = cpu.b,
        0b011_001 => cpu.e = cpu.c,
        0b011_010 => cpu.e = cpu.d,
        0b011_011 => cpu.e = cpu.e,
        0b011_100 => cpu.e = cpu.h,
        0b011_101 => cpu.e = cpu.l,
        0b011_110 => {
            let hl = cpu.lee_hl();
            cpu.e = cpu.mem.lee_byte_de_mem(hl);
        }
        0b011_111 => cpu.e = cpu.a,

        0b100_000 => cpu.h = cpu.b,
        0b100_001 => cpu.h = cpu.c,
        0b100_010 => cpu.h = cpu.d,
        0b100_011 => cpu.h = cpu.e,
        0b100_100 => cpu.h = cpu.h,
        0b100_101 => cpu.h = cpu.l,
        0b100_110 => {
            let hl = cpu.lee_hl();
            cpu.h = cpu.mem.lee_byte_de_mem(hl);
        }
        0b100_111 => cpu.h = cpu.a,

        0b101_000 => cpu.l = cpu.b,
        0b101_001 => cpu.l = cpu.c,
        0b101_010 => cpu.l = cpu.d,
        0b101_011 => cpu.l = cpu.e,
        0b101_100 => cpu.l = cpu.h,
        0b101_101 => cpu.l = cpu.l,
        0b101_110 => {
            let hl = cpu.lee_hl();
            cpu.l = cpu.mem.lee_byte_de_mem(hl);
        }
        0b101_111 => cpu.l = cpu.a,

        0b110_000 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.b)
        }
        0b110_001 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.c)
        }
        0b110_010 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.d)
        }
        0b110_011 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.e)
        }
        0b110_100 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.h)
        }
        0b110_101 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.l)
        }

        0b110_111 => {
            let hl = cpu.lee_hl();
            cpu.mem.escribe_byte_en_mem(hl, cpu.a)
        }

        0b111_000 => cpu.a = cpu.b,
        0b111_001 => cpu.a = cpu.c,
        0b111_010 => cpu.a = cpu.d,
        0b111_011 => cpu.a = cpu.e,
        0b111_100 => cpu.a = cpu.h,
        0b111_101 => cpu.a = cpu.l,
        0b111_110 => {
            let hl = cpu.lee_hl();
            cpu.a = cpu.mem.lee_byte_de_mem(hl);
        }
        0b111_111 => cpu.a = cpu.a,

        _ => panic!("opcion no encontrada en bas_ld_R1_R2"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// ld R,J 	11i11101 01rrr10b 	8 	- 	- 	- 	- 	- 	- 	- 	- 	R := J
pub fn bas_ld_R_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld J,R 	11i11101 0110brrr 	8 	- 	- 	- 	- 	- 	- 	- 	- 	J := R
pub fn bas_ld_J_R(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld ixh,ixl 	11011101 01100101 	8 	- 	- 	- 	- 	- 	- 	- 	- 	ixh := ixl
pub fn bas_ld_ixh_ixl(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld ixl,ixh 	11011101 01101100 	8 	- 	- 	- 	- 	- 	- 	- 	- 	ixl :=
pub fn bas_ld_ixl_ixh(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld iyh,iyl 	11111101 01100101 	8 	- 	- 	- 	- 	- 	- 	- 	- 	iyh := iyl
pub fn bas_ld_iyh_iyl(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld iyl,iyh 	11111101 01101100 	8 	- 	- 	- 	- 	- 	- 	- 	- 	iyl := iyh
pub fn bas_ld_iyl_iyh(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld R,N 	00rrr110 nnnnnnnn 	7 	- 	- 	- 	- 	- 	- 	- 	- 	R := N
pub fn bas_ld_R_N(cpu: &mut CPU) {
    match cpu.r0 & 0b00_111_000 {
        0b00_111_000 => cpu.a = cpu.r1,
        0b00_000_000 => cpu.b = cpu.r1,
        0b00_001_000 => cpu.c = cpu.r1,
        0b00_010_000 => cpu.d = cpu.r1,
        0b00_011_000 => cpu.e = cpu.r1,
        0b00_100_000 => cpu.h = cpu.r1,
        0b00_101_000 => cpu.l = cpu.r1,
        _ => panic!("Instruccion en bas_ld_R_N no reconocida"),
    }

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// ld R,(hl) 	01rrr110 	7 	- 	- 	- 	- 	- 	- 	- 	- 	R := (hl)
pub fn bas_ld_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld R,(I+D) 	11i11101 01rrr110 dddddddd 	19 	- 	- 	- 	- 	- 	- 	- 	- 	R := (I+D)
pub fn bas_ld_R_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld (hl),R 	01110rrr 	7 	- 	- 	- 	- 	- 	- 	- 	- 	(hl) := R
pub fn bas_ld_OhlO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld (hl),N 	00110110 nnnnnnnn 	10 	- 	- 	- 	- 	- 	- 	- 	- 	(hl) := N
pub fn bas_ld_OhlO_N(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld (I+D),R 	11i11101 01110rrr dddddddd 	19 	- 	- 	- 	- 	- 	- 	- 	- 	(I+D) := R
pub fn bas_ld_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld (I+D),N 	11i11101 00110110 dddddddd nnnnnnnn 	19 	- 	- 	- 	- 	- 	- 	- 	- 	(I+D) := N
pub fn bas_ld_OImDO_N(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld a,(bc) 	00001010 	7 	- 	- 	- 	- 	- 	- 	- 	- 	a := (bc)
pub fn bas_ld_a_ObcO(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld a,(de) 	00011010 	7 	- 	- 	- 	- 	- 	- 	- 	- 	a := (de)
pub fn bas_ld_a_OdeO(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld a,(A) 	00111010 alalalal ahahahah 	13 	- 	- 	- 	- 	- 	- 	- 	- 	a := (A)
pub fn bas_ld_a_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld (bc),a 	00000010 	7 	- 	- 	- 	- 	- 	- 	- 	- 	(bc) := a
pub fn bas_ld_ObcO_a(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld (de),a 	00010010 	7 	- 	- 	- 	- 	- 	- 	- 	- 	(de) := a
pub fn bas_ld_OdeO_a(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld (A),a 	00110010 alalalal ahahahah 	13 	- 	- 	- 	- 	- 	- 	- 	- 	(A) := a
pub fn bas_ld_OAO_a(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld i,a 	11101101 01000111 	9 	- 	- 	- 	- 	- 	- 	- 	- 	i := a
pub fn bas_ld_i_a(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld r,a 	11101101 01001111 	9 	- 	- 	- 	- 	- 	- 	- 	- 	r := a
pub fn bas_ld_r_a(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld a,i 	11101101 01010111 	9 	+ 	+ 	+ 	0 	+ 	X 	0 	- 	a := i, pf := iff2
pub fn bas_ld_a_i(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld a,r 	11101101 01011111 	9 	+ 	+ 	+ 	0 	+ 	X 	0 	- 	a := r, pf := iff2
pub fn bas_ld_a_r(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld Q,A 	00qq0001 alalalal ahahahah 	10 	- 	- 	- 	- 	- 	- 	- 	- 	Q := A
pub fn bas_ld_Q_A(cpu: &mut CPU) {
    match cpu.r0 & 0b00_11_0000 {
        0b00_00_0000 => {
            cpu.c = cpu.r1;
            cpu.b = cpu.r2;
        }
        0b00_01_0000 => {
            cpu.e = cpu.r1;
            cpu.d = cpu.r2;
        }
        0b00_10_0000 => {
            cpu.l = cpu.r1;
            cpu.h = cpu.r2;
        }
        0b00_11_0000 => {
            cpu.sp = cpu.r1r2;
        }
        _ => panic!("Instruccion en bas_ld_Q no reconocida"),
    }


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// ld I,A 	11i11101 00100001 alalalal ahahahah 	14 	- 	- 	- 	- 	- 	- 	- 	- 	I := A
pub fn bas_ld_I_A(cpu: &mut CPU) { fn_no_impl(cpu); }


// ld hl,(A) 	00101010 alalalal ahahahah 	16 	- 	- 	- 	- 	- 	- 	- 	- 	hl := (A)
pub fn bas_ld_hl_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld I,(A) 	11i11101 00101010 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	- 	I := (A)
pub fn bas_ld_I_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }


// ld (A),hl 	00100010 alalalal ahahahah 	16 	- 	- 	- 	- 	- 	- 	- 	- 	(A) := hl
pub fn bas_ld_OAO_hl(cpu: &mut CPU) { fn_no_impl(cpu); }


// ld (A),I 	11i11101 00100010 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	- 	(A) := I
pub fn bas_ld_OAO_I(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld sp,hl 	11111001 	6 	- 	- 	- 	- 	- 	- 	- 	- 	sp := hl
pub fn bas_ld_sp_hl(cpu: &mut CPU) { fn_no_impl(cpu); }

//ld sp,I 	11i11101 11111001 	10 	- 	- 	- 	- 	- 	- 	- 	- 	sp := I
pub fn bas_ld_sp_I(cpu: &mut CPU) { fn_no_impl(cpu); }

//ldd 	11101101 10101000 	16 	- 	- 	X 	0 	X 	C 	0 	- 	tmp := (hl), (de) := tmp, de -= 1, hl -= 1,
//bc -= 1, xf := [tmp + a].1, yf = [tmp + a].3
pub fn bas_ldd(cpu: &mut CPU) { fn_no_impl(cpu); }

//lddr 	11101101 10111000 	21/16 	- 	- 	X 	0 	X 	C 	0 	- 	ldd, if bc <> 0 then pc -= 2
pub fn bas_lddr(cpu: &mut CPU) { fn_no_impl(cpu); }

//ldi 	11101101 10100000 	16 	- 	- 	X 	0 	X 	C 	0 	- 	tmp := (hl), (de) := tmp, de += 1, hl += 1,
//bc -= 1, xf := [tmp + a].1, yf = [tmp + a].3
pub fn bas_ldi(cpu: &mut CPU) { fn_no_impl(cpu); }

//ldir 	11101101 10110000 	21/16 	- 	- 	X 	0 	X 	C 	0 	- 	ldi, if bc <> 0 then pc -= 2
pub fn bas_ldir(cpu: &mut CPU) { fn_no_impl(cpu); }

//neg 	11101101 01***100 	8 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a := 0 - a
pub fn bas_neg(cpu: &mut CPU) { fn_no_impl(cpu); }

//nop 	00000000 	4 	- 	- 	- 	- 	- 	- 	- 	- 	nothing
pub fn bas_nop(cpu: &mut CPU) { fn_no_impl(cpu); }

// or R 	10110rrr 	4 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a OR R
pub fn bas_or_R(cpu: &mut CPU) {
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            cpu.a = cpu.a | cpu.a;
        }
        0b00000_000 => {
            cpu.a = cpu.a | cpu.b;
        }
        0b00000_001 => {
            cpu.a = cpu.a | cpu.c;
        }
        0b00000_010 => {
            cpu.a = cpu.a | cpu.d;
        }
        0b00000_011 => {
            cpu.a = cpu.a | cpu.e;
        }
        0b00000_100 => {
            cpu.a = cpu.a | cpu.h;
        }
        0b00000_101 => {
            cpu.a = cpu.a | cpu.l;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            cpu.a = cpu.a | dato;
        }
        _ => panic!("Instruccion en bas_or_R no reconocida"),
    }

    cpu.set_flag(FLAG_S, cpu.a & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, cpu.a == 0);
    cpu.set_flag(FLAG_PV, paridad_bits_u8(cpu.a)); // Como paridad
    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_N, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// or J 	11i11101 1011010b 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a OR J
pub fn bas_or_J(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// or N 	11110110 nnnnnnnn 	7 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a OR N
pub fn bas_or_N(cpu: &mut CPU) { fn_no_impl(cpu); }

//or (hl) 	10110110 	7 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a OR (hl)
pub fn bas_or_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//or (I+D) 	11i11101 10110110 dddddddd 	19 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a OR (I+D)
pub fn bas_or_OImJO(cpu: &mut CPU) { fn_no_impl(cpu); }

//out (N),a 	11010011 nnnnnnnn 	11 	- 	- 	- 	- 	- 	- 	- 	- 	((N)) := a
pub fn bas_out_ONO_a(cpu: &mut CPU) { fn_no_impl(cpu); }

// out (c),R 	11101101 01rrr001 	12 	- 	- 	- 	- 	- 	- 	- 	- 	((c)) := R
pub fn bas_out_OcO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// out (c),0 	11101101 01110001 	12 	- 	- 	- 	- 	- 	- 	- 	- 	((c)) := ? (seems to vary with CPU)
pub fn bas_out_OcO_0(cpu: &mut CPU) { fn_no_impl(cpu); }

//outd 	11101101 10101011 	16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	tmp := (hl), ((c)) := tmp, hl -= 1,
//b -= 1 => flags, nf := tmp.7, tmp2 = tmp + l,
//pf := parity of [[tmp2 AND 0x07] XOR b],
//hf := cf := tmp2 > 255
pub fn bas_outd(cpu: &mut CPU) { fn_no_impl(cpu); }

//otdr 	11101101 10111011 	21/16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	outd, if b <> 0 then pc -= 2
pub fn bas_otdr(cpu: &mut CPU) { fn_no_impl(cpu); }

//outi 	11101101 10100011 	16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	tmp := (hl), ((c)) := tmp, hl += 1,
//b -= 1 => flags, nf := tmp.7, tmp2 = tmp + l,
//pf := parity of [[tmp2 AND 0x07] XOR b],
//hf := cf := tmp2 > 255
pub fn bas_outi(cpu: &mut CPU) { fn_no_impl(cpu); }

//otir 	11101101 10110011 	21/16 	+ 	+ 	+ 	X 	+ 	X 	X 	X 	outi, if b <> 0 then pc -= 2
pub fn bas_otir(cpu: &mut CPU) { fn_no_impl(cpu); }

// pop P 	11pp0001 	10 	- 	- 	- 	- 	- 	- 	- 	- 	P := (sp), sp += 2
pub fn bas_pop_P(cpu: &mut CPU) { fn_no_impl(cpu); }

// pop I 	11i11101 11100001 	14 	- 	- 	- 	- 	- 	- 	- 	- 	I := (sp), sp += 2
pub fn bas_pop_I(cpu: &mut CPU) { fn_no_impl(cpu); }

// push P 	11pp0101 	11 	- 	- 	- 	- 	- 	- 	- 	- 	sp -= 2, (sp) := P
pub fn bas_push_P(cpu: &mut CPU) { fn_no_impl(cpu); }

// push I 	11i11101 11100101 	15 	- 	- 	- 	- 	- 	- 	- 	- 	sp -= 2, (sp) := I
pub fn bas_push_I(cpu: &mut CPU) { fn_no_impl(cpu); }


// res B,(hl) 	11001011 10bbb110 	15 	- 	- 	- 	- 	- 	- 	- 	- 	(hl) := (hl) AND ~[1 << B]
pub fn bas_res_B_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// res B,(I+D) 	11i11101 11001011 dddddddd 10bbb110 	23 	- 	- 	- 	- 	- 	- 	- 	-
// (I+D) := (I+D) AND ~[1 << B]
pub fn bas_res_B_OImDO_ix(cpu: &mut CPU) { fn_no_impl(cpu); }

// res B,(I+D)->R 	11i11101 11001011 dddddddd 10bbbrrr 	23 	- 	- 	- 	- 	- 	- 	- 	-
// (I+D) := R := (I+D) AND ~[1 << B]
pub fn bas_res_B_OImD_R(cpu: &mut CPU) { fn_no_impl(cpu); }


// ret 	11001001 	10 	- 	- 	- 	- 	- 	- 	- 	- 	pc := (sp), sp += 2
pub fn bas_ret(cpu: &mut CPU) { fn_no_impl(cpu); }

//ret C 	11ccc000 	11/5 	- 	- 	- 	- 	- 	- 	- 	- 	if C then pc := (sp), sp += 2
pub fn bas_ret_C(cpu: &mut CPU) { fn_no_impl(cpu); }

//reti 	11101101 01**1101 	14 	- 	- 	- 	- 	- 	- 	- 	- 	pc := (sp), sp += 2, iff1 := iff2
pub fn bas_reti(cpu: &mut CPU) { fn_no_impl(cpu); }

//retn 	11101101 01**0101 	14 	- 	- 	- 	- 	- 	- 	- 	- 	pc := (sp), sp += 2, iff1 := iff2
pub fn bas_retn(cpu: &mut CPU) { fn_no_impl(cpu); }

//rla 	00010111 	4 	- 	- 	+ 	0 	+ 	- 	0 	X 	ocf := cf, cf := a.7, a := [a << 1] + ocf
pub fn bas_rla(cpu: &mut CPU) { fn_no_impl(cpu); }

// rl R 	11001011 00010rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X
pub fn bas_rl_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rl (hl) 	11001011 00010110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	ocf := cf, cf := (hl).7, (hl) := [(hl) << 1] + ocf
pub fn bas_rl_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//rl (I+D) 	11i11101 11001011 dddddddd 00010110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	ocf := cf, cf := (I+D).7,
//(I+D) := [(I+D) << 1] + ocf
pub fn bas_rl_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rl (I+D)->R 	11i11101 11001011 dddddddd 00010rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// ocf := cf, cf := (I+D).7, (I+D) := R := [(I+D) << 1] + ocf
pub fn bas_rl_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rlca 	00000111 	4 	- 	- 	+ 	0 	+ 	- 	0 	X 	cf := a.7, a := [a << 1] + cf
pub fn bas_rlca(cpu: &mut CPU) { fn_no_impl(cpu); }

// rlc R 	11001011 00000rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := R.7, R := [R << 1] + cf
pub fn bas_rlc_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rlc (hl) 	11001011 00000110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).7, (hl) := [(hl) << 1] + cf
pub fn bas_rlc_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rlc (I+D) 	11i11101 11001011 dddddddd 00000110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := [(I+D) << 1] + cf
pub fn bas_rlc_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rlc (I+D)->R 	11i11101 11001011 dddddddd 00000rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := R := [(I+D) << 1] + cf
pub fn bas_rlc_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rld 	11101101 01101111 	18 	+ 	+ 	+ 	0 	+ 	P 	0 	- 	tmp := [(hl) << 4] + [a AND 0x0f], (hl) := tmp,
//a := [a AND 0xf0] + [tmp >> 8] => flags
pub fn bas_rld(cpu: &mut CPU) { fn_no_impl(cpu); }

//rra 	00011111 	4 	- 	- 	+ 	0 	+ 	- 	0 	X 	ocf := cf, cf := a.0, a := [a >> 1] + [ocf << 7]
pub fn bas_rra(cpu: &mut CPU) { fn_no_impl(cpu); }

// rr R 	11001011 00011rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// ocf := cf, cf := R.0, R := [R >> 1] + [ocf << 7]
pub fn bas_rr_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rr (hl) 	11001011 00011110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	ocf := cf, cf := (hl).0,
//(hl) := [(hl) >> 1] + [ocf << 7]
pub fn bas_rr_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rr (I+D) 	11i11101 11001011 dddddddd 00011110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// ocf := cf, cf := (I+D).0, (I+D) := [(I+D) >> 1] + [ocf << 7]
pub fn bas_rr_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rr (I+D)->R 	11i11101 11001011 dddddddd 00011rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// ocf := cf, cf := (I+D).0, (I+D) := R := [(I+D) >> 1] + [ocf << 7]
pub fn bas_rr_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rrca 	00001111 	4 	- 	- 	+ 	0 	+ 	- 	0 	X 	cf := a.0, a := [a >> 1] + [cf << 7]
pub fn bas_rrca(cpu: &mut CPU) { fn_no_impl(cpu); }

// rrc R 	11001011 00001rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := R.0, R := [R >> 1] + [cf << 7]
pub fn bas_rrc_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rrc (hl) 	11001011 00001110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).0, (hl) := [(hl) >> 1] + [cf << 7]
pub fn bas_rrc_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// rrc (I+D) 	11i11101 11001011 dddddddd 00001110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, (I+D) := [(I+D) >> 1] + [cf << 7]
pub fn bas_rrc_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }


// rrc (I+D)->R 	11i11101 11001011 dddddddd 00001rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, (I+D) := R := [(I+D) >> 1] + [cf << 7]
pub fn bas_rrc_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// rrd 	11101101 01100111 	18 	+ 	+ 	+ 	0 	+ 	P 	0 	- 	tmp := (hl),
//(hl) := [tmp >> 4] + [[a AND 0x0f] << 4],
//a := [a AND 0xf0] + [tmp AND 0x0f] => flags
pub fn bas_rrd(cpu: &mut CPU) { fn_no_impl(cpu); }

// rst S 	11sss111 	11 	- 	- 	- 	- 	- 	- 	- 	- 	sp -= 2, (sp) := pc, pc := S
pub fn bas_rst_S(cpu: &mut CPU) { fn_no_impl(cpu); }

// sbc a,R 	10011rrr 	4 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= R + cf
pub fn bas_sbc_a_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    let mut carry = 0;
    if cpu.get_c_flag() { carry = 1 } else { carry = 0 };
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            let a = (cpu.a).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(a);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, a));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, a, resultado));
        }
        0b00000_000 => {
            let b = (cpu.b).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(b);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, b));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, b, resultado));
        }
        0b00000_001 => {
            let c = (cpu.c).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(c);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, c));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, c, resultado));
        }
        0b00000_010 => {
            let d = (cpu.d).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(d);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, d));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, d, resultado));
        }
        0b00000_011 => {
            let e = (cpu.e).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(e);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, e));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, e, resultado));
        }
        0b00000_100 => {
            let h = (cpu.h).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(h);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, h));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, h, resultado));
        }
        0b00000_101 => {
            let l = (cpu.l).wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(l);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, l));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, l, resultado));
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            let dato_inc = dato.wrapping_add(carry);
            resultado = cpu.a.wrapping_sub(dato_inc);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, dato_inc));
            cpu.set_flag(FLAG_PV, !overflow_en_resta_u8(cpu.a, dato_inc, resultado));
        }
        _ => panic!("Instruccion en bas_sbc_a_R no reconocida"),
    }

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_N, true);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// sbc a,J 	11i11101 1001110b 	8 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= J + cf
pub fn bas_sbc_a_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// sbc a,N 	11011110 nnnnnnnn 	7 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= N + cf
pub fn bas_sbc_a_N(cpu: &mut CPU) { fn_no_impl(cpu); }

//sbc a,(hl) 	10011110 	7 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= (hl) + cf
pub fn bas_sbc_a_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sbc a,(I+D) 	11i11101 10011110 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= (I+D) + cf
pub fn bas_sbc_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sbc hl,Q 	11101101 01qq0010 	15 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	hl -= Q + cf
pub fn bas_sbc_hl_Q(cpu: &mut CPU) { fn_no_impl(cpu); }

// scf 	00110111 	4 	- 	- 	A 	0 	A 	- 	0 	1 	nothing else
pub fn bas_scf(cpu: &mut CPU) { fn_no_impl(cpu); }


// set B,(hl) 	11001011 11bbb110 	15 	- 	- 	- 	- 	- 	- 	- 	- 	(hl) := (hl) OR [1 << B]
pub fn bas_set_B_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// set B,(I+D) 	11i11101 11001011 dddddddd 11bbb110 	23 	- 	- 	- 	- 	- 	- 	- 	-
// (I+D) := (I+D) OR [1 << B]
pub fn bas_set_B_OImDO_ix(cpu: &mut CPU) { fn_no_impl(cpu); }

// set B,(I+D)->R 	11i11101 11001011 dddddddd 11bbbrrr 	23 	- 	- 	- 	- 	- 	- 	- 	-
// (I+D) := R := (I+D) OR [1 << B]
pub fn bas_set_B_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sla R 	11001011 00100rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := R.7, R := R << 1
pub fn bas_sla_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sla (hl) 	11001011 00100110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).7, (hl) := (hl) << 1
pub fn bas_sla_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sla (I+D) 	11i11101 11001011 dddddddd 00100110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := (I+D) << 1
pub fn bas_sla_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sla (I+D)->R 	11i11101 11001011 dddddddd 00100rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := R := (I+D) << 1
pub fn bas_sla_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sra R 	11001011 00101rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := R.0, R := R >> 1, R.7 := R.6
pub fn bas_sra_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sra (hl) 	11001011 00101110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).0, (hl) := (hl) >> 1, (hl).7 := (hl).6
pub fn bas_sra_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sra (I+D) 	11i11101 11001011 dddddddd 00101110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, (I+D) := (I+D) >> 1, (I+D).7 := (I+D).6
pub fn bas_sra_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sra (I+D)->R 	11i11101 11001011 dddddddd 00101rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, tmp := (I+D) >> 1, tmp.7 := tmp.6   (I+D) := R := tmp
pub fn bas_sra_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sll R 	11001011 00110rrr 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := R.7, R := [R << 1] + 1
pub fn bas_sll_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sll (hl) 	11001011 00110110 	15 	+ 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).7, (hl) := [(hl) << 1] + 1
pub fn bas_sll_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// sll (I+D) 	11i11101 11001011 dddddddd 00110110 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := [(I+D) << 1] + 1
pub fn bas_sll_OImD(cpu: &mut CPU) { fn_no_impl(cpu); }

// sll (I+D)->R 	11i11101 11001011 dddddddd 00110rrr 	23 	+ 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).7, (I+D) := R := [(I+D) << 1] + 1
pub fn bas_sll_OImD_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// srl R 	11001011 00111rrr 	8 	0 	+ 	+ 	0 	+ 	P 	0 	X 	cf := R.0, R := R >> 1
pub fn bas_srl_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// srl (hl) 	11001011 00111110 	15 	0 	+ 	+ 	0 	+ 	P 	0 	X 	cf := (hl).0, (hl) := (hl) >> 1
pub fn bas_srl_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// srl (I+D) 	11i11101 11001011 dddddddd 00111110 	23 	0 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, (I+D) := (I+D) >> 1
pub fn bas_srl_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// srl (I+D)->R 	11i11101 11001011 dddddddd 00111rrr 	23 	0 	+ 	+ 	0 	+ 	P 	0 	X
// cf := (I+D).0, (I+D) := R := (I+D) >> 1
pub fn bas_srl_OImDO_R(cpu: &mut CPU) { fn_no_impl(cpu); }

// sub R 	10010rrr 	4 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= R
pub fn bas_sub_R(cpu: &mut CPU) {
    let mut resultado: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            resultado = cpu.a.wrapping_sub(cpu.a);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.a, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.a as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.a));

            cpu.a = resultado;
        }
        0b00000_000 => {
            resultado = cpu.a.wrapping_sub(cpu.b);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.b, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.b as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.b));

            cpu.a = resultado;
        }
        0b00000_001 => {
            resultado = cpu.a.wrapping_sub(cpu.c);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.c, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.c as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.c));

            cpu.a = resultado;
        }
        0b00000_010 => {
            resultado = cpu.a.wrapping_sub(cpu.d);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.d, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.d as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.d));

            cpu.a = resultado;
        }
        0b00000_011 => {
            resultado = cpu.a.wrapping_sub(cpu.e);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.e, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.e as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.e));

            cpu.a = resultado;
        }
        0b00000_100 => {
            resultado = cpu.a.wrapping_sub(cpu.h);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.h, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.h as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.h));

            cpu.a = resultado;
        }
        0b00000_101 => {
            resultado = cpu.a.wrapping_sub(cpu.l);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.l, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.l as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.l));

            cpu.a = resultado;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            resultado = cpu.a.wrapping_sub(dato);

            cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, dato, resultado));
            cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(dato as u16)) & 0x100) != 0);
            cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, dato));

            cpu.mem.escribe_byte_en_mem(hl, resultado);
        }
        _ => panic!("Instruccion en bas_sub_R no reconocida"),
    }

    cpu.set_flag(FLAG_N, true);
    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// sub J 	11i11101 1001010b 	8 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= J
pub fn bas_sub_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// sub N 	11010110 nnnnnnnn 	7 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= N
pub fn bas_sub_N(cpu: &mut CPU) { fn_no_impl(cpu); }

//sub (hl) 	10010110 	7 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= (hl)
pub fn bas_sub_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

//sub (I+D) 	11i11101 10010110 dddddddd 	19 	+ 	+ 	+ 	+ 	+ 	V 	1 	+ 	a -= (I+D)
pub fn bas_sub_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// xor R 	10101rrr 	4 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a XOR R
pub fn bas_xor_R(cpu: &mut CPU) {
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            cpu.a = cpu.a ^ cpu.a;
        }
        0b00000_000 => {
            cpu.a = cpu.a ^ cpu.b;
        }
        0b00000_001 => {
            cpu.a = cpu.a ^ cpu.c;
        }
        0b00000_010 => {
            cpu.a = cpu.a ^ cpu.d;
        }
        0b00000_011 => {
            cpu.a = cpu.a ^ cpu.e;
        }
        0b00000_100 => {
            cpu.a = cpu.a ^ cpu.h;
        }
        0b00000_101 => {
            cpu.a = cpu.a ^ cpu.l;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            cpu.a = cpu.a ^ dato;
        }
        _ => panic!("Instruccion en bas_xor_R no reconocida"),
    }

    cpu.set_flag(FLAG_S, cpu.a & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, cpu.a == 0);
    cpu.set_flag(FLAG_PV, paridad_bits_u8(cpu.a));// Como paridad
    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_N, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

// xor J 	11i11101 1010110b 	8 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a XOR J
pub fn bas_xor_J(cpu: &mut CPU) { fn_no_impl(cpu); }

// xor N 	11101110 nnnnnnnn 	7 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a XOR N
pub fn bas_xor_N(cpu: &mut CPU) { fn_no_impl(cpu); }

//xor (hl) 	10101110 	7 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a XOR (hl)
pub fn bas_xor_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

// xor (I+D) 	11i11101 10101110 dddddddd 	19 	+ 	+ 	+ 	0 	+ 	P 	0 	0 	a := a XOR (I+D)
pub fn bas_xor_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn fn_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion normal no implementada\n\
    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
    r3 = #{:02X}\n",
                   cpu.pc, cpu.r0, cpu.r1, cpu.r2, cpu.r3));
}

// O = ()     p = '    m = +       n = valor hex de 8 bits
// *************************** 0 ***********************************
// 0x00    nop 	00000000 	- 	- 	- 	- 	- 	- 	- 	- 	nothing
pub fn nop(cpu: &mut CPU) {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn nop_txt(cpu: &mut CPU) {
    cpu.texto(&format!("NOP"));
}

/// 0x01 "ld bc,NN"     ld Q,A
///     The 16-bit integer value is loaded to the BC register pair.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0x01
///     =================================
///     |             N Low             |
///     =================================
///     |             N High            |
///     =================================
///     T-States: 10
pub fn ld_bc_nn(cpu: &mut CPU) { bas_ld_Q_A(cpu); }

pub fn ld_bc_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD BC,#{:04X}", cpu.r1r2)); }

/// 0x02   "ld (bc),a" 	00000010 	- 	- 	- 	- 	- 	- 	- 	- 	(bc) := a
///     The contents of the A are loaded to the memory location
///     specified by the contents of the register pair BC.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0x02
///     =================================
///     T-States: 7
pub fn ldObcO_a(cpu: &mut CPU) {
    let bc = cpu.lee_bc();
    cpu.mem.escribe_byte_en_mem(bc, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldObcO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (BC),A"));
}

/// 0x03   "inc bc"
///     The contents of register pair BC are incremented.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0x03
///     =================================
///     T-States: 6

pub fn inc_bc(cpu: &mut CPU) {
    bas_inc_Q(cpu);
}

pub fn inc_bc_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC BC"));
}

/// 0x04   "inc b"
///     Register B is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0x04
///     =================================
///     T-States: 4
pub fn inc_b(cpu: &mut CPU) { bas_inc_R(cpu); }

pub fn inc_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC B"));
}

/// 0x05   "dec b"
///     Register B is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0x05
///     =================================
///     T-States: 4
pub fn dec_b(cpu: &mut CPU) { bas_dec_R(cpu); }

pub fn dec_b_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC B")); }

/// 0x06   "ld b,N"
///     The 8-bit integer N is loaded to B.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0x06
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_b_n(cpu: &mut CPU) { bas_ld_R_N(cpu); }

pub fn ld_b_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,#{:02X}", cpu.r1)); }

/// 0x07   "rlca"
///     The contents of  A are rotated left 1 bit position. The
///     sign bit (bit 7) is copied to the Carry flag and also
///     to bit 0.
///     S, Z, P/V are not affected.
///     H, N are reset.
///     C is data from bit 7 of A.
///     =================================
///     | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 0x07
///     =================================
///     T-States: 4
pub fn rlca(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn rlca_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

/// 0x08 Difiere según procesador (LR35902->LD(NN),SP)
pub fn ldOnnO_spGB(cpu: &mut CPU) {
    cpu.mem.escribe_2bytes_en_mem(cpu.r1r2, cpu.sp);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_spGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),SP", cpu.r1r2));
}

/// 0x08 Difiere según procesador (Z80->EX AF,AF')
///     "ex af,af'"
///     The 2-byte contents of the register pairs AF and AF' are exchanged.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0x08
///     =================================
///     T-States: 4
pub fn ex_af_afp(cpu: &mut CPU) {
    let a = cpu.a;
    let f = cpu.f;
    cpu.a = cpu.ap;
    cpu.f = cpu.fp;
    cpu.ap = a;
    cpu.fp = f;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ex_af_afp_txt(cpu: &mut CPU) {
    cpu.texto(&format!("EX AF,AF'"));
}

/// 0x09   "add hl,bc"
///     The contents of BC are added to the contents of HL and
///     the result is stored in HL.
///     S, Z, P/V are not affected.
///     H is set if carry from bit 11; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 15; otherwise, it is reset.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0x09
///     =================================
///     T-States: 11
pub fn add_hl_bc(cpu: &mut CPU) { bas_add_hl_Q(cpu); }

pub fn add_hl_bc_txt(cpu: &mut CPU) {
    cpu.texto(&format!("ADD HL,BC"));
}

/// 0x0A   "ld a,(bc)" 	00001010 	- 	- 	- 	- 	- 	- 	- 	- 	a := (bc)
///     The contents of the memory location specified by BC are loaded to A.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0x0A
///     =================================
///     T-States: 7
pub fn ld_aObcO(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn ld_aObcO_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x0B   "dec bc"            dec Q 	00qq1011 	- 	- 	- 	- 	- 	- 	- 	- 	Q -= 1
///     The contents of register pair BC are decremented.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 0x0B
///     =================================
///     T-States: 6
pub fn dec_bc(cpu: &mut CPU) {
    bas_dec_Q(cpu);
}

pub fn dec_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC BC")); }

/// 0x0C   "inc c"             inc R 	00rrr100 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	R += 1
///     Register C is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0x0C
///     =================================
///     T-States: 4
pub fn inc_c(cpu: &mut CPU) { bas_inc_R(cpu); }

pub fn inc_c_txt(cpu: &mut CPU) { cpu.texto(&format!("INC C")); }

/// 0x0D  "dec c"                 dec R 	00rrr101 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	R -= 1
///     Register C is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0x0D
///     =================================
///     T-States: 4
pub fn dec_c(cpu: &mut CPU) { bas_dec_R(cpu); }

pub fn dec_c_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC C")); }

/// 0x0E   "ld c,N"
///     The 8-bit integer N is loaded to C.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 0 | 0x0E
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_c_n(cpu: &mut CPU) { bas_ld_R_N(cpu); }

pub fn ld_c_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,#{:02X}", cpu.r1)); }

/// 0x0F   "rrca"
///     The contents of A are rotated right 1 bit position. Bit 0 is
///     copied to the Carry flag and also to bit 7.
///     S, Z, P/V are not affected.
///     H, N are reset.
///     C is data from bit 0 of A.
///     =================================
///     | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0x0F
///     =================================
///     T-States: 4
pub fn rrca(cpu: &mut CPU) {
    let bit0: bool = (0b0000_0001 & cpu.a) != 0;

    // Rotación
    let mut nuevo_valor = cpu.a >> 1;
    nuevo_valor = nuevo_valor & 0b0111_1111;

    if bit0 { nuevo_valor |= 0b1000_0000; }

    cpu.set_flag(FLAG_Z, nuevo_valor == 0);
    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_C, bit0);

    cpu.a = nuevo_valor;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rrca_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RRCA"));
}
// *************************** 1 ***********************************

/// 0x10   "djnz E"
///     This instruction is similar to the conditional jump
///     instructions except that value of B is used to determine
///     branching. B is decremented, and if a nonzero value remains,
///     the value of displacement E is added to PC. The next
///     instruction is fetched from the location designated by
///     the new contents of the PC. The jump is measured from the
///     address of the instruction op code and contains a range of
///     –126 to +129 bytes. The assembler automatically adjusts for
///     the twice incremented PC. If the result of decrementing leaves
///     B with a zero value, the next instruction executed is taken
///     from the location following this instruction.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0x10
///     =================================
///     |             E-2               |
///     =================================
///     T-States: 13

pub fn djnz_n(cpu: &mut CPU) {
    cpu.pc += cpu.get_bytes_instruccion();
    cpu.b = (cpu.b).wrapping_sub(1);
    if cpu.b != 0 {
        cpu.pc = suma_compl2_a_un_u16(cpu.pc, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn djnz_n_txt(cpu: &mut CPU) {
    let mut d = cpu.pc + cpu.get_bytes_instruccion();
    let direccion = suma_compl2_a_un_u16(d, cpu.r1);
    cpu.texto(&format!("DJNZ {:04X}", direccion));
}

/// 0x11   "ld de,NN"
///     The 16-bit integer value is loaded to the DE register pair.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0x11
///     =================================
///     |             N Low             |
///     =================================
///     |             N High            |
///     =================================
///     T-States: 10
pub fn ld_de_nn(cpu: &mut CPU) { bas_ld_Q_A(cpu); }

pub fn ld_de_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD DE,#{:04X}", cpu.r1r2)); }

/// 0x12   "ld (de),a"
///     The contents of the A are loaded to the memory location
///     specified by the contents of the register pair DE.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 0x12
///     =================================
///     T-States: 7
pub fn ldOdeO_a(cpu: &mut CPU) {
    let de = cpu.lee_de();
    cpu.mem.escribe_byte_en_mem(de, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOdeO_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(DE),A")); }

/// 0x13   "inc de"
///     The contents of register pair DE are incremented.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 0x13
///     =================================
///     T-States: 6
pub fn inc_de(cpu: &mut CPU) { bas_inc_Q(cpu); }

pub fn inc_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC DE"));
}

/// 0x14   "inc d"
///     Register D is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0x14
///     =================================
///     T-States: 4
pub fn inc_d(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_d_txt(cpu: &mut CPU) { cpu.texto(&format!("INC D")); }

/// 0x15   "dec d"
///     Register D is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0x15
///     =================================
///     T-States: 4
pub fn dec_d(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_d_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC D")); }

/// 0x16   "ld d,N"
///     The 8-bit integer N is loaded to D.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 0x16
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_d_n(cpu: &mut CPU) { bas_ld_R_N(cpu); }

pub fn ld_d_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD D #{:02X}", cpu.r1));
}


/// 0x17   "rla"
///     The contents of A are rotated left 1 bit position through the
///     Carry flag. The previous contents of the Carry flag are copied
///     to bit 0.
///     S, Z, P/V are not affected.
///     H, N are reset.
///     C is data from bit 7 of A.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0x17
///     =================================
///     T-States: 4
pub fn rla(cpu: &mut CPU) {
    let viejo_c_flag = cpu.get_c_flag();
    cpu.set_flag(FLAG_C, (0b1000_0000 & cpu.a) != 0);

    // Rotación
    let mut nuevo_valor = cpu.a << 1;
    nuevo_valor = nuevo_valor & 0b1111_1110;
    if viejo_c_flag { nuevo_valor |= 0b0000_0001; }

    cpu.set_flag(FLAG_Z, nuevo_valor == 0);
    cpu.set_flag(FLAG_N, false);
    cpu.set_flag(FLAG_H, false);

    cpu.a = nuevo_valor;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rla_txt(cpu: &mut CPU) { cpu.texto(&format!("RLA")); }

/// 0x18   "jr e"
///     This instruction provides for unconditional branching
///     to other segments of a program. The value of displacement E is
///     added to PC and the next instruction is fetched from the location
///     designated by the new contents of the PC. This jump is measured
///     from the address of the instruction op code and contains a range
///     of –126 to +129 bytes. The assembler automatically adjusts for
///     the twice incremented PC.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 0x18
///     =================================
///     |             E-2               |
///     =================================
///     T-States: 12
pub fn jr_n(cpu: &mut CPU) {
    cpu.pc += 2;
    cpu.pc = suma_compl2_a_un_u16(cpu.pc, cpu.r1);

    cpu.t += cpu.get_t_instruccion();
}


pub fn jr_n_txt(cpu: &mut CPU) {
    let direccion = suma_compl2_a_un_u16(cpu.pc, cpu.r1) + 2;
    cpu.texto(&format!("JR {:04X}", direccion));
}

/// 0x19   "add hl,de"
///     The contents of DE are added to the contents of HL and
///     the result is stored in HL.
///     S, Z, P/V are not affected.
///     H is set if carry from bit 11; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 15; otherwise, it is reset.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0x19
///     =================================
///     T-States: 11
pub fn add_hl_de(cpu: &mut CPU) { bas_add_hl_Q(cpu); }

pub fn add_hl_de_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,DE")); }

/// 0x1A   "ld a,(de)"
///     The contents of the memory location specified by DE are loaded to A.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0x1A
///     =================================
///     T-States: 7
pub fn ld_aOdeO(cpu: &mut CPU) {
    let direccion = cpu.lee_de();
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_aOdeO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A(DE)")); }

/// 0x1B   "dec de"
///     The contents of register pair DE are decremented.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 1 | 0x1B
///     =================================
///     T-States: 6
pub fn dec_de(cpu: &mut CPU) { bas_dec_Q(cpu); }

pub fn dec_de_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC DE")); }

/// 0x1C   "inc e"
///     Register E is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 1 | 0 | 0 | 0x1C
///     =================================
///     T-States: 4
pub fn inc_e(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC E"));
}

/// 0x1D   "dec e"
///     Register E is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 1 | 0 | 1 | 0x1D
///     =================================
///     T-States: 4
pub fn dec_e(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_e_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC E")); }

/// 0x1E   "ld e,N"
///     The 8-bit integer N is loaded to E.
///     =================================
///     | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0x1E
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_e_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
}

pub fn ld_e_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD E,#{:02X}", cpu.r1));
}

/// 0x1F   "rra"
///     The contents of A are rotated right 1 bit position through the
///     Carry flag. The previous contents of the Carry flag are copied
///     to bit 7.
///     S, Z, P/V are not affected.
///     H, N are reset.
///     C is data from bit 0 of A.
///     =================================
///     | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0x1F
///     =================================
///     T-States: 4
pub fn rra(cpu: &mut CPU) {
    if cpu.get_c_flag() {
        cpu.set_flag(FLAG_C, cpu.a & 0b0000_0001 != 0);
        cpu.a = (cpu.a >> 1) | 0b1000_0000;
    } else {
        cpu.set_flag(FLAG_C, cpu.a & 0b0000_0001 != 0);
        cpu.a = (cpu.a >> 1) & 0b0111_1111;
    }

    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_N, false);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn rra_txt(cpu: &mut CPU) { cpu.texto(&format!("RRA")); }

// *************************** 2 ***********************************
/// 0x20 NN   "JR NZ,E"
///     This instruction provides for conditional branching to
///     other segments of a program depending on the results of a test
///     (Z flag is not set). If the test evaluates to *true*, the value of displacement
///     E is added to PC and the next instruction is fetched from the
///     location designated by the new contents of the PC. The jump is
///     measured from the address of the instruction op code and contains
///     a range of –126 to +129 bytes. The assembler automatically adjusts
///     for the twice incremented PC.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0x20
///     =================================
///     |             E-2               |
///     =================================
///     T-States: Condition is met: 12
///     Condition is not met: 7
fn jr_nz_n(cpu: &mut CPU) {
    let salto = suma_compl2_a_un_u16(cpu.pc + 2, cpu.r1);
    if !cpu.get_z_flag() {
        cpu.pc = salto;

        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

fn jr_nz_n_txt(cpu: &mut CPU) {
    let salto = suma_compl2_a_un_u16(cpu.pc + 2, cpu.r1);
    cpu.texto(&format!("JR NZ #{:04X}", salto));
}

/// 0x21   "ld hl,NN"
///     The 16-bit integer value is loaded to the HL register pair.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0x21
///     =================================
///     |             N Low             |
///     =================================
///     |             N High            |
///     =================================
///     T-States: 10
pub fn ld_hl_nn(cpu: &mut CPU) { bas_ld_Q_A(cpu); }

pub fn ld_hl_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL#{:04X}", cpu.r1r2)); }


// 0x22 Difiere según procesador (LR35902->LDI  (HL),A)
pub fn ldiOhlO_aGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    cpu.escribe_hl(hl.wrapping_add(1));

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldiOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDI(HL),A")); }

/// 0x22 Difiere según procesador (Z80->LD  (nn),HL)
///     "ld (NN),hl"
///     The contents of the low-order portion of HL (L) are loaded to memory
///     address (NN), and the contents of the high-order portion of HL (H)
///     are loaded to the next highest memory address(NN + 1).
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0x22
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 16
pub fn ldOnnO_hl(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.l);
    cpu.mem.escribe_byte_en_mem(cpu.r1r2 + 1, cpu.h);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),HL", cpu.r1r2));
}

/// 0x23   "inc hl"
///     The contents of register pair HL are incremented.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 1 | 0x23
///     =================================
///     T-States: 6
pub fn inc_hl(cpu: &mut CPU) { bas_inc_Q(cpu); }

pub fn inc_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("INC HL")); }

/// 0x24   "inc h"
///     Register H is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0x24
///     =================================
///     T-States: 4
pub fn inc_h(cpu: &mut CPU) { bas_inc_R(cpu); }

pub fn inc_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC H"));
}


/// 0x25   "dec h"
///     Register H is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 1 | 0x25
///     =================================
///     T-States: 4
pub fn dec_h(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC H"));
}


/// 0x26   "ld h,N"
///     The 8-bit integer N is loaded to H.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 0x26
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_h_n(cpu: &mut CPU) { bas_ld_R_N(cpu); }

pub fn ld_h_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,#{:02X}", cpu.r1));
}

/// 0x27   "daa"
///     This instruction conditionally adjusts A for BCD addition
///     and subtraction operations. For addition(ADD, ADC, INC) or
///     subtraction(SUB, SBC, DEC, NEG), the following table indicates
///     the operation being performed:
///     ====================================================
///     |Oper.|C before|Upper|H before|Lower|Number|C after|
///     |     |DAA     |Digit|Daa     |Digit|Added |Daa    |
///     ====================================================
///     | ADD |   0    | 9-0 |   0    | 0-9 |  00  |   0   |
///     |     |   0    | 0-8 |   0    | A-F |  06  |   0   |
///     |     |   0    | 0-9 |   1    | 0-3 |  06  |   0   |
///     |     |   0    | A-F |   0    | 0-9 |  60  |   1   |
///     ----------------------------------------------------
///     | ADC |   0    | 9-F |   0    | A-F |  66  |   1   |
///     ----------------------------------------------------
///     | INC |   0    | A-F |   1    | 0-3 |  66  |   1   |
///     |     |   1    | 0-2 |   0    | 0-9 |  60  |   1   |
///     |     |   1    | 0-2 |   0    | A-F |  66  |   1   |
///     |     |   1    | 0-3 |   1    | 0-3 |  66  |   1   |
///     ----------------------------------------------------
///     | SUB |   0    | 0-9 |   0    | 0-9 |  00  |   0   |
///     ----------------------------------------------------
///     | SBC |   0    | 0-8 |   1    | 6-F |  FA  |   0   |
///     ----------------------------------------------------
///     | DEC |   1    | 7-F |   0    | 0-9 |  A0  |   1   |
///     ----------------------------------------------------
///     | NEG |   1    | 6-7 |   1    | 6-F |  9A  |   1   |
///     ====================================================
///     S is set if most-significant bit of the A is 1 after an
///     operation; otherwise, it is reset.
///     Z is set if A is 0 after an operation; otherwise, it is reset.
///     H: see the DAA instruction table.
///     P/V is set if A is at even parity after an operation;
///     otherwise, it is reset.
///     N is not affected.
///     C: see the DAA instruction table.
///     =================================
///     | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 0x27
///     =================================
///     T-States: 4
pub fn daa(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn daa_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x28   "JR Z,E"
///     This instruction provides for conditional branching to
///     other segments of a program depending on the results of a test
///     (Z flag is set). If the test evaluates to *true*, the value of displacement
///     E is added to PC and the next instruction is fetched from the
///     location designated by the new contents of the PC. The jump is
///     measured from the address of the instruction op code and contains
///     a range of –126 to +129 bytes. The assembler automatically adjusts
///     for the twice incremented PC.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0x28
///     =================================
///     |             E-2               |
///     =================================
///     T-States: Condition is met: 12
///     Condition is not met: 7
pub fn jr_z_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());
    if cpu.get_z_flag() {
        cpu.pc = suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn jr_z_n_txt(cpu: &mut CPU) {
    let salto = suma_compl2_a_un_u16(cpu.pc.wrapping_add(2), cpu.r1);
    cpu.texto(&format!("JR Z(#{:04X})", salto));
}

/// 0x29   "add hl,hl"
///     The contents of HL are added to the contents of HL and
///     the result is stored in HL.
///     S, Z, P/V are not affected.
///     H is set if carry from bit 11; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 15; otherwise, it is reset.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0x29
///     =================================
///     T-States: 11
pub fn add_hl_hl(cpu: &mut CPU) { bas_add_hl_Q(cpu); }

pub fn add_hl_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,HL")); }

// 0x2A Difiere según procesador (LR35902->LDI A,(HL))
pub fn ldi_aOhlOGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.a = cpu.mem.lee_byte_de_mem(hl);

    cpu.escribe_hl(hl.wrapping_add(1));

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldi_aOhlOGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDI A(HL)")); }

/// 0x2A NN NN    Difiere según procesador (Z80->LD HL,(NN)
///     "ld hl,(NN)"
///     The contents of memory address (NN) are loaded to the
///     low-order portion of HL (L), and the contents of the next
///     highest memory address (NN + 1) are loaded to the high-order
///     portion of HL (H).
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0x2A
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 16
pub fn ld_hlOnnO(cpu: &mut CPU) {
    cpu.l = cpu.mem.lee_byte_de_mem(cpu.r1r2);
    cpu.h = cpu.mem.lee_byte_de_mem(cpu.r1r2 + 1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_hlOnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL(#{:04X})", cpu.r1r2)); }

/// 0x2B   "dec hl"
///     The contents of register pair HL are decremented.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0x2B
///     =================================
///     T-States: 6
pub fn dec_hl(cpu: &mut CPU) { bas_dec_Q(cpu); }

pub fn dec_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC HL")); }

/// 0x2C   "inc l"
///     Register L is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 0 | 0x2C
///     =================================
///     T-States: 4
pub fn inc_l(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_l_txt(cpu: &mut CPU) { cpu.texto(&format!("INC L")); }

/// 0x2D   "dec l"
///     Register L is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 2 | 0 | 1 | 1 | 0 | 1 | 0x2D
///     =================================
///     T-States: 4
pub fn dec_l(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_l_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC L")); }

/// 0x2E   "ld l,N"
///     The 8-bit integer N is loaded to H.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0 | 0x2E
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_l_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
}

pub fn ld_l_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L#{:02X}", cpu.r1)); }

/// 0x2F   "cpl"
///     The contents of A are inverted (one's complement).
///     S, Z, P/V, C are not affected.
///     H and N are set.
///     =================================
///     | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 1 | 0x2F
///     =================================
///     T-States: 4
pub fn cpl(cpu: &mut CPU) {
    cpu.a = cpu.a ^ 0xFF;
}

pub fn cpl_txt(cpu: &mut CPU) { cpu.texto(&format!("CPL")); }

// *************************** 3 ***********************************

/// 0x30   "JR NC,E"
///     This instruction provides for conditional branching to
///     other segments of a program depending on the results of a test
///     (C flag is not set). If the test evaluates to *true*, the value of displacement
///     E is added to PC and the next instruction is fetched from the
///     location designated by the new contents of the PC. The jump is
///     measured from the address of the instruction op code and contains
///     a range of –126 to +129 bytes. The assembler automatically adjusts
///     for the twice incremented PC.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0x30
///     =================================
///     |             E-2               |
///     =================================
///     T-States: Condition is met: 12
///     Condition is not met: 7
pub fn jr_nc_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());
    if !cpu.get_c_flag() {
        cpu.pc = suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn jr_nc_n_txt(cpu: &mut CPU) {
    let salto = suma_compl2_a_un_u16(cpu.pc.wrapping_add(2), cpu.r1);
    cpu.texto(&format!("JR NC #{:04X}", salto));
}

/// 0x31   "ld sp,NN"
///     The 16-bit integer value is loaded to the SP register pair.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 0x31
///     =================================
///     |             N Low             |
///     =================================
///     |             N High            |
///     =================================
///     T-States: 10
pub fn ld_sp_nn(cpu: &mut CPU) { bas_ld_Q_A(cpu); }

pub fn ld_sp_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP#{:04X}", cpu.r1r2));
}

/// 0x32 Difiere según procesador (LR35902->LDD  (HL),A)
pub fn lddOhlO_aGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);
    hl = hl.wrapping_sub(1);

    cpu.escribe_hl(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn lddOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDD (HL),A")); }

/// 0x32 Difiere según procesador (Z80->LD  (nn),A)
///     "ld (NN),a"
///     The contents of A are loaded to the memory address specified by
///     the operand NN
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 0x32
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 13
pub fn ldOnnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}

/// 0x33   "inc sp"
///     The contents of register pair SP are incremented.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0x33
///     =================================
///     T-States: 6
pub fn inc_sp(cpu: &mut CPU) { bas_inc_Q(cpu); }

pub fn inc_sp_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC SP"));
}

/// 0x34   "inc (hl)"
///     The byte contained in the address specified by the contents HL
///     is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if (HL) was 0x7F before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 | 0x34
///     =================================
///     T-States: 11
pub fn inc_OhlO(cpu: &mut CPU) { bas_inc_R(cpu); }

pub fn inc_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC(HL)"));
}

/// 0x35   "dec (hl)"
///     The byte contained in the address specified by the contents HL
///     is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if (HL) was 0x80 before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 | 0x35
///     =================================
///     T-States: 11
pub fn dec_OhlO(cpu: &mut CPU) { bas_dec_R(cpu); }

pub fn dec_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC(HL)"));
}

/// 0x36   "ld (hl),N"
///     The N 8-bit value is loaded to the memory address specified by HL.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 1 | 1 | 0 | 0x36
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 10
pub fn ld_OhlO_n(cpu: &mut CPU) {
    let hl = cpu.lee_hl();

    cpu.mem.escribe_byte_en_mem(hl, cpu.r1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_OhlO_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL)#{:02X}", cpu.r1)); }

/// 0x37   "scf"
///     The Carry flag in F is set.
///     Other flags are not affected.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 1 | 1 | 1 | 0x37
///     =================================
///     T-States: 4
pub fn scf(cpu: &mut CPU) {
    cpu.set_flag(FLAG_C, true);
    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_N, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn scf_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SCF"));
}


/// 0x38  "JR C,E"
///     This instruction provides for conditional branching to
///     other segments of a program depending on the results of a test
///     (C flag is set). If the test evaluates to *true*, the value of displacement
///     E is added to PC and the next instruction is fetched from the
///     location designated by the new contents of the PC. The jump is
///     measured from the address of the instruction op code and contains
///     a range of –126 to +129 bytes. The assembler automatically adjusts
///     for the twice incremented PC.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 0x38
///     =================================
///     |             E-2               |
///     =================================
///     T-States: Condition is met: 12
///     Condition is not met: 7
pub fn jr_c_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());

    if cpu.get_c_flag() {
        cpu.pc = suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn jr_c_n_txt(cpu: &mut CPU) {
    let mut salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());

    salto = suma_compl2_a_un_u16(salto, cpu.r1);
    cpu.texto(&format!("JR C #{:04X}", salto));
}


/// 0x39   "add hl,sp"
///     The contents of SP are added to the contents of HL and
///     the result is stored in HL.
///     S, Z, P/V are not affected.
///     H is set if carry from bit 11; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 15; otherwise, it is reset.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 0 | 0 | 1 | 0x39
///     =================================
///     T-States: 11
pub fn add_hl_sp(cpu: &mut CPU) { bas_add_hl_Q(cpu); }

pub fn add_hl_sp_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,SP")); }

// 0x3A Difiere según procesador (LR35902->LDD A(HL))
pub fn ldd_a_OhlOGB(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn ldd_a_OhlOGB_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x3A Difiere según procesador (Z80->LD a(NN))
///     "ld a,(NN)"
///     The contents of the memory location specified by the operands
///     NN are loaded to A.
///     =================================
///     | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0 | 0x3A
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 13
pub fn ld_a_OnnO(cpu: &mut CPU) {
    cpu.a = cpu.mem.lee_byte_de_mem(cpu.r1r2);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_OnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A(#{:04X})", cpu.r1r2)); }

/// 0x3B   "dec sp"
///     The contents of register pair SP are decremented.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0x3B
///     =================================
///     T-States: 6
pub fn dec_sp(cpu: &mut CPU) { bas_dec_Q(cpu); }

pub fn dec_sp_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC SP")); }

/// 0x3C   "inc a"
///     Register A is incremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if r was 7Fh before operation; otherwise, it is reset.
///     N is reset.
///     C is not affected.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 0x3C
///     =================================
///     T-States: 4
pub fn inc_a(cpu: &mut CPU) { bas_inc_R(cpu); }

pub fn inc_a_txt(cpu: &mut CPU) { cpu.texto(&format!("INC A")); }

/// 0x3D   "dec a"
///     Register A is decremented.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4, otherwise, it is reset.
///     P/V is set if m was 80h before operation; otherwise, it is reset.
///     N is set.
///     C is not affected.
///     =================================
///     | 0 | 0 | 2 | 0 | 1 | 1 | 0 | 1 | 0x3D
///     =================================
///     T-States: 4
pub fn dec_a(cpu: &mut CPU) { bas_dec_R(cpu); }

pub fn dec_a_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC A")); }

/// 0x3E   "ld a,N"
///     The 8-bit integer N is loaded to A.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 1 | 1 | 0 | 0x3E
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 7
pub fn ld_a_n(cpu: &mut CPU) { bas_ld_R_N(cpu); }

pub fn ld_a_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,#{:02X}", cpu.r1)); }

/// 0x3F   "ccf"
///     The Carry flag in F is inverted.
///     Other flags are not affected.
///     =================================
///     | 0 | 0 | 1 | 1 | 1 | 1 | 1 | 1 | 0x3f
///     =================================
///     T-States: 4 (4)
pub fn ccf(cpu: &mut CPU) {
    cpu.set_flag(FLAG_H, cpu.get_c_flag());
    cpu.set_flag(FLAG_C, !cpu.get_c_flag());
    cpu.set_flag(FLAG_N, false);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ccf_txt(cpu: &mut CPU) { cpu.texto(&format!("CCF")); }

// *************************** 4 ***********************************
// 0x40
pub fn ld_b_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,B")); }

/// 0x41   "ld b,c"
///     The contents of C are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0x41
///     =================================
///     T-States: 4
pub fn ld_b_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,C")); }

/// 0x42   "ld b,d"
///     The contents of D are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0x42
///     =================================
///     T-States: 4
pub fn ld_b_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,D")); }

/// 0x43   "ld b,e"
///     The contents of E are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 1 | 0x43
///     =================================
///     T-States: 4
pub fn ld_b_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,E")); }

/// 0x44   "ld b,h"
///     The contents of H are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0x44
///     =================================
///     T-States: 4
pub fn ld_b_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,H")); }

/// 0x45   "ld b,l"
///     The contents of L are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 1 | 0x45
///     =================================
///     T-States: 4
pub fn ld_b_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,L")); }

/// 0x46   "ld b,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 1 | 1 | 0 | 0x46
///     =================================
///     T-States: 7
pub fn ld_b_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,(HL)")); }


/// 0x47   "ld b,a"
///     The contents of A are loaded to B.
///     =================================
///     | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0x47
///     =================================
///     T-States: 4
pub fn ld_b_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_b_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,A")); }

/// 0x48   "ld c,b"
///     The contents of B are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0x48
///     =================================
///     T-States: 4
pub fn ld_c_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,B")); }

/// 0x49   "ld c,c"
pub fn ld_c_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,C")); }

/// 0x4A   "ld c,d"
///     The contents of D are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0x4A
///     =================================
///     T-States: 4
pub fn ld_c_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,D")); }

/// 0x4B   "ld c,e"
///     The contents of E are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0x4B
///     =================================
///     T-States: 4
pub fn ld_c_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,E")); }

/// 0x4C   "ld c,h"
///     The contents of H are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0x4C
///     =================================
///     T-States: 4
pub fn ld_c_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,H")); }

/// 0x4D   "ld c,l"
///     The contents of L are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0x4D
///     =================================
///     T-States: 4
pub fn ld_c_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,E")); }

/// 0x4E   "ld c,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 0 | 0x4E
///     =================================
///     T-States: 7
pub fn ld_c_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,(HL)")); }

/// 0x4F   "ld c,a"
///     The contents of A are loaded to C.
///     =================================
///     | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0x4F
///     =================================
///     T-States: 4
pub fn ld_c_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_c_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD C,A")); }

// *************************** 5 ***********************************
/// 0x50   "ld d,b"
///     The contents of B are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0x50
///     =================================
///     T-States: 4
pub fn ld_d_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,B")); }

/// 0x51   "ld d,c"
///     The contents of C are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0x51
///     =================================
///     T-States: 4
pub fn ld_d_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,C")); }

// 0x52"ld d,d"
pub fn ld_d_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,D")); }

/// 0x53   "ld d,e"
///     The contents of E are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 0x53
///     =================================
///     T-States: 4
pub fn ld_d_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,E")); }

/// 0x54   "ld d,h"
///     The contents of H are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0x54
///     =================================
///     T-States: 4
pub fn ld_d_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,H")); }

/// 0x55   "ld d,l"
///     The contents of L are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0x55
///     =================================
///     T-States: 4
pub fn ld_d_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,L")); }

/// 0x56   "ld d,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 0x56
///     =================================
///     T-States: 7
pub fn ld_d_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D(HL)")); }

/// 0x57   "ld d,a"
///     The contents of A are loaded to D.
///     =================================
///     | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 1 | 0x57
///     =================================
///     T-States: 4
pub fn ld_d_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_d_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,A")); }

/// 0x58   "ld e,b"
///     The contents of B are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 0x58
///     =================================
///     T-States: 4
pub fn ld_e_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,B")); }

/// 0x59   "ld e,c"
///     The contents of C are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 0x59
///     =================================
///     T-States: 4
pub fn ld_e_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,C")); }

/// 0x5A   "ld e,d"
///     The contents of D are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 0 | 1 | 0 | 0x5A
///     =================================
///     T-States: 4
pub fn ld_e_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,D")); }

// 0x5B   "ld e,e"
pub fn ld_e_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,E")); }

/// 0x5C   "ld e,h"
///     The contents of H are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 0x5C
///     =================================
///     T-States: 4
pub fn ld_e_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,H")); }

/// 0x5D   "ld e,l"
///     The contents of L are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 0 | 1 | 1 | 0x5D
///     =================================
///     T-States: 4
pub fn ld_e_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,L")); }

/// 0x5E   "ld e,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 1 | 1 | 0 | 0x5E
///     =================================
///     T-States: 7
pub fn ld_e_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,(HL)")); }

/// 0x5F   "ld e,a"
///     The contents of A are loaded to E.
///     =================================
///     | 0 | 1 | 0 | 1 | 1 | 1 | 1 | 1 | 0x5F
///     =================================
///     T-States: 4
pub fn ld_e_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_e_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,A")); }

// *************************** 6 ***********************************
/// 0x60   "ld h,b"
///     The contents of B are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0x60
///     =================================
///     T-States: 4
pub fn ld_h_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,B")); }

/// 0x61   "ld h,c"
///     The contents of C are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0x61
///     =================================
///     T-States: 4
pub fn ld_h_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,C")); }

/// 0x62   "ld h,d"
///     The contents of D are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 0x62
///     =================================
///     T-States: 4
pub fn ld_h_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,D")); }

/// 0x63   "ld h,e"
///     The contents of E are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 1 | 0x63
///     =================================
///     T-States: 4
pub fn ld_h_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,E")); }


/// 0x64"ld h,h"
pub fn ld_h_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,H")); }

/// 0x65   "ld h,l"
///     The contents of L are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0x65
///     =================================
///     T-States: 4
pub fn ld_h_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,L")); }

/// 0x66   "ld h,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 0x66
///     =================================
///     T-States: 7
pub fn ld_h_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,(HL)")); }

/// 0x67   "ld h,a"
///     The contents of A are loaded to H.
///     =================================
///     | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0x67
///     =================================
///     T-States: 4
pub fn ld_h_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_h_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD H,A")); }

/// 0x68   "ld l,b"
///     The contents of B are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0x68
///     =================================
///     T-States: 4
pub fn ld_l_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,B")); }

/// 0x69   "ld l,c"
///     The contents of C are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 0x69
///     =================================
///     T-States: 4
pub fn ld_l_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,C")); }

/// 0x6A   "ld l,d"
///     The contents of D are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 0x6A
///     =================================
///     T-States: 4
pub fn ld_l_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,D")); }

/// 0x6B   "ld l,e"
///     The contents of E are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 0 | 1 | 1 | 0x6B
///     =================================
///     T-States: 4
pub fn ld_l_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,E")); }

/// 0x6C   "ld l,h"
///     The contents of H are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 1 | 0 | 0 | 0x6C
///     =================================
///     T-States: 4
pub fn ld_l_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,H")); }

/// 0x6D   "ld l,l"
pub fn ld_l_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,L")); }

/// 0x6E   "ld l,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 0x6E
///     =================================
///     T-States: 7
pub fn ld_l_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,(HL)")); }

/// 0x6F   "ld l,a"
///     The contents of A are loaded to L.
///     =================================
///     | 0 | 1 | 1 | 0 | 1 | 1 | 1 | 1 | 0x6F
///     =================================
///     T-States: 4
pub fn ld_l_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_l_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L,A")); }

// *************************** 7 ***********************************
/// 0x70   "ld (hl),b"
///     The contents of B are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0x70
///     =================================
///     T-States: 4, 3 (7)
pub fn ldOhlO_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),B")); }

/// 0x71   "ld (hl),c"
///     The contents of C are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 0x71
///     =================================
///     T-States: 7
pub fn ldOhlO_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),C")); }

/// 0x72   "ld (hl),d"
///     The contents of D are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0x72
///     =================================
///     T-States: 7
pub fn ldOhlO_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),D")); }

/// 0x73   "ld (hl),e"
///     The contents of E are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0x73
///     =================================
///     T-States: 7
pub fn ldOhlO_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),E")); }

/// 0x74   "ld (hl),h" operation
///     The contents of H are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 0x74
///     =================================
///     T-States: 7
pub fn ldOhlO_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),H")); }

/// 0x75   "ld (hl),l"
///     The contents of L are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 1 | 0x75
///     =================================
///     T-States: 7
pub fn ldOhlO_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),L")); }

/// 0x76   "halt"
///     The HALT instruction suspends CPU operation until a subsequent
///     interrupt or reset is received.While in the HALT state,
///     the processor executes NOPs to maintain memory refresh logic.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 | 0x76
///     =================================
///     T-States: 4

/// 0x77   "ld (hl),a"
///     The contents of A are loaded to the memory location specified
///     by the contents of HL.
///     =================================
///     | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0x77
///     =================================
///     T-States: 7
pub fn ldOhlO_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ldOhlO_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),A")); }

/// 0x78   "ld a,b"
///     The contents of B are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0x78
///     =================================
///     T-States: 4
pub fn ld_a_b(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,B")); }

/// 0x79   "ld a,c"
///     The contents of C are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 0x79
///     =================================
///     T-States: 4
pub fn ld_a_c(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,C")); }

/// 0x7A   "ld a,d"
///     The contents of D are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0x7A
///     =================================
///     T-States: 4
pub fn ld_a_d(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,D")); }

/// 0x7B   "ld a,e"
///     The contents of E are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 0x7B
///     =================================
///     T-States: 4
pub fn ld_a_e(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,E")); }

/// 0x7C   "ld a,h"
///     The contents of H are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 0x7C
///     =================================
///     T-States: 4
pub fn ld_a_h(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,H")); }

/// 0x7D   "ld a,l"
///     The contents of L are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 1 | 0 | 1 | 0x7D
///     =================================
///     T-States: 4
pub fn ld_a_l(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,L")); }

/// 0x7E   "ld a,(hl)"
///     The 8-bit contents of memory location (HL) are loaded to A.
///     =================================
///     | 0 | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 0x7E
///     =================================
///     T-States: 7
pub fn ld_a_OhlO(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,(HL)")); }

/// 0x7F   "ld a,a"
pub fn ld_a_a(cpu: &mut CPU) { bas_ld_R1_R2(cpu); }

pub fn ld_a_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,A")); }

// *************************** 8 ***********************************
/// 0x80   "add a,b"
///     The contents of B are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0x80
///     =================================
///     T-States: 4
pub fn add_a_b(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_b_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,B")); }

/// 0x81   "add a,c"
///     The contents of C are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0x81
///     =================================
///     T-States: 4
pub fn add_a_c(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_c_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,C")); }

/// 0x82   "add a,d"
///     The contents of D are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0x82
///     =================================
///     T-States: 4
pub fn add_a_d(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_d_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,D")); }

/// 0x83   "add a,e"
///     The contents of E are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0x83
///     =================================
///     T-States: 4
pub fn add_a_e(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_e_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,E")); }

/// 0x84   "add a,h"
///     The contents of H are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0x84
///     =================================
///     T-States: 4
pub fn add_a_h(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_h_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,H")); }

/// 0x85   "add a,l"
///     The contents of L are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0x85
///     =================================
///     T-States: 4
pub fn add_a_l(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_l_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,L")); }

/// 0x86   "add a,(hl)"
///     The byte at the memory address specified by the contents of HL
///     is added to the contents of A, and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 0x86
///     =================================
///     T-States: 4, 3 (7)
pub fn add_a_OhlO(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,(HL)")); }

/// 0x87   "add a,a"
///     The contents of B are added to the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 0 | 1 | 1 | 1 | 0x87
///     =================================
///     T-States: 4
pub fn add_a_a(cpu: &mut CPU) { bas_add_a_R(cpu); }

pub fn add_a_a_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,A")); }

/// 0x88   "adc a,b"
///     The contents of B and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0x88
///     =================================
///     T-States: 4
pub fn adc_a_b(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_b_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x89   "adc a,c"
///     The contents of C and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 1 | 0x89
///     =================================
///     T-States: 4
pub fn adc_a_c(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_c_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8A   "adc a,d"
///     The contents of D and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 0x8A
///     =================================
///     T-States: 4
pub fn adc_a_d(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_d_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8Badc   "a,e"
///     The contents of E and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 0 | 1 | 1 | 0x8B
///     =================================
///     T-States: 4
pub fn adc_a_e(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_e_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8C   "adc a,h"
///     The contents of H and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 1 | 0 | 0 | 0x8C
///     =================================
///     T-States: 4
pub fn adc_a_h(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_h_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8D   "adc a,l"
///     The contents of L and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0x8D
///     =================================
///     T-States: 4
pub fn adc_a_l(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_l_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8E   "adc a,(hl)"
///     The byte at the memory address specified by the contents of HL
///     and the C flag is added to the contents of A, and the
///     result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 1 | 1 | 0 | 0x8E
///     =================================
///     T-States: 7
pub fn adc_a_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_OhlO_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0x8F   "adc a,a"
///     The contents of A and the C flag are added to the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if carry from bit 3; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is reset.
///     C is set if carry from bit 7; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 0 | 1 | 1 | 1 | 1 | 0x8F
///     =================================
///     T-States: 4
pub fn adc_a_a(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn adc_a_a_txt(cpu: &mut CPU) { fn_no_impl(cpu); }


// *************************** 9 ***********************************
/// 0x90   "sub b"
///     The contents of B are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0x90
///     =================================
///     T-States: 4
pub fn sub_b(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_b_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB B")); }

/// 0x91   "sub c"
///     The contents of C are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0x91
///     =================================
///     T-States: 4
pub fn sub_c(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_c_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB C")); }

/// 0x92   "sub d"
///     The contents of D are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 0x92
///     =================================
///     T-States: 4
pub fn sub_d(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_d_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB D")); }

/// 0x93   "sub e"
///     The contents of E are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 0 | 1 | 1 | 0x93
///     =================================
///     T-States: 4
pub fn sub_e(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_e_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB E")); }

/// 0x94   "sub h"
///     The contents of H are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0x94
///     =================================
///     T-States: 4
pub fn sub_h(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_h_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB H")); }

/// 0x95   "sub l"
///     The contents of L are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0x95
///     =================================
///     T-States: 4
pub fn sub_l(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_l_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB L")); }

/// 0x96   "sub (hl)"
///     The byte at the memory address specified by the contents of HL
///     is subtracted from the contents of A, and the
///     result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 0 | 0x96
///     =================================
///     T-States: 7
pub fn subOhlO(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn subOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(HL)")); }

/// 0x97   "sub a"
///     The contents of A are subtracted from the contents of A, and the result is
///     stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 0 | 1 | 1 | 1 | 0x97
///     =================================
///     T-States: 4
pub fn sub_a(cpu: &mut CPU) { bas_sub_R(cpu); }

pub fn sub_a_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB A")); }

/// 0x98   "sbc b"
///     The contents of B and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 0x98
///     =================================
///     T-States: 4

/// 0x99   "sbc c"
///     The contents of C and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 1 | 0x99
///     =================================
///     T-States: 4

/// 0x9A   "sbc d"
///     The contents of D and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0x9A
///     =================================
///     T-States: 4

/// 0x9B   "sbc e"
///     The contents of E and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 1 | 0x9B
///     =================================
///     T-States: 4

/// 0x9C   "sbc h"
///     The contents of H and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 1 | 0 | 0 | 0x9C
///     =================================
///     T-States: 4

/// 0x9D   "sbc l"
///     The contents of L and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 1 | 0 | 1 | 0x9D
///     =================================
///     T-States: 4

/// 0x9E   "sbc (hl)"
///     The byte at the memory address specified by the contents of HL
///     and the C flag is subtracted from the contents of A, and the
///     result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0 | 0x9E
///     =================================
///     T-States: 7

/// 0x9F   "sbc a"
///     The contents of A and the C flag are subtracted from the contents of A,
///     and the result is stored in A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 1 | 0x9F
///     =================================
///     T-States: 4
pub fn sbc_a_a(cpu: &mut CPU) { bas_sbc_a_R(cpu); }

pub fn sbc_a_a_txt(cpu: &mut CPU) { cpu.texto(&format!("SBC A,A")); }

// *************************** A ***********************************
/// 0xA0   "and b"
///     A logical AND operation is performed between B and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0xA0
///     =================================
///     T-States: 4
pub fn and_b(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_b_txt(cpu: &mut CPU) { cpu.texto(&format!("AND B")); }

/// 0xA1   "and c"
///     A logical AND operation is performed between C and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 1 | 0xA1
///     =================================
///     T-States: 4
pub fn and_c(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_c_txt(cpu: &mut CPU) { cpu.texto(&format!("AND C")); }

/// 0xA2   "and d"
///     A logical AND operation is performed between D and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0xA2
///     =================================
///     T-States: 4
pub fn and_d(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_d_txt(cpu: &mut CPU) { cpu.texto(&format!("AND D")); }

/// 0xA3   "and e"
///     A logical AND operation is performed between E and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 1 | 0xA3
///     =================================
///     T-States: 4
pub fn and_e(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_e_txt(cpu: &mut CPU) { cpu.texto(&format!("AND E")); }

/// 0xA4   "and h"
///     A logical AND operation is performed between H and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 0xA4
///     =================================
///     T-States: 4
pub fn and_h(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_h_txt(cpu: &mut CPU) { cpu.texto(&format!("AND H")); }

/// 0xA5   "and l"
///     A logical AND operation is performed between L and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 1 | 0xA5
///     =================================
///     T-States: 4
pub fn and_l(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_l_txt(cpu: &mut CPU) { cpu.texto(&format!("AND L")); }

/// 0xA6   "and (hl)"
///     A logical AND operation is performed between the byte at the
///     memory address specified by the contents of HL and the byte
///     contained in A; the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 0xA6
///     =================================
///     T-States: 7
pub fn and_OhlO(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("AND(HL)")); }

/// 0xA7   "and A"
///     A logical AND operation is performed between A and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 1 | 0xA7
///     =================================
///     T-States: 4
pub fn and_a(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_a_txt(cpu: &mut CPU) { cpu.texto(&format!("AND A")); }

/// 0xA8   "xor b"
///     A logical XOR operation is performed between B and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 0xA8
///     =================================
///     T-States: 4
pub fn xor_b(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_b_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR B")); }

/// 0xA9   "xor c"
///     A logical XOR operation is performed between C and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 1 | 0xA9
///     =================================
///     T-States: 4
pub fn xor_c(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_c_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR C")); }

/// 0xAA   "xor d"
///     A logical XOR operation is performed between D and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0xAA
///     =================================
///     T-States: 4
pub fn xor_d(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_d_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR D")); }

/// 0xAB   "xor e"
///     A logical XOR operation is performed between E and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0xAB
///     =================================
///     T-States: 4
pub fn xor_e(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_e_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR E")); }

/// 0xAC   "xor h"
///     A logical XOR operation is performed between H and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 0 | 0xAC
///     =================================
///     T-States: 4
pub fn xor_h(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_h_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR H")); }

/// 0xAD   "xor l"
///     A logical XOR operation is performed between L and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 1 | 0 | 1 | 0xAD
///     =================================
///     T-States: 4
pub fn xor_l(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_l_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR L")); }

/// 0xAE   "xor (hl)"
///     A logical XOR operation is performed between the byte at the
///     memory address specified by the contents of HL and the byte
///     contained in A; the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 1 | 1 | 0 | 0xAE
///     =================================
///     T-States: 7
pub fn xor_OhlO(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR(HL)")); }

/// 0xAF   "xor a"     Xor consigo mismo pone a 0 y modifica flags
///     A logical XOR operation is performed between A and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 0 | 1 | 1 | 1 | 1 | 0xAF
///     =================================
///     T-States: 4
pub fn xor_a(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_a_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR A")); }

// *************************** B ***********************************
/// 0xB0   "or b"
///     A logical OR operation is performed between B and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0xB0
///     =================================
///     T-States: 4
pub fn or_b(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_b_txt(cpu: &mut CPU) { cpu.texto(&format!("OR B")); }

/// 0xB1   "or c"
///     A logical OR operation is performed between C and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 1 | 0xB1
///     =================================
///     T-States: 4
pub fn or_c(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_c_txt(cpu: &mut CPU) { cpu.texto(&format!("OR C")); }

/// 0xB2   "or d"
///     A logical OR operation is performed between D and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 0 | 0xB2
///     =================================
///     T-States: 4
pub fn or_d(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_d_txt(cpu: &mut CPU) { cpu.texto(&format!("OR D")); }

/// 0xB3   "or e"
///     A logical OR operation is performed between E and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 1 | 0xB3
///     =================================
///     T-States: 4
pub fn or_e(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_e_txt(cpu: &mut CPU) { cpu.texto(&format!("OR E")); }

/// 0xB4   "or h"
///     A logical OR operation is performed between H and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 1 | 0 | 0 | 0xB4
///     =================================
///     T-States: 4
pub fn or_h(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_h_txt(cpu: &mut CPU) { cpu.texto(&format!("OR H")); }

/// 0xB5   "or l"
///     A logical OR operation is performed between L and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 1 | 0 | 1 | 0xB5
///     =================================
///     T-States: 4
pub fn or_l(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_l_txt(cpu: &mut CPU) { cpu.texto(&format!("OR L")); }

/// 0xB6   "or (hl)"
///     A logical OR operation is performed between the byte at the
///     memory address specified by the contents of HL and the byte
///     contained in A; the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 1 | 1 | 0 | 0xB6
///     =================================
///     T-States: 4, 3 (7)
pub fn or_OhlO(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("OR(HL)")); }

/// 0xB7   "or a"
///     A logical OR operation is performed between A and the byte contained in A;
///     the result is stored in the Accumulator.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is reset.
///     P/V is reset if overflow; otherwise, it is reset.
///     N is reset.
///     C is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 0 | 1 | 1 | 1 | 0xB7
///     =================================
///     T-States: 4
pub fn or_a(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_a_txt(cpu: &mut CPU) { cpu.texto(&format!("OR A")); }

/// 0xB8   "cp b"
///     The contents of B are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 0 | 0xB8
///     =================================
///     T-States: 4
pub fn cp_b(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_b_txt(cpu: &mut CPU) { cpu.texto(&format!("CP B")); }

/// 0xB9   "cp c"
///     The contents of C are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 1 | 0xB9
///     =================================
///     T-States: 4
pub fn cp_c(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_c_txt(cpu: &mut CPU) { cpu.texto(&format!("CP C")); }

/// 0xBA   "cp d"
///     The contents of D are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 0 | 0xBA
///     =================================
///     T-States: 4
pub fn cp_d(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_d_txt(cpu: &mut CPU) { cpu.texto(&format!("CP D")); }

/// 0xBB   "cp e"
///     The contents of E are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0xBB
///     =================================
///     T-States: 4
pub fn cp_e(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_e_txt(cpu: &mut CPU) { cpu.texto(&format!("CP E")); }

/// 0xBC   "cp h"
///     The contents of H are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 0xBC
///     =================================
///     T-States: 4
pub fn cp_h(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_h_txt(cpu: &mut CPU) { cpu.texto(&format!("CP H")); }

/// 0xBD   "cp l"
///     The contents of L are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 1 | 0 | 1 | 0xBD
///     =================================
///     T-States: 4
pub fn cp_l(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_l_txt(cpu: &mut CPU) { cpu.texto(&format!("CP L")); }

/// 0xBE   "cp (hl)"
///     The contents of the byte at the memory address specified by
///     the contents of HL are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 1 | 1 | 0 | 0xBE
///     =================================
///     T-States: 4, 3 (7)
pub fn cpOhlO(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cpOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("CP (HL)")); }

/// 0xBF   "cp a"
///     The contents of A are compared with the contents of A.
///     If there is a true compare, the Z flag is set. The execution of
///     this instruction does not affect A.
///     S is set if result is negative; otherwise, it is reset.
///     Z is set if result is 0; otherwise, it is reset.
///     H is set if borrow from bit 4; otherwise, it is reset.
///     P/V is set if overflow; otherwise, it is reset.
///     N is set.
///     C is set if borrow; otherwise, it is reset.
///     =================================
///     | 1 | 0 | 1 | 1 | 1 | 1 | 1 | 1 | 0xBF
///     =================================
///     T-States: 4
pub fn cp_a(cpu: &mut CPU) { bas_cp_R(cpu); }

pub fn cp_a_txt(cpu: &mut CPU) { cpu.texto(&format!("CP A")); }


// *************************** C ***********************************
/// 0xC0   "ret nz"
///     If Z flag is not set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0xC0
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_nz(cpu: &mut CPU) {
    if !cpu.get_z_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn ret_nz_txt(cpu: &mut CPU) { cpu.texto(&format!("RET NZ")); }

/// 0xC1   "pop bc"
///     The top two bytes of the external memory last-in, first-out (LIFO)
///     stack are popped to register pair BC. SP holds the 16-bit address
///     of the current top of the stack. This instruction first loads to
///     the low-order portion of RR, the byte at the memory location
///     corresponding to the contents of SP; then SP is incremented and
///     the contents of the corresponding adjacent memory location are
///     loaded to the high-order portion of RR and the SP is now incremented
///     again.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0xC1
///     =================================
///     T-States: 10
pub fn pop_bc(cpu: &mut CPU) {
    let bc: u16 = cpu.pop();
    cpu.escribe_bc(bc);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn pop_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("POP BC")); }

/// 0xC2   "jp nz,NN"
///     If Z flag is not set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0 | 0xC2
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn jp_nz(cpu: &mut CPU) {
    if !cpu.get_z_flag() {
        cpu.pc = cpu.r1r2;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
    }
    cpu.t += cpu.get_t_instruccion();
}

pub fn jp_nz_txt(cpu: &mut CPU) { cpu.texto(&format!("JP NZ #{:04X}", cpu.r1r2)); }

/// 0xC3   "jp NN"
///     Operand NN is loaded to PC. The next instruction is fetched
///     from the location designated by the new contents of the PC.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 1 | 0xC3
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn jp_nn(cpu: &mut CPU) {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc = cpu.r1r2;
}

pub fn jp_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("JP #{:04X}", cpu.r1r2)); }

/// 0xC4   "call nz,NN"
///     If flag Z is not set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0xC4
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn call_nz_nn(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    if !cpu.get_z_flag() {
        cpu.push(salto_al_volver);
        cpu.pc = cpu.r1r2;
        cpu.t += cpu.get_t_instruccion() + 7;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn call_nz_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("CALL Z #{:04X}", cpu.r1r2)); }

/// 0xC5   "push bc"
///     The contents of the register pair BC are pushed to the external
///     memory last-in, first-out (LIFO) stack. SP holds the 16-bit
///     address of the current top of the Stack. This instruction first
///     decrements SP and loads the high-order byte of register pair RR
///     to the memory address specified by SP. Then SP is decremented again
///     and loads the low-order byte of RR to the memory location
///     corresponding to this new address in SP.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 1 | 0xC5
///     =================================
///     T-States: 10
pub fn push_bc(cpu: &mut CPU) {
    let bc = cpu.lee_bc();
    cpu.push(bc);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH BC")); }

/// 0xC6   "add a,n" ??????????????????? TODO: se lo salta
pub fn add_a_n(cpu: &mut CPU) {
    let resultado = cpu.a.wrapping_add(cpu.r1);

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, overflow_en_suma_u8(cpu.a, cpu.r1, resultado));
    cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_add(cpu.r1 as u16)) & 0x100) != 0);
    cpu.set_flag(FLAG_H, ((cpu.a & 0x0F) + (cpu.r1 & 0x0F)) > 0x0F);
    cpu.set_flag(FLAG_N, false);

    cpu.a = resultado;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn add_a_n_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,#{:02X}", cpu.r1)); }

/// 0xC7   "rst 00h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0000H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 0 | 0 | 0 | 1 | 1 | 1 | 0xC7
///     =================================
///     T-States: 11
pub fn rst_00(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0000;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_00_txt(cpu: &mut CPU) { cpu.texto(&format!("RST #0000")); }

/// 0xC8   "ret z"
///     If Z flag is set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0xC8
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_z(cpu: &mut CPU) {
    if cpu.get_z_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn ret_z_txt(cpu: &mut CPU) { cpu.texto(&format!("RET Z")); }

/// 0xC9   "ret"
///     The byte at the memory location specified by the contents of SP
///     is moved to the low-order eight bits of PC. SP is now incremented
///     and the byte at the memory location specified by the new contents
///     of this instruction is fetched from the memory location specified
///     by PC.
///     This instruction is normally used to return to the main line
///     program at the completion of a routine entered by a CALL
///     instruction.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 1 | 0xC9
///     =================================
///     T-States: 10
pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.pop();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ret_txt(cpu: &mut CPU) { cpu.texto(&format!("RET")); }

/// 0xCA   "jp z,NN"
///     If Z flag is set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0xCA
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 4, 3, 3 (10)

/// 0xCB   -----EXTENSION--------------------------------------------------------
pub fn CB(cpu: &mut CPU) {
    /*
     self.obtiene_intruccion_y_bytes_posteriores();

        // Ejecuta instruccion
        //self.funciones[self.r0 as usize](self);
        let f: Funcion = self.funciones[self.r0 as usize];
        let ff = f.get_puntero_a_funcion();
        ff(self);
        //self.funciones[self.r0 as usize](self);
    */
    // Ejecuta instruccion
    let f: Funcion = cpu.funciones_cb[cpu.r1 as usize];
    let ff = f.get_puntero_a_funcion();
    ff(cpu);
    //cpu.funciones_cb[cpu.r1 as usize](cpu);
}

pub fn CB_txt(cpu: &mut CPU) {
    // Ejecuta instruccion
    let f: Funcion = cpu.funciones_cb[cpu.r1 as usize];
    let ff = f.get_puntero_txt_a_funcion();
    ff(cpu);

    //cpu.funciones_cb_txt[cpu.r1 as usize](cpu);
}

/// 0xCC   "call z,NN"
///     If flag Z is set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0xCC
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn call_z_nn(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    if cpu.get_z_flag() {
        cpu.push(salto_al_volver);
        cpu.pc = cpu.r1r2;
        cpu.t += cpu.get_t_instruccion() + 7;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn call_z_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("CALL Z #{:04X}", cpu.r1r2)); }

/// 0xCD   "call NN"
///     The current contents of PC are pushed onto the top of the
///     external memory stack. The operands NN are then loaded to PC to
///     point to the address in memory at which the first op code of a
///     subroutine is to be fetched. At the end of the subroutine, a RET
///     instruction can be used to return to the original program flow by
///     popping the top of the stack back to PC. The push is accomplished
///     by first decrementing the current contents of SP, loading the
///     high-order byte of the PC contents to the memory address now pointed
///     to by SP; then decrementing SP again, and loading the low-order
///     byte of the PC contents to the top of stack.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0xCD
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 17
pub fn call_nn(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = cpu.r1r2;

    cpu.t += cpu.get_t_instruccion();
}

pub fn call_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("CALL #{:04X}", cpu.r1r2)); }

/// 0xCE   "adc a,n" ???????? TODO: se lo salta

/// 0xCF   "rst 08h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0008H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 1 | 0xCF
///     =================================
///     T-States: 11
pub fn rst_08(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0008;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_08_txt(cpu: &mut CPU) { cpu.texto(&format!("RST 08")); }

// *************************** D ***********************************
/// 0xD0   "ret nc"
///     If C flag is not set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0xD0
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_nc(cpu: &mut CPU) {
    if !cpu.get_c_flag() {
        cpu.pc = cpu.pop();
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
    }

    cpu.t += cpu.get_t_instruccion();
}

pub fn ret_nc_txt(cpu: &mut CPU) { cpu.texto(&format!("RET NC")); }

/// 0xD1   "pop de"
///     The top two bytes of the external memory last-in, first-out (LIFO)
///     stack are popped to register pair DE. SP holds the 16-bit address
///     of the current top of the stack. This instruction first loads to
///     the low-order portion of RR, the byte at the memory location
///     corresponding to the contents of SP; then SP is incremented and
///     the contents of the corresponding adjacent memory location are
///     loaded to the high-order portion of RR and the SP is now incremented
///     again.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0xD1
///     =================================
///     T-States: 10
pub fn pop_de(cpu: &mut CPU) {
    let de = desconcatena_un_u16_en_dos_u8(cpu.pop());
    cpu.d = de.0;
    cpu.e = de.1;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn pop_de_txt(cpu: &mut CPU) { cpu.texto(&format!("POP DE")); }

/// 0xD2   "jp nc,NN"
///     If C flag is not set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0xD2
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn jp_nc_nn(cpu: &mut CPU) {
    if !cpu.get_c_flag() {
        cpu.pc = cpu.r1r2;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
    }

    cpu.t += cpu.get_t_instruccion();
}

pub fn jp_nc_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("JP NC#{:04X}", cpu.r1r2)); }

/// 0xD3   "out (N),a"
///     The operand N is placed on the bottom half (A0 through A7) of
///     the address bus to select the I/O device at one of 256 possible
///     ports. The contents of A also appear on the top half(A8 through
///     A15) of the address bus at this time. Then the byte contained
///     in A is placed on the data bus and written to the selected
///     peripheral device.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 0xD3
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 11
pub fn out_OnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_port(cpu.r1, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn out_OnO_a_txt(cpu: &mut CPU) { cpu.texto(&format!("OUT(#{:02X}),A", cpu.r1)); }

/// 0xD4"call nc,NN"
///     If flag C is not set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0xD4
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10


/// 0xD5   "push de"
///     The contents of the register pair DE are pushed to the external
///     memory last-in, first-out (LIFO) stack. SP holds the 16-bit
///     address of the current top of the Stack. This instruction first
///     decrements SP and loads the high-order byte of register pair RR
///     to the memory address specified by SP. Then SP is decremented again
///     and loads the low-order byte of RR to the memory location
///     corresponding to this new address in SP.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0xD5
///     =================================
///     T-States: 10
pub fn push_de(cpu: &mut CPU) {
    let de = cpu.lee_de();
    cpu.push(de);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_de_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH DE")); }

/// 0xD6 TODO: No esta
pub fn sub_n(cpu: &mut CPU) {
    let resultado = cpu.a.wrapping_sub(cpu.r1);

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.r1, resultado));
    cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.r1 as u16)) & 0x100) != 0);
    cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.r1));
    cpu.set_flag(FLAG_N, true);

    cpu.a = resultado;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_n_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(#{:02X})", cpu.r1)); }

/// 0xD7   "rst 10h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0010H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 1 | 1 | 1 | 0xD7
///     =================================
///     T-States: 11
pub fn rst_10(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0010;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_10_txt(cpu: &mut CPU) { cpu.texto(&format!("RST #0010")); }

/// 0xD8   "ret c"
///     If C flag is set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 0xD8
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_c(cpu: &mut CPU) {
    if cpu.get_c_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn ret_c_txt(cpu: &mut CPU) { cpu.texto(&format!("RET C")); }

/// 0xD9 Difiere según procesador (LR35902->RETI)
pub fn retiGB(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn retiGB_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0xD9 Difiere según procesador (Z80->EXX)
///   "exx"
///     Each 2-byte value in register pairs BC, DE, and HL is exchanged
///     with the 2-byte value in BC', DE', and HL', respectively.
///     =================================
///     | 1 | 1 | 0 | 1 | 1 | 0 | 0 | 1 | 0xD9
///     =================================
///     T-States: 4
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

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn exx_txt(cpu: &mut CPU) { cpu.texto(&format!("EXX")); }

/// 0xDA   "jp c,NN"
///     If C flag is not set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 0 | 0xDA
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10

/// 0xDB   "in a,(N)"
///     The operand N is placed on the bottom half (A0 through A7) of
///     the address bus to select the I/O device at one of 256 possible
///     ports. The contents of A also appear on the top half (A8 through
///     A15) of the address bus at this time. Then one byte from the
///     selected port is placed on the data bus and written to A
///     in the CPU.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 0xDB
///     =================================
///     |            8-bit              |
///     =================================
///     T-States: 11

/// 0xDC   "call c,NN"
///     If flag C is set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 0xDC
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 4, 3, 3 (10) / 4, 3, 4, 3, 3 (17)

/// 0xDD
/// 0xDE
/// 0xDF   "rst 18h"

///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0018H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 0 | 1 | 1 | 1 | 1 | 1 | 0xDF
///     =================================
///     T-States: 11

// *************************** E ***********************************
/// 0xE0 Difiere según procesador (LR35902->LD(#FF00+N),A)
pub fn ldOff00_m_nO_aGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.r1 as u16);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOff00_m_nO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD ($FF00+#{:02X}),A", cpu.r1));
}

/// 0xE0 Difiere según procesador (Z80->RET NV)
///   "ret po"
///     If PV flag is not set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0xE0
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_po(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn ret_po_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0xE1   "pop hl"
///     The top two bytes of the external memory last-in, first-out (LIFO)
///     stack are popped to register pair HL. SP holds the 16-bit address
///     of the current top of the stack. This instruction first loads to
///     the low-order portion of RR, the byte at the memory location
///     corresponding to the contents of SP; then SP is incremented and
///     the contents of the corresponding adjacent memory location are
///     loaded to the high-order portion of RR and the SP is now incremented
///     again.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 0xE1
///     =================================
///     T-States: 10
pub fn pop_hl(cpu: &mut CPU) {
    let hl = cpu.pop();
    cpu.escribe_hl(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn pop_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("POP HL")); }

/// 0xE2 Difiere según procesador (LR35902->LD(#FF00+C),A)
pub fn ldOff00_m_cO_aGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.c as u16);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOff00_m_cO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LD ($FF00+C),A")); }

/// 0xE2 Difiere según procesador (Z80->JP PO,NN)
/// "jp po,NN"
///     If PV flag is not set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 0xE2
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn jp_po_nn(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn jp_po_nn_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0xE3   "ex (sp),hl"
///     The low-order byte contained in HL is exchanged with the contents
///     of the memory address specified by the contents of SP, and the
///     high-order byte of HL is exchanged with the next highest memory
///     address (SP+1).
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 1 | 0xE3
///     =================================
///     T-States: 4, 3, 4, 3, 5 (19)
pub fn exOspO_hl(cpu: &mut CPU) {
    let l = cpu.mem.lee_byte_de_mem(cpu.sp);
    let h = cpu.mem.lee_byte_de_mem(cpu.sp + 1);
    cpu.mem.escribe_byte_en_mem(cpu.sp, cpu.l);
    cpu.mem.escribe_byte_en_mem(cpu.sp + 1, cpu.h);
    cpu.l = l;
    cpu.h = h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn exOspO_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("EX(SP),HL")); }

/// 0xE4   "call po,NN"
///     If flag PV is not set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0xE4
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10

/// 0xE5   "push hl"
///     The contents of the register pair HL are pushed to the external
///     memory last-in, first-out (LIFO) stack. SP holds the 16-bit
///     address of the current top of the Stack. This instruction first
///     decrements SP and loads the high-order byte of register pair RR
///     to the memory address specified by SP. Then SP is decremented again
///     and loads the low-order byte of RR to the memory location
///     corresponding to this new address in SP.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0xE5
///     =================================
///     T-States: 10
pub fn push_hl(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.push(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn push_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH HL")); }


// 0xE6   TODO: No sale
pub fn and_n(cpu: &mut CPU) {
    let resultado = cpu.a & cpu.r1;

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, paridad_bits_u8(resultado));// Como paridad
    cpu.set_flag(FLAG_H, true);
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_N, false);

    cpu.a = resultado;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn and_n_txt(cpu: &mut CPU) { cpu.texto(&format!("AND #{:02X}", cpu.r1)); }

/// 0xE7   "rst 20h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0020H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0xE7
///     =================================
///     T-States: 11
pub fn rst_20(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0020;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_20_txt(cpu: &mut CPU) { cpu.texto(&format!("RST #0020")); }

/// 0xE8   "ret pe"
///     If PV flag is not set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 0 | 0xE8
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5

/// 0xE9   "jp (hl)"
///     PC is loaded with the contents of HL. The next instruction is
///     fetched from the location designated by the new contents of PC.
///     =================================
///     | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 0xE9
///     =================================
///     T-States: 4
pub fn jpOhlO(cpu: &mut CPU) {
    cpu.pc = cpu.lee_hl();

    cpu.t += cpu.get_t_instruccion();
}

pub fn jpOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("JP(HL)")); }

/// 0xEA  Difiere según procesador (LR35902->LD (nn),A)
pub fn ldOnnO_aGB(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2)); }

/// 0xEA  Difiere según procesador (Z80->JP  V,nn)
///   "jp pe,NN"
///     If PV flag is set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 0 | 0xEA
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10
pub fn jp_pe_nn(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn jp_pe_nn_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

/// 0xEB   "ex de,hl"
///     The 2-byte contents of register pairs DE and HL are exchanged.
///     =================================
///     | 1 | 1 | 1 | 0 | 1 | 0 | 1 | 1 | 0xEB
///     =================================
///     T-States: 4
pub fn ex_de_hl(cpu: &mut CPU) {
    let dtemp = cpu.d;
    let etemp = cpu.e;
    cpu.d = cpu.h;
    cpu.e = cpu.l;
    cpu.h = dtemp;
    cpu.l = etemp;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ex_de_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("EX DE,HL")); }


// 0xED   -----EXTENSION--------------------------------------------------------
pub fn ED(cpu: &mut CPU) {
    /*
     self.obtiene_intruccion_y_bytes_posteriores();

        // Ejecuta instruccion
        //self.funciones[self.r0 as usize](self);
        let f: Funcion = self.funciones[self.r0 as usize];
        let ff = f.get_puntero_a_funcion();
        ff(self);
        //self.funciones[self.r0 as usize](self);
    */
    // Ejecuta instruccion
    let f: Funcion = cpu.funciones_ed[cpu.r1 as usize];

    let ff = f.get_puntero_a_funcion();
    ff(cpu);

    //cpu.funciones_ed[cpu.r1 as usize](cpu);
}

pub fn ED_txt(cpu: &mut CPU) {
    let f: Funcion = cpu.funciones_ed[cpu.r1 as usize];
    let ff = f.get_puntero_txt_a_funcion();
    ff(cpu);
// Ejecuta instruccion
//cpu.funciones_ed_txt[cpu.r1 as usize](cpu);
}

/// 0xEF   "rst 28h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0028H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 1 | 0xEF
///     =================================
///     T-States: 11

// *************************** F ***********************************
/// 0xF0 Difiere según procesador (LR35902->LDH  A,(n))
pub fn ld_a_Off00_m_nOGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.r1 as u16);
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_Off00_m_nOGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A,($FF00+#{:02X})", cpu.r1)); }

/// 0xF0 Difiere según procesador (Z80->RET P)
///     "ret p"
///     If S flag is not set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0xF0
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5
pub fn ret_p(cpu: &mut CPU) {
    if cpu.get_pv_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn ret_p_txt(cpu: &mut CPU) { cpu.texto(&format!("RET P")); }

/// 0xF1   "pop af"
///     The top two bytes of the external memory last-in, first-out (LIFO)
///     stack are popped to register pair AF. SP holds the 16-bit address
///     of the current top of the stack. This instruction first loads to
///     the low-order portion of RR, the byte at the memory location
///     corresponding to the contents of SP; then SP is incremented and
///     the contents of the corresponding adjacent memory location are
///     loaded to the high-order portion of RR and the SP is now incremented
///     again.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 1 | 0xF1
///     =================================
///     T-States: 10
pub fn pop_af(cpu: &mut CPU) {
    let af = cpu.pop();
    cpu.escribe_af(af);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn pop_af_txt(cpu: &mut CPU) { cpu.texto(&format!("POP AF")); }

/// 0xF2   "jp p,NN"
///     If S flag is not set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 0 | 0xF2
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10

/// 0xF3   "di"
///     Disables the maskable interrupt by resetting the interrupt
///     enable flip-flops (IFF1 and IFF2).
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0xF3
///     =================================
///     T-States: 4
pub fn di(cpu: &mut CPU) {
    cpu.permitida_interrupcion = false;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn di_txt(cpu: &mut CPU) { cpu.texto(&format!("DI")); }

/// 0xF4   "call p,NN"
///     If flag S is not set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 0xF4
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10

/// 0xF5   "push af"
///     The contents of the register pair BC are pushed to the external
///     memory last-in, first-out (LIFO) stack. SP holds the 16-bit
///     address of the current top of the Stack. This instruction first
///     decrements SP and loads the high-order byte of register pair RR
///     to the memory address specified by SP. Then SP is decremented again
///     and loads the low-order byte of RR to the memory location
///     corresponding to this new address in SP.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 1 | 0xF5
///     =================================
///     T-States: 10
pub fn push_af(cpu: &mut CPU) {
    let af = cpu.lee_af();
    cpu.push(af);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_af_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH AF")); }

/// 0xF6   TODO: No esta
pub fn or_n(cpu: &mut CPU) {
    let resultado = cpu.a | cpu.r1;

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, paridad_bits_u8(resultado));// Como paridad
    cpu.set_flag(FLAG_H, false);
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_N, false);

    cpu.a = resultado;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn or_n_txt(cpu: &mut CPU) { cpu.texto(&format!("OR #{:02X}", cpu.r1)); }

/// 0xF7   "rst 30h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0030H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0xF7
///     =================================
///     T-States: 11
pub fn rst_30(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0030;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_30_txt(cpu: &mut CPU) { cpu.texto(&format!("RST #0030")); }

/// 0xF8   "ret m"
///     If S flag is set, the byte at the memory location specified
///     by the contents of SP is moved to the low-order 8 bits of PC.
///     SP is incremented and the byte at the memory location specified by
///     the new contents of the SP are moved to the high-order eight bits of
///     PC.The SP is incremented again. The next op code following this
///     instruction is fetched from the memory location specified by the PC.
///     This instruction is normally used to return to the main line program at
///     the completion of a routine entered by a CALL instruction.
///     If condition X is false, PC is simply incremented as usual, and the
///     program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0xF8
///     =================================
///     T-States: If X is true: 11
///     If X is false: 5

/// 0xF9   "ld sp,hl"
///     The contents of HL are loaded to SP.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 1 | 0xF9
///     =================================
///     T-States: 4
pub fn ld_sp_hl(cpu: &mut CPU) {
    let hl = cpu.lee_hl();

    cpu.sp = hl;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_sp_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP,HL"));
}

/// 0xFA   "jp m,NN"
///     If S flag is set, the instruction loads operand NN
///     to PC, and the program continues with the instruction
///     beginning at address NN.
///     If condition X is false, PC is incremented as usual, and
///     the program continues with the next sequential instruction.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 0 | 1 | 0 | 0xFA
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10

/// 0xFB   "ei"
///     Sets both interrupt enable flip flops (IFFI and IFF2) to a
///     logic 1 value, allowing recognition of any maskable interrupt.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 0xFB
///     =================================
///     T-States: 4
pub fn ei(cpu: &mut CPU) {
    cpu.permitida_interrupcion = true;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ei_txt(cpu: &mut CPU) { cpu.texto(&format!("EI")); }

/// 0xFC   "call m,NN"
///     If flag S is set, this instruction pushes the current
///     contents of PC onto the top of the external memory stack, then
///     loads the operands NN to PC to point to the address in memory
///     at which the first op code of a subroutine is to be fetched.
///     At the end of the subroutine, a RET instruction can be used to
///     return to the original program flow by popping the top of the
///     stack back to PC. If condition X is false, PC is incremented as
///     usual, and the program continues with the next sequential
///     instruction. The stack push is accomplished by first decrementing
///     the current contents of SP, loading the high-order byte of the PC
///     contents to the memory address now pointed to by SP; then
///     decrementing SP again, and loading the low-order byte of the PC
///     contents to the top of the stack.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 0xFC
///     =================================
///     |           8-bit L             |
///     =================================
///     |           8-bit H             |
///     =================================
///     T-States: 10


/// 0xFD   -----EXTENSION--------------------------------------------------------
pub fn FD(cpu: &mut CPU) {
    /*
     self.obtiene_intruccion_y_bytes_posteriores();

        // Ejecuta instruccion
        //self.funciones[self.r0 as usize](self);
        let f: Funcion = self.funciones[self.r0 as usize];
        let ff = f.get_puntero_a_funcion();
        ff(self);
        //self.funciones[self.r0 as usize](self);
    */
    // Ejecuta instruccion
    let f: Funcion = cpu.funciones_fd[cpu.r1 as usize];
    let ff = f.get_puntero_a_funcion();
    ff(cpu);
    //cpu.funciones_cb[cpu.r1 as usize](cpu);
}

pub fn FD_txt(cpu: &mut CPU) {
// Ejecuta instruccion
    let f: Funcion = cpu.funciones_fd[cpu.r1 as usize];

    let ff = f.get_puntero_txt_a_funcion();

    ff(cpu);


//cpu.funciones_cb_txt[cpu.r1 as usize](cpu);
}

/// 0xFE
pub fn cp_n(cpu: &mut CPU) {
    let resultado = cpu.a.wrapping_sub(cpu.r1);

    cpu.set_flag(FLAG_S, resultado & FLAG_S != 0);
    cpu.set_flag(FLAG_Z, resultado == 0);
    cpu.set_flag(FLAG_PV, overflow_en_resta_u8(cpu.a, cpu.r1, resultado));
    cpu.set_flag(FLAG_C, (((cpu.a as u16).wrapping_sub(cpu.r1 as u16)) & 0x100) != 0);
    cpu.set_flag(FLAG_H, half_carry_en_resta_u8_sub(cpu.a, cpu.r1));
    cpu.set_flag(FLAG_N, true);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_n_txt(cpu: &mut CPU) { cpu.texto(&format!("CP #{:02X}", cpu.r1)); }


/// 0xFF   "rst 38h"
///     The current PC contents are pushed onto the external memory stack,
///     and the Page 0 memory location assigned by operand N is loaded to
///     PC. Program execution then begins with the op code in the address
///     now pointed to by PC. The push is performed by first decrementing
///     the contents of SP, loading the high-order byte of PC to the
///     memory address now pointed to by SP, decrementing SP again, and
///     loading the low-order byte of PC to the address now pointed to by
///     SP. The Restart instruction allows for a jump to address 0038H.
///     Because all addresses are stored in Page 0 of memory, the high-order
///     byte of PC is loaded with 0x00.
///     =================================
///     | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 0xFF
///     =================================
///     T-States: 5, 3, 3 (11)
pub fn rst_38(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn rst_38_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

