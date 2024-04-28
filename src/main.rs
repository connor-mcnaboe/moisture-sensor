use std::{error::Error, thread};

use rppal::i2c::I2c;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::time::Duration;

use moisture_sensor::itc_config::{
    ADS1115_ADDRESS, ADS1115_REG_POINTER_CONFIG, ADS1115_REG_POINTER_CONVERT, CONFIG_HIGH,
    CONFIG_LOW,
};
use moisture_sensor::tcp_server_config::{SERVER_ADDRESS, SERVER_PORT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("{SERVER_ADDRESS}:{SERVER_PORT}")).await?;
    let (tx, _rx) = broadcast::channel(10); // buffer size of 10

    // Spawn a separate thread for reading ADC
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut i2c = I2c::new().expect("Failed to initialize I2C");
        i2c.set_slave_address(ADS1115_ADDRESS)
            .expect("Failed to set slave address");
        config_adc_single_ended(&mut i2c, 0).expect("Failed to configure ADC");

        loop {
            let adc_value = read_adc_value(&mut i2c).expect("Failed to read ADC value");
            tx_clone
                .send(format!("ADC Value from A0: {}", adc_value))
                .expect("Failed to send ADC value");
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client: {:?}", addr);
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            loop {
                if let Ok(msg) = rx.recv().await {
                    if socket.write_all(msg.as_bytes()).await.is_err() {
                        break; // Client has disconnected
                    }
                }
            }
            println!("Client {:?} has disconnected", addr);
        });
    }
}

fn config_adc_single_ended(i2c: &mut I2c, channel: u8) -> Result<(), Box<dyn Error>> {
    i2c.block_write(
        ADS1115_REG_POINTER_CONFIG,
        &[CONFIG_HIGH | (channel << 4), CONFIG_LOW],
    )?;
    Ok(())
}

fn read_adc_value(i2c: &mut I2c) -> Result<i16, Box<dyn Error>> {
    // Ensure that configuration and triggering of new conversion happens each time
    config_adc_single_ended(i2c, 0)?; // Re-configure and trigger a new conversion
    thread::sleep(Duration::from_millis(10)); // Wait for conversion to complete

    let mut buf = [0; 2]; // 16 Bit, 2 byte buffer.
    i2c.write(&[ADS1115_REG_POINTER_CONVERT])?;
    i2c.read(&mut buf)?;
    let result = i16::from_be_bytes(buf);
    Ok(result)
}
