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

panic!(format!("0x19 ADD HL DE    hl16 = #{:04X}  de16 = #{:04X}\n", hl16, de16));

*/

use crate::cpu::{CPU, Funcion};
use crate::procesador::PROCESADOR;

// bytes, time   datos sacados de fichero:
// https://github.com/malandrin/gbe/blob/master/gbe/opcodes_info.cpp
// En t que difieren segun el código poner el valor menor y luego en la función sumar lo que haga
// falta

pub fn mete_funciones_normales(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones[0x00 as usize].set_punt_y_val_a_fn(nop, nop_txt, 1, 4);
    cpu.funciones[0x01 as usize].set_punt_y_val_a_fn(ld_bc_nn, ld_bc_nn_txt, 3, 12);
    cpu.funciones[0x02 as usize].set_punt_y_val_a_fn(ldObcO_a, ldObcO_a_txt, 1, 8);
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
    cpu.funciones[0x0A as usize].set_punt_y_val_a_fn(ld_aObcO, ld_aObcO_txt, 1, 8);
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

    cpu.funciones[0x11 as usize].set_punt_y_val_a_fn(ld_de_nn, ld_de_nn_txt, 3, 12);
    cpu.funciones[0x12 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x13 as usize].set_punt_y_val_a_fn(inc_de, inc_de_txt, 1, 6);
    cpu.funciones[0x14 as usize].set_punt_y_val_a_fn(inc_d, inc_d_txt, 1, 4);
    cpu.funciones[0x15 as usize].set_punt_y_val_a_fn(dec_d, dec_d_txt, 1, 4);
    cpu.funciones[0x16 as usize].set_punt_y_val_a_fn(ld_d_n, ld_d_n_txt, 2, 7);
    cpu.funciones[0x17 as usize].set_punt_y_val_a_fn(rla, rla_txt, 1, 4);
    cpu.funciones[0x18 as usize].set_punt_y_val_a_fn(jr_n, jr_n_txt, 2, 12);
    cpu.funciones[0x19 as usize].set_punt_y_val_a_fn(add_hl_de, add_hl_de_txt, 1, 11);
    cpu.funciones[0x1A as usize].set_punt_y_val_a_fn(ld_aOdeO, ld_aOdeO_txt, 1, 8);
    cpu.funciones[0x1B as usize].set_punt_y_val_a_fn(dec_de, dec_de_txt, 1, 6);
    cpu.funciones[0x1C as usize].set_punt_y_val_a_fn(inc_e, inc_e_txt, 1, 4);
    cpu.funciones[0x1D as usize].set_punt_y_val_a_fn(dec_e, dec_e_txt, 1, 4);
    cpu.funciones[0x1E as usize].set_punt_y_val_a_fn(ld_e_n, ld_e_n_txt, 2, 7);
    cpu.funciones[0x1F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);


    // *************************** 2 ***********************************
    cpu.funciones[0x20 as usize].set_punt_y_val_a_fn(jr_nz_n, jr_nz_n_txt, 2, 7);
    cpu.funciones[0x21 as usize].set_punt_y_val_a_fn(ld_hl_nn, ld_hl_nn_txt, 3, 12);

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

    cpu.funciones[0x31 as usize].set_punt_y_val_a_fn(ld_sp_nn, ld_sp_nn_txt, 3, 12);

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
    cpu.funciones[0x46 as usize].set_punt_y_val_a_fn(ld_b_OhlO, ld_b_OhlO_txt, 1, 8);
    cpu.funciones[0x47 as usize].set_punt_y_val_a_fn(ld_b_a, ld_b_a_txt, 1, 4);
    cpu.funciones[0x48 as usize].set_punt_y_val_a_fn(ld_c_b, ld_c_b_txt, 1, 4);
    cpu.funciones[0x49 as usize].set_punt_y_val_a_fn(ld_c_c, ld_c_c_txt, 1, 4);
    cpu.funciones[0x4A as usize].set_punt_y_val_a_fn(ld_c_d, ld_c_d_txt, 1, 4);
    cpu.funciones[0x4B as usize].set_punt_y_val_a_fn(ld_c_e, ld_c_e_txt, 1, 4);
    cpu.funciones[0x4C as usize].set_punt_y_val_a_fn(ld_c_h, ld_c_h_txt, 1, 4);
    cpu.funciones[0x4D as usize].set_punt_y_val_a_fn(ld_c_l, ld_c_l_txt, 1, 4);
    cpu.funciones[0x4E as usize].set_punt_y_val_a_fn(ld_c_OhlO, ld_c_OhlO_txt, 1, 8);
    cpu.funciones[0x4F as usize].set_punt_y_val_a_fn(ld_c_a, ld_c_a_txt, 1, 4);
    // *************************** 5 ***********************************
    cpu.funciones[0x50 as usize].set_punt_y_val_a_fn(ld_d_b, ld_d_b_txt, 1, 4);
    cpu.funciones[0x51 as usize].set_punt_y_val_a_fn(ld_d_c, ld_d_c_txt, 1, 4);
    cpu.funciones[0x52 as usize].set_punt_y_val_a_fn(ld_d_d, ld_d_d_txt, 1, 4);
    cpu.funciones[0x53 as usize].set_punt_y_val_a_fn(ld_d_e, ld_d_e_txt, 1, 4);
    cpu.funciones[0x54 as usize].set_punt_y_val_a_fn(ld_d_h, ld_d_h_txt, 1, 4);
    cpu.funciones[0x55 as usize].set_punt_y_val_a_fn(ld_d_l, ld_d_l_txt, 1, 4);
    cpu.funciones[0x56 as usize].set_punt_y_val_a_fn(ld_d_OhlO, ld_d_OhlO_txt, 1, 8);
    cpu.funciones[0x57 as usize].set_punt_y_val_a_fn(ld_d_a, ld_d_a_txt, 1, 4);
    cpu.funciones[0x58 as usize].set_punt_y_val_a_fn(ld_e_b, ld_e_b_txt, 1, 4);
    cpu.funciones[0x59 as usize].set_punt_y_val_a_fn(ld_e_c, ld_e_c_txt, 1, 4);
    cpu.funciones[0x5A as usize].set_punt_y_val_a_fn(ld_e_d, ld_e_d_txt, 1, 4);
    cpu.funciones[0x5B as usize].set_punt_y_val_a_fn(ld_e_e, ld_e_e_txt, 1, 4);
    cpu.funciones[0x5C as usize].set_punt_y_val_a_fn(ld_e_h, ld_e_h_txt, 1, 4);
    cpu.funciones[0x5D as usize].set_punt_y_val_a_fn(ld_e_l, ld_e_l_txt, 1, 4);
    cpu.funciones[0x5E as usize].set_punt_y_val_a_fn(ld_e_OhlO, ld_e_OhlO_txt, 1, 8);
    cpu.funciones[0x5F as usize].set_punt_y_val_a_fn(ld_e_a, ld_e_a_txt, 1, 4);

    // *************************** 6 ***********************************
    cpu.funciones[0x60 as usize].set_punt_y_val_a_fn(ld_h_b, ld_h_b_txt, 1, 4);
    cpu.funciones[0x61 as usize].set_punt_y_val_a_fn(ld_h_c, ld_h_c_txt, 1, 4);
    cpu.funciones[0x62 as usize].set_punt_y_val_a_fn(ld_h_d, ld_h_d_txt, 1, 4);
    cpu.funciones[0x63 as usize].set_punt_y_val_a_fn(ld_h_e, ld_h_e_txt, 1, 4);
    cpu.funciones[0x64 as usize].set_punt_y_val_a_fn(ld_h_h, ld_h_h_txt, 1, 4);
    cpu.funciones[0x65 as usize].set_punt_y_val_a_fn(ld_h_l, ld_h_l_txt, 1, 4);
    cpu.funciones[0x66 as usize].set_punt_y_val_a_fn(ld_h_OhlO, ld_h_OhlO_txt, 1, 8);
    cpu.funciones[0x67 as usize].set_punt_y_val_a_fn(ld_h_a, ld_h_a_txt, 1, 4);
    cpu.funciones[0x68 as usize].set_punt_y_val_a_fn(ld_l_b, ld_l_b_txt, 1, 4);
    cpu.funciones[0x69 as usize].set_punt_y_val_a_fn(ld_l_c, ld_l_c_txt, 1, 4);
    cpu.funciones[0x6A as usize].set_punt_y_val_a_fn(ld_l_d, ld_l_d_txt, 1, 4);
    cpu.funciones[0x6B as usize].set_punt_y_val_a_fn(ld_l_e, ld_l_e_txt, 1, 4);
    cpu.funciones[0x6C as usize].set_punt_y_val_a_fn(ld_l_h, ld_l_h_txt, 1, 4);
    cpu.funciones[0x6D as usize].set_punt_y_val_a_fn(ld_l_l, ld_l_l_txt, 1, 4);
    cpu.funciones[0x6E as usize].set_punt_y_val_a_fn(ld_l_OhlO, ld_l_OhlO_txt, 1, 8);
    cpu.funciones[0x6F as usize].set_punt_y_val_a_fn(ld_l_a, ld_l_a_txt, 1, 4);
    // *************************** 7 ***********************************
    cpu.funciones[0x70 as usize].set_punt_y_val_a_fn(ldOhlO_b, ldOhlO_b_txt, 1, 8);
    cpu.funciones[0x71 as usize].set_punt_y_val_a_fn(ldOhlO_c, ldOhlO_c_txt, 1, 8);
    cpu.funciones[0x72 as usize].set_punt_y_val_a_fn(ldOhlO_d, ldOhlO_d_txt, 1, 8);
    cpu.funciones[0x73 as usize].set_punt_y_val_a_fn(ldOhlO_e, ldOhlO_e_txt, 1, 8);
    cpu.funciones[0x74 as usize].set_punt_y_val_a_fn(ldOhlO_h, ldOhlO_h_txt, 1, 8);
    cpu.funciones[0x75 as usize].set_punt_y_val_a_fn(ldOhlO_l, ldOhlO_l_txt, 1, 8);
    cpu.funciones[0x76 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x77 as usize].set_punt_y_val_a_fn(ldOhlO_a, ldOhlO_a_txt, 1, 8);
    cpu.funciones[0x78 as usize].set_punt_y_val_a_fn(ld_a_b, ld_a_b_txt, 1, 4);
    cpu.funciones[0x79 as usize].set_punt_y_val_a_fn(ld_a_c, ld_a_c_txt, 1, 4);
    cpu.funciones[0x7A as usize].set_punt_y_val_a_fn(ld_a_d, ld_a_d_txt, 1, 4);
    cpu.funciones[0x7B as usize].set_punt_y_val_a_fn(ld_a_e, ld_a_e_txt, 1, 4);
    cpu.funciones[0x7C as usize].set_punt_y_val_a_fn(ld_a_h, ld_a_h_txt, 1, 4);
    cpu.funciones[0x7D as usize].set_punt_y_val_a_fn(ld_a_l, ld_a_l_txt, 1, 4);
    cpu.funciones[0x7E as usize].set_punt_y_val_a_fn(ld_a_OhlO, ld_a_OhlO_txt, 1, 8);
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
    cpu.funciones[0x8E as usize].set_punt_y_val_a_fn(adc_a_OhlO, adc_a_OhlO_txt, 1, 8);
    cpu.funciones[0x8F as usize].set_punt_y_val_a_fn(adc_a_a, adc_a_a_txt, 1, 4);
    // *************************** 9 ***********************************
    cpu.funciones[0x90 as usize].set_punt_y_val_a_fn(sub_b, sub_b_txt, 1, 4);
    cpu.funciones[0x91 as usize].set_punt_y_val_a_fn(sub_c, sub_c_txt, 1, 4);
    cpu.funciones[0x92 as usize].set_punt_y_val_a_fn(sub_d, sub_d_txt, 1, 4);
    cpu.funciones[0x93 as usize].set_punt_y_val_a_fn(sub_e, sub_e_txt, 1, 4);
    cpu.funciones[0x94 as usize].set_punt_y_val_a_fn(sub_h, sub_h_txt, 1, 4);
    cpu.funciones[0x95 as usize].set_punt_y_val_a_fn(sub_l, sub_l_txt, 1, 4);
    cpu.funciones[0x96 as usize].set_punt_y_val_a_fn(subOhlO, subOhlO, 1, 8);
    cpu.funciones[0x97 as usize].set_punt_y_val_a_fn(sub_a, sub_a_txt, 1, 4);
    cpu.funciones[0x98 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x99 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9A as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9B as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9C as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9D as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x9E as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x9F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    // *************************** A ***********************************
    cpu.funciones[0xA0 as usize].set_punt_y_val_a_fn(and_b, and_b_txt, 1, 4);
    cpu.funciones[0xA1 as usize].set_punt_y_val_a_fn(and_c, and_c_txt, 1, 4);
    cpu.funciones[0xA2 as usize].set_punt_y_val_a_fn(and_d, and_d_txt, 1, 4);
    cpu.funciones[0xA3 as usize].set_punt_y_val_a_fn(and_e, and_e_txt, 1, 4);
    cpu.funciones[0xA4 as usize].set_punt_y_val_a_fn(and_h, and_h_txt, 1, 4);
    cpu.funciones[0xA5 as usize].set_punt_y_val_a_fn(and_l, and_l_txt, 1, 4);
    cpu.funciones[0xA6 as usize].set_punt_y_val_a_fn(and_OhlO, and_OhlO_txt, 1, 8);
    cpu.funciones[0xA7 as usize].set_punt_y_val_a_fn(and_a, and_a_txt, 1, 4);
    cpu.funciones[0xA8 as usize].set_punt_y_val_a_fn(xor_b, xor_b_txt, 1, 4);
    cpu.funciones[0xA9 as usize].set_punt_y_val_a_fn(xor_c, xor_c_txt, 1, 4);
    cpu.funciones[0xAA as usize].set_punt_y_val_a_fn(xor_d, xor_d_txt, 1, 4);
    cpu.funciones[0xAB as usize].set_punt_y_val_a_fn(xor_e, xor_e_txt, 1, 4);
    cpu.funciones[0xAC as usize].set_punt_y_val_a_fn(xor_h, xor_h_txt, 1, 4);
    cpu.funciones[0xAD as usize].set_punt_y_val_a_fn(xor_l, xor_l_txt, 1, 4);
    cpu.funciones[0xAE as usize].set_punt_y_val_a_fn(xor_OhlO, xor_OhlO_txt, 1, 8);
    cpu.funciones[0xAF as usize].set_punt_y_val_a_fn(xor_a, xor_a_txt, 1, 4);
    // *************************** B ***********************************
    cpu.funciones[0xB0 as usize].set_punt_y_val_a_fn(or_b, or_b_txt, 1, 4);
    cpu.funciones[0xB1 as usize].set_punt_y_val_a_fn(or_c, or_c_txt, 1, 4);
    cpu.funciones[0xB2 as usize].set_punt_y_val_a_fn(or_d, or_d_txt, 1, 4);
    cpu.funciones[0xB3 as usize].set_punt_y_val_a_fn(or_e, or_e_txt, 1, 4);
    cpu.funciones[0xB4 as usize].set_punt_y_val_a_fn(or_h, or_h_txt, 1, 4);
    cpu.funciones[0xB5 as usize].set_punt_y_val_a_fn(or_l, or_l_txt, 1, 4);
    cpu.funciones[0xB6 as usize].set_punt_y_val_a_fn(or_OhlO, or_OhlO_txt, 1, 8);
    cpu.funciones[0xB7 as usize].set_punt_y_val_a_fn(or_a, or_a, 1, 4);
    cpu.funciones[0xB8 as usize].set_punt_y_val_a_fn(cp_b, cp_b_txt, 1, 4);
    cpu.funciones[0xB9 as usize].set_punt_y_val_a_fn(cp_c, cp_c_txt, 1, 4);
    cpu.funciones[0xBA as usize].set_punt_y_val_a_fn(cp_d, cp_d_txt, 1, 4);
    cpu.funciones[0xBB as usize].set_punt_y_val_a_fn(cp_e, cp_e_txt, 1, 4);
    cpu.funciones[0xBC as usize].set_punt_y_val_a_fn(cp_h, cp_h_txt, 1, 4);
    cpu.funciones[0xBD as usize].set_punt_y_val_a_fn(cp_l, cp_l_txt, 1, 4);
    cpu.funciones[0xBE as usize].set_punt_y_val_a_fn(cpOhlO, cpOhlO_txt, 1, 8);
    cpu.funciones[0xBF as usize].set_punt_y_val_a_fn(cp_a, cp_a_txt, 1, 4);

    // *************************** C ***********************************
    cpu.funciones[0xC0 as usize].set_punt_y_val_a_fn(ret_nz, ret_nz_txt, 1, 5);
    cpu.funciones[0xC1 as usize].set_punt_y_val_a_fn(pop_bc, pop_bc_txt, 1, 12);
    cpu.funciones[0xC2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xC3 as usize].set_punt_y_val_a_fn(jp_nn, jp_nn_txt, 3, 16);
    cpu.funciones[0xC4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xC5 as usize].set_punt_y_val_a_fn(push_bc, push_bc_txt, 1, 16);
    cpu.funciones[0xC6 as usize].set_punt_y_val_a_fn(add_a_n, add_a_n_txt, 2, 7);
    cpu.funciones[0xC7 as usize].set_punt_y_val_a_fn(rst_00, rst_00_txt, 1, 11);
    cpu.funciones[0xC8 as usize].set_punt_y_val_a_fn(ret_z, ret_z_txt, 1, 5);
    cpu.funciones[0xC9 as usize].set_punt_y_val_a_fn(ret, ret_txt, 1, 16);
    cpu.funciones[0xCA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xCB as usize].set_punt_y_val_a_fn(CB, CB_txt, 0, 0); // Extensión
    cpu.funciones[0xCC as usize].set_punt_y_val_a_fn(call_z_nn, call_z_nn_txt, 3, 10);
    cpu.funciones[0xCD as usize].set_punt_y_val_a_fn(call_nn, call_nn_txt, 3, 14);
    cpu.funciones[0xCE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xCF as usize].set_punt_y_val_a_fn(rst_08, rst_08_txt, 1, 11);
    // *************************** D ***********************************
    cpu.funciones[0xD0 as usize].set_punt_y_val_a_fn(ret_nc, ret_nc_txt, 1, 8);
    cpu.funciones[0xD1 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 10);
    cpu.funciones[0xD2 as usize].set_punt_y_val_a_fn(jp_nc_nn, jp_nc_nn_txt, 3, 10);
    cpu.funciones[0xD3 as usize].set_punt_y_val_a_fn(out_OnO_a, out_OnO_a_txt, 2, 11);
    cpu.funciones[0xD4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 13);
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

    cpu.funciones[0xDA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xDB as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xDC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xDD as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xDE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 8);
    cpu.funciones[0xDF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);

    // *************************** E ***********************************
    // LR35902->LD (FF00+u8),A         Z-80->RET NV
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE0 as usize].set_punt_y_val_a_fn(ldOff00_m_nO_aGB, ldOff00_m_nO_aGB_txt, 2, 12);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE0 as usize].set_punt_y_val_a_fn(ret_po, ret_po_txt, 1, 10);
        }
    }
    cpu.funciones[0xE1 as usize].set_punt_y_val_a_fn(pop_hl, pop_hl_txt, 1, 12);
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
    cpu.funciones[0xE4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
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
            cpu.funciones[0xE8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 16);
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
    cpu.funciones[0xEB as usize].set_punt_y_val_a_fn(ex_de_hl, ex_de_hl_txt, 1, 0);
    cpu.funciones[0xEC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xED as usize].set_punt_y_val_a_fn(ED, ED_txt, 0, 0); // Extensión
    cpu.funciones[0xEE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 8);
    cpu.funciones[0xEF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);

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
    cpu.funciones[0xF1 as usize].set_punt_y_val_a_fn(pop_af, pop_af_txt, 1, 12);

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
    cpu.funciones[0xF4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
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
            cpu.funciones[0xF8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 6);
        }
    }

    cpu.funciones[0xF9 as usize].set_punt_y_val_a_fn(ld_sp_hl, ld_sp_hl_txt, 1, 8);

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
    cpu.funciones[0xFC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xFD as usize].set_punt_y_val_a_fn(FD, FD_txt, 0, 0); // Extensión
    cpu.funciones[0xFE as usize].set_punt_y_val_a_fn(cp_n, cp_n_txt, 2, 7);
    cpu.funciones[0xFF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
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
    let mut kk: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            kk = cpu.a.wrapping_add(cpu.a);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.a, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.a);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.a);
            cpu.a = kk;
        }
        0b00000_000 => {
            kk = cpu.a.wrapping_add(cpu.b);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.b, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.b);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.b);
            cpu.b = kk;
        }
        0b00000_001 => {
            kk = cpu.a.wrapping_add(cpu.c);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.c, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.c);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.c);
            cpu.c = kk;
        }
        0b00000_010 => {
            kk = cpu.a.wrapping_add(cpu.d);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.d, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.d);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.d);
            cpu.d = kk;
        }
        0b00000_011 => {
            kk = cpu.a.wrapping_add(cpu.e);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.e, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.e);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.e);
            cpu.e = kk;
        }
        0b00000_100 => {
            kk = cpu.a.wrapping_add(cpu.h);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.h, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.h);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.h);
            cpu.h = kk;
        }
        0b00000_101 => {
            kk = cpu.a.wrapping_add(cpu.l);

            cpu.flag_v_u8_en_suma(cpu.a, cpu.l, kk);
            cpu.flag_c_u8_en_suma(cpu.a, cpu.l);
            cpu.flag_h_u8_en_suma(cpu.a, cpu.l);
            cpu.l = kk;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            kk = cpu.a.wrapping_add(dato);

            cpu.flag_v_u8_en_suma(cpu.a, dato, kk);
            cpu.flag_c_u8_en_suma(cpu.a, dato);
            cpu.flag_h_u8_en_suma(cpu.a, dato);
            cpu.a = kk;
        }
        _ => panic!("Instruccion en bas_add_a_R no reconocida"),
    }

    cpu.reset_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);
    //cpu.flag_h_u8(kk);


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
            hl = hl.wrapping_add(bc);

            cpu.flag_c_u16_en_suma(hl, bc);
        }
        0b00_01_0000 => {
            let de = cpu.lee_de();
            hl = hl.wrapping_add(de);

            cpu.flag_c_u16_en_suma(hl, de);
        }
        0b00_10_0000 => {
            hl = hl.wrapping_add(hl);

            cpu.flag_c_u16_en_suma(hl, hl);
        }
        0b00_11_0000 => {
            hl = hl.wrapping_add(cpu.sp);

            cpu.flag_c_u16_en_suma(hl, cpu.sp);
        }
        _ => panic!("Instruccion en add_hl_Q no reconocida"),
    }

    cpu.escribe_hl(hl);
    cpu.flag_h_u16(hl);
    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}


