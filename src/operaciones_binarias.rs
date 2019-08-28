use crate::constantes::*;
use crate::cpu::CPU;

pub fn flag_v_u8_en_suma(cpu: &mut CPU, valor_a: u8, valor_b: u8, resultado: u8) {
    let ov = overflow_en_suma_u8(valor_a, valor_b, resultado);
    cpu.set_flag(FLAG_PV, ov);
}

pub fn overflow_en_suma_u8(valor_a: u8, valor_b: u8, resultado: u8) -> bool {
    (((valor_a ^ valor_b) & 0x80) == 0          // mismo signo     0x80 = 0b1000_0000
        && ((valor_b ^ resultado) & 0x80) != 0) // no es el mismo signo
}

pub fn flag_c_u8_en_suma(cpu: &mut CPU, valor_a: u8, valor_b: u8) { // C
    let valor_a16 = valor_a as u16;
    let valor_b16 = valor_b as u16;

    cpu.set_flag(FLAG_C, ((valor_a16.wrapping_add(valor_b16)) & 0x100) != 0);
}

pub fn flag_h_u8_en_suma(cpu: &mut CPU, valor1: u8, valor2: u8) { // H  carry del bit 3 al 4
    cpu.set_flag(FLAG_H, ((valor1 & 0x0f) + (valor2 & 0x0f)) > 0x0F);
}

pub fn flag_v_u8_en_resta(cpu: &mut CPU, valor_a: u8, valor_b: u8, resultado: u8) { // P  (P/V usado como overflow)
    let ov = overflow_en_resta_u8(valor_a, valor_b, resultado);
    cpu.set_flag(FLAG_PV, ov);
}

pub fn flag_c_u8_en_resta(cpu: &mut CPU, valor_a: u8, valor_b: u8) { // C
    let valor_a16 = valor_a as u16;
    let valor_b16 = valor_b as u16;

    cpu.set_flag(FLAG_C, ((valor_a16.wrapping_sub(valor_b16)) & 0x100) != 0);
}

pub fn flag_c_u16_en_suma(cpu: &mut CPU, valor_a: u16, valor_b: u16) { // C
    let valor_a32 = valor_a as u32;
    let valor_b32 = valor_b as u32;

    cpu.set_flag(FLAG_C, ((valor_a32.wrapping_add(valor_b32)) & 0x10000) != 0);
}

pub fn suma_u16_mas_u16(cpu: &mut CPU, valor_a: u16, valor_b: u16) -> u16 {
    flag_c_u16_en_suma(cpu, valor_a, valor_b); // C

    //cpu.reset_n_flag(); // N  TODO: Comprobar que pasa en todos los casos
    cpu.set_flag(FLAG_N, false);
    let resultado = valor_a.wrapping_add(valor_b);


    // Probar si hay acarreo de medio byte (bit 11 en 16 bits)
    flag_h_u16(cpu, resultado);


// TODO No todos les afecta Z????????
//        if resultado == 0 {
//            self.set_z_flag();
//        } else {
//            self.reset_z_flag();
//        }

    resultado
}

pub fn inc_16bits(cpu: &mut CPU, valor: u16) -> u16 {
    suma_u16_mas_u16(cpu, valor, 1)
}

pub fn dec_8bits(cpu: &mut CPU, valor: u8) -> u8 {
    resta_u8_menos_u8(cpu, valor, 1)
}

pub fn dec_16bits(valor: u16) -> u16 {
    resta_u16_menos_u16(valor, 1)
}

/// Devuelve valor_a - valor_b y modifica flags
pub fn resta_u8_menos_u8(cpu: &mut CPU, valor_a: u8, valor_b: u8) -> u8 {
    let nuevo_valor = valor_a.wrapping_sub(valor_b);

    cpu.set_flag(FLAG_S,
                 (nuevo_valor & FLAG_S) != 0);

    cpu.set_flag(FLAG_Z,
                 nuevo_valor == 0);

    let h = half_carry_en_resta_u8_sub(valor_a, valor_b);
    cpu.set_flag(FLAG_H, h);

    let ov = overflow_en_resta_u8(valor_a, valor_b, nuevo_valor);
    cpu.set_flag(FLAG_PV, ov);

    // Carry TODO: No estoy seguro

    cpu.set_flag(FLAG_C, valor_a < valor_b);
    cpu.set_flag(FLAG_N, true);

    nuevo_valor
}

pub fn resta_u16_menos_u16(valor_a: u16, valor_b: u16) -> u16 {
// TODO: Faltan Flags (Flags afectados: C N P/V H Z S)
// TODO: Probar si hay acarreo de medio byte (flag H) no lo tengo claro con 16 bits

//        if self.half_carry_en_resta_u8_sub(valor_a, valor_b) {
//            self.set_h_flag();
//        } else {
//            self.reset_h_flag();
//        }

    let nuevo_valor = valor_a.wrapping_sub(valor_b);

// flag Z lo quito, lddr no lo necesita
//        if nuevo_valor == 0 {
//            self.set_z_flag();
//        } else {
//            self.reset_z_flag();
//        }

    nuevo_valor
}

