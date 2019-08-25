pub const ANCHO_PANTALLA: usize = 160 * 2;
pub const ALTO_PANTALLA: usize = 144 * 2;


//Colores ALPHA,R,G,B
pub const VERDE_MUY_OSCURO: u32 = 0xFF0F380F;
pub const VERDE_OSCURO: u32 = 0xFF306230;
pub const VERDE_ILUMINADO: u32 = 0xFF8BAC0F;
pub const VERDE_MUY_ILUMINADO: u32 = 0xFF9BBC0F;


// FLAGS EN Z80 ****************************
// Bit           7  6  5  4  3   2   1  0
// Posicion      S  Z  X  H  X  P/V  N  C   (X = no usado)
// Mascaras de flags
pub const FLAG_C: u8 = 1u8 << 0;
pub const FLAG_N: u8 = 1u8 << 1;
pub const FLAG_PV: u8 = 1u8 << 2;
pub const FLAG_H: u8 = 1u8 << 4;
pub const FLAG_Z: u8 = 1u8 << 6;
pub const FLAG_S: u8 = 1u8 << 7;