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
    cpu.funciones[0x05 as usize] = dec_b;
    cpu.funciones_txt[0x05 as usize] = dec_b_txt;
    cpu.funciones[0x06 as usize] = ld_b_n;
    cpu.funciones_txt[0x06 as usize] = ld_b_n_txt;
    cpu.funciones[0x0C as usize] = inc_c;
    cpu.funciones_txt[0x0C as usize] = inc_c_txt;
    cpu.funciones[0x0E as usize] = ld_c_n;
    cpu.funciones_txt[0x0E as usize] = ld_c_n_txt;
    // *************************** 1 ***********************************
    cpu.funciones[0x11 as usize] = ld_de_nn;
    cpu.funciones_txt[0x11 as usize] = ld_de_nn_txt;
    cpu.funciones[0x13 as usize] = inc_de;
    cpu.funciones_txt[0x13 as usize] = inc_de_txt;
    cpu.funciones[0x17 as usize] = rla;
    cpu.funciones_txt[0x17 as usize] = rla_txt;
    cpu.funciones[0x1A as usize] = ld_aOdeO;
    cpu.funciones_txt[0x1A as usize] = ld_aOdeO_txt;
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
    cpu.funciones[0x2A as usize] = ld_hlOnnO;
    cpu.funciones_txt[0x2A as usize] = ld_hlOnnO_txt;
    cpu.funciones[0x2B as usize] = dec_hl;
    cpu.funciones_txt[0x2B as usize] = dec_hl_txt;
    // *************************** 3 ***********************************
    cpu.funciones[0x31 as usize] = ld_spOnnO;
    cpu.funciones_txt[0x31 as usize] = ld_spOnnO_txt;
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
    cpu.funciones[0x36 as usize] = ld_OhlO_n;
    cpu.funciones_txt[0x36 as usize] = ld_OhlO_n_txt;
    cpu.funciones[0x3E as usize] = ld_a_n;
    cpu.funciones_txt[0x3E as usize] = ld_a_n_txt;
    // *************************** 4 ***********************************
    cpu.funciones[0x47 as usize] = ld_b_a;
    cpu.funciones_txt[0x47 as usize] = ld_b_a_txt;
    cpu.funciones[0x4F as usize] = ld_c_a;
    cpu.funciones_txt[0x4F as usize] = ld_c_a_txt;
    // *************************** 5 ***********************************
    // *************************** 6 ***********************************
    cpu.funciones[0x62 as usize] = ld_h_d;
    cpu.funciones_txt[0x62 as usize] = ld_h_d_txt;
    cpu.funciones[0x6B as usize] = ld_l_e;
    cpu.funciones_txt[0x6B as usize] = ld_l_e_txt;
    // *************************** 7 ***********************************
    cpu.funciones[0x77 as usize] = ldOhlO_a;
    cpu.funciones_txt[0x77 as usize] = ldOhlO_a_txt;
    cpu.funciones[0x7B as usize] = ld_a_e;
    cpu.funciones_txt[0x7B as usize] = ld_a_e_txt;
    // *************************** 8 ***********************************
    // *************************** 9 ***********************************
    // *************************** A ***********************************
    cpu.funciones[0xAF as usize] = xor_a;
    cpu.funciones_txt[0xAF as usize] = xor_a_txt;
    // *************************** B ***********************************
    cpu.funciones[0xBC as usize] = cp_h;
    cpu.funciones_txt[0xBc as usize] = cp_h_txt;
    // *************************** C ***********************************
    cpu.funciones[0xC1 as usize] = pop_bc;
    cpu.funciones_txt[0xC1 as usize] = pop_bc_txt;
    cpu.funciones[0xC3 as usize] = jp_nn;
    cpu.funciones_txt[0xC3 as usize] = jp_nn_txt;
    cpu.funciones[0xC5 as usize] = push_bc;
    cpu.funciones_txt[0xC5 as usize] = push_bc_txt;
    cpu.funciones[0xC9 as usize] = ret;
    cpu.funciones_txt[0xC9 as usize] = ret_txt;
    cpu.funciones[0xCB as usize] = CB; // Extensión
    cpu.funciones_txt[0xCB as usize] = CB_txt;
    cpu.funciones[0xCD as usize] = call_nn;
    cpu.funciones_txt[0xCD as usize] = call_nn_txt;
    // *************************** D ***********************************
    cpu.funciones[0xD3 as usize] = out_OnO_a;
    cpu.funciones_txt[0xD3 as usize] = out_OnO_a_txt;
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
    cpu.funciones[0xED as usize] = ED; // Extensión
    cpu.funciones_txt[0xED as usize] = ED_txt;

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


    // *************************** F ***********************************
    cpu.funciones[0xF3 as usize] = di;
    cpu.funciones_txt[0xF3 as usize] = di_txt;
    cpu.funciones[0xFE as usize] = cp_n;
    cpu.funciones_txt[0xFE as usize] = cp_n_txt;
}


