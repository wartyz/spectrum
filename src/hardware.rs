#![allow(non_snake_case)]

pub const ANCHO_PANTALLA_GB: usize = 160 * 2;
pub const ALTO_PANTALLA_GB: usize = 144 * 2;

pub const ANCHO_PANTALLA_Z80: usize = 0;
pub const ALTO_PANTALLA_Z80: usize = 0;

use crate::cpu::CPU;
use crate::procesador::PROCESADOR;


// Aqui se crean las iteraciones del hardware con la memoria o la cpu
#[derive(Copy, Clone)]
pub enum CHIPSET {
    ChipSetZ80,
    ChipSetSharpLr35902,
}

struct HwZ80 {}

impl HwZ80 {
    pub fn new() -> HwZ80 {
        HwZ80 {}
    }
}

struct HwSharpLr35902 {
    ly: u8,

}

impl HwSharpLr35902 {
    pub fn new() -> HwSharpLr35902 {
        HwSharpLr35902 {
            ly: 0,

        }
    }
}

pub struct Hardware {
    chipset: CHIPSET,
    ancho_pixels: usize,
    alto_pixels: usize,
    hw_z80: HwZ80,
    hw_sharp_lr_35902: HwSharpLr35902,
    viewport: Vec<u32>,
}

impl Hardware {
    pub fn new(cpu: &CPU) -> Hardware {
        match cpu.procesador {
            PROCESADOR::SharpLr35902 => {
                Hardware {
                    chipset: CHIPSET::ChipSetSharpLr35902,
                    ancho_pixels: ANCHO_PANTALLA_GB,
                    alto_pixels: ALTO_PANTALLA_GB,
                    hw_z80: HwZ80::new(),
                    hw_sharp_lr_35902: HwSharpLr35902::new(),
                    viewport: vec![60; ANCHO_PANTALLA_GB * ALTO_PANTALLA_GB],
                }
            }
            PROCESADOR::Z80 => {
                Hardware {
                    chipset: CHIPSET::ChipSetZ80,
                    ancho_pixels: ANCHO_PANTALLA_Z80,
                    alto_pixels: ALTO_PANTALLA_Z80,
                    hw_z80: HwZ80::new(),
                    hw_sharp_lr_35902: HwSharpLr35902::new(),
                    viewport: vec![0; ANCHO_PANTALLA_Z80 * ALTO_PANTALLA_Z80],
                }
            }
            _ => panic!("Procesador no reconocido en hardware")
        }
    }
    pub fn get_viewport(&mut self) -> &Vec<u32> {
        &self.viewport
    }

    pub fn get_ancho_pixels(&mut self) -> usize {
        self.ancho_pixels
    }
    pub fn get_alto_pixels(&mut self) -> usize {
        self.alto_pixels
    }

    pub fn ejecuta_hardware(&mut self, cpu: &mut CPU) {
        match self.chipset {
            CHIPSET::ChipSetSharpLr35902 => self.hardware_GB(cpu),
            CHIPSET::ChipSetZ80 => self.hardware_Z80(cpu),
            _ => panic!("Chipset no reconocido en hardware")
        };
    }

    pub fn hardware_GB(&mut self, cpu: &mut CPU) {
        if cpu.t % 20 == 1 {
            self.hw_sharp_lr_35902.ly += 1;
        }
        if self.hw_sharp_lr_35902.ly == 154 {
            self.hw_sharp_lr_35902.ly = 0;
        }
        cpu.mem.escribe_byte_en_mem(0xFF44, self.hw_sharp_lr_35902.ly);
    }

    pub fn hardware_Z80(&mut self, cpu: &mut CPU) {}
}