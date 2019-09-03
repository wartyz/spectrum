#![allow(non_snake_case)]

use crossterm::*;
use crate::mem::MEM;


use crate::instrucciones_basicas::*;
use crate::instrucciones_cb::*;
use crate::instrucciones_ed::*;
use crate::instrucciones_fd::*;
use crate::instrucciones_fdcb::*;
use crate::procesador::PROCESADOR;
use crate::operaciones_binarias::*;

use super::constantes::*;


use std::io::Write;

#[derive(Copy, Clone)]
pub struct Funcion {
    pub puntero_a_funcion: fn(&mut CPU),
    pub puntero_a_funcion_txt: fn(&mut CPU),
    pub bytes: u16,
    pub t: usize,

    // TODO: Flags afectados
}

impl Funcion {
    pub fn new() -> Funcion {
        let mut f = Funcion {
            puntero_a_funcion: fn_no_impl,
            puntero_a_funcion_txt: fn_no_impl,
            bytes: 0,
            t: 0,
        };
        f
    }

    pub fn get_puntero_a_funcion(&self) -> fn(&mut CPU) {
        self.puntero_a_funcion
    }
    pub fn get_puntero_txt_a_funcion(&self) -> fn(&mut CPU) {
        self.puntero_a_funcion_txt
    }
    pub fn get_bytes_de_instruccion(&self) -> u16 {
        self.bytes
    }
    pub fn get_time_de_instruccion(&self) -> usize {
        self.t
    }

    pub fn set_punt_y_val_a_fn(
        &mut self,
        puntero_a_funcion: fn(&mut CPU),
        puntero_a_funcion_txt: fn(&mut CPU),
        bytes: u16,
        t: usize) {
        self.puntero_a_funcion = puntero_a_funcion;
        self.puntero_a_funcion_txt = puntero_a_funcion_txt;
        self.bytes = bytes;
        self.t = t;
    }
}


pub struct CPU {
    pub procesador: PROCESADOR,
    // Registros básicos A F    B C    D E    H L
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    // Registros A' F'   B' C'   D' E'    H' L'
    pub ap: u8,
    pub fp: u8,
    pub bp: u8,
    pub cp: u8,
    pub dp: u8,
    pub ep: u8,
    pub hp: u8,
    pub lp: u8,
    // Registros especiales IX, IY, I, R
    pub ixh: u8,
    pub ixl: u8,
    pub iyh: u8,
    pub iyl: u8,
    pub i: u8,
    pub r: u8,

    // PC=Contador de programa    SP=Stack Pointer
    pub pc: u16,
    pub sp: u16,
    //t -> time cycles
    pub t: usize,

    // Arreglos que contienen objetos función que a su vez contienen datos y punteros
    pub funciones: [Funcion; 256],
    pub funciones_ed: [Funcion; 256],
    pub funciones_cb: [Funcion; 256],
    pub funciones_fd: [Funcion; 256],
    pub funciones_fdcb: [Funcion; 256],

    // Bytes leidos de memoria:
    pub r0: u8,
    pub r1: u8,
    pub r2: u8,
    pub r3: u8,
    pub r1r2: u16,
    pub r2r3: u16,

    // Modo de interrupción
    pub im: u8,
    // Permitir interrupciones
    pub permitida_interrupcion: bool,

    // Habilita debug
    pub debug: bool,

    // Habilita escribir en fichero de salida
    pub file_salida: bool,

    // Memória
    pub mem: MEM,
}

//// FLAGS EN Z80 ****************************
//// Bit           7  6  5  4  3   2   1  0
//// Posicion      S  Z  X  H  X  P/V  N  C   (X = no usado)

impl CPU {
    pub fn new(mem: MEM, procesador: PROCESADOR) -> CPU {

// Rellenamos arreglo de funciones con objetos Funcion

        let funciones: [Funcion; 256] = [Funcion::new(); 256];
        let funcionesED: [Funcion; 256] = [Funcion::new(); 256];
        let funcionesCB: [Funcion; 256] = [Funcion::new(); 256];
        let funcionesFD: [Funcion; 256] = [Funcion::new(); 256];
        let funcionesFDCB: [Funcion; 256] = [Funcion::new(); 256];

        let mut cpu = CPU {
// OJO Cambiar si se usa otro procesador!
            procesador: procesador,
            a: 0xFF,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0xFF,
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
            ixh: 0,
            ixl: 0,
            iyh: 0,
            iyl: 0,
            i: 0,
            r: 0,
            pc: 0,
            sp: 0xFFFF,
            t: 0,
            funciones: funciones,
            funciones_ed: funcionesED,
            funciones_cb: funcionesCB,
            funciones_fd: funcionesFD,
            funciones_fdcb: funcionesFDCB,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r1r2: 0,
            r2r3: 0,
            im: 0,
            permitida_interrupcion: false,
            debug: false,
            file_salida: false,
            mem: mem,

        };
        mete_funciones_normales(&mut cpu);
        mete_funciones_cb(&mut cpu);
        mete_funciones_ed(&mut cpu);
        mete_funciones_fd(&mut cpu);
        mete_funciones_fdcb(&mut cpu);


        cpu
    }


// FLAGS EN Z80 ****************************
// Bit           7  6  5  4  3   2   1  0
// Posicion      S  Z  X  H  X  P/V  N  C   (X = no usado)
// FLAGS EN SharpLr35902 ****************************
// Bit           7  6  5  4  3   2   1  0
// Posicion      Z  N  H  C  0   0   0  0