// add I,Q 	11i11101 00qq1001 	15 	- 	- 	+ 	+ 	+ 	- 	0 	+ 	I += Q
pub fn bas_add_I_Q(cpu: &mut CPU) { fn_no_impl(cpu); }

// and R 	10100rrr 	4 	+ 	+ 	+ 	1 	+ 	P 	0 	0 	a := a AND R
pub fn bas_and_R(cpu: &mut CPU) {
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            cpu.a = cpu.a & cpu.a;
        }
        0b00000_000 => {
            cpu.a = cpu.a & cpu.b;
        }
        0b00000_001 => {
            cpu.a = cpu.a & cpu.c;
        }
        0b00000_010 => {
            cpu.a = cpu.a & cpu.d;
        }
        0b00000_011 => {
            cpu.a = cpu.a & cpu.e;
        }
        0b00000_100 => {
            cpu.a = cpu.a & cpu.h;
        }
        0b00000_101 => {
            cpu.a = cpu.a & cpu.l;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            cpu.a = cpu.a & dato;
        }
        _ => panic!("Instruccion en bas_and_R no reconocida"),
    }

    cpu.flag_s_u8(cpu.a);
    cpu.flag_z_u8(cpu.a);
    cpu.flag_p_u8(cpu.a); // Como paridad
    cpu.set_h_flag();

    cpu.reset_c_flag();
    cpu.reset_n_flag();


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
pub fn bas_bit_B_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

