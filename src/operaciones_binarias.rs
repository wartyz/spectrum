use crate::constantes::*;
use crate::cpu::CPU;

pub fn overflow_en_suma_u8(valor_a: u8, valor_b: u8, resultado: u8) -> bool {
    (((valor_a ^ valor_b) & 0x80) == 0          // mismo signo     0x80 = 0b1000_0000
        && ((valor_b ^ resultado) & 0x80) != 0) // no es el mismo signo
}

pub fn overflow_en_suma_u16(valor_a: u16, valor_b: u16, resultado: u16) -> bool {
    // mismo signo     0x80 = 0b1000_0000_0000_0000
    (((valor_a ^ valor_b) & 0x8000) == 0
        && ((valor_b ^ resultado) & 0x8000) != 0) // no es el mismo signo
}

pub fn overflow_en_resta_u8(valor_a: u8, valor_b: u8, resultado: u8) -> bool {
    // ojo valor_b es el sustraendo
    (((valor_a ^ valor_b) & 0x80) != 0          // no es el mismo signo  0x80 = 0b1000_0000
        && ((valor_b ^ resultado) & 0x80) == 0) // mismo signo
}

pub fn overflow_en_resta_u16(valor_a: u16, valor_b: u16, resultado: u16) -> bool {
    // ojo valor_b es el sustraendo
    // no es el mismo signo  0x8000 = 0b1000_0000_0000_0000
    (((valor_a ^ valor_b) & 0x8000) != 0
        && ((valor_b ^ resultado) & 0x8000) == 0) // mismo signo
}

/// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en suma
pub fn calc_half_carry_on_u16_sum(valor_a: u16, valor_b: u16) -> bool {
    ((valor_a & 0xFFF) + (valor_b & 0xFFF)) > 0xFFF
    // experimento
    //((valor_a & 0b1000_0000_0000) == 1) && ((valor_b & 0b1000_0000_0000) == 1)
}

/// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en resta
pub fn calc_half_carry_on_u16_sub(valor_a: u16, valor_b: u16) -> bool {
    (valor_a & 0xFFF) < (valor_b & 0xFFF)
}

/// Devuelve true si hay acarreo de medio byte en suma
fn calc_half_carry_on_u8_sum(valor_a: u8, valor_b: u8) -> bool {
    ((valor_a & 0xF) + (valor_b & 0xF)) > 0xF
    // experimento
    //(valor_a & 0b1000) == 1 && (valor_b & 0b1000) == 1
}

/// Devuelve true si hay acarreo de medio byte en resta
fn calc_half_carry_on_u8_sub(valor_a: u8, valor_b: u8) -> bool {
    (valor_a & 0xF) < (valor_b & 0xF)
}

/// Devuelve true si hay acarreo de medio byte en resta
pub fn half_carry_en_resta_u8_sub(valor_a: u8, valor_b: u8) -> bool {
    //(valor_a & 0xF) < (valor_b & 0xF)
    // experimento:
    (valor_a & 0b1_0000) == 0 && (valor_b & 0b1_0000) == 1
//    let resultado = valor_a.wrapping_sub(valor_b);
//    ((valor_a ^ resultado) & 0b1_0000) != 0
}


// Devuelve true si hay un numero par de unos
pub fn paridad_bits_u8(valor: u8) -> bool {
    let mut unos: u8 = 0;
    for n in 0..=7 {
        if valor & (1 << n) != 0 {
            unos += 1;
        }
    }
    (unos % 2) == 0
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

