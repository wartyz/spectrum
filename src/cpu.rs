#![allow(non_snake_case)]

use crossterm::*;
use crate::mem::MEM;


use crate::instrucciones_normales::*;
use crate::instrucciones_cb::*;
use crate::instrucciones_ed::*;

use std::fmt;

pub enum PROCESADOR {
    Z80,
    SharpLr35902,
}

pub struct CPU {
    pub procesador: PROCESADOR,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ap: u8,
    pub bp: u8,
    pub cp: u8,
    pub dp: u8,
    pub ep: u8,
    pub fp: u8,
    pub hp: u8,
    pub lp: u8,
    pub i: u8,
    pub pc: u16,
    pub sp: u16,
    //t -> time cycles
    pub t: usize,

    pub funciones: [fn(&mut CPU); 256],
    pub funciones_txt: [fn(&mut CPU); 256],

    pub funciones_ed: [fn(&mut CPU); 256],
    pub funciones_ed_txt: [fn(&mut CPU); 256],

    pub funciones_cb: [fn(&mut CPU); 256],
    pub funciones_cb_txt: [fn(&mut CPU); 256],
    // Bytes leidos de memoria:
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub r1r2: u16,
    pub r2r3: u16,

    debug: bool,
    pub mem: MEM,
}

impl CPU {
    pub fn new(mem: MEM) -> CPU {
        // Rellenamos arreglo de funciones con funciones nop()
        let funciones: [fn(&mut CPU); 256] = [funcion_no_implementada; 256];
        let funciones_txt: [fn(&mut CPU); 256] = [funcion_no_implementada; 256];

        let funcionesED: [fn(&mut CPU); 256] = [funcionED_no_implementada; 256];
        let funcionesED_txt: [fn(&mut CPU); 256] = [funcionED_no_implementada; 256];

        let funcionesCB: [fn(&mut CPU); 256] = [funcionCB_no_implementada; 256];
        let funcionesCB_txt: [fn(&mut CPU); 256] = [funcionCB_no_implementada; 256];

        let mut cpu = CPU {
            // OJO Cambiar si se usa otro procesador!
            procesador: PROCESADOR::Z80,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            ap: 0,
            bp: 0,
            cp: 0,
            dp: 0,
            ep: 0,
            fp: 0,
            hp: 0,
            lp: 0,

            i: 0,

            pc: 0,
            sp: 0,
            t: 0,

            funciones: funciones,
            funciones_txt: funciones_txt,
            funciones_ed: funciones,
            funciones_ed_txt: funciones_txt,
            funciones_cb: funciones,
            funciones_cb_txt: funciones_txt,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r1r2: 0,
            r2r3: 0,

            debug: false,
            mem: mem,
        };
        mete_funciones_normales(&mut cpu);
        mete_funciones_cb(&mut cpu);
        mete_funciones_ed(&mut cpu);
        cpu
    }


    // FLAGS *****************************************************
    // Bit           7  6  5  4  3   2   1  0
    // Posicion      S  Z  X  H  X  P/V  N  C   (X = no usado)

    // Funciones GET de FLAGS
    /// Función general que usan las demás funciones de flag
    /// Recibe una máscara indicando el flag a leer y devuelve true o false segun sea 1 o 0
    pub fn get_flag(&self, bit_mask: u8) -> bool {
        (self.f & bit_mask) != 0
    }
    pub fn get_z_flag(&self) -> bool {
        self.get_flag(0b1000_0000)
    }
    pub fn get_n_flag(&self) -> bool {
        self.get_flag(0b0100_0000)
    }
    pub fn get_h_flag(&self) -> bool {
        self.get_flag(0b0010_0000)
    }
    pub fn get_c_flag(&self) -> bool {
        self.get_flag(0b0001_0000)
    }
    pub fn get_p_flag(&self) -> bool {
        self.get_flag(0b0000_0100)
    }