    // Funciones GET de FLAGS
    /// Función general que usan las demás funciones de flag
    /// Recibe una máscara indicando el flag a leer y devuelve true o false segun sea 1 o 0
    ///

    pub fn get_flag(&self, bit_mask: u8) -> bool {
        (self.f & bit_mask) != 0
    }
    pub fn get_s_flag(&self) -> bool { self.get_flag(FLAG_S) }
    pub fn get_z_flag(&self) -> bool {
        self.get_flag(FLAG_Z)
    }
    pub fn get_h_flag(&self) -> bool {
        self.get_flag(FLAG_H)
    }
    pub fn get_pv_flag(&self) -> bool {
        self.get_flag(FLAG_PV)
    }
    pub fn get_n_flag(&self) -> bool {
        self.get_flag(FLAG_N)
    }
    pub fn get_c_flag(&self) -> bool { self.get_flag(FLAG_C) }


    // FUNCIONES ARITMETICAS **************************************
    /*
    El flag P/V tiene dos funciones:
    Paridad (para instrucciones lógicas) y
    oVerflow (para instrucciones aritméticas)

    para instrucciones aritméticas de 8-bits el Z80 assume que todos los operandos son enteros
    con signo en el flag de overflow.

    The algorithm for calculating P/V flag for ADD instruction is:

    if (((reg_a ^ operand) & 0x80) == 0 /* Same sign */
    && ((reg_a ^ result) & 0x80) != 0) /* Not same sign */
    {
       overflow = 1;
    } else {
      overflow = 0;
    }

    Para una instrucción SUB es:
    0x80 = 0b1000_0000
    if (((reg_a ^ operand) & 0x80) != 0 /* Not same sign */
    && ((operand ^ result) & 0x80) == 0) /* Same sign */
    {
         overflow = 1;
    } else {
         overflow = 0;
    }

    In fact even for INC (going from $7f to $80) and DEC (going from $80 to $7f) is calculated this overflow flag.

    De hecho, incluso para INC (que va de $ 7f a $ 80f) y DEC (que va de $ 80 a $ 7f) se calcula este indicador de desbordamiento.

    */

    // Funcion que usan las funciones de manejo de flags
    pub fn set_flag(&mut self, flag: u8, condicion: bool) {
        if condicion {
            self.f = self.f | flag
        } else {
            self.f = self.f & (!flag);
        }
    }


    // Devuelve registros multiples concatenando los registros simples
    pub fn lee_af(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.a, self.f) }

    pub fn lee_afp(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.ap, self.fp) }

    pub fn lee_bc(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.b, self.c) }

    pub fn lee_bcp(&mut self) -> u16 {
        concatena_dos_u8_en_un_u16(self.bp, self.cp)
    }

    pub fn lee_de(&mut self) -> u16 {
        concatena_dos_u8_en_un_u16(self.d, self.e)
    }

    pub fn lee_dep(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.dp, self.ep) }

    pub fn lee_hl(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.h, self.l) }

    pub fn lee_hlp(&mut self) -> u16 { concatena_dos_u8_en_un_u16(self.hp, self.lp) }

    pub fn lee_ix(&mut self) -> u16 {
        concatena_dos_u8_en_un_u16(self.ixh, self.ixl)
    }

    pub fn lee_iy(&mut self) -> u16 {
        concatena_dos_u8_en_un_u16(self.iyh, self.iyl)
    }

    // Recibe un u16 de registros en pareja y escribe en cada uno de ellos
    pub fn escribe_af(&mut self, af: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(af);
        self.f = hltupla.1;
        self.a = hltupla.0;
    }

    pub fn escribe_afp(&mut self, afp: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(afp);
        self.ap = hltupla.1;
        self.fp = hltupla.0;
    }

    pub fn escribe_bc(&mut self, bc: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(bc);
        self.c = hltupla.1;
        self.b = hltupla.0;
    }

    pub fn escribe_bcp(&mut self, bcp: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(bcp);
        self.cp = hltupla.1;
        self.bp = hltupla.0;
    }

    pub fn escribe_de(&mut self, de: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(de);
        self.e = hltupla.1;
        self.d = hltupla.0;
    }

    pub fn escribe_dep(&mut self, dep: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(dep);
        self.ep = hltupla.1;
        self.dp = hltupla.0;
    }

    pub fn escribe_hl(&mut self, hl: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(hl);
        self.l = hltupla.1;
        self.h = hltupla.0;
    }

    pub fn escribe_hlp(&mut self, hlp: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(hlp);
        self.lp = hltupla.1;
        self.hp = hltupla.0;
    }

    pub fn escribe_ix(&mut self, ix: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(ix);
        self.ixl = hltupla.1;
        self.ixh = hltupla.0;
    }

    pub fn escribe_iy(&mut self, iy: u16) {
        let hltupla = desconcatena_un_u16_en_dos_u8(iy);
        self.iyl = hltupla.1;
        self.iyh = hltupla.0;
    }

    // FUNCIONES DE STACK ********************************************************
    /// Pone en el stack un valor de 16 bits y modifica el puntero
    pub fn push(&mut self, addr: u16) {
        let addr_tupla = desconcatena_un_u16_en_dos_u8(addr);

//        println!("push pc = {:04X}  sp = {:04X}  spL->{:02X}  spH->{:02X}",
//                 self.pc, self.sp,
//                 addr_tupla.1,
//                 addr_tupla.0);

        self.sp -= 1;
        self.mem.escribe_byte_en_mem(self.sp, addr_tupla.0);
        self.sp -= 1;
        self.mem.escribe_byte_en_mem(self.sp, addr_tupla.1);
    }

    /// Saca del stack un valor de 16 bits y modifica el puntero
    pub fn pop(&mut self) -> u16 {
        let addr_bajo = self.mem.lee_byte_de_mem(self.sp);
        self.sp += 1;
        let addr_alto = self.mem.lee_byte_de_mem(self.sp);
        self.sp += 1;
//        println!("pop pc = {:04X}  sp = {:04X}  spL->{:02X}  spH->{:02X} ",
//                 self.pc, self.sp,
//                 addr_bajo,
//                 addr_alto);


        let addr = concatena_dos_u8_en_un_u16(addr_alto, addr_bajo);
        addr
    }

