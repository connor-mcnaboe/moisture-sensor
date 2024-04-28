// ADS1115 default I2C address
pub const ADS1115_ADDRESS: u16 = 0x48;

// ADS1115 Register pointers
pub const ADS1115_REG_POINTER_CONVERT: u8 = 0x00;
pub const ADS1115_REG_POINTER_CONFIG: u8 = 0x01;

// Set the config register for single-ended input on A0
// - OS: 1 (start a conversion)
// - MUX: 100 (A0 single-ended)
// - PGA: 010 (±2.048V range)
// - MODE: 1 (single-shot mode)
// - DR: 100 (128 SPS)
// - COMP_MODE, COMP_POL, COMP_LAT, COMP_QUE: 0 (disable comparator)
pub const CONFIG_HIGH: u8 = 0b1000_1001;
pub const CONFIG_LOW: u8 = 0b1000_0011; // DR = 100, COMP_MODE = 0, COMP_POL = 0, COMP_LAT = 0, COMP_QUE = 11