    // Funciones SET de FLAGS
    pub fn set_z_flag(&mut self) {
        self.f = self.f | 0b1000_0000;
    }
    pub fn reset_z_flag(&mut self) {
        self.f = self.f & 0b0111_1111;
    }
    pub fn set_n_flag(&mut self) {
        self.f = self.f | 0b0100_0000;
    }
    pub fn reset_n_flag(&mut self) {
        self.f = self.f & 0b1011_1111;
    }
    pub fn set_h_flag(&mut self) {
        self.f = self.f | 0b0010_0000;
    }
    pub fn reset_h_flag(&mut self) {
        self.f = self.f & 0b1101_1111;
    }
    pub fn set_c_flag(&mut self) {
        self.f = self.f | 0b0001_0000;
    }
    pub fn reset_c_flag(&mut self) {
        self.f = self.f & 0b1110_1111;
    }
    pub fn set_s_flag(&mut self) {
        self.f = self.f | 0b1000_0000;
    }
    pub fn reset_s_flag(&mut self) {
        self.f = self.f & 0b0111_1111;
    }
    pub fn set_pv_flag(&mut self) {
        self.f = self.f | 0b0000_0100;
    }
    pub fn reset_pv_flag(&mut self) {
        self.f = self.f & 0b1111_1011;
    }

    // FUNCIONES ARITMETICAS **************************************
    fn prueba_paridad_u8(&self, valor: u8) -> bool {
        valor & 0x0000_0001 == 0
    }
    /// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en suma
    fn calc_half_carry_on_u16_sum(&self, valor_a: u16, valor_b: u16) -> bool {
        ((valor_a & 0xFFF) + (valor_b & 0xFFF)) & 0x1000 == 0x1000
    }

    /// Devuelve true si hay acarreo de medio byte entre bit 11 y 12 eun un u16 en resta
    fn calc_half_carry_on_u16_sub(&self, valor_a: u16, valor_b: u16) -> bool {
        (valor_a & 0xFFF) < (valor_b & 0xFFF)
    }

    /// Devuelve true si hay acarreo de medio byte en suma
    fn calc_half_carry_on_u8_sum(&self, valor_a: u8, valor_b: u8) -> bool {
        ((valor_a & 0xF) + (valor_b & 0xF)) & 0x10 == 0x10
    }

    /// Devuelve true si hay acarreo de medio byte en resta
    fn calc_half_carry_on_u8_sub(&self, valor_a: u8, valor_b: u8) -> bool {
        (valor_a & 0xF) < (valor_b & 0xF)
    }
    /// Devuelve true si hay acarreo de medio byte en resta
    fn half_carry_en_resta_u8_sub(&self, valor_a: u8, valor_b: u8) -> bool {
        (valor_a & 0xF) < (valor_b & 0xF)
    }
    /// Devuelve valor_a - valor_b y modifica flags
    pub fn resta_u8_menos_u8(&mut self, valor_a: u8, valor_b: u8) -> u8 {
        // Probar si hay acarreo de medio byte (flag H)
        if self.half_carry_en_resta_u8_sub(valor_a, valor_b) {
            self.set_h_flag();
        } else {
            self.reset_h_flag();
        }

        let nuevo_valor = valor_a.wrapping_sub(valor_b);

        // flags Z y N
        if nuevo_valor == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        nuevo_valor
    }
    pub fn resta_u16_menos_u16(&mut self, valor_a: u16, valor_b: u16) -> u16 {
        // TODO: Faltan Flags (Flags afectados: C N P/V H Z S)
        // TODO: Probar si hay acarreo de medio byte (flag H) no lo tengo claro con 16 bits

//        if self.half_carry_en_resta_u8_sub(valor_a, valor_b) {
//            self.set_h_flag();
//        } else {
//            self.reset_h_flag();
//        }

        let nuevo_valor = valor_a.wrapping_sub(valor_b);

        // flag Z
        if nuevo_valor == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }

        nuevo_valor
    }

    pub fn suma_u8_mas_u8(&mut self, valor_a: u8, valor_b: u8) -> u8 {
        // Probar si hay acarreo de medio byte
        if self.calc_half_carry_on_u8_sum(valor_a, valor_b) {
            self.set_h_flag();
        } else {
            self.reset_h_flag();
        }

        let resultado = valor_a.wrapping_add(valor_b);

        // Establece los flags
        if resultado == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        resultado
    }
    pub fn suma_u16_mas_u16(&mut self, valor_a: u16, valor_b: u16) -> u16 {
        // TODO: Probar si hay acarreo de medio byte (flag H) no lo tengo claro con 16 bits
        // TODO: Faltan Flags (Flags afectados: C N P/V H Z S)
        // Probar si hay acarreo de medio byte
//        if self.calc_half_carry_on_u8_sum(valor_a, valor_b) {
//            self.set_h_flag();
//        } else {
//            self.reset_h_flag();
//        }

        let resultado = valor_a.wrapping_add(valor_b);

        // Establece los flags
        if resultado == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }

        resultado
    }


    pub fn and_u8_con_u8(&mut self, valor_a: u8, valor_b: u8) -> u8 {
        self.reset_c_flag();
        self.reset_n_flag();
        self.set_h_flag();

        let resultado = valor_a & valor_b;
        if resultado == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }

        if self.prueba_paridad_u8(resultado) {
            self.set_pv_flag();
        } else {
            self.reset_pv_flag();
        }

        resultado
    }

    pub fn inc_8bits(&mut self, valor: u8) -> u8 {
        self.suma_u8_mas_u8(valor, 1)
    }
    pub fn inc_16bits(&mut self, valor: u16) -> u16 {
        self.suma_u16_mas_u16(valor, 1)
    }
    pub fn dec_8bits(&mut self, valor: u8) -> u8 { self.resta_u8_menos_u8(valor, 1) }
    pub fn dec_16bits(&mut self, valor: u16) -> u16 {
        self.resta_u16_menos_u16(valor, 1)
    }

    pub fn concatena_dos_u8_en_un_u16(&mut self, hight: u8, low: u8) -> u16 {
        ((hight as u16) << 8) | (low as u16)
    }
    pub fn desconcatena_un_u16_en_dos_u8(&mut self, valor: u16) -> (u8, u8) {
        let hight = ((valor & 0b1111_1111_0000_0000) >> 8) as u8;
        let low = (valor & 0b0000_0000_1111_1111) as u8;
        (hight, low)
    }
    // FUNCIONES DE STACK ********************************************************
    /// Pone en el stack un valor de 16 bits y modifica el puntero
    pub fn push(&mut self, addr: u16) {
        let addr_0: u8 = ((addr & 0xFF00) >> 8) as u8;
        let addr_1: u8 = (addr & 0x00FF) as u8;

        self.mem.escribe_byte_en_mem(self.sp, addr_1);
        self.sp -= 1;
        self.mem.escribe_byte_en_mem(self.sp, addr_0);
        self.sp -= 1;
    }

    /// Saca del stack un valor de 16 bits y modifica el puntero
    pub fn pop(&mut self) -> u16 {
        self.sp += 1;
        let addr_0 = self.mem.lee_byte_de_mem(self.sp);
        self.sp += 1;
        let addr_1 = self.mem.lee_byte_de_mem(self.sp);

        let addr_016 = (addr_0 as u16) << 8;
        let addr = addr_016 | (addr_1 as u16);
        addr
    }

    // FIN de FUNCIONES DE STACK **********************************

    // FUNCIONES DE BIT *******************************************
    /// Pone flags segun bit de registro
    pub fn bit(&mut self, reg: u8, bit: u8) {
        if reg & (1 << bit) == 0 {
            self.set_z_flag();
        };
        self.reset_n_flag();
        self.set_h_flag();
    }
    // FUNCIONES DE ROTACION DE BITS *******************************************
