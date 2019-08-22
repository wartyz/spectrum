#![allow(non_snake_case)]

use main::cpu::CPU;
use main::mem::MEM;

use crossterm::*;

use std::fs::File;
use std::io::Read;
use main::procesador::PROCESADOR;
use main::hardware::Hardware;

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


    println!("Presionar una tecla para empezar (x para salir)");
    //while input().read_char().unwrap() != 'x' {

    loop {
        if cpu.pc == 0x00F4 { // BreakPoint
            cpu.establece_debug();
            while input().read_char().unwrap() != 'x' {
                cpu.limpia_consola();
                cpu.imprime_cpu();
                cpu.imprime_opcode();
                cpu.imprime_stack();
                cpu.imprime_ports();
                cpu.imprime_memoria(0x0134);

                cpu.ejecuta_instruccion();
                hardware.ejecuta_hardware(&mut cpu);
            }
            break;
        } else {
            cpu.ejecuta_instruccion();
            hardware.ejecuta_hardware(&mut cpu);
        }
    }

    cursor().goto(15, 10);
}