// O = ()     p = '    m = +       n = valor hex de 8 bits
// *************************** 0 ***********************************
// 0x00
pub fn nop(cpu: &mut CPU) {
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn nop_txt(cpu: &mut CPU) {
//    let txt = format!("NOP opcode = #{:02X}", cpu.r0);
//    cpu.texto(&txt);
    cpu.texto(&format!("NOP opcode = #{:02X}", cpu.r0));
}

// 0x01 NN NN
pub fn ld_bc_nn(cpu: &mut CPU) {
    panic!("0x01 ld_bc_nn: funcion no implementada");
}

// 0x02
pub fn ldObcO_a(cpu: &mut CPU) {
    panic!("0x02 ldObcO_a: funcion no implementada");
}

// 0x03
pub fn inc_bc(cpu: &mut CPU) {
    panic!("0x03 inc_bc: funcion no implementada");
}

// 0x04
pub fn inc_b(cpu: &mut CPU) {
    panic!("0x04 inc_b: funcion no implementada");
}

// 0x05
pub fn dec_b(cpu: &mut CPU) {
    cpu.b = cpu.dec_8bits(cpu.b);
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn dec_b_txt(cpu: &mut CPU) {
    //let txt = format!("DEC B");
    cpu.texto(&format!("DEC B"));
}

// 0x06
pub fn ld_b_n(cpu: &mut CPU) {
    cpu.b = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_b_n_txt(cpu: &mut CPU) {
    //let txt = format!("LD B,#{:02X}", cpu.r1);
    cpu.texto(&format!("LD B,#{:02X}", cpu.r1));
}

// 0x07
pub fn rlca(cpu: &mut CPU) {
    panic!("0x07 rlca: funcion no implementada");
}

// 0x08
pub fn ex_af_afp(cpu: &mut CPU) {
    panic!("0x08 ex_af_afp: funcion no implementada");
}

// 0x09
pub fn add_hl_bc(cpu: &mut CPU) {
    panic!("0x09 add_hl_bc: funcion no implementada");
}

// 0x0A
pub fn ld_aObcO(cpu: &mut CPU) { panic!("0x0A ld_aObcO: funcion no implementada"); }

// 0x0B
pub fn dec_bc(cpu: &mut CPU) { panic!("0x0B dec_bc: funcion no implementada"); }

// 0x0C
pub fn inc_c(cpu: &mut CPU) {
    cpu.c = cpu.inc_8bits(cpu.c);

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn inc_c_txt(cpu: &mut CPU) {
    //let txt = format!("INC C");
    cpu.texto(&format!("INC C"));
}

// 0x0D
pub fn dec_c(cpu: &mut CPU) { panic!("0x0D dec_c: funcion no implementada"); }

// 0x0E
pub fn ld_c_n(cpu: &mut CPU) {
    cpu.c = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_c_n_txt(cpu: &mut CPU) {
//    let txt = format!("LD C,#{:02X}", cpu.r1);
//    cpu.texto(&txt);
    cpu.texto(&format!("LD C,#{:02X}", cpu.r1));
}

// 0x0F
pub fn rrca(cpu: &mut CPU) { panic!("0x0F rrca: funcion no implementada"); }
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

pub fn ld_de_nn_txt(cpu: &mut CPU) {
    //let txt = format!("LD DE,#{:04X}", cpu.r1r2);
    cpu.texto(&format!("LD DE,#{:04X}", cpu.r1r2));
}

// 0x12
pub fn ldOdeO_a(cpu: &mut CPU) { panic!("0x12 ldOdeO_a: funcion no implementada"); }

// 0x13
pub fn inc_de(cpu: &mut CPU) {
    let d16 = (cpu.d as u16) << 8;
    let mut de: u16 = d16 | (cpu.e as u16);
    de = de.wrapping_add(1);
    cpu.d = ((de & 0xFF00) >> 8) as u8;
    cpu.e = (de & 0x00FF) as u8;

    cpu.pc += 1;
    cpu.t += 13;
}

pub fn inc_de_txt(cpu: &mut CPU) {
    //let txt = format!("INC DE");
    cpu.texto(&format!("INC DE"));
}

// 0x14
pub fn inc_d(cpu: &mut CPU) { panic!("0x14 inc_d: funcion no implementada"); }

// 0x15
pub fn dec_d(cpu: &mut CPU) { panic!("0x15 dec_d: funcion no implementada"); }

// 0x16
pub fn ld_d_n(cpu: &mut CPU) { panic!("0x16 ld_d_n: funcion no implementada"); }

// 0x17
pub fn rla(cpu: &mut CPU) {
    cpu.a = cpu.do_rl_n(cpu.a);
    cpu.pc += 1;
    cpu.t += 4;
}

pub fn rla_txt(cpu: &mut CPU) {
    //let txt = format!("RLA");
    cpu.texto(&format!("RLA"));
}

// 0x18
pub fn jr_n(cpu: &mut CPU) { panic!("0x18 jr_n: funcion no implementada"); }

// 0x19
pub fn add_hl_de(cpu: &mut CPU) { panic!("0x19 add_hl_de: funcion no implementada"); }

// 0x1A
pub fn ld_aOdeO(cpu: &mut CPU) {
    let d16 = (cpu.d as u16) << 8;
    let de: u16 = d16 | (cpu.e as u16);
    cpu.a = cpu.mem.lee_byte_de_mem(de);
    cpu.pc += 1;

    cpu.t += 7;
}

pub fn ld_aOdeO_txt(cpu: &mut CPU) {
    //let txt = format!("LD A(DE)");
    cpu.texto(&format!("LD A(DE)"));
}

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
    //let txt = format!("JR NZ#{:02X}", cpu.pc.wrapping_add(cpu.r1 as u16));
    let salto = cpu.pc.wrapping_add(2);
    //let txt = format!("JR NZ #{:04X}", salto.wrapping_add((cpu.r1 as i8) as u16));
    cpu.texto(&format!("JR NZ #{:04X}", salto.wrapping_add((cpu.r1 as i8) as u16)));
}

// 0x21 NN NN
pub fn ld_hl_nn(cpu: &mut CPU) {
    cpu.h = cpu.r2;
    cpu.l = cpu.r1;

    cpu.pc += 3;
    cpu.t += 10;
}

pub fn ld_hl_nn_txt(cpu: &mut CPU) {
    //let txt = format!("LD HL#{:04X}", cpu.r1r2);
    cpu.texto(&format!("LD HL#{:04X}", cpu.r1r2));
}


// 0x22 Difiere según procesador (LR35902->LDI  (HL),A)
pub fn ldiOhlO_aGB(cpu: &mut CPU) {
    let h16 = (cpu.h as u16) << 8;
    let mut hl: u16 = h16 | (cpu.l as u16);
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    hl = hl.wrapping_add(1);

    cpu.h = ((hl & 0xFF00) >> 8) as u8;
    cpu.l = (hl & 0x00FF) as u8;

    cpu.pc += 1;

    cpu.t += 8;
}

pub fn ldiOhlO_aGB_txt(cpu: &mut CPU) {
    //let txt = format!("LDI(HL),A");
    cpu.texto(&format!("LDI(HL),A"));
}

// 0x22 Difiere según procesador (Z80->LD  (nn),HL)
pub fn ldOnnO_hl(cpu: &mut CPU) {}

pub fn ldOnnO_hl_txt(cpu: &mut CPU) {}

// 0x23
pub fn inc_hl(cpu: &mut CPU) {
    let h16 = (cpu.h as u16) << 8;
    let mut hl: u16 = h16 | (cpu.l as u16);
    hl = hl.wrapping_add(1);
    cpu.h = ((hl & 0xFF00) >> 8) as u8;
    cpu.l = (hl & 0x00FF) as u8;

    cpu.pc += 1;
    cpu.t += 6;
}

pub fn inc_hl_txt(cpu: &mut CPU) {
    //let txt = format!("INC HL");
    cpu.texto(&format!("INC HL"));
}

// 0x24
pub fn inc_h(cpu: &mut CPU) { panic!("0x24 inc_h: funcion no implementada"); }

// 0x25
pub fn dec_h(cpu: &mut CPU) { panic!("0x25 dec_h: funcion no implementada"); }

// 0x26
pub fn ld_h_n(cpu: &mut CPU) { panic!("0x26 ld_h_n: funcion no implementada"); }

// 0x27
pub fn daa(cpu: &mut CPU) { panic!("0x27 daa: funcion no implementada"); }

// 0x 28
pub fn jr_z_n(cpu: &mut CPU) { panic!("0x28 jr_z_n: funcion no implementada"); }

// 0x29
pub fn add_hl_hl(cpu: &mut CPU) { panic!("0x298 add_hl_hl: funcion no implementada"); }

// 0x2A NN NN
pub fn ld_hlOnnO(cpu: &mut CPU) {
    cpu.l = cpu.mem.lee_byte_de_mem(cpu.r1r2);
    cpu.h = cpu.mem.lee_byte_de_mem(cpu.r2r3);

    cpu.t += 16;
    cpu.pc += 3;
}

pub fn ld_hlOnnO_txt(cpu: &mut CPU) {
    //let txt = format!("LD HL(#{:04X})", cpu.r1r2);
    cpu.texto(&format!("LD HL(#{:04X})", cpu.r1r2));
}

// 0x2B
pub fn dec_hl(cpu: &mut CPU) {
    let h16 = (cpu.h as u16) << 8;
    let mut hl: u16 = h16 | (cpu.l as u16);
    hl = hl.wrapping_sub(1);
    cpu.h = ((hl & 0xFF00) >> 8) as u8;
    cpu.l = (hl & 0x00FF) as u8;

    cpu.pc += 1;
    cpu.t += 6;
}

pub fn dec_hl_txt(cpu: &mut CPU) {
    //let txt = format!("DEC HL");
    cpu.texto(&format!("DEC HL"));
}

// 0x2C
pub fn inc_l(cpu: &mut CPU) { panic!("0x2C inc_l: funcion no implementada"); }

// 0x2D
pub fn dec_l(cpu: &mut CPU) { panic!("0x2D dec_l: funcion no implementada"); }

// 0x2E
pub fn ld_l_n(cpu: &mut CPU) { panic!("0x2E ld_l_n: funcion no implementada"); }

// 0x2F
pub fn cpl(cpu: &mut CPU) { panic!("0x2F cpl: funcion no implementada"); }

// *************************** 3 ***********************************
// 0x31
pub fn ld_spOnnO(cpu: &mut CPU) {
    cpu.sp = cpu.r1r2; // LD SP,d16
    cpu.pc += 3;

    cpu.t += 10;
}

pub fn ld_spOnnO_txt(cpu: &mut CPU) {
    //let txt = format!("LD SP#{:04X}", cpu.r1r2);
    cpu.texto(&format!("LD SP#{:04X}", cpu.r1r2));
}

// 0x32 Difiere según procesador (LR35902->LDD  (HL),A)
pub fn lddOhlO_aGB(cpu: &mut CPU) {
    let h16 = (cpu.h as u16) << 8;
    let mut hl: u16 = h16 | (cpu.l as u16);
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    hl = hl.wrapping_sub(1);

    cpu.h = ((hl & 0xFF00) >> 8) as u8;
    cpu.l = (hl & 0x00FF) as u8;

    cpu.pc += 1;

    cpu.t += 8;
}

pub fn lddOhlO_aGB_txt(cpu: &mut CPU) {
    //let txt = format!("LDD (HL),A");
    cpu.texto(&format!("LDD (HL),A"));
}

// 0x32 Difiere según procesador (Z80->LD  (nn),A)
pub fn ldOnnO_a(cpu: &mut CPU) {
    cpu.mem.escribe_byte_en_mem(cpu.r1r2, cpu.a);

    cpu.pc += 3;

    cpu.t += 13;
}

pub fn ldOnnO_a_txt(cpu: &mut CPU) {
    let n16 = (cpu.r2 as u16) << 8;
    let mut nn: u16 = n16 | (cpu.r1 as u16);
    //let txt = format!("LD (#{:04X}),A", cpu.r1r2);
    cpu.texto(&format!("LD (#{:04X}),A", cpu.r1r2));
}


// 0x36 NN
pub fn ld_OhlO_n(cpu: &mut CPU) {
    let hl16 = ((cpu.h as u16) << 8) | (cpu.l as u16);
    cpu.mem.escribe_byte_en_mem(hl16, cpu.r1);

    cpu.t += 10;
    cpu.pc += 2;
}

pub fn ld_OhlO_n_txt(cpu: &mut CPU) {
    //let txt = format!("LD(HL)#{:02X}", cpu.r1);
    cpu.texto(&format!("LD(HL)#{:02X}", cpu.r1));
}

// 0x3E
pub fn ld_a_n(cpu: &mut CPU) {
    cpu.a = cpu.r1;

    cpu.t += 7;
    cpu.pc += 2;
}

pub fn ld_a_n_txt(cpu: &mut CPU) {
    let txt = format!("LD A,#{:02X}", cpu.r1);
    cpu.texto(&format!("LD A,#{:02X}", cpu.r1));
}

// *************************** 4 ***********************************
// 0x47
pub fn ld_b_a(cpu: &mut CPU) {
    cpu.b = cpu.a;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_b_a_txt(cpu: &mut CPU) {
    //let txt = format!("LD B,A");
    cpu.texto(&format!("LD B,A"));
}


// 0x4F
pub fn ld_c_a(cpu: &mut CPU) {
    cpu.c = cpu.a;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_c_a_txt(cpu: &mut CPU) {
    //let txt = format!("LD C,A");
    cpu.texto(&format!("LD C,A"));
}

// *************************** 5 ***********************************
// *************************** 6 ***********************************
// 0x62
pub fn ld_h_d(cpu: &mut CPU) {
    cpu.h = cpu.d;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_h_d_txt(cpu: &mut CPU) {
    //let txt = format!("LD H,D");
    cpu.texto(&format!("LD H,D"));
}

// 0x6B
pub fn ld_l_e(cpu: &mut CPU) {
    cpu.l = cpu.e;

    cpu.t += 4;
    cpu.pc += 1;
}

pub fn ld_l_e_txt(cpu: &mut CPU) {
    //let txt = format!("LD L,E");
    cpu.texto(&format!("LD L,E"));
}

// *************************** 7 ***********************************
// 0x77
pub fn ldOhlO_a(cpu: &mut CPU) {
    let h16 = (cpu.h as u16) << 8;
    let hl: u16 = h16 | (cpu.l as u16);
    cpu.mem.escribe_byte_en_mem(hl, cpu.a);

    cpu.pc += 1;

    cpu.t += 7;
}

pub fn ldOhlO_a_txt(cpu: &mut CPU) {
    //let txt = format!("LD(HL),A");
    cpu.texto(&format!("LD(HL),A"));
}

// 0x7B
pub fn ld_a_e(cpu: &mut CPU) {
    cpu.a = cpu.e;
    cpu.pc += 1;

    cpu.t += 4;
}

pub fn ld_a_e_txt(cpu: &mut CPU) {
    //let txt = format!("LD A,E");
    cpu.texto(&format!("LD A,E"));
}


// *************************** 8 ***********************************
// *************************** 9 ***********************************
// *************************** A ***********************************
// 0xAF
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
    //let txt = format!("JP #{:04X}", cpu.r1r2);
    cpu.texto(&format!("JP #{:04X}", cpu.r1r2));
}

// 0xC5
pub fn push_bc(cpu: &mut CPU) {
    let b16 = (cpu.b as u16) << 8;
    let bc: u16 = b16 | (cpu.c as u16);
    cpu.push(bc);
    cpu.pc += 1;

    cpu.t += 11;
}

pub fn push_bc_txt(cpu: &mut CPU) {
    //let txt = format!("PUSH BC");
    cpu.texto(&format!("PUSH BC"));
}

// 0xC9
pub fn ret(cpu: &mut CPU) {
    cpu.pc = cpu.pop();

    cpu.t += 10;
}

pub fn ret_txt(cpu: &mut CPU) {
    //let txt = format!("RET");
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
    //let txt = format!("CALL #{:04X}", cpu.r1r2);
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
    //let txt = format!("OUT(#{:02X}),A", cpu.r1);
    cpu.texto(&format!("OUT(#{:02X}),A", cpu.r1));
}

// *************************** E ***********************************
// 0xE0 Difiere según procesador (LR35902->LD(#FF00+N),A)
pub fn ldOff00_m_nO_aGB(cpu: &mut CPU) {
    let addr: u16 = 0xFF00 + cpu.r1 as u16;
    cpu.mem.escribe_byte_en_mem(addr, cpu.a);

    cpu.pc += 2;

    cpu.t += 12;
}

pub fn ldOff00_m_nO_aGB_txt(cpu: &mut CPU) {
    //let txt = format!("LD ($FF00+#{:02X}),A", cpu.r1);
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
    //let txt = format!("LD ($FF00+C),A");
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
    panic!("0xEA ldOnnO_aGB: funcion no implementada");
}

pub fn ldOnnO_aGB_txt(cpu: &mut CPU) {
    panic!("0xEA ldOnnO_aGB_txt: funcion no implementada");
}

// 0xEA  Difiere según procesador (Z80->JP  V,nn)
pub fn jp_pe_nn(cpu: &mut CPU) {
    panic!("0xEA jp_pe_nn: funcion no implementada");
}

pub fn jp_pe_nn_txt(cpu: &mut CPU) {
    panic!("0xEA jp_pe_nn_txt: funcion no implementada");
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
// 0xF3 TODO: Debe deshablitar la interrupción enmascarada
pub fn di(cpu: &mut CPU) {
    cpu.t += 4;
    cpu.pc += 1;
}

pub fn di_txt(cpu: &mut CPU) {
    //let txt = format!("DI");
    cpu.texto(&format!("DI"));
}

// 0xFE
pub fn cp_n(cpu: &mut CPU) {
    let _ = cpu.do_sub(cpu.a, cpu.r1);

    cpu.pc += 2;
    cpu.t += 7;
}

pub fn cp_n_txt(cpu: &mut CPU) {
    //let txt = format!("CP #{:02X}", cpu.r1);
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