// call A 	11001101 alalalal ahahahah 	17 	- 	- 	- 	- 	- 	- 	- 	- 	sp -= 2, (sp) := pc, pc := A
pub fn bas_call_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//call C,A 	11ccc100 alalalal ahahahah 	17/10 	- 	- 	- 	- 	- 	- 	- 	- 	if C then sp -= 2, (sp) := pc, pc := A
pub fn bas_call_C_A(cpu: &mut CPU) { fn_no_impl(cpu); }

//ccf 	00111111 	4 	- 	- 	A 	X 	A 	- 	0 	X 	hf := cf, cf := ~cf
pub fn bas_ccf(cpu: &mut CPU) { fn_no_impl(cpu); }

// cp R 	10111rrr 	4 	+ 	+ 	X 	+ 	X 	V 	1 	+ 	tmp := a - R, xf := R.5, yf = R.3
pub fn bas_cp_R(cpu: &mut CPU) {
    let mut kk: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            kk = cpu.a.wrapping_sub(cpu.a);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.a, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.a);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.a);
        }
        0b00000_000 => {
            kk = cpu.a.wrapping_sub(cpu.b);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.b, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.b);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.b);
        }
        0b00000_001 => {
            kk = cpu.a.wrapping_sub(cpu.c);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.c, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.c);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.c);
        }
        0b00000_010 => {
            kk = cpu.a.wrapping_sub(cpu.d);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.d, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.d);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.d);
        }
        0b00000_011 => {
            kk = cpu.a.wrapping_sub(cpu.e);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.e, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.e);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.e);
        }
        0b00000_100 => {
            kk = cpu.a.wrapping_sub(cpu.h);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.h, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.h);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.h);
        }
        0b00000_101 => {
            kk = cpu.a.wrapping_sub(cpu.l);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.l, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.l);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.l);
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            kk = cpu.a.wrapping_sub(dato);

            cpu.flag_v_u8_en_resta(cpu.a, dato, kk);
            cpu.flag_c_u8_en_resta(cpu.a, dato);
            cpu.flag_h_u8_en_resta(cpu.a, dato);
        }
        _ => panic!("Instruccion en bas_cp_R no reconocida"),
    }

    cpu.set_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);


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
    let mut kk: u8 = 0;
    match cpu.r0 & 0b00_111_000 {
        0b00_111_000 => {
            kk = cpu.a.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.a, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.a, 1);
            cpu.a = kk;
        }
        0b00_000_000 => {
            kk = cpu.b.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.b, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.b, 1);
            cpu.b = kk;
        }
        0b00_001_000 => {
            kk = cpu.c.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.c, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.c, 1);
            cpu.c = kk;
        }
        0b00_010_000 => {
            kk = cpu.d.wrapping_sub(1);

            cpu.flag_v_u8_en_suma(cpu.d, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.d, 1);
            cpu.d = kk;
        }
        0b00_011_000 => {
            kk = cpu.e.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.e, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.e, 1);
            cpu.e = kk;
        }
        0b00_100_000 => {
            kk = cpu.h.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.h, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.h, 1);
            cpu.h = kk;
        }
        0b00_101_000 => {
            kk = cpu.l.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(cpu.l, 1, kk);
            cpu.flag_h_u8_en_resta(cpu.l, 1);
            cpu.l = kk;
        }
        0b00_110_000 => {
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            kk = dato.wrapping_sub(1);

            cpu.flag_v_u8_en_resta(dato, 1, kk);
            cpu.flag_h_u8_en_resta(dato, 1);
            cpu.mem.escribe_byte_en_mem(hl, kk);
        }
        _ => panic!("Instruccion en bas_dec_R no reconocida"),
    }

    cpu.set_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);


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
    let mut kk: u8 = 0;
    match cpu.r0 & 0b00_111_000 {
        0b00_111_000 => {
            kk = cpu.a.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.a, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.a, 1);
            cpu.a = kk;
        }
        0b00_000_000 => {
            kk = cpu.b.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.b, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.b, 1);
            cpu.b = kk;
        }
        0b00_001_000 => {
            kk = cpu.c.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.c, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.c, 1);
            cpu.c = kk;
        }
        0b00_010_000 => {
            kk = cpu.d.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.d, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.d, 1);
            cpu.d = kk;
        }
        0b00_011_000 => {
            kk = cpu.e.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.e, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.e, 1);
            cpu.e = kk;
        }
        0b00_100_000 => {
            kk = cpu.h.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.h, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.h, 1);
            cpu.h = kk;
        }
        0b00_101_000 => {
            kk = cpu.l.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.l, 1, kk);
            cpu.flag_h_u8_en_suma(cpu.l, 1);
            cpu.l = kk;
        }
        0b00_110_000 => { //(hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            kk = dato.wrapping_add(1);

            cpu.flag_v_u8_en_suma(cpu.l, 1, kk);
            cpu.flag_h_u8_en_suma(dato, 1);
            cpu.mem.escribe_byte_en_mem(hl, kk);
        }
        _ => panic!("Instruccion en bas_inc_R no reconocida"),
    }

    cpu.reset_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);
    //cpu.flag_h_u8(kk);


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
pub fn bas_ld_R1_R2(cpu: &mut CPU) { fn_no_impl(cpu); }

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
pub fn bas_ld_ObcO_a(cpu: &mut CPU) {
    let direccion = cpu.lee_bc();
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

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

// ld Q,(A) 	11101101 01qq1011 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	-
// Q := (A) [ld hl,(A) has a faster non-prefixed duplicate, see below.]
pub fn bas_ld_Q_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld hl,(A) 	00101010 alalalal ahahahah 	16 	- 	- 	- 	- 	- 	- 	- 	- 	hl := (A)
pub fn bas_ld_hl_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld I,(A) 	11i11101 00101010 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	- 	I := (A)
pub fn bas_ld_I_OAO(cpu: &mut CPU) { fn_no_impl(cpu); }

// ld (A),Q 	11101101 01qq0011 alalalal ahahahah 	20 	- 	- 	- 	- 	- 	- 	- 	-
// (A) := Q [ld (A),hl has a faster non-prefixed duplicate, see below.]
pub fn bas_ld_OAO_Q(cpu: &mut CPU) { fn_no_impl(cpu); }

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

    cpu.flag_s_u8(cpu.a);
    cpu.flag_z_u8(cpu.a);
    cpu.flag_p_u8(cpu.a); // Como paridad
    cpu.reset_h_flag();

    cpu.reset_c_flag();
    cpu.reset_n_flag();


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
pub fn bas_res_B_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

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
pub fn bas_sbc_a_R(cpu: &mut CPU) { fn_no_impl(cpu); }

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
pub fn bas_set_B_OImDO(cpu: &mut CPU) { fn_no_impl(cpu); }

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
    let mut kk: u8 = 0;
    match cpu.r0 & 0b00000_111 {
        0b00000_111 => {
            kk = cpu.a.wrapping_sub(cpu.a);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.a, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.a);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.a);

            cpu.a = kk;
        }
        0b00000_000 => {
            kk = cpu.a.wrapping_sub(cpu.b);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.b, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.b);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.b);
            cpu.a = kk;
        }
        0b00000_001 => {
            kk = cpu.a.wrapping_sub(cpu.c);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.c, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.c);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.c);
            cpu.a = kk;
        }
        0b00000_010 => {
            kk = cpu.a.wrapping_sub(cpu.d);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.d, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.d);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.d);
            cpu.a = kk;
        }
        0b00000_011 => {
            kk = cpu.a.wrapping_sub(cpu.e);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.e, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.e);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.e);
            cpu.a = kk;
        }
        0b00000_100 => {
            kk = cpu.a.wrapping_sub(cpu.h);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.h, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.h);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.h);
            cpu.a = kk;
        }
        0b00000_101 => {
            kk = cpu.a.wrapping_sub(cpu.l);

            cpu.flag_v_u8_en_resta(cpu.a, cpu.l, kk);
            cpu.flag_c_u8_en_resta(cpu.a, cpu.l);
            cpu.flag_h_u8_en_resta(cpu.a, cpu.l);
            cpu.a = kk;
        }
        0b00000_110 => { // (hl)
            let hl = cpu.lee_hl();
            let dato = cpu.mem.lee_byte_de_mem(hl);
            kk = cpu.a.wrapping_sub(dato);

            cpu.flag_v_u8_en_resta(cpu.a, dato, kk);
            cpu.flag_c_u8_en_resta(cpu.a, dato);
            cpu.flag_h_u8_en_resta(cpu.a, dato);
            cpu.mem.escribe_byte_en_mem(hl, kk);
        }
        _ => panic!("Instruccion en bas_sub_R no reconocida"),
    }

    cpu.set_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);

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

    cpu.flag_s_u8(cpu.a);
    cpu.flag_z_u8(cpu.a);
    cpu.flag_p_u8(cpu.a); // Como paridad
    cpu.reset_h_flag();

    cpu.reset_c_flag();
    cpu.reset_n_flag();


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
    cpu.texto(&format!("NOP opcode = #{:02X}", cpu.r0));
}

