//! Example to communicate with ADS1256 ADC board from Raspberry PI
//! The High-Precision AD/DA board was used for testing.
//! [AD/DA board ]https://www.waveshare.com/wiki/High-Precision_AD/DA_Board
//!

extern crate ads1256;
extern crate linux_embedded_hal as linux_hal;

use linux_hal::spidev::{self, SpidevOptions};
use linux_hal::sysfs_gpio::Direction;
use linux_hal::{Delay, Pin, Spidev};

use ads1256::{Channel, Config, SamplingRate, ADS1256, PGA};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> ! {
    println!("Hello ADS1256 driver..");

    let mut spi = Spidev::open("/dev/spidev0.1").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1500_000)
        .lsb_first(false)
        .mode(spidev::SpiModeFlags::SPI_MODE_1)
        .build();
    spi.configure(&options).unwrap();
    let spi = Arc::new(Mutex::new(spi));

    // init ads1256 chip select pin
    let cs_pin0 = Pin::new(22);
    cs_pin0.export().unwrap();
    while !cs_pin0.is_exported() {}
    cs_pin0.set_direction(Direction::Out).unwrap();
    cs_pin0.set_value(0).unwrap();

    let cs_pin1 = Pin::new(13);
    cs_pin1.export().unwrap();
    while !cs_pin1.is_exported() {}
    cs_pin1.set_direction(Direction::Out).unwrap();
    cs_pin1.set_value(0).unwrap();

    let cs_pin2 = Pin::new(16);
    cs_pin2.export().unwrap();
    while !cs_pin2.is_exported() {}
    cs_pin2.set_direction(Direction::Out).unwrap();
    cs_pin2.set_value(0).unwrap();

    let cs_pin3 = Pin::new(32);
    cs_pin3.export().unwrap();
    while !cs_pin3.is_exported() {}
    cs_pin3.set_direction(Direction::Out).unwrap();
    cs_pin3.set_value(0).unwrap();

    // init ads1256 reset pin
    let rst_pin0 = Pin::new(18);
    rst_pin0.export().unwrap();
    while !rst_pin0.is_exported() {}
    rst_pin0.set_direction(Direction::Out).unwrap();

    let rst_pin1 = Pin::new(6);
    rst_pin1.export().unwrap();
    while !rst_pin1.is_exported() {}
    rst_pin1.set_direction(Direction::Out).unwrap();

    let rst_pin2 = Pin::new(20);
    rst_pin2.export().unwrap();
    while !rst_pin2.is_exported() {}
    rst_pin2.set_direction(Direction::Out).unwrap();

    let rst_pin3 = Pin::new(31);
    rst_pin3.export().unwrap();
    while !rst_pin3.is_exported() {}
    rst_pin3.set_direction(Direction::Out).unwrap();

    // init ads1256 data ready pin
    let drdy_pin0 = Pin::new(17);
    drdy_pin0.export().unwrap();
    while !drdy_pin0.is_exported() {}
    drdy_pin0.set_direction(Direction::In).unwrap();

    let drdy_pin1 = Pin::new(5);
    drdy_pin1.export().unwrap();
    while !drdy_pin1.is_exported() {}
    drdy_pin1.set_direction(Direction::In).unwrap();

    let drdy_pin2 = Pin::new(21);
    drdy_pin2.export().unwrap();
    while !drdy_pin2.is_exported() {}
    drdy_pin2.set_direction(Direction::In).unwrap();

    let drdy_pin3 = Pin::new(30);
    drdy_pin3.export().unwrap();
    while !drdy_pin3.is_exported() {}
    drdy_pin3.set_direction(Direction::In).unwrap();

    // reset the adc
    rst_pin0.set_value(0).unwrap();
    thread::sleep(Duration::from_micros(1)); //t16 delay (0.52us)
    rst_pin0.set_value(1).unwrap();

    rst_pin1.set_value(0).unwrap();
    thread::sleep(Duration::from_micros(1)); //t16 delay (0.52us)
    rst_pin1.set_value(1).unwrap();

    rst_pin2.set_value(0).unwrap();
    thread::sleep(Duration::from_micros(1)); //t16 delay (0.52us)
    rst_pin2.set_value(1).unwrap();

    rst_pin3.set_value(0).unwrap();
    thread::sleep(Duration::from_micros(1)); //t16 delay (0.52us)
    rst_pin3.set_value(1).unwrap();

    // wait for setup
    thread::sleep(Duration::from_millis(200));

    // create adc instance
    let mut adc0 = ADS1256::new(spi.clone(), cs_pin0, rst_pin0, drdy_pin0, Delay).unwrap();
    let config = Config::new(SamplingRate::Sps30000, PGA::Gain1);
    adc0.set_config(&config).unwrap();

    let mut adc1 = ADS1256::new(spi.clone(), cs_pin1, rst_pin1, drdy_pin1, Delay).unwrap();
    adc1.set_config(&config).unwrap();

    let mut adc2 = ADS1256::new(spi.clone(), cs_pin2, rst_pin2, drdy_pin2, Delay).unwrap();
    adc2.set_config(&config).unwrap();

    let mut adc3 = ADS1256::new(spi.clone(), cs_pin3, rst_pin3, drdy_pin3, Delay).unwrap();
    adc3.set_config(&config).unwrap();

    //read all single ended channels in one-shot mode
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("*************************************************");
        for ch in &[
            Channel::AIN0,
            Channel::AIN1,
            Channel::AIN2,
            Channel::AIN3,
            Channel::AIN4,
            Channel::AIN5,
            Channel::AIN6,
            Channel::AIN7,
        ] {
            let code = adc0.read_channel(*ch, Channel::AINCOM).unwrap();
            let in_volt = adc0.convert_to_volt(code);

            println!("ADC0 Channel {:?} : {:#08x}, {:.20} V ", ch, code, in_volt);
        }

        for ch in &[
            Channel::AIN0,
            Channel::AIN1,
            Channel::AIN2,
            Channel::AIN3,
            Channel::AIN4,
            Channel::AIN5,
            Channel::AIN6,
            Channel::AIN7,
        ] {
            let code = adc1.read_channel(*ch, Channel::AINCOM).unwrap();
            let in_volt = adc1.convert_to_volt(code);

            println!("ADC1 Channel {:?} : {:#08x}, {:.20} V ", ch, code, in_volt);
        }

        for ch in &[
            Channel::AIN0,
            Channel::AIN1,
            Channel::AIN2,
            Channel::AIN3,
            Channel::AIN4,
            Channel::AIN5,
            Channel::AIN6,
            Channel::AIN7,
        ] {
            let code = adc2.read_channel(*ch, Channel::AINCOM).unwrap();
            let in_volt = adc2.convert_to_volt(code);

            println!("ADC2 Channel {:?} : {:#08x}, {:.20} V ", ch, code, in_volt);
        }

        for ch in &[
            Channel::AIN0,
            Channel::AIN1,
            Channel::AIN2,
            Channel::AIN3,
            Channel::AIN4,
            Channel::AIN5,
            Channel::AIN6,
            Channel::AIN7,
        ] {
            let code = adc3.read_channel(*ch, Channel::AINCOM).unwrap();
            let in_volt = adc3.convert_to_volt(code);

            println!("ADC3 Channel {:?} : {:#08x}, {:.20} V ", ch, code, in_volt);
        }
    }
}