pub fn overflow_en_resta_u8(valor_a: u8, valor_b: u8, resultado: u8) -> bool {
    // ojo valor_b es el sustraendo
    (((valor_a ^ valor_b) & 0x80) != 0          // no es el mismo signo  0x80 = 0b1000_0000
        && ((valor_b ^ resultado) & 0x80) == 0) // mismo signo
}


pub fn overflow_en_suma_u16(valor_a: u16, valor_b: u16, resultado: u16) -> bool {
    // mismo signo     0x80 = 0b1000_0000_0000_0000
    (((valor_a ^ valor_b) & 0x8000) == 0
        && ((valor_b ^ resultado) & 0x8000) != 0) // no es el mismo signo
}

pub fn overflow_en_resta_u16(valor_a: u16, valor_b: u16, resultado: u16) -> bool {
    // ojo valor_b es el sustraendo
    // no es el mismo signo  0x8000 = 0b1000_0000_0000_0000
    (((valor_a ^ valor_b) & 0x8000) != 0
        && ((valor_b ^ resultado) & 0x8000) == 0) // mismo signo
}

/// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en suma
pub fn calc_half_carry_on_u16_sum(valor_a: u16, valor_b: u16) -> bool {
    ((valor_a & 0xFFF) + (valor_b & 0xFFF)) & 0x1000 == 0x1000
}

/// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en resta
pub fn calc_half_carry_on_u16_sub(valor_a: u16, valor_b: u16) -> bool {
    (valor_a & 0xFFF) < (valor_b & 0xFFF)
}

/// Devuelve true si hay acarreo de medio byte en suma
fn calc_half_carry_on_u8_sum(valor_a: u8, valor_b: u8) -> bool {
    ((valor_a & 0xF) + (valor_b & 0xF)) & 0x10 == 0x10
}

/// Devuelve true si hay acarreo de medio byte en resta
fn calc_half_carry_on_u8_sub(valor_a: u8, valor_b: u8) -> bool {
    (valor_a & 0xF) < (valor_b & 0xF)
}

/// Devuelve true si hay acarreo de medio byte en resta
fn half_carry_en_resta_u8_sub(valor_a: u8, valor_b: u8) -> bool {
    (valor_a & 0xF) < (valor_b & 0xF)
}

// Funcion que usan las funciones de manejo de flags
pub fn set_flag(cpu: &mut CPU, flag: u8, condicion: bool) {
    if condicion {
        cpu.f = cpu.f | flag
    } else {
        cpu.f = cpu.f & (!flag);
    }
}

pub fn suma_u8_mas_u8(cpu: &mut CPU, valor_a: u8, valor_b: u8) -> u8 {
// TODO: Faltan Flags (Flags afectados: C N P/V H Z S)

    let valor_a16 = valor_a as u16;
    let valor_b16 = valor_b as u16;

    let resultado16 = valor_a16 + valor_b16;
    let resultado = valor_a.wrapping_add(valor_b);

    cpu.set_flag(FLAG_C, (0b1_0000_0000 & resultado16) != 0);

    let h = calc_half_carry_on_u8_sum(valor_a, valor_b);
    cpu.set_flag(FLAG_H, h);

    cpu.set_flag(FLAG_Z, resultado == 0);

    cpu.set_flag(FLAG_N, false);

    resultado
}

pub fn flag_p_u8(cpu: &mut CPU, valor: u8) { // P  (P/V usado como paridad)
    let mut unos: u8 = 0;
    for n in 0..=7 {
        if valor & (1 << n) != 0 {
            unos += 1;
        }
    }

    cpu.set_flag(FLAG_PV, (unos % 2) == 0);
}

pub fn flag_h_u16(cpu: &mut CPU, valor: u16) { // H carry del bit 11 al 12
    cpu.set_flag(FLAG_H, ((valor & 0x0FFF) + 1) > 0x0FFF);
}

pub fn flag_h_u8_en_resta(cpu: &mut CPU, valor1: u8, valor2: u8) { // H  carry del bit 3 al 4
    cpu.set_flag(FLAG_H, (valor1 & 0x0F) < (valor2 & 0x0F));
}

pub fn suma_compl2_a_un_u16(valoru16: u16, valorcomp2: u8) -> u16 {
    let mut valorcomp2_u16;

    // Comprueba si el u8 es negativo
    // Paso el complemento a 2 de 8 bits a complemento a 2 de 16 bits
    if valorcomp2 & FLAG_S != 0 {    // valorcomp2 es negativo
        valorcomp2_u16 = 0b1111_1111_0000_0000 | (valorcomp2 as u16);
    } else {     // valorcomp2 es positivo
        valorcomp2_u16 = valorcomp2 as u16;
    }
    valorcomp2_u16 = valorcomp2_u16.wrapping_add(valoru16);

    valorcomp2_u16
}


pub fn concatena_dos_u8_en_un_u16(hight: u8, low: u8) -> u16 {
    ((hight as u16) << 8) | (low as u16)
}

pub fn desconcatena_un_u16_en_dos_u8(valor: u16) -> (u8, u8) {
    let hight = ((valor & 0b1111_1111_0000_0000) >> 8) as u8;
    let low = (valor & 0b0000_0000_1111_1111) as u8;
    (hight, low)
}