// FIN de FUNCIONES DE STACK **********************************


    // FUNCIONES DE DEBUG *******************************************
    pub fn establece_debug(&mut self) {
        self.debug = true;
    }

    pub fn quita_debug(&mut self) {
        self.debug = false;
    }


    // FUNCIONES DE LOS DATOS DEL ARREGLO DE FUNCIONES ***************
    pub fn get_objeto_funcion_segun_arreglo(&mut self) -> Funcion {
        let mut f: Funcion;
        match self.r0 {
            0xED => {
                f = self.funciones_ed[self.r1 as usize];
            }
            0xCB => {
                f = self.funciones_cb[self.r1 as usize];
            }
            0xFD => {
                match self.r1 {
                    0xCB => { f = self.funciones_fdcb[self.r3 as usize]; }
                    _ => { f = self.funciones_fd[self.r1 as usize]; }
                }
            }
            _ => {
                f = self.funciones[self.r0 as usize];
            }
        }
        f
    }

    pub fn get_t_instruccion(&mut self) -> usize {
        self.get_objeto_funcion_segun_arreglo().get_time_de_instruccion()
    }

    pub fn get_bytes_instruccion(&mut self) -> u16 {
        self.get_objeto_funcion_segun_arreglo().get_bytes_de_instruccion()
    }
