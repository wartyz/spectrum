#![allow(non_snake_case)]

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
    hw_z80: HwZ80,
    hw_sharp_lr_35902: HwSharpLr35902,

}

impl Hardware {
    pub fn new(cpu: &CPU) -> Hardware {
        let p = match cpu.procesador {
            PROCESADOR::SharpLr35902 => CHIPSET::ChipSetSharpLr35902,
            PROCESADOR::Z80 => CHIPSET::ChipSetZ80,
            _ => panic!("Procesador no reconocido en hardware")
        };
        let hardware = Hardware {
            chipset: p,
            hw_z80: HwZ80::new(),
            hw_sharp_lr_35902: HwSharpLr35902::new(),
        };
        hardware
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