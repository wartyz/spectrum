#![allow(non_snake_case)]

/*pub const ANCHO_PANTALLA: usize = 160 * 2;
pub const ALTO_PANTALLA: usize = 144 * 2;


//Colores ALPHA,R,G,B
pub const VERDE_MUY_OSCURO: u32 = 0xFF0F380F;
pub const VERDE_OSCURO: u32 = 0xFF306230;
pub const VERDE_ILUMINADO: u32 = 0xFF8BAC0F;
pub const VERDE_MUY_ILUMINADO: u32 = 0xFF9BBC0F;*/

//use minifb::{Key, Window, WindowOptions};
use image::{ImageBuffer, RgbImage};

use main::cpu::CPU;
use main::mem::MEM;

use crossterm::*;

use std::fs::File;
use std::io::Read;
use main::procesador::PROCESADOR;
use main::hardware::Hardware;
use std::{thread, time};

fn main() {
    let procesador = PROCESADOR::SharpLr35902; // <---------------- OJO ------


    // Lee el fichero ROM
    let mut f = File::open("src/ROMS/Tetris (World).gb").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file).unwrap();


    // Pone el fichero ROM en la memoria RAM
    let mut mem = MEM::new(procesador);
    mem.rellena_mem_desde_fichero_rom(&rom_file);
    mem.cierra_rom();


    let mut cpu = CPU::new(mem, procesador);
    //cpu.establece_debug();

    //cpu.limpia_consola();

    let mut hardware = Hardware::new(&cpu);
    // Crea screen y ventana


//    let mut screen = vec![VERDE_MUY_ILUMINADO;
//                          hardware.get_ancho_pixels() * hardware.get_alto_pixels()];
//    let mut window = Window::new(
//        "Ventana - ESC para salir",
//        ANCHO_PANTALLA,
//        ALTO_PANTALLA,
//        WindowOptions::default(),
//    ).unwrap_or_else(|e| { panic!("{}", e); });

    println!("Presionar una tecla para empezar (x para salir)");
    //while input().read_char().unwrap() != 'x' {

    //let viewport: Vec<u32> = vec![VERDE_ILUMINADO; ANCHO_PANTALLA * ALTO_PANTALLA];
    //while window.is_open() && !window.is_key_down(Key::Escape) {

    loop {
        //if window.is_open() && window.is_key_down(Key::T) {
        if cpu.pc > 0x000C { // BreakPoint
            cpu.establece_debug();


            //if window.is_key_down(Key::T) {

            while input().read_char().unwrap() != 'x' {
                //if cpu.debug {
                cpu.limpia_consola();
                cpu.imprime_cpu();
                cpu.imprime_opcode();
                cpu.imprime_stack();
                cpu.imprime_ports();
                cpu.imprime_memoria(0x0134);
                // }

                cpu.ejecuta_instruccion();
                hardware.ejecuta_hardware(&mut cpu);
            }
            break;

            //let tiempo_retardo = time::Duration::from_millis(100);
            //let now = time::Instant::now();

            //thread::sleep(tiempo_retardo);
        } else {
            cpu.ejecuta_instruccion();
            hardware.ejecuta_hardware(&mut cpu);
        }
    }


    //window.update_with_buffer(hardware.get_viewport()).unwrap();
    //}


    cursor().goto(15, 10);
}
