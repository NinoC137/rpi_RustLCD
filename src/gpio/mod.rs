use std::{fs, path::Path};

pub trait OutputPin {
    fn set_high(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn set_low(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SysfsGpioPin {
    pin: u8,
}

impl SysfsGpioPin {
    pub fn new(pin: u8) -> Result<Self, Box<dyn std::error::Error>> {
        let gpio_dir = format!("/sys/class/gpio/gpio{}", pin);
        if !Path::new(&gpio_dir).exists() {
            let _ = fs::write("/sys/class/gpio/export", format!("{}", pin));
        }
        fs::write(format!("{}/direction", gpio_dir), "out")?;
        Ok(Self { pin })
    }

    fn write_value(&self, v: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(format!("/sys/class/gpio/gpio{}/value", self.pin), v)?;
        Ok(())
    }
}

impl OutputPin for SysfsGpioPin {
    fn set_high(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_value("1")
    }

    fn set_low(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_value("0")
    }
}
