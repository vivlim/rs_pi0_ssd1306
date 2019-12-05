use rppal::gpio::Gpio;
use rppal::i2c::I2c;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics::{text_6x8, egcircle};
use ssd1306::prelude::*;
use ssd1306::Builder;

fn main() -> Result<(), failure::Error> {
    let gpio = Gpio::new()?;
    let i2c = I2c::new()?;

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();

    let c = egcircle!((20, 20), 8, fill = Some(BinaryColor::On));
    let t = text_6x8!("Hello Rust!", fill = Some(BinaryColor::On)).translate(Point::new(20, 36));

    disp.draw(c.into_iter());
    disp.draw(t);

    disp.flush().unwrap(); // Fail not implemented for this error kind

    Ok(())

}