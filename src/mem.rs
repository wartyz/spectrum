use crossterm::*;

pub struct MEM {
    ram: [u8; 65_536],
    ports: [u8; 256],
}


impl MEM {
    pub fn new() -> MEM {
        let mem = MEM {
            ram: [0; 65_536],//0x0000 to 0xFFFF
            ports: [0; 256], //0x00 to 0xFF
        };
        mem
    }

    pub fn escribe_byte_en_mem(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn lee_byte_de_mem(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }


    pub fn escribe_byte_en_port(&mut self, address: u8, value: u8) {
        self.ports[address as usize] = value;
    }

    pub fn lee_byte_de_port(&self, address: u8) -> u8 {
        self.ports[address as usize]
    }


    pub fn rellena_mem_desde_fichero_rom(&mut self, rom_file: &[u8]) {
        //let bytes = &rom_file[..rom_file.len()];
        let mut i: u16 = 0x0000;
        for &byte in rom_file.iter() {
            //println!("{:#x}", (byte as u16));
            self.escribe_byte_en_mem(i, byte);
            i += 1;
        }
    }
}