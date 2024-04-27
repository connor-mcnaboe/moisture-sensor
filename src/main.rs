use rppal::i2c::I2c;
use std::error::Error;
use std::thread;
use std::time::Duration;

// ADS1115 default I2C address
const ADS1115_ADDRESS: u16 = 0x48;

// ADS1115 Register pointers
const ADS1115_REG_POINTER_CONVERT: u8 = 0x00;
const ADS1115_REG_POINTER_CONFIG: u8 = 0x01;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize I2C
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(ADS1115_ADDRESS)?;

    // Configure ADC to read from A0
    config_adc_single_ended(&mut i2c, 0)?;

    loop {
        // Read the conversion result
        let adc_value = read_adc_value(&mut i2c)?;
        println!("ADC Value from A0: {}", adc_value);
        thread::sleep(Duration::from_secs(1));
    }
}

fn config_adc_single_ended(i2c: &mut I2c, channel: u8) -> Result<(), Box<dyn Error>> {
    let config_high = 0b1000_1001 | (channel << 4);  // OS = 1, MUX = 100, PGA = 010, MODE = 1
    let config_low = 0b1000_0011; // DR = 100, COMP_MODE = 0, COMP_POL = 0, COMP_LAT = 0, COMP_QUE = 11
    i2c.block_write(ADS1115_REG_POINTER_CONFIG, &[config_high, config_low])?;
    Ok(())
}

fn read_adc_value(i2c: &mut I2c) -> Result<i16, Box<dyn Error>> {
    // Ensure that configuration and triggering of new conversion happens each time
    config_adc_single_ended(i2c, 0)?; // Re-configure and trigger a new conversion
    thread::sleep(Duration::from_millis(10)); // Wait for conversion to complete

    let mut buf = [0; 2];
    i2c.write(&[ADS1115_REG_POINTER_CONVERT])?;
    i2c.read(&mut buf)?;
    let result = i16::from_be_bytes(buf);
    Ok(result)
}