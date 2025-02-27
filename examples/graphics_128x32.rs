//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Run on a Blue Pill with `cargo run --example graphics_128x32`.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::PrimitiveStyle,
};
use panic_semihosting as _;
use sh1107::{prelude::*, Builder};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let yoffset = 8;

    Line::new(
        Point::new(8, 16 + yoffset),
        Point::new(8 + 16, 16 + yoffset),
    )
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut disp)
    .unwrap();

    Line::new(Point::new(8, 16 + yoffset), Point::new(8 + 8, yoffset))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut disp)
        .unwrap();

    Line::new(Point::new(8 + 16, 16 + yoffset), Point::new(8 + 8, yoffset))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut disp)
        .unwrap();

    Rectangle::new(Point::new(48, yoffset), Point::new(48 + 16, 16 + yoffset))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(96, yoffset + 8), 8)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
