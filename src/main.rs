//  This file is part of esc-rs.
//  esc-rs is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  esc-rs is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use stm32f3xx_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::{
    dma::{dma1::C7, Channel, Target},
    pac,
    prelude::*,
    serial::Serial,
};

#[entry]
fn main() -> ! {
    hprintln!("Hello World").unwrap();

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut led = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let pins = (
        gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl),
        gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl),
    );

    // 1 start bit, 8 data bits, n stop bits, parity odd
    // picocom -b 115200 -p 1 -d 8 -y o
    dp.USART2
        .cr1
        .write(|w| w.m().bit8().ps().odd().te().enabled().re().enabled());

    let serial = Serial::usart2(dp.USART2, pins, 115200.bps(), clocks, &mut rcc.apb1);
    let (mut tx, mut rx) = serial.split();

    let tx_buf: &mut _ = singleton!(: [u8; 16] = *b"hello, DMA world").unwrap();
    let rx_buf = singleton!(: [u8; 9] = [0; 9]).unwrap();

    let dma1 = dp.DMA1.split(&mut rcc.ahb);
    let (tx_channel, rx_channel) = (dma1.ch7, dma1.ch6);

    led.set_high().unwrap();
    // let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    // hprintln!("\nsent\n").unwrap();

    let mut buffer: Option<&mut [u8; 16]> = Some(tx_buf);
    let mut channel: Option<C7> = Some(tx_channel);
    let mut tx: Option<_> = Some(tx);
    loop {
        if let (Some(buf), Some(chn), Some(t)) = (buffer.take(), channel.take(), tx.take()) {
            let sending = t.write_all(buf, chn);
            let (buf, chn, t) = sending.wait();
            buffer = Some(buf);
            channel = Some(chn);
            tx = Some(t);
        }
        // for i in 0..5 {
        //     match tx.write(tx_buf[i]) {
        //         Ok(()) => (),
        //         Err(_) => (),
        //     }
        //     match rx.read() {
        //         Ok(r) => {
        //             hprintln!("received: {}", r).unwrap();
        //         }
        //         Err(_) => (),
        //     }
        // }
        // let receiving = rx.read_exact(rx_buf, rx_channel);

        // let (tx_buf, tx_channel, tx) = sending.wait();
        // let (rx_buf, rx_channel, rx) = receiving.wait();

        led.set_low().unwrap();
        delay.delay_ms(2000_u32);
        led.set_high().unwrap();
        delay.delay_ms(2000_u32);
    }
}
