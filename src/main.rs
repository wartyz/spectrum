#![allow(non_snake_case)]

use main::cpu::CPU;
use main::mem::MEM;

use crossterm::*;

use std::fs::File;
use std::io::Read;

fn main() {
    // Lee el fichero ROM
    let mut f = File::open("src/ROMS/ZXSpectrum48.rom").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file).unwrap();

    // Pone el fichero ROM en la memoria RAM
    let mut mem = MEM::new();
    mem.rellena_mem_desde_fichero_rom(&rom_file);

    let mut cpu = CPU::new(mem);
    //cpu.establece_debug();

    //cpu.limpia_consola();
    println!("Presionar una tecla para empezar (x para salir)");
    //while input().read_char().unwrap() != 'x' {

    loop {
        if cpu.pc == 0x0EEB { // BreakPoint
            cpu.establece_debug();
            while input().read_char().unwrap() != 'x' {
                cpu.limpia_consola();
                cpu.imprime_cpu();
                cpu.imprime_opcode();
                cpu.imprime_stack();
                cpu.imprime_ports();
                cpu.imprime_memoria(0xFF00);

                cpu.ejecuta_instruccion();
            }
            break;
        } else {
            cpu.ejecuta_instruccion();
        }
    }

    cursor().goto(15, 10);
}
