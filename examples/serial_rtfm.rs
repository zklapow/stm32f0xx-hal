#![no_std]
#![no_main]

#[macro_use]
extern crate nb;
#[allow(unused)]
extern crate panic_halt;

use core::fmt::Write;

use cortex_m::{asm, peripheral::syst::SystClkSource};
use nb::Result as NbResult;
use rtfm::app;
use stm32f0::stm32f0x0;

use stm32f0xx_hal::{
    adc::Adc,
    gpio::{gpioa, gpiob, Alternate, Analog, Input, Output, PushPull, AF1},
    prelude::*,
    rcc::Clocks,
    serial::{self, Error, Rx, Serial, Tx, Event},
    timers,
};

#[app(device = stm32f0::stm32f0x0)]
const APP: () = {
    static mut LED0: gpioa::PA0<Output<PushPull>> = ();
    static mut LED1: gpioa::PA1<Output<PushPull>> = ();

    static mut TX: Tx<stm32f0x0::USART1> = ();
    static mut RX: Rx<stm32f0x0::USART1> = ();

    #[init]
    fn init() {
        let mut dp: stm32f0::stm32f0x0::Peripherals = device;

        let mut rcc: stm32f0xx_hal::rcc::Rcc = dp.RCC.constrain();
        let mut gpioa: gpioa::Parts = dp.GPIOA.split();
        let mut gpiob: gpiob::Parts = dp.GPIOB.split();

        let clocks = rcc.cfgr.hse(16.mhz()).sysclk(32.mhz()).freeze();

        let mut serial = configure_serial(
            gpioa.pa9.into_alternate_af1(),
            gpioa.pa10.into_alternate_af1(),
            dp.USART1,
            clocks,
        );

        serial.listen(Event::Rxne);

        let (mut tx, mut rx) = serial.split();

        let mut led = gpioa.pa0.into_push_pull_output();
        let mut led1 = gpioa.pa1.into_push_pull_output();
        led.set_high();
        led1.set_high();

        LED0 = led;
        LED1 = led1;
        TX = tx;
        RX = rx;
    }

    #[task(capacity = 100, resources = [TX])]
    fn return_byte(byte: u8) {
        block!(resources.TX.write(byte)).ok();
    }

    #[interrupt(spawn = [return_byte], resources = [RX])]
    fn USART1() {
        let data = block!(resources.RX.read()).expect("Failed to read from UART");
        spawn.return_byte(data).ok();
    }

    extern "C" {
        fn TIM17();
    }
};

fn configure_serial(
    txpin: gpioa::PA9<Alternate<AF1>>,
    rxpin: gpioa::PA10<Alternate<AF1>>,
    usart: stm32f0x0::USART1,
    clocks: Clocks,
) -> Serial<stm32f0x0::USART1, gpioa::PA9<Alternate<AF1>>, gpioa::PA10<Alternate<AF1>>> {
    let serial = Serial::usart1(usart, (txpin, rxpin), 115_200.bps(), clocks);

    return serial;
}
