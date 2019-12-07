use rppal::gpio::Gpio;
use rppal::i2c::I2c;

use std::collections::VecDeque;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics::{text_6x8, egcircle};
use ssd1306::prelude::*;
use ssd1306::Builder;

#[derive(EnumIter, Debug, Copy, Clone)]
#[repr(u8)]
pub enum InputKey {
    Left = 27,
    Right = 23,
    Center = 4,
    Up = 17,
    Down = 22,
    A = 5,
    B = 6,
}

#[derive(Debug)]
pub enum KeyEventKind {
    Press, // Key is held down
    Click, // Key is pushed and released
}

pub struct KeyEvent {
    key: InputKey,
    kind: KeyEventKind,
}

pub struct Input {
    input_key: InputKey,
    pin_state: rppal::gpio::Level,
    gpio_pin: rppal::gpio::InputPin,
}

fn dispatch_input_events(inputs: &mut Vec<Input>, gpio: &Gpio, input_events: &mut VecDeque<KeyEvent>, target_view: &mut View) {
    for input in inputs.iter_mut() {
        let current_level = input.gpio_pin.read();
        //println!("pin {:?} state: {:?}", input.input_key, current_level);

        if current_level == rppal::gpio::Level::Low { // pressed state
            input_events.push_back(
                KeyEvent {
                    key: input.input_key,
                    kind: KeyEventKind::Press,
                });
        }

        if current_level != input.pin_state {
            input.pin_state = current_level; // update state

            if current_level == rppal::gpio::Level::High { // released state
                input_events.push_back(
                    KeyEvent {
                        key: input.input_key,
                        kind: KeyEventKind::Click,
                    });
            }
        }
    }

    for event in input_events.drain(..) {
        println!("handling input: {:?} {:?}", event.key, event.kind);
        target_view.handle_key(event);
    }
}

pub struct View {
    circle_x: i32,
    circle_y: i32,
}

impl View {
    fn handle_key(&mut self, event: KeyEvent){
        const dist: i32 = 5;
        match event.kind {
            KeyEventKind::Press => match event.key {
                InputKey::Left => self.circle_x = (self.circle_x - dist) % 128,
                InputKey::Right => self.circle_x = (self.circle_x + dist) % 128,
                InputKey::Up => self.circle_y = (self.circle_y - dist) % 64,
                InputKey::Down => self.circle_y = (self.circle_y + dist) % 64,
            _ => (),
            },
            KeyEventKind::Click => match event.key {
                InputKey::A => println!("handling a click"),
                InputKey::B => println!("handling b click"),
                _ => (),
            }
        }
        println!("circle at ({}, {})", self.circle_x, self.circle_y);
    }

    fn draw<T>(&mut self, disp: &mut GraphicsMode<T>, color: BinaryColor) -> Result<(), T::Error> where T: ssd1306::interface::DisplayInterface { 
        let c = egcircle!((self.circle_x, self.circle_y), 8, fill = Some(color));
        disp.draw(c.into_iter());
        Ok(())
    }
}

fn main() -> Result<(), failure::Error> {
    let gpio = Gpio::new()?;
    let i2c = I2c::new()?;

    let mut inputs: Vec<Input> = 
        InputKey::iter().map(
            |key| Input {
                input_key: key,
                pin_state: rppal::gpio::Level::Low,
                gpio_pin: gpio.get(key as u8).unwrap().into_input_pullup(),
            }).collect();

    let mut input_events: VecDeque<KeyEvent> = VecDeque::new();

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();

    //let t = text_6x8!("Hello Rust!", fill = Some(BinaryColor::On)).translate(Point::new(20, 36));
    
    let mut view = View {
        circle_x: 20,
        circle_y: 20,
    };

    loop {
        view.draw(&mut disp, BinaryColor::Off).unwrap(); // clear previous frame
        dispatch_input_events(&mut inputs, &gpio, &mut input_events, &mut view);
        view.draw(&mut disp, BinaryColor::On).unwrap(); // draw new frame
        disp.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }


    Ok(())

}