// 0x01 NN NN      ld Q,A
pub fn ld_bc_nn(cpu: &mut CPU) {
    bas_ld_Q_A(cpu);
//    cpu.c = cpu.r1;
//    cpu.b = cpu.r2;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_bc_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD BC,#{:04X}", cpu.r1r2)); }

// 0x02    ld (bc),a 	00000010 	- 	- 	- 	- 	- 	- 	- 	- 	(bc) := a
pub fn ldObcO_a(cpu: &mut CPU) {
    bas_ld_ObcO_a(cpu);
//    let direccion = cpu.lee_bc();
//    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldObcO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (BC),A"));
}

// 0x03
pub fn inc_bc(cpu: &mut CPU) {
    bas_inc_Q(cpu);
}

pub fn inc_bc_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC BC"));
}

// 0x04
pub fn inc_b(cpu: &mut CPU) {
//    cpu.b = cpu.inc_8bits(cpu.b);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
    bas_inc_R(cpu);
}

pub fn inc_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC B"));
}

// 0x05
pub fn dec_b(cpu: &mut CPU) {
    bas_dec_R(cpu);
//    cpu.b = cpu.dec_8bits(cpu.b);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_b_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC B")); }

// 0x06
pub fn ld_b_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
//    cpu.b = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD B,#{:02X}", cpu.r1)); }

// 0x07
pub fn rlca(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn rlca_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0x08 Difiere según procesador (LR35902->LD(NN),SP)
pub fn ldOnnO_spGB(cpu: &mut CPU) {
    cpu.mem.escribe_2bytes_en_mem(cpu.r1r2, cpu.sp);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_spGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),SP", cpu.r1r2));
}

// 0x08 Difiere según procesador (Z80->EX AF,AF')
pub fn ex_af_afp(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn ex_af_afp_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0x09
pub fn add_hl_bc(cpu: &mut CPU) {
    bas_add_hl_Q(cpu);
//    let bc = cpu.lee_bc();
//    let hl = cpu.lee_hl();
//
//    let hl = cpu.suma_u16_mas_u16(hl, bc);
//    cpu.escribe_hl(hl);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn add_hl_bc_txt(cpu: &mut CPU) {
    cpu.texto(&format!("ADD HL,BC"));
}

// 0x0A  ld a,(bc) 	00001010 	- 	- 	- 	- 	- 	- 	- 	- 	a := (bc)
pub fn ld_aObcO(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn ld_aObcO_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

// 0x0B  dec Q 	00qq1011 	- 	- 	- 	- 	- 	- 	- 	- 	Q -= 1
pub fn dec_bc(cpu: &mut CPU) {
    bas_dec_Q(cpu);
//    let mut bc = cpu.lee_bc();
//    bc = cpu.dec_16bits(bc);
//
//    cpu.escribe_bc(bc);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC BC")); }

// 0x0C  inc R 	00rrr100 	+ 	+ 	+ 	+ 	+ 	V 	0 	- 	R += 1
pub fn inc_c(cpu: &mut CPU) {
    bas_inc_R(cpu);
//    cpu.c = cpu.inc_8bits(cpu.c);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_c_txt(cpu: &mut CPU) { cpu.texto(&format!("INC C")); }

// 0x0D  dec R 	00rrr101 	+ 	+ 	+ 	+ 	+ 	V 	1 	- 	R -= 1
pub fn dec_c(cpu: &mut CPU) {
    bas_dec_R(cpu);
//    cpu.c = cpu.dec_8bits(cpu.c);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC C"));
}

// 0x0E
pub fn ld_c_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
//    cpu.c = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
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
    cpu.flag_z_u8(nuevo_valor);
    cpu.reset_n_flag();
    cpu.reset_h_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rrca_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RRCA"));
}
// *************************** 1 ***********************************

// 0x10
pub fn djnz_n(cpu: &mut CPU) {
    cpu.pc += cpu.get_bytes_instruccion();
    cpu.b = cpu.dec_8bits(cpu.b);
    if cpu.b != 0 {
        cpu.pc = cpu.suma_compl2_a_un_u16(cpu.pc, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn djnz_n_txt(cpu: &mut CPU) { cpu.texto(&format!("DJNZ")); }

// 0x11 NN NN
pub fn ld_de_nn(cpu: &mut CPU) {
    bas_ld_Q_A(cpu);
//    cpu.d = cpu.r2;
//    cpu.e = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_de_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD DE,#{:04X}", cpu.r1r2)); }

// 0x12
pub fn ldOdeO_a(cpu: &mut CPU) { fn_no_impl(cpu); }

// 0x13
pub fn inc_de(cpu: &mut CPU) {
    bas_inc_Q(cpu);
//    let mut de = cpu.lee_de();
//    de = cpu.inc_16bits(de);
//
//    cpu.escribe_de(de);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_de_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC DE"));
}

// 0x14
pub fn inc_d(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_d_txt(cpu: &mut CPU) { cpu.texto(&format!("INC D")); }

// 0x15
pub fn dec_d(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_d_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC D")); }

// 0x16
pub fn ld_d_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
//    cpu.d = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
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

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn rla_txt(cpu: &mut CPU) { cpu.texto(&format!("RLA")); }

// 0x18
pub fn jr_n(cpu: &mut CPU) {
    cpu.pc += 2;  // parece que hace falta TODO: comprobar
    cpu.pc = cpu.suma_compl2_a_un_u16(cpu.pc, cpu.r1);

    cpu.t += cpu.get_t_instruccion();
}


pub fn jr_n_txt(cpu: &mut CPU) {
    let direccion = cpu.suma_compl2_a_un_u16(cpu.pc, cpu.r1) + 2;
    cpu.texto(&format!("JR {:04X}", direccion));
}

// 0x19
pub fn add_hl_de(cpu: &mut CPU) {
    bas_add_hl_Q(cpu);
//    let mut hl = cpu.lee_hl();
//    let de = cpu.lee_de();
//
//    hl = cpu.suma_u16_mas_u16(hl, de);
//
//    cpu.escribe_hl(hl);
//
//    cpu.pc += cpu.get_bytes_instruccion();
//    cpu.t += cpu.get_t_instruccion();
}

pub fn add_hl_de_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,DE")); }

// 0x1A
pub fn ld_aOdeO(cpu: &mut CPU) {
    let direccion = cpu.lee_de();
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_aOdeO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD A(DE)")); }

// 0x1B
pub fn dec_de(cpu: &mut CPU) {
    bas_dec_Q(cpu);
//    let mut de = cpu.lee_de();
//    de = cpu.dec_16bits(de);
//    cpu.escribe_de(de);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_de_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC DE")); }

// 0x1C
pub fn inc_e(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC E"));
}

// 0x1D
pub fn dec_e(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_e_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC E")); }

// 0x1E
pub fn ld_e_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
}

pub fn ld_e_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD E,#{:02X}", cpu.r1));
}

// 0x1F
pub fn rra(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn rra_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

// *************************** 2 ***********************************
// 0x20 NN
fn jr_nz_n(cpu: &mut CPU) {
    let salto = cpu.suma_compl2_a_un_u16(cpu.pc + 2, cpu.r1);
    if !cpu.get_z_flag() {
        cpu.pc = salto;

        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

fn jr_nz_n_txt(cpu: &mut CPU) {
    let salto = cpu.suma_compl2_a_un_u16(cpu.pc + 2, cpu.r1);
    cpu.texto(&format!("JR NZ #{:04X}", salto));
}

// 0x21 NN NN
pub fn ld_hl_nn(cpu: &mut CPU) {
    bas_ld_Q_A(cpu);
//    cpu.h = cpu.r2;
//    cpu.l = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_hl_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL#{:04X}", cpu.r1r2)); }


// 0x22 Difiere según procesador (LR35902->LDI  (HL),A)
pub fn ldiOhlO_aGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    hl = cpu.inc_16bits(hl);
    cpu.escribe_hl(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldiOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDI(HL),A")); }

// 0x22 Difiere según procesador (Z80->LD  (nn),HL)
pub fn ldOnnO_hl(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.l);
    cpu.mem.escribe_byte_en_mem(cpu.r1r2 + 1, cpu.h);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(#{:04X}),HL", cpu.r1r2));
}

// 0x23
pub fn inc_hl(cpu: &mut CPU) {
    bas_inc_Q(cpu);
//    let mut hl = cpu.lee_hl();
//    hl = cpu.inc_16bits(hl);
//    cpu.escribe_hl(hl);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("INC HL")); }

// 0x24
pub fn inc_h(cpu: &mut CPU) {
    bas_inc_R(cpu);
//    cpu.h = cpu.inc_8bits(cpu.h);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC H"));
}


// 0x25
pub fn dec_h(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC H"));
}


// 0x26
pub fn ld_h_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
//    cpu.h = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,#{:02X}", cpu.r1));
}

// 0x27
pub fn daa(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn daa_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

// 0x28
pub fn jr_z_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());
    if cpu.get_z_flag() {
        cpu.pc = cpu.suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn jr_z_n_txt(cpu: &mut CPU) {
    let salto = cpu.suma_compl2_a_un_u16(cpu.pc.wrapping_add(2), cpu.r1);
    cpu.texto(&format!("JR Z(#{:04X})", salto));
}

// 0x29
pub fn add_hl_hl(cpu: &mut CPU) {
    bas_add_hl_Q(cpu);
//    let mut hl = cpu.lee_hl();
//
//    hl = cpu.suma_u16_mas_u16(hl, hl);
//
//    cpu.escribe_hl(hl);
//
//    cpu.pc += cpu.get_bytes_instruccion();
//    cpu.t += cpu.get_t_instruccion();
}

pub fn add_hl_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,HL")); }

// 0x2A Difiere según procesador (LR35902->LDI A,(HL))
pub fn ldi_aOhlOGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.a = cpu.mem.lee_byte_de_mem(hl);

    hl = cpu.inc_16bits(hl);
    cpu.escribe_hl(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldi_aOhlOGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDI A(HL)")); }

// 0x2A NN NN    Difiere según procesador (Z80->LD HL,(NN)
pub fn ld_hlOnnO(cpu: &mut CPU) {
    cpu.l = cpu.mem.lee_byte_de_mem(cpu.r1r2);
    cpu.h = cpu.mem.lee_byte_de_mem(cpu.r1r2 + 1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_hlOnnO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD HL(#{:04X})", cpu.r1r2)); }

// 0x2B
pub fn dec_hl(cpu: &mut CPU) {
    bas_dec_Q(cpu);
//    let mut hl = cpu.lee_hl();
//    hl = cpu.dec_16bits(hl);
//    cpu.escribe_hl(hl);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC HL")); }

// 0x2C
pub fn inc_l(cpu: &mut CPU) {
    bas_inc_R(cpu);
}

pub fn inc_l_txt(cpu: &mut CPU) { cpu.texto(&format!("INC L")); }

// 0x2D
pub fn dec_l(cpu: &mut CPU) {
    bas_dec_R(cpu);
}

pub fn dec_l_txt(cpu: &mut CPU) { cpu.texto(&format!("DEC L")); }

// 0x2E
pub fn ld_l_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
}

pub fn ld_l_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD L#{:02X}", cpu.r1)); }

// 0x2F
pub fn cpl(cpu: &mut CPU) {
    cpu.a = cpu.a ^ 0xFF;
}

pub fn cpl_txt(cpu: &mut CPU) { cpu.texto(&format!("CPL")); }

// *************************** 3 ***********************************

// 0x30
pub fn jr_nc_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());
    if !cpu.get_c_flag() {
        cpu.pc = cpu.suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn jr_nc_n_txt(cpu: &mut CPU) {
    let salto = cpu.suma_compl2_a_un_u16(cpu.pc.wrapping_add(2), cpu.r1);
    cpu.texto(&format!("JR NC #{:04X}", salto));
}

// 0x31
pub fn ld_sp_nn(cpu: &mut CPU) {
    bas_ld_Q_A(cpu);
//    cpu.sp = cpu.r1r2; // LD SP,d16
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_sp_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP#{:04X}", cpu.r1r2));
}

// 0x32 Difiere según procesador (LR35902->LDD  (HL),A)
pub fn lddOhlO_aGB(cpu: &mut CPU) {
    let mut hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);
    hl = cpu.dec_16bits(hl);

    cpu.escribe_hl(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn lddOhlO_aGB_txt(cpu: &mut CPU) { cpu.texto(&format!("LDD (HL),A")); }

// 0x32 Difiere según procesador (Z80->LD  (nn),A)
pub fn ldOnnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}

// 0x33
pub fn inc_sp(cpu: &mut CPU) {
    bas_inc_Q(cpu);
//    cpu.sp = cpu.inc_16bits(cpu.sp);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_sp_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC SP"));
}

// 0x34       inc (hl)
pub fn inc_OhlO(cpu: &mut CPU) {
    bas_inc_R(cpu);

//    let hl = cpu.lee_hl();
//    let valor = cpu.mem.lee_byte_de_mem(hl);
//
//
//    let resultado = valor.wrapping_add(1);
//    cpu.flag_v_u8_en_suma(valor, 1, resultado);
//    cpu.mem.escribe_byte_en_mem(hl, resultado);
//
//    cpu.reset_n_flag();
//    cpu.flag_s_u8(resultado);
//    cpu.flag_z_u8(resultado);
//    cpu.flag_h_u8(resultado);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC(HL)"));
}

// 0x35
pub fn dec_OhlO(cpu: &mut CPU) {
    bas_dec_R(cpu);
//    let hl = cpu.lee_hl();
//    let valor = cpu.mem.lee_byte_de_mem(hl);
//
//    let resultado = valor.wrapping_sub(1);
//    cpu.flag_v_u8_en_suma(valor, 1, resultado);
//    cpu.mem.escribe_byte_en_mem(hl, resultado);
//
//    cpu.set_n_flag();
//    cpu.flag_s_u8(resultado);
//    cpu.flag_z_u8(resultado);
//    cpu.flag_h_u8(resultado);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC(HL)"));
}

// 0x36 NN
pub fn ld_OhlO_n(cpu: &mut CPU) {
    let hl = cpu.lee_hl();

    cpu.mem.escribe_byte_en_mem(hl, cpu.r1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_OhlO_n_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL)#{:02X}", cpu.r1)); }

// 0x37
pub fn scf(cpu: &mut CPU) {
    cpu.set_c_flag();
    cpu.reset_h_flag();
    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn scf_txt(cpu: &mut CPU) {
    cpu.texto(&format!("SCF"));
}


// 0x38
pub fn jr_c_n(cpu: &mut CPU) {
    let salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());

    if cpu.get_c_flag() {
        cpu.pc = cpu.suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += cpu.get_t_instruccion() + 5;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn jr_c_n_txt(cpu: &mut CPU) {
    let mut salto = cpu.pc.wrapping_add(cpu.get_bytes_instruccion());

    salto = cpu.suma_compl2_a_un_u16(salto, cpu.r1);
    cpu.texto(&format!("JR C #{:04X}", salto));
}


// 0x39
pub fn add_hl_sp(cpu: &mut CPU) {
    bas_add_hl_Q(cpu);
}

pub fn add_hl_sp_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD HL,SP")); }

// 0x3A Difiere según procesador (LR35902->LDD A(HL))
pub fn ldd_a_OhlOGB(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn ldd_a_OhlOGB_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0x3A Difiere según procesador (Z80->LD a(NN))
pub fn ld_a_OnnO(cpu: &mut CPU) {
    cpu.a = cpu.mem.lee_byte_de_mem(cpu.r1r2);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_OnnO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A(#{:04X})", cpu.r1r2));
}

// 0x3B
pub fn dec_sp(cpu: &mut CPU) {
    bas_dec_Q(cpu);
}

pub fn dec_sp_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC SP"));
}

// 0x3C
pub fn inc_a(cpu: &mut CPU) {
    bas_inc_R(cpu);


//    cpu.a = cpu.inc_8bits(cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn inc_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("INC A"));
}

// 0x3D
pub fn dec_a(cpu: &mut CPU) {
    bas_dec_R(cpu);
//    cpu.a = cpu.dec_8bits(cpu.a);
////
////    cpu.t += cpu.get_t_instruccion();
////    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn dec_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DEC A"));
}

// 0x3E
pub fn ld_a_n(cpu: &mut CPU) {
    bas_ld_R_N(cpu);
//    cpu.a = cpu.r1;
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,#{:02X}", cpu.r1));
}

// 0x3F
pub fn ccf(cpu: &mut CPU) {
    if cpu.get_c_flag() {
        cpu.reset_c_flag();
    } else {
        cpu.set_c_flag();
    }

    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ccf_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CCF"));
}

// *************************** 4 ***********************************
// 0x40
pub fn ld_b_b(cpu: &mut CPU) {
    cpu.b = cpu.b;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,B"));
}

// 0x41
pub fn ld_b_c(cpu: &mut CPU) {
    cpu.b = cpu.c;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,C"));
}

// 0x42
pub fn ld_b_d(cpu: &mut CPU) {
    cpu.b = cpu.d;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,D"));
}

// 0x43
pub fn ld_b_e(cpu: &mut CPU) {
    cpu.b = cpu.e;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,E"));
}

// 0x44
pub fn ld_b_h(cpu: &mut CPU) {
    cpu.b = cpu.h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,H"));
}

// 0x45
pub fn ld_b_l(cpu: &mut CPU) {
    cpu.b = cpu.l;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,L"));
}

// 0x46
pub fn ld_b_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.b = cpu.mem.lee_byte_de_mem(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,(HL)"));
}


// 0x47
pub fn ld_b_a(cpu: &mut CPU) {
    cpu.b = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_b_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD B,A"));
}

// 0x48
pub fn ld_c_b(cpu: &mut CPU) {
    cpu.c = cpu.b;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,B"));
}

// 0x49
pub fn ld_c_c(cpu: &mut CPU) {
    cpu.c = cpu.c;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,C"));
}

// 0x4A
pub fn ld_c_d(cpu: &mut CPU) {
    cpu.c = cpu.d;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,D"));
}

// 0x4B
pub fn ld_c_e(cpu: &mut CPU) {
    cpu.c = cpu.e;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,E"));
}

// 0x4C
pub fn ld_c_h(cpu: &mut CPU) {
    cpu.c = cpu.h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,H"));
}

// 0x4D
pub fn ld_c_l(cpu: &mut CPU) {
    cpu.c = cpu.l;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,E"));
}

// 0x4E
pub fn ld_c_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.c = cpu.mem.lee_byte_de_mem(hl);

    cpu.t += 7;
    cpu.pc += 1;
}

pub fn ld_c_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,(HL)"));
}

// 0x4F
pub fn ld_c_a(cpu: &mut CPU) {
    cpu.c = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_c_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD C,A"));
}

// *************************** 5 ***********************************
// 0x50
pub fn ld_d_b(cpu: &mut CPU) {
    cpu.d = cpu.b;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,B")); }

// 0x51
pub fn ld_d_c(cpu: &mut CPU) {
    cpu.d = cpu.c;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,C")); }

// 0x52
pub fn ld_d_d(cpu: &mut CPU) {
    cpu.d = cpu.d;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,D")); }

// 0x53
pub fn ld_d_e(cpu: &mut CPU) {
    cpu.d = cpu.e;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,E")); }

// 0x54
pub fn ld_d_h(cpu: &mut CPU) {
    cpu.d = cpu.h;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,H")); }

// 0x55
pub fn ld_d_l(cpu: &mut CPU) {
    cpu.d = cpu.l;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,L")); }

// 0x56
pub fn ld_d_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.d = cpu.mem.lee_byte_de_mem(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D(HL)")); }

// 0x57
pub fn ld_d_a(cpu: &mut CPU) {
    cpu.d = cpu.a;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_d_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD D,A")); }

// 0x58
pub fn ld_e_b(cpu: &mut CPU) {
    cpu.e = cpu.b;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,B")); }

// 0x59
pub fn ld_e_c(cpu: &mut CPU) {
    cpu.e = cpu.c;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,C")); }

// 0x5A
pub fn ld_e_d(cpu: &mut CPU) {
    cpu.e = cpu.d;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,D")); }

// 0x5B
pub fn ld_e_e(cpu: &mut CPU) {
    cpu.e = cpu.e;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,E")); }

// 0x5C
pub fn ld_e_h(cpu: &mut CPU) {
    cpu.e = cpu.h;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_e_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,H")); }

// 0x5D
pub fn ld_e_l(cpu: &mut CPU) {
    cpu.e = cpu.l;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,L")); }

// 0x5E
pub fn ld_e_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.e = cpu.mem.lee_byte_de_mem(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,(HL)")); }

// 0x5F
pub fn ld_e_a(cpu: &mut CPU) {
    cpu.e = cpu.a;

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_e_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD E,A")); }

// *************************** 6 ***********************************
// 0x60
pub fn ld_h_b(cpu: &mut CPU) {
    cpu.h = cpu.b;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,B"));
}

// 0x61
pub fn ld_h_c(cpu: &mut CPU) {
    cpu.h = cpu.c;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,C"));
}

// 0x62
pub fn ld_h_d(cpu: &mut CPU) {
    cpu.h = cpu.d;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,D"));
}

// 0x63
pub fn ld_h_e(cpu: &mut CPU) {
    cpu.h = cpu.e;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,E"));
}


// 0x64
pub fn ld_h_h(cpu: &mut CPU) {
    cpu.h = cpu.h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,H"));
}

// 0x65
pub fn ld_h_l(cpu: &mut CPU) {
    cpu.h = cpu.l;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,L"));
}

// 0x66
pub fn ld_h_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.h = cpu.mem.lee_byte_de_mem(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,(HL)"));
}

// 0x67
pub fn ld_h_a(cpu: &mut CPU) {
    cpu.h = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_h_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD H,A"));
}

// 0x68
pub fn ld_l_b(cpu: &mut CPU) {
    cpu.l = cpu.b;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,B"));
}

// 0x69
pub fn ld_l_c(cpu: &mut CPU) {
    cpu.l = cpu.c;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,C"));
}

// 0x6A
pub fn ld_l_d(cpu: &mut CPU) {
    cpu.l = cpu.d;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,D"));
}

// 0x6B
pub fn ld_l_e(cpu: &mut CPU) {
    cpu.l = cpu.e;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,E"));
}

// 0x6C
pub fn ld_l_h(cpu: &mut CPU) {
    cpu.l = cpu.h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,H"));
}

// 0x6D
pub fn ld_l_l(cpu: &mut CPU) {
    cpu.l = cpu.l;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,L"));
}

// 0x6E
pub fn ld_l_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.l = cpu.mem.lee_byte_de_mem(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,(HL)"));
}

// 0x6F
pub fn ld_l_a(cpu: &mut CPU) {
    cpu.l = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_l_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD L,A"));
}

// *************************** 7 ***********************************
// 0x70
pub fn ldOhlO_b(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.b);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_b_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),B")); }

// 0x71
pub fn ldOhlO_c(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.c);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_c_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),C")); }

// 0x72
pub fn ldOhlO_d(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.d);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_d_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),D")); }

// 0x73
pub fn ldOhlO_e(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.e);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_e_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),E")); }

// 0x74
pub fn ldOhlO_h(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.h);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_h_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),H")); }

// 0x75
pub fn ldOhlO_l(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.l);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_l_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),L")); }


// 0x77
pub fn ldOhlO_a(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOhlO_a_txt(cpu: &mut CPU) { cpu.texto(&format!("LD(HL),A")); }

// 0x78
pub fn ld_a_b(cpu: &mut CPU) {
    cpu.a = cpu.b;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,B"));
}

// 0x79
pub fn ld_a_c(cpu: &mut CPU) {
    cpu.a = cpu.c;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,C"));
}

// 0x7A
pub fn ld_a_d(cpu: &mut CPU) {
    cpu.a = cpu.d;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,D"));
}

// 0x7B
pub fn ld_a_e(cpu: &mut CPU) {
    cpu.a = cpu.e;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,E"));
}

// 0x7C
pub fn ld_a_h(cpu: &mut CPU) {
    cpu.a = cpu.h;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,H"));
}

// 0x7D
pub fn ld_a_l(cpu: &mut CPU) {
    cpu.a = cpu.l;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,L"));
}

// 0x7E
pub fn ld_a_OhlO(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.a = cpu.mem.lee_byte_de_mem(hl);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_OhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,(HL)"));
}

// 0x7F
pub fn ld_a_a(cpu: &mut CPU) {
    cpu.a = cpu.a;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,A"));
}

// *************************** 8 ***********************************
//0x80
pub fn add_a_b(cpu: &mut CPU) {
    bas_add_a_R(cpu);
}

pub fn add_a_b_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,B")); }

//0x81
pub fn add_a_c(cpu: &mut CPU) { bas_add_a_R(cpu); }


pub fn add_a_c_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,C")); }

//0x82
pub fn add_a_d(cpu: &mut CPU) { bas_add_a_R(cpu); }


pub fn add_a_d_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,D")); }

//0x83
pub fn add_a_e(cpu: &mut CPU) {
    bas_add_a_R(cpu);
}


pub fn add_a_e_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,E")); }

//0x84
pub fn add_a_h(cpu: &mut CPU) { bas_add_a_R(cpu); }


pub fn add_a_h_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,H")); }

//0x85
pub fn add_a_l(cpu: &mut CPU) { bas_add_a_R(cpu); }


pub fn add_a_l_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,L")); }

//0x86
pub fn add_a_OhlO(cpu: &mut CPU) { bas_add_a_R(cpu); }


pub fn add_a_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,(HL)")); }

//0x87
pub fn add_a_a(cpu: &mut CPU) {
    bas_add_a_R(cpu);
//    cpu.a = cpu.suma_u8_mas_u8(cpu.a, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}


pub fn add_a_a_txt(cpu: &mut CPU) { cpu.texto(&format!("ADD A,A")); }

//0x88
pub fn adc_a_b(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_b_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x89
pub fn adc_a_c(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_c_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8A
pub fn adc_a_d(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_d_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8B
pub fn adc_a_e(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_e_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8C
pub fn adc_a_h(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_h_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8D
pub fn adc_a_l(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_l_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8E
pub fn adc_a_OhlO(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_OhlO_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

//0x8F
pub fn adc_a_a(cpu: &mut CPU) { fn_no_impl(cpu); }


pub fn adc_a_a_txt(cpu: &mut CPU) { fn_no_impl(cpu); }


// *************************** 9 ***********************************
// 0x90
pub fn sub_b(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.b);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_b_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB B")); }

// 0x91
pub fn sub_c(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.c);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_c_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB C")); }

// 0x92
pub fn sub_d(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.d);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_d_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB D")); }

// 0x93
pub fn sub_e(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.e);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_e_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB E")); }

// 0x94
pub fn sub_h(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.h);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_h_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB H")); }

// 0x95
pub fn sub_l(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.l);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_l_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB L")); }

// 0x96
pub fn subOhlO(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    let hl = cpu.lee_hl();
//    let dato = cpu.mem.lee_byte_de_mem(hl);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, dato);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn subOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(HL)")); }

// 0x97
pub fn sub_a(cpu: &mut CPU) {
    bas_sub_R(cpu);
//    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_a_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB A")); }

// *************************** A ***********************************
// 0xA0
pub fn and_b(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_b_txt(cpu: &mut CPU) { cpu.texto(&format!("AND B")); }

// 0xA1
pub fn and_c(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_c_txt(cpu: &mut CPU) { cpu.texto(&format!("AND C")); }

// 0xA2
pub fn and_d(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_d_txt(cpu: &mut CPU) { cpu.texto(&format!("AND D")); }

// 0xA3
pub fn and_e(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_e_txt(cpu: &mut CPU) { cpu.texto(&format!("AND E")); }

// 0xA4
pub fn and_h(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_h_txt(cpu: &mut CPU) { cpu.texto(&format!("AND H")); }

// 0xA5
pub fn and_l(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_l_txt(cpu: &mut CPU) { cpu.texto(&format!("AND L")); }

// 0xA6
pub fn and_OhlO(cpu: &mut CPU) { bas_and_R(cpu); }

pub fn and_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("AND(HL)")); }

// 0xA7
pub fn and_a(cpu: &mut CPU) {
    bas_and_R(cpu);
//    cpu.a = cpu.and_u8_con_u8(cpu.a, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn and_a_txt(cpu: &mut CPU) { cpu.texto(&format!("AND A")); }

// 0xA8
pub fn xor_b(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_b_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR B")); }

// 0xA9
pub fn xor_c(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_c_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR C")); }

// 0xAA
pub fn xor_d(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_d_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR D")); }

// 0xAB
pub fn xor_e(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_e_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR E")); }

// 0xAC
pub fn xor_h(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_h_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR H")); }

// 0xAD
pub fn xor_l(cpu: &mut CPU) { bas_xor_R(cpu); }

pub fn xor_l_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR L")); }

// 0xAE
pub fn xor_OhlO(cpu: &mut CPU) {
    bas_xor_R(cpu);
//    let hl = cpu.lee_hl();
//    let dato = cpu.mem.lee_byte_de_mem(hl);
//    cpu.a = cpu.xor_u8_con_u8(cpu.a, dato);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn xor_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR(HL)")); }

// 0xAF Xor consigo mismo pone a 0 y modifica flags
pub fn xor_a(cpu: &mut CPU) {
    bas_xor_R(cpu);
//    cpu.a = cpu.xor_u8_con_u8(cpu.a, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn xor_a_txt(cpu: &mut CPU) { cpu.texto(&format!("XOR A")); }

// *************************** B ***********************************
// 0xB0
pub fn or_b(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_b_txt(cpu: &mut CPU) { cpu.texto(&format!("OR B")); }

// 0xB1
pub fn or_c(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_c_txt(cpu: &mut CPU) { cpu.texto(&format!("OR C")); }

// 0xB2
pub fn or_d(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_d_txt(cpu: &mut CPU) { cpu.texto(&format!("OR D")); }

// 0xB3
pub fn or_e(cpu: &mut CPU) {
    bas_or_R(cpu);
//    cpu.a = cpu.or_u8_con_u8(cpu.a, cpu.e);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn or_e_txt(cpu: &mut CPU) { cpu.texto(&format!("OR E")); }

// 0xB4
pub fn or_h(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_h_txt(cpu: &mut CPU) { cpu.texto(&format!("OR H")); }

// 0xB5
pub fn or_l(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_l_txt(cpu: &mut CPU) { cpu.texto(&format!("OR L")); }

// 0xB6
pub fn or_OhlO(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_OhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("OR(HL)")); }

// 0xB7
pub fn or_a(cpu: &mut CPU) { bas_or_R(cpu); }

pub fn or_a_txt(cpu: &mut CPU) { cpu.texto(&format!("OR A")); }

// 0xB8
pub fn cp_b(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.b);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP B"));
}

// 0xB9
pub fn cp_c(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.c);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP C"));
}

// 0xBA
pub fn cp_d(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.d);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP D"));
}

// 0xBB
pub fn cp_e(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.e);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_e_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP E"));
}

// 0xBC
pub fn cp_h(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.h);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP H"));
}

// 0xBD
pub fn cp_l(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.l);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP L"));
}

// 0xBE
pub fn cpOhlO(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let hl = cpu.lee_hl();
//    let dato = cpu.mem.lee_byte_de_mem(hl);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, dato);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cpOhlO_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP (HL)"));
}

// 0xBF
pub fn cp_a(cpu: &mut CPU) {
    bas_cp_R(cpu);
//    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.a);
//
//    cpu.t += cpu.get_t_instruccion();
//    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP A"));
}


// *************************** C ***********************************
// 0xC0
pub fn ret_nz(cpu: &mut CPU) {
    if !cpu.get_z_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn ret_nz_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RET NZ"));
}

// 0xC1
pub fn pop_bc(cpu: &mut CPU) {
    let bc: u16 = cpu.pop();
    cpu.escribe_bc(bc);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn pop_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("POP BC")); }

// 0xC3 NN NN
pub fn jp_nn(cpu: &mut CPU) {
    cpu.t += cpu.get_t_instruccion();
    cpu.pc = cpu.r1r2;
}

pub fn jp_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("JP #{:04X}", cpu.r1r2)); }

// 0xC5
pub fn push_bc(cpu: &mut CPU) {
    let bc = cpu.lee_bc();
    cpu.push(bc);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_bc_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH BC")); }

// 0xC6
pub fn add_a_n(cpu: &mut CPU) {
    let kk = cpu.a.wrapping_add(cpu.r1);
    cpu.flag_v_u8_en_suma(cpu.a, cpu.r1, kk);
    cpu.flag_c_u8_en_suma(cpu.a, cpu.r1);
    cpu.flag_h_u8_en_suma(cpu.a, cpu.r1);
    cpu.a = kk;

    cpu.reset_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);
    //cpu.flag_h_u8(kk);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn add_a_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("ADD A,#{:02X}", cpu.r1));
}

// 0xC7
pub fn rst_00(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0000;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_00_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RST #0000"));
}

// 0xC8
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


// 0xC9
pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.pop();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ret_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RET"));
}

// 0xCB   -----EXTENSION--------------------------------------------------------
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

// 0xCC
pub fn call_z_nn(cpu: &mut CPU) {
    if cpu.get_z_flag() {
        cpu.pc = cpu.r1r2;
        cpu.t += cpu.get_t_instruccion() + 7;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
        cpu.t += cpu.get_t_instruccion();
    }
}

pub fn call_z_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("CALL Z #{:04X}", cpu.r1r2)); }

// 0xCD
pub fn call_nn(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = cpu.r1r2;

    cpu.t += cpu.get_t_instruccion();
}

pub fn call_nn_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CALL #{:04X}", cpu.r1r2));
}


pub fn rst_08(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0008;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_08_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RST 08"));
}

// *************************** D ***********************************
// 0xD0
pub fn ret_nc(cpu: &mut CPU) {
    if !cpu.get_c_flag() {
        cpu.pc = cpu.pop();
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
    }

    cpu.t += cpu.get_t_instruccion();
}

pub fn ret_nc_txt(cpu: &mut CPU) { cpu.texto(&format!("RET NC")); }

// 0xD2
pub fn jp_nc_nn(cpu: &mut CPU) {
    if !cpu.get_c_flag() {
        cpu.pc = cpu.r1r2;
    } else {
        cpu.pc += cpu.get_bytes_instruccion();
    }

    cpu.t += cpu.get_t_instruccion();
}

pub fn jp_nc_nn_txt(cpu: &mut CPU) { cpu.texto(&format!("JP NC#{:04X}", cpu.r1r2)); }

// 0xD3
pub fn out_OnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_port(cpu.r1, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn out_OnO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("OUT(#{:02X}),A", cpu.r1));
}

// 0xD5
pub fn push_de(cpu: &mut CPU) {
    let de = cpu.lee_de();
    cpu.push(de);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_de_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH DE")); }

// 0xD6
pub fn sub_n(cpu: &mut CPU) {
    let kk = cpu.a.wrapping_sub(cpu.r1);
    cpu.flag_v_u8_en_resta(cpu.a, cpu.r1, kk);
    cpu.flag_c_u8_en_resta(cpu.a, cpu.r1);
    cpu.flag_h_u8_en_resta(cpu.a, cpu.r1);
    cpu.a = kk;

    cpu.set_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn sub_n_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(#{:02X})", cpu.r1)); }

// 0xD7
pub fn rst_10(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0010;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_10_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RST #0010"));
}

// 0xD8
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

// 0xD9 Difiere según procesador (LR35902->RETI)
pub fn retiGB(cpu: &mut CPU) { fn_no_impl(cpu); }

pub fn retiGB_txt(cpu: &mut CPU) { fn_no_impl(cpu); }

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

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn exx_txt(cpu: &mut CPU) { cpu.texto(&format!("EXX")); }

// *************************** E ***********************************
// 0xE0 Difiere según procesador (LR35902->LD(#FF00+N),A)
pub fn ldOff00_m_nO_aGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.r1 as u16);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOff00_m_nO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD ($FF00+#{:02X}),A", cpu.r1));
}

// 0xE0 Difiere según procesador (Z80->RET NV)
pub fn ret_po(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn ret_po_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0xE1
pub fn pop_hl(cpu: &mut CPU) {
    let hl = cpu.pop();
    cpu.escribe_hl(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn pop_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("POP HL")); }

// 0xE2 Difiere según procesador (LR35902->LD(#FF00+C),A)
pub fn ldOff00_m_cO_aGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.c as u16);
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOff00_m_cO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD ($FF00+C),A"));
}

// 0xE2 Difiere según procesador (Z80->JP PO,NN)
pub fn jp_po_nn(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn jp_po_nn_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0xE3
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

pub fn exOspO_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("EX(SP),HL"));
}

// 0xE5
pub fn push_hl(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.push(hl);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn push_hl_txt(cpu: &mut CPU) { cpu.texto(&format!("PUSH HL")); }


// 0xE6
pub fn and_n(cpu: &mut CPU) {
    cpu.a = cpu.a & cpu.r1;

    cpu.flag_s_u8(cpu.a);
    cpu.flag_z_u8(cpu.a);
    cpu.flag_p_u8(cpu.a); // Como paridad
    cpu.set_h_flag();

    cpu.reset_c_flag();
    cpu.reset_n_flag();

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn and_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("AND #{:02X}", cpu.r1));
}

// 0xE7
pub fn rst_20(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0020;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_20_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RST #0020"));
}

// 0xE9
pub fn jpOhlO(cpu: &mut CPU) {
    cpu.pc = cpu.lee_hl();

    cpu.t += cpu.get_t_instruccion();
}

pub fn jpOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("JP(HL)")); }

// 0xEA  Difiere según procesador (LR35902->LD (nn),A)
pub fn ldOnnO_aGB(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ldOnnO_aGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}

// 0xEA  Difiere según procesador (Z80->JP  V,nn)
pub fn jp_pe_nn(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

pub fn jp_pe_nn_txt(cpu: &mut CPU) {
    fn_no_impl(cpu);
}

// 0xEB
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

pub fn ex_de_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("EX DE,HL"));
}

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

// *************************** F ***********************************
// 0xF0 Difiere según procesador (LR35902->LDH  A,(n))
pub fn ld_a_Off00_m_nOGB(cpu: &mut CPU) {
    let direccion: u16 = 0xFF00 + (cpu.r1 as u16);
    cpu.a = cpu.mem.lee_byte_de_mem(direccion);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_a_Off00_m_nOGB_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,($FF00+#{:02X})", cpu.r1));
}

// 0xF0 Difiere según procesador (Z80->RET P)
pub fn ret_p(cpu: &mut CPU) {
    if cpu.get_pv_flag() {
        cpu.pc = cpu.pop();
        cpu.t += cpu.get_t_instruccion() + 6;
    } else {
        cpu.t += cpu.get_t_instruccion();
        cpu.pc += cpu.get_bytes_instruccion();
    }
}

pub fn ret_p_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RET P"));
}

// 0xF1
pub fn pop_af(cpu: &mut CPU) {
    let af = cpu.pop();
    cpu.escribe_af(af);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn pop_af_txt(cpu: &mut CPU) {
    cpu.texto(&format!("POP AF"));
}

// 0xF3
pub fn di(cpu: &mut CPU) {
    cpu.permitida_interrupcion = false;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn di_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DI"));
}

// 0xF5
pub fn push_af(cpu: &mut CPU) {
    let af = cpu.lee_af();
    cpu.push(af);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn push_af_txt(cpu: &mut CPU) {
    cpu.texto(&format!("PUSH AF"));
}

// 0xF6
pub fn or_n(cpu: &mut CPU) {
    cpu.a = cpu.a | cpu.r1;

    cpu.flag_s_u8(cpu.a);
    cpu.flag_z_u8(cpu.a);
    cpu.flag_p_u8(cpu.a); // Como paridad
    cpu.reset_h_flag();

    cpu.reset_c_flag();
    cpu.reset_n_flag();

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn or_n_txt(cpu: &mut CPU) { cpu.texto(&format!("OR #{:02X}", cpu.r1)); }

// 0xF7
pub fn rst_30(cpu: &mut CPU) {
    let salto_al_volver = cpu.pc + cpu.get_bytes_instruccion();
    cpu.push(salto_al_volver);
    cpu.pc = 0x0030;

    cpu.t += cpu.get_t_instruccion();
}

pub fn rst_30_txt(cpu: &mut CPU) {
    cpu.texto(&format!("RST #0030"));
}

// 0xF9
pub fn ld_sp_hl(cpu: &mut CPU) {
    let hl = cpu.lee_hl();
    cpu.sp = hl;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ld_sp_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP,HL"));
}

// 0xFB
pub fn ei(cpu: &mut CPU) {
    cpu.permitida_interrupcion = true;

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn ei_txt(cpu: &mut CPU) { cpu.texto(&format!("EI")); }

// 0xFE
pub fn cp_n(cpu: &mut CPU) {
    let kk = cpu.a.wrapping_sub(cpu.r1);

    cpu.flag_v_u8_en_resta(cpu.a, cpu.r1, kk);
    cpu.flag_c_u8_en_resta(cpu.a, cpu.r1);
    cpu.flag_h_u8_en_resta(cpu.a, cpu.r1);

    cpu.set_n_flag();
    cpu.flag_s_u8(kk);
    cpu.flag_z_u8(kk);


    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn cp_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("CP #{:02X}", cpu.r1));
}


// 0xFD   -----EXTENSION--------------------------------------------------------
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