//    pub fn do_rl_n(&mut self, register_value: u8) -> u8 {
//        let old_c_flag = self.get_c_flag();
//        let c_flag: bool = (0b1000_0000 & register_value) != 0;
//        if c_flag {
//            self.set_c_flag();
//        } else {
//            self.reset_c_flag();
//        }
//
//        // Rotación
//        let mut new_register_value = register_value << 1;
//        new_register_value = new_register_value & 0b1111_1110;
//        if old_c_flag {
//            new_register_value |= 0b0000_0001;
//        }
//
//        //maneja flags
//        if new_register_value == 0 {
//            self.set_z_flag();
//        } else {
//            self.reset_z_flag();
//        }
//        self.reset_n_flag();
//        self.reset_h_flag();
//
//        new_register_value
//    }

    // FUNCIONES DE DEBUG *******************************************
    pub fn establece_debug(&mut self) {
        self.debug = true;
    }

    pub fn quita_debug(&mut self) {
        self.debug = false;
    }

    pub fn ejecuta_instruccion(&mut self) {
        self.obtiene_intruccion_y_bytes_posteriores();

        // Ejecuta instruccion
        self.funciones[self.r0 as usize](self);
    }
    /// Lee de memoria el opcode y los bytes posteriores
    pub fn obtiene_intruccion_y_bytes_posteriores(&mut self) {
        self.r0 = self.mem.lee_byte_de_mem(self.pc);
        self.r1 = self.mem.lee_byte_de_mem(self.pc + 1);
        self.r2 = self.mem.lee_byte_de_mem(self.pc + 2);
        self.r3 = self.mem.lee_byte_de_mem(self.pc + 3);

        // Invirtiendo posición de 16 bits ya que es BIG ENDIAN
        self.r1r2 = ((self.r2 as u16) << 8) | self.r1 as u16;
        self.r2r3 = ((self.r3 as u16) << 8) | self.r2 as u16;
    }

    pub fn limpia_consola(&self) {
        if self.debug {
            // Crea terminal
            let mut terminal = terminal();

            // Borra todas las lineas del terminal;
            terminal.clear(ClearType::All);
        }
    }

    pub fn texto(&self, txt: &String) {
        if self.debug {
            cursor().goto(48, 1);
            print!("{:04X}", self.pc);
            print!("|-> ");
            println!("{}", txt);
        }
    }

    pub fn imprime_opcode(&mut self) {
        if self.debug {
            cursor().goto(62, 12);
            print!("<-");


            let empieza = self.pc - 4;
            for lin in 0..=8 {
                cursor().goto(60, lin + 8);
                print!("{:02X}", self.mem.lee_byte_de_mem(empieza + lin));
            }
        }
    }

    pub fn imprime_stack(&mut self) {
        if self.debug {
            cursor().goto(48, 4);
            print!("STACK {:04X}", self.sp);
            cursor().goto(48, 5);
            if self.sp < 0xFFFE {
                print!("->{:02X} {:02X}",
                       self.mem.lee_byte_de_mem(self.sp + 2),
                       self.mem.lee_byte_de_mem(self.sp + 1));
            } else {
                print!("->{:04X} ", 0x0000);
            }
        }
    }

    pub fn imprime_memoria(&mut self, inicio: u16) {
        if self.debug {
            // Crea cursor
            let mut cursor = cursor();

            cursor.hide();


            for col in 0..=15 {
                for lin in 0..=15 {
                    cursor.goto(74, lin + 1);
                    print!("{:04X}", inicio + (lin * 16));
                    cursor.goto(80 + 3 * col, lin + 1);

                    print!("{:02X}  ", self.mem.lee_byte_de_mem(inicio + (lin * 16) + (col)));
                }
            }
            cursor.goto(100, 20);
        }
    }

    pub fn imprime_ports(&mut self) {
        if self.debug {
            // Crea cursor
            let mut cursor = cursor();

            cursor.hide();

            cursor.goto(1, 10);
            print!("{}PORTS{}",
                   Colored::Fg(Color::Blue),
                   Colored::Fg(Color::White), );
            for col in 0..=15 {
                for lin in 0..=15 {
                    cursor.goto(1, lin + 11);
                    print!("{:04X}", lin * 16);
                    cursor.goto(8 + 3 * col, lin + 11);

                    print!("{:02X}  ", self.mem.lee_byte_de_port(((lin * 16) + (col)) as u8));
                }
            }
            cursor.goto(100, 20);
        }
    }
    pub fn imprime_cpu(&mut self) {
        if self.debug {
            // Crea cursor
            let mut cursor = cursor();

            cursor.hide();

            // Crea entrada a terminal
            let mut stdin = input().read_sync();

            cursor.goto(1, 1);
            println!("A  = 0b{:08b} (0x{:02X})", self.a, self.a);

            cursor.goto(1, 2);
            println!("BC = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.b, self.c, self.b, self.c);
            cursor.goto(1, 3);
            println!("DE = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.d, self.e, self.d, self.e);
            cursor.goto(1, 4);
            println!("HL = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.h, self.l, self.h, self.l);

//            cursor.goto(1, 5);
//            println!("SP = 0x{:04X}", self.sp);

            cursor.goto(15, 5);
            println!("I = 0x{:02X}", self.i);

            cursor.goto(1, 6);
            println!("PC = 0x{:04X}", self.pc);

            cursor.goto(1, 8);
            println!("{}FLAGS{}      Z = {:#?} N = {:#?}",
                     Colored::Fg(Color::Red),
                     Colored::Fg(Color::White),
                     self.get_z_flag(),
                     self.get_n_flag());
            cursor.goto(1, 9);
            println!("           H = {:#?} C = {:#?}",
                     self.get_h_flag(),
                     self.get_c_flag());

            self.obtiene_intruccion_y_bytes_posteriores();
            self.funciones_txt[self.r0 as usize](self);


            cursor.goto(10, 10);
        }
    }
}