// FIN FUNCIONES DE LOS DATOS DEL ARREGLO DE FUNCIONES ***************


    pub fn ejecuta_instruccion(&mut self) {
        self.obtiene_intruccion_y_bytes_posteriores();

        // Ejecuta instruccion
        //self.funciones[self.r0 as usize](self);
        let f: Funcion = self.funciones[self.r0 as usize];
        let ff = f.get_puntero_a_funcion();
        //DESCOMENTAR ESTA LINEA PARA VER EL DEBUG DE LAS INSTRUCCIONES
//        println!("PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}   r3 = #{:02X}",
//                 self.pc, self.r0, self.r1, self.r2, self.r3);

        ff(self);
        //self.funciones[self.r0 as usize](self);
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

//        // TODO: Proteccion de memoria provisional
//        if self.pc > 0x386D {
//
//            panic!(format!("Intento de leer una instruccion en zona superior a 0x386E\n\
//    PC = #{:04X}  r0 = #{:02X}  r1 = #{:02X}  r2 = #{:02X}  \
//    r3 = #{:02X}\n",
//                           self.pc, self.r0, self.r1, self.r2, self.r3));
//        }
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
            print!("|  |-> ");
            println!("{}", txt);
        }
        cursor().goto(100, 20);
    }

    pub fn imprime_opcode(&mut self) {
        if self.debug {
            let bytes = self.get_objeto_funcion_segun_arreglo().get_bytes_de_instruccion();

            for lin in 0..=4 {
                cursor().goto(53, lin + 1);
                if lin < bytes {
                    print!("{}{:02X}{}",
                           Colored::Fg(Color::Red),
                           self.mem.lee_byte_de_mem(self.pc + lin),
                           Colored::Fg(Color::White));
                } else {
                    print!("{}{:02X}",
                           Colored::Fg(Color::White),
                           self.mem.lee_byte_de_mem(self.pc + lin));
                }
            }
        }
        cursor().goto(100, 20);
    }

    pub fn imprime_stack(&mut self) {
        if self.debug {
            cursor().goto(24, 7);
            print!("STACK {:04X}", self.sp);
            cursor().goto(34, 7);
            if self.sp < 0xFFFE {
                print!("->{:02X} {:02X}",
                       self.mem.lee_byte_de_mem(self.sp + 2),
                       self.mem.lee_byte_de_mem(self.sp + 1));
            } else {
                print!("->{:04X} ", 0x0000);
            }
        }
        cursor().goto(100, 20);
    }

    pub fn imprime_memoria(&mut self, inicio: u16) {
        if self.debug {
// Crea cursor
            let mut cursor = cursor();

            cursor.hide();


            for col in 0..=15 {
                for lin in 0..=15 {
                    cursor.goto(78, lin + 1);
                    print!("{:04X}", inicio + (lin * 16));
                    cursor.goto(84 + 3 * col, lin + 1);

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

            cursor.goto(1, 12);
            print!("{}PORTS{}",
                   Colored::Fg(Color::Blue),
                   Colored::Fg(Color::White), );
            for col in 0..=15 {
                for lin in 0..=15 {
                    cursor.goto(1, lin + 13);
                    print!("{:04X}", lin * 16);
                    cursor.goto(8 + 3 * col, lin + 13);

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
//let mut stdin = input().read_sync();

            cursor.goto(1, 1);
            println!("AF = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.a, self.f, self.a, self.f);
            cursor.goto(1, 2);
            println!("BC = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.b, self.c, self.b, self.c);
            cursor.goto(1, 3);
            println!("DE = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.d, self.e, self.d, self.e);
            cursor.goto(1, 4);
            println!("HL = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.h, self.l, self.h, self.l);
            cursor.goto(1, 5);
            println!("IX = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.ixh, self.ixl, self.ixh,
                     self.ixl);
            cursor.goto(1, 6);
            println!("IY = 0b{:08b} 0b{:08b} (0x{:02X}) (0x{:02X})", self.iyh, self.iyl, self.iyh,
                     self.iyl);


//            cursor.goto(1, 5);
//            println!("SP = 0x{:04X}", self.sp);

//cursor.goto(15, 5);
//println!("I = 0x{:02X}", self.i);

            cursor.goto(1, 7);
            println!("PC = 0x{:04X}", self.pc);

            cursor.goto(1, 9);

            println!("{}FLAGS{}    S   Z   H  P/V  N   C",
                     Colored::Fg(Color::Red),
                     Colored::Fg(Color::White));
            let mut s: u8 = 0;
            if self.get_s_flag() { s = 1; }
            let mut z: u8 = 0;
            if self.get_z_flag() { z = 1; }
            let mut h: u8 = 0;
            if self.get_h_flag() { h = 1; }
            let mut pv: u8 = 0;
            if self.get_pv_flag() { pv = 1; }
            let mut n: u8 = 0;
            if self.get_n_flag() { n = 1; }
            let mut c: u8 = 0;
            if self.get_c_flag() { c = 1; }


            cursor.goto(1, 10);
            println!("         {}   {}   {}   {}   {}   {}", s, z, h, pv, n, c);

            cursor.goto(40, 10);
            if self.procesador == PROCESADOR::SharpLr35902 {
                let mut f = 0;
                if self.get_z_flag() { f += 0x80 }
                if self.get_n_flag() { f += 0x40 }
                if self.get_h_flag() { f += 0x20 }
                if self.get_c_flag() { f += 0x10 }

                println!("F en Gameboy = 0x{:02X}", f);
            }

//            println!("{}FLAGS{}      Z = {:#?} N = {:#?}",
//                     Colored::Fg(Color::Red),
//                     Colored::Fg(Color::White),
//                     self.get_z_flag(),
//                     self.get_n_flag());
//            cursor.goto(1, 9);
//            println!("           H = {:#?} C = {:#?}",
//                     self.get_h_flag(),
//                     self.get_c_flag());

            self.obtiene_intruccion_y_bytes_posteriores();

            // Ejecuta instruccion _txt
            self.get_objeto_funcion_segun_arreglo().get_puntero_txt_a_funcion()(self);

            cursor.goto(10, 10);
        }
        cursor().goto(100, 20);
    }

    // IMPRIMIR EN FICHERO *************************************
    pub fn habilita_imprimir_en_fichero(&mut self) {
        self.file_salida = true;
    }
    pub fn deshabilita_imprimir_en_fichero(&mut self) {
        self.file_salida = false;
    }

    pub fn imprime_en_fichero1(&mut self, file: &mut std::fs::File) {
        //let bytes = self.get_objeto_funcion_segun_arreglo().get_bytes_de_instruccion();

        let opcode0 = format!("{:02X}", self.mem.lee_byte_de_mem(self.pc));
        let opcode1 = format!("{:02X}", self.mem.lee_byte_de_mem(self.pc + 1));
        let opcode2 = format!("{:02X}", self.mem.lee_byte_de_mem(self.pc + 2));
        let opcode3 = format!("{:02X} ", self.mem.lee_byte_de_mem(self.pc + 3));

        //let f: Funcion = self.funciones[self.mem.lee_byte_de_mem(self.pc) as usize];
        let f: Funcion = self.get_objeto_funcion_segun_arreglo();
        let bytes = f.get_bytes_de_instruccion();
//        let opcode = match bytes {
//            1 => format!("{}", opcode0),
//            2 => format!("{} {}", opcode0, opcode1),
//            3 => format!("{} {} {}", opcode0, opcode1, opcode2),
//            4 => format!("{} {} {} {}", opcode0, opcode1, opcode2, opcode3),
//            _ => format!("Error en imprime fichero r0={:02X} r1={:02X} bytes={}", self.r0, self.r1,
//                         bytes),
//        };
        let opcode = format!("{} {} {} {}", opcode0, opcode1, opcode2, opcode3);
        let texto = format!("0/{:04X}   {}\n", self.pc, opcode);
        file.write_all(texto.as_bytes()).expect("fallo al escribir");
    }

    pub fn imprime_en_fichero2(&mut self, file: &mut std::fs::File) {
        let mut s = 0;
        if self.get_s_flag() { s = 1; } else { s = 0; };
        let mut z = 0;
        if self.get_z_flag() { z = 1; } else { z = 0; };
        let mut h = 0;
        if self.get_h_flag() { h = 1; } else { h = 0; };
        let mut p = 0;
        if self.get_pv_flag() { p = 1; } else { p = 0; };
        let mut n = 0;
        if self.get_n_flag() { n = 1; } else { n = 0; };
        let mut c = 0;
        if self.get_c_flag() { c = 1; } else { c = 0; };

        let flags = format!("S={} Z={} H={} P/O={} N={} C={}", s, z, h, p, n, c);
        let registros =
            format!(
                "A={:02X} BC={:02X}{:02X} DE={:02X}{:02X} HL={:02X}{:02X} SP={:04X} PC={:04X}",
                self.a,
                self.b,
                self.c,
                self.d,
                self.e,
                self.h,
                self.l,
                self.sp,
                self.pc);

        let texto = format!("{} {}
   PSW=3C      PSW=00     A=00 BC=0000 DE=0000 HL=0000 IX={:02X}{:02X} IY={:02X}{:02X}\n",
                            flags,
                            registros,
                            self.ixh, self.ixl,
                            self.iyh, self.iyl,
        );


//        let texto = "0 / 11E2   A7    and    a
//S=0 Z=0 H=1 P/O=1 N=0 C=0 A=3F BC=0000 DE=FFFF HL=3FFF SP=FFFF PC=11E3
//   PSW=3C      PSW=00     A=00 BC=0000 DE=0000 HL=0000 IX=0000 IY=0000\n";
        file.write_all(texto.as_bytes()).expect("fallo al escribir");
        //file.write_all("\nSegunda parte".as_bytes()).expect("fallo al escribir");
    }
}
