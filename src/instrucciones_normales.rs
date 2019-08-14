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
use crate::cpu::{CPU, PROCESADOR, Funcion};


pub fn mete_valores_normales() -> [[i16; 2]; 256] {
    // bytes, time   datos sacados de fichero:
    // https://github.com/malandrin/gbe/blob/master/gbe/opcodes_info.cpp
    [
        // 0-8     1-9     2-A    3-B     4-C     5-D      6-E     7-F
        [1, 4], [3, 12], [1, 8], [1, 8], [1, 4], [1, 4], [2, 8], [1, 4],//0
        [3, 20], [1, 8], [1, 8], [1, 8], [1, 4], [1, 4], [2, 8], [1, 41],//0

        [1308, 2], [10, 3], [7, 1], [6, 1], [4, 1], [4, 1], [7, 2], [4, 1],//1
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//1

        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//2
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//2

        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//3
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//3

        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//4
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//4
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//5
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//5
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//6
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//6
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//7
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//7
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//8
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//8
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//9
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//9
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//A
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//A
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//B
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//B
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//C
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//C
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//D
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//D
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//E
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//E
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//F
        [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0],//F
    ]
}


pub fn mete_funciones_normales(cpu: &mut CPU) {
    // *************************** 0 ***********************************
    cpu.funciones[0x00 as usize].set_punt_y_val_a_fn(nop, nop_txt, 1, 4);
    cpu.funciones[0x01 as usize].set_punt_y_val_a_fn(ld_bc_nn, ld_bc_nn_txt, 3, 12);
    cpu.funciones[0x02 as usize].set_punt_y_val_a_fn(ldObcO_a, ldObcO_a_txt, 1, 8);
    cpu.funciones[0x03 as usize].set_punt_y_val_a_fn(inc_bc, inc_bc_txt, 1, 8);
    cpu.funciones[0x04 as usize].set_punt_y_val_a_fn(inc_b, inc_b_txt, 1, 4);
    cpu.funciones[0x05 as usize].set_punt_y_val_a_fn(dec_b, dec_b_txt, 1, 4);
    cpu.funciones[0x06 as usize].set_punt_y_val_a_fn(ld_b_n, ld_b_n_txt, 2, 8);
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
    cpu.funciones[0x09 as usize].set_punt_y_val_a_fn(add_hl_bc, add_hl_bc_txt, 1, 8);
    cpu.funciones[0x0A as usize].set_punt_y_val_a_fn(ld_aObcO, ld_aObcO_txt, 1, 8);
    cpu.funciones[0x0B as usize].set_punt_y_val_a_fn(dec_bc, dec_bc_txt, 1, 8);
    cpu.funciones[0x0C as usize].set_punt_y_val_a_fn(inc_c, inc_c_txt, 1, 4);
    cpu.funciones[0x0D as usize].set_punt_y_val_a_fn(dec_c, dec_c_txt, 1, 4);
    cpu.funciones[0x0E as usize].set_punt_y_val_a_fn(ld_c_n, ld_c_n_txt, 2, 8);
    cpu.funciones[0x0F as usize].set_punt_y_val_a_fn(rrca, rrca_txt, 1, 4);

    // *************************** 1 ***********************************
    // LR35902-> STOP               Z80->DJNZ
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x10 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 4);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x10 as usize].set_punt_y_val_a_fn(djnz_n, djnz_n_txt, 2, 10);
        }
    };

    cpu.funciones[0x11 as usize].set_punt_y_val_a_fn(ld_de_nn, ld_de_nn_txt, 3, 12);
    cpu.funciones[0x12 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x13 as usize].set_punt_y_val_a_fn(inc_de, inc_de_txt, 1, 8);
    cpu.funciones[0x14 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0x15 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x16 as usize].set_punt_y_val_a_fn(ld_d_n, ld_d_n_txt, 2, 8);
    cpu.funciones[0x17 as usize].set_punt_y_val_a_fn(rla, rla_txt, 1, 4);
    cpu.funciones[0x18 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 12);
    cpu.funciones[0x19 as usize].set_punt_y_val_a_fn(add_hl_de, add_hl_de_txt, 1, 8);
    cpu.funciones[0x1A as usize].set_punt_y_val_a_fn(ld_aOdeO, ld_aOdeO_txt, 1, 8);
    cpu.funciones[0x1B as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x1C as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x1D as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x1E as usize].set_punt_y_val_a_fn(ld_d_n, ld_d_n_txt, 2, 8);
    cpu.funciones[0x1F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);


    // *************************** 2 ***********************************
    cpu.funciones[0x20 as usize].set_punt_y_val_a_fn(jr_nz_n, jr_nz_n_txt, 2, 8);
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
    cpu.funciones[0x23 as usize].set_punt_y_val_a_fn(inc_hl, inc_hl_txt, 1, 8);
    cpu.funciones[0x24 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x25 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0x26 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 8);
    cpu.funciones[0x27 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x28 as usize].set_punt_y_val_a_fn(jr_z_n, jr_z_n_txt, 2, 8);
    cpu.funciones[0x29 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);

    // LR35902->LDI  A,(HL)        Z-80->LD  HL,(nn)
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x2A as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x2A as usize].set_punt_y_val_a_fn(ld_hlOnnO, ld_hlOnnO_txt, 1, 8);
        }
    };

    cpu.funciones[0x2B as usize].set_punt_y_val_a_fn(dec_hl, dec_hl_txt, 1, 8);
    cpu.funciones[0x2C as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x2D as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x2E as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 8);
    cpu.funciones[0x2F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);

    // *************************** 3 ***********************************

    cpu.funciones[0x30 as usize].set_punt_y_val_a_fn(jr_nc_n, jr_nc_n_txt, 2, 10);

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

    cpu.funciones[0x33 as usize].set_punt_y_val_a_fn(inc_sp, inc_sp_txt, 1, 8);
    cpu.funciones[0x34 as usize].set_punt_y_val_a_fn(inc_OhlO, inc_OhlO_txt, 1, 12);
    cpu.funciones[0x35 as usize].set_punt_y_val_a_fn(dec_OhlO, dec_OhlO_txt, 1, 12);
    cpu.funciones[0x36 as usize].set_punt_y_val_a_fn(ld_OhlO_n, ld_OhlO_n_txt, 2, 12);
    cpu.funciones[0x37 as usize].set_punt_y_val_a_fn(scf, scf_txt, 1, 4);
    cpu.funciones[0x38 as usize].set_punt_y_val_a_fn(jr_c_n, jr_c_n_txt, 1, 8);
    cpu.funciones[0x39 as usize].set_punt_y_val_a_fn(add_hl_sp, add_hl_sp_txt, 1, 0);

    // LR35902->LDD  A,(HL)     Z-80->LD  A,(nn)
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0x3A as usize].set_punt_y_val_a_fn(ldd_a_OhlOGB, ldd_a_OhlOGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0x3A as usize].set_punt_y_val_a_fn(ld_a_OnnO, ld_a_OnnO_txt, 3, 13);
        }
    };

    cpu.funciones[0x3B as usize].set_punt_y_val_a_fn(dec_sp, dec_sp_txt, 1, 0);
    cpu.funciones[0x3C as usize].set_punt_y_val_a_fn(inc_a, inc_a_txt, 1, 4);
    cpu.funciones[0x3D as usize].set_punt_y_val_a_fn(dec_a, dec_a_txt, 1, 4);
    cpu.funciones[0x3E as usize].set_punt_y_val_a_fn(ld_a_n, ld_a_n_txt, 2, 8);
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
    cpu.funciones[0x70 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x71 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x72 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x73 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x74 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x75 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x76 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x77 as usize].set_punt_y_val_a_fn(ldOhlO_a, ldOhlO_a_txt, 1, 8);
    cpu.funciones[0x78 as usize].set_punt_y_val_a_fn(ld_a_b, ld_a_b_txt, 1, 4);
    cpu.funciones[0x79 as usize].set_punt_y_val_a_fn(ld_a_c, ld_a_c_txt, 1, 4);
    cpu.funciones[0x7A as usize].set_punt_y_val_a_fn(ld_a_d, ld_a_d_txt, 1, 4);
    cpu.funciones[0x7B as usize].set_punt_y_val_a_fn(ld_a_e, ld_a_e_txt, 1, 4);
    cpu.funciones[0x7C as usize].set_punt_y_val_a_fn(ld_a_h, ld_a_h_txt, 1, 4);
    cpu.funciones[0x7D as usize].set_punt_y_val_a_fn(ld_a_l, ld_a_l_txt, 1, 4);
    cpu.funciones[0x7E as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x7F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);

    // *************************** 8 ***********************************
    cpu.funciones[0x80 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x81 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x82 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x83 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x84 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x85 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x86 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x87 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x88 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x89 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x8A as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x8B as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x8C as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x8D as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0x8E as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0x8F as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
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
    cpu.funciones[0xB0 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB1 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB3 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB5 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB6 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xB7 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xB9 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xBA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xBB as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xBC as usize].set_punt_y_val_a_fn(cp_h, cp_h_txt, 1, 4);
    cpu.funciones[0xBD as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
    cpu.funciones[0xBE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xBF as usize].set_punt_y_val_a_fn(cp_h, cp_h_txt, 1, 4);

    // *************************** C ***********************************
    cpu.funciones[0xC0 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xC1 as usize].set_punt_y_val_a_fn(pop_bc, pop_bc_txt, 1, 12);
    cpu.funciones[0xC2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xC3 as usize].set_punt_y_val_a_fn(jp_nn, jp_nn_txt, 3, 16);
    cpu.funciones[0xC4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xC5 as usize].set_punt_y_val_a_fn(push_bc, push_bc_txt, 1, 16);
    cpu.funciones[0xC6 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 8);
    cpu.funciones[0xC7 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    cpu.funciones[0xC8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xC9 as usize].set_punt_y_val_a_fn(ret, ret_txt, 1, 16);
    cpu.funciones[0xCA as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xCB as usize].set_punt_y_val_a_fn(CB, CB_txt, 0, 0); // Extensión
    cpu.funciones[0xCC as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xCD as usize].set_punt_y_val_a_fn(call_nn, call_nn_txt, 3, 14);
    cpu.funciones[0xCE as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xCF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    // *************************** D ***********************************
    cpu.funciones[0xD0 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);
    cpu.funciones[0xD1 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 12);
    cpu.funciones[0xD2 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xD3 as usize].set_punt_y_val_a_fn(out_OnO_a, out_OnO_a_txt, 1, 0);
    cpu.funciones[0xD4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 3, 12);
    cpu.funciones[0xD5 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    cpu.funciones[0xD6 as usize].set_punt_y_val_a_fn(sub_n, sub_n_txt, 2, 8);
    cpu.funciones[0xD7 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    cpu.funciones[0xD8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 8);

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
    cpu.funciones[0xE1 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 12);
    // LR35902->LD(C),A         Z-80->JP NV,nn
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE2 as usize].set_punt_y_val_a_fn(ldOff00_m_cO_aGB, ldOff00_m_cO_aGB_txt, 1, 8);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE2 as usize].set_punt_y_val_a_fn(jp_po_nn, jp_po_nn_txt, 3, 10);
        }
    }
    cpu.funciones[0xE3 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xE4 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 0);
    cpu.funciones[0xE5 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    cpu.funciones[0xE6 as usize].set_punt_y_val_a_fn(and_n, and_n_txt, 2, 8);
    cpu.funciones[0xE7 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    // LR35902->ADD  SP,e          Z80->RET V
    match cpu.procesador {
        PROCESADOR::SharpLr35902 => {
            cpu.funciones[0xE8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 16);
        }
        PROCESADOR::Z80 => {
            cpu.funciones[0xE8 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 2, 16);
        }
    }

    cpu.funciones[0xE9 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 4);
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
            cpu.funciones[0xF0 as usize].set_punt_y_val_a_fn(ret_p, ret_p_txt, 1, 10);
        }
    }
    cpu.funciones[0xF1 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 12);

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
    cpu.funciones[0xF5 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
    cpu.funciones[0xF6 as usize].set_punt_y_val_a_fn(or_n, or_n_txt, 2, 8);
    cpu.funciones[0xF7 as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);

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
    cpu.funciones[0xFE as usize].set_punt_y_val_a_fn(cp_n, cp_n_txt, 2, 8);
    cpu.funciones[0xFF as usize].set_punt_y_val_a_fn(fn_no_impl, fn_no_impl, 1, 16);
}

pub fn fn_no_impl(cpu: &mut CPU) {
    panic!(format!("Funcion normal no implementada\n\
    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
    r3 = #{:02X}\n",
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
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.b, cpu.c);
    let direccion = cpu.lee_bc();
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
pub fn djnz_n(cpu: &mut CPU) {
    cpu.pc += cpu.get_bytes_instruccion();
    cpu.b = cpu.dec_8bits(cpu.b);
    if cpu.b != 0 {
        cpu.pc = cpu.suma_compl2_a_un_u16(cpu.pc, cpu.r1);
    }

    cpu.t += cpu.get_t_instruccion();
}

pub fn djnz_n_txt(cpu: &mut CPU) { cpu.texto(&format!("DJNZ")); }

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
    //let mut de = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    let de = cpu.lee_de();

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
    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
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

    let hl16 = cpu.lee_hl();
    let de16 = cpu.lee_de();

    let hl16 = hl16.wrapping_add(de16);

    cpu.escribe_hl(hl16);

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
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.d, cpu.e);
    let direccion = cpu.lee_de();
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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();
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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();

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
        cpu.pc = cpu.suma_compl2_a_un_u16(salto, cpu.r1);
        cpu.t += 12;
    } else {
        cpu.pc += 2;
        cpu.t += 7;
    }
}

pub fn jr_z_n_txt(cpu: &mut CPU) {
    let salto = cpu.suma_compl2_a_un_u16(cpu.pc.wrapping_add(2), cpu.r1);
    cpu.texto(&format!("JR Z(#{:04X})", salto));
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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();
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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();
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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();

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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();

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
    //let hl16 = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl16 = cpu.lee_hl();

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
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let direccion = cpu.lee_hl();
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
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let direccion = cpu.lee_hl();
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
    let direccion = cpu.lee_hl();
    let dato = cpu.mem.lee_byte_de_mem(direccion);
    cpu.d = dato;

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
    let direccion = cpu.lee_hl();
    let dato = cpu.mem.lee_byte_de_mem(direccion);

    cpu.e = dato;

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
/*

cpu.funciones[0x60 as usize].set_punt_y_val_a_fn(ld_h_b, ld_h_b_txt, 1, 4);
    cpu.funciones[0x61 as usize].set_punt_y_val_a_fn(ld_h_c, ld_h_c_txt, 1, 4);
    cpu.funciones[0x62 as usize].set_punt_y_val_a_fn(ld_h_d, ld_h_d_txt, 1, 4);
    cpu.funciones[0x63 as usize].set_punt_y_val_a_fn(ld_h_e, ld_h_e_txt, 1, 4);
    cpu.funciones[0x64 as usize].set_punt_y_val_a_fn(ld_h_h, ld_h_h_txt, 1, 4);
    cpu.funciones[0x65 as usize].set_punt_y_val_a_fn(ld_h_l, ld_h_l_txt, 1, 4);
    cpu.funciones[0x66 as usize].set_punt_y_val_a_fn(ld_hOhlO, ld_hOhlO_txt, 1, 8);
    cpu.funciones[0x67 as usize].set_punt_y_val_a_fn(ld_h_a, ld_h_a_txt, 1, 4);
    cpu.funciones[0x68 as usize].set_punt_y_val_a_fn(ld_l_b, ld_l_b_txt, 1, 4);
    cpu.funciones[0x69 as usize].set_punt_y_val_a_fn(ld_l_c, ld_l_c_txt, 1, 4);
    cpu.funciones[0x6A as usize].set_punt_y_val_a_fn(ld_l_d, ld_l_d_txt, 1, 4);
    cpu.funciones[0x6B as usize].set_punt_y_val_a_fn(ld_l_e, ld_l_e_txt, 1, 4);
    cpu.funciones[0x6C as usize].set_punt_y_val_a_fn(ld_l_h, ld_l_h_txt, 1, 4);
    cpu.funciones[0x6D as usize].set_punt_y_val_a_fn(ld_l_l, ld_l_l_txt, 1, 4);
    cpu.funciones[0x6E as usize].set_punt_y_val_a_fn(ld_lOhlO, ld_lOhlO_txt, 1, 8);
    cpu.funciones[0x6F as usize].set_punt_y_val_a_fn(ld_l_a, ld_l_a, 1, 4);




*/
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
    let direccion = cpu.lee_hl();
    cpu.h = cpu.mem.lee_byte_de_mem(direccion);

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
    let direccion = cpu.lee_hl();
    cpu.l = cpu.mem.lee_byte_de_mem(direccion);

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
// 0x77
pub fn ldOhlO_a(cpu: &mut CPU) {
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let direccion = cpu.lee_hl();
    cpu.mem.escribe_byte_en_mem(direccion, cpu.a);

    cpu.pc += 1;
    cpu.t += 7;
}

pub fn ldOhlO_a_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD(HL),A"));
}

// 0x78
pub fn ld_a_b(cpu: &mut CPU) {
    cpu.a = cpu.b;
    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_a_b_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,B"));
}

// 0x79
pub fn ld_a_c(cpu: &mut CPU) {
    cpu.a = cpu.c;
    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_a_c_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,C"));
}

// 0x7A
pub fn ld_a_d(cpu: &mut CPU) {
    cpu.a = cpu.d;
    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_a_d_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,D"));
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

// 0x7C
pub fn ld_a_h(cpu: &mut CPU) {
    cpu.a = cpu.h;
    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_a_h_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,H"));
}

// 0x7D
pub fn ld_a_l(cpu: &mut CPU) {
    cpu.a = cpu.l;
    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn ld_a_l_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD A,L"));
}

// *************************** 8 ***********************************
// *************************** 9 ***********************************
// 0x90
pub fn sub_b(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.b);

    cpu.pc += cpu.get_bytes_instruccion();

    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_b_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB B")); }

// 0x91
pub fn sub_c(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.c);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_c_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB C")); }

// 0x92
pub fn sub_d(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.d);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_d_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB D")); }

// 0x93
pub fn sub_e(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.e);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_e_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB E")); }

// 0x94
pub fn sub_h(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.h);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_h_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB H")); }

// 0x95
pub fn sub_l(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.l);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_l_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB L")); }

// 0x96
pub fn subOhlO(cpu: &mut CPU) {
    let direccion = cpu.lee_hl();
    let dato = cpu.mem.lee_byte_de_mem(direccion);
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, dato);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn subOhlO_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(HL)")); }

// 0x97
pub fn sub_a(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.a);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_a_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB A")); }

// *************************** A ***********************************
// 0xA0
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

// 0xAF Xor consigo mismo pone a 0 y modifica flags
pub fn xor_a(cpu: &mut CPU) {
    cpu.a ^= cpu.a;
    //panic!(format!("0xAF XOR A   cpu.a = #{:02X}\n", cpu.a));
    if cpu.a < 0 { cpu.set_s_flag(); } else { cpu.reset_s_flag(); }  // S
    if cpu.a == 0 { cpu.set_z_flag(); } else { cpu.reset_z_flag(); } // Z
    cpu.reset_h_flag();                                              // H
    if (cpu.a & 0b0000_0001) != 0 { cpu.reset_pv_flag(); } else { cpu.set_pv_flag(); }  // P/V
    cpu.reset_n_flag();            // N
    cpu.reset_c_flag();            // C


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
    //let direccion = cpu.concatena_dos_u8_en_un_u16(cpu.b, cpu.c);
    let direccion = cpu.lee_bc();
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


// 0xD6
pub fn sub_n(cpu: &mut CPU) {
    cpu.a = cpu.resta_u8_menos_u8(cpu.a, cpu.r1);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn sub_n_txt(cpu: &mut CPU) { cpu.texto(&format!("SUB(#{:02X})", cpu.r1)); }

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


// 0xE6
pub fn and_n(cpu: &mut CPU) {
    cpu.a = cpu.and_u8_con_u8(cpu.a, cpu.r1);

    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t = cpu.get_t_instruccion();
}

pub fn and_n_txt(cpu: &mut CPU) {
    cpu.texto(&format!("AND #{:02X}", cpu.r1));
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
    if cpu.get_pv_flag() {
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
    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn di_txt(cpu: &mut CPU) {
    cpu.texto(&format!("DI"));
}

// 0xF6
pub fn or_n(cpu: &mut CPU) {
    cpu.a = cpu.or_u8_con_u8(cpu.a, cpu.r1);

    cpu.t += cpu.get_t_instruccion();
    cpu.pc += cpu.get_bytes_instruccion();
}

pub fn or_n_txt(cpu: &mut CPU) { cpu.texto(&format!("OR #{:02X}", cpu.r1)); }

// 0xF9
pub fn ld_sp_hl(cpu: &mut CPU) {
    //let hl = cpu.concatena_dos_u8_en_un_u16(cpu.h, cpu.l);
    let hl = cpu.lee_hl();
    cpu.sp = hl;

    cpu.t += 6;
    cpu.pc += 1;
}

pub fn ld_sp_hl_txt(cpu: &mut CPU) {
    cpu.texto(&format!("LD SP,HL"));
}

// 0xFB
pub fn ei(cpu: &mut CPU) {
    cpu.permitida_interrupcion = true;
    cpu.pc += cpu.get_bytes_instruccion();
    cpu.t += cpu.get_t_instruccion();
}

pub fn ei_txt(cpu: &mut CPU) { cpu.texto(&format!("EI")); }

// 0xFE
pub fn cp_n(cpu: &mut CPU) {
    let _ = cpu.resta_u8_menos_u8(cpu.a, cpu.r1);

    cpu.pc += 2;
    cpu.t += 7;
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