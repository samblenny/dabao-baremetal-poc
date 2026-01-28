// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright 2026 Sam Blenny
//
#![no_std]
#![no_main]
extern crate dabao_baremetal_poc;
use dabao_baremetal_poc::{gpio, ticktimer, uart};
use gpio::{AF, GpioPin};

/// UART example for bao1x dabao evaluation board
///
/// Initializes UART2 and demonstrates the ticktimer module by repeatedly
/// printing "hello, world!" with the current millisecond timestamp
/// (from the TICKTIMER peripheral). Waits for button press/release cycles
/// on the PROG button (PC13) between prints, using ticktimer::millis() for
/// debouncing. Uses uart::tick() to service the DMA TX queue.
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Configure PB13 and PB14 for UART2
    gpio::set_alternate_function(GpioPin::PortB(gpio::PB13), AF::AF1);
    gpio::set_alternate_function(GpioPin::PortB(gpio::PB14), AF::AF1);

    // Configure PC13 (PROG button) as input with pull-up
    gpio::set_alternate_function(GpioPin::PortC(gpio::PC13), AF::AF0);
    gpio::disable_output(GpioPin::PortC(gpio::PC13));
    gpio::enable_pullup(GpioPin::PortC(gpio::PC13));

    // Initialize UART2
    uart::init();

    loop {
        // Print message prefix
        uart::write(b"hello, world! [millis() = ");

        // Get current time and convert to decimal string
        let ms = ticktimer::millis();
        let mut buf = [0u8; 20];
        let len = format_u64(ms, &mut buf);

        // Print the decimal milliseconds and line ending
        uart::write(&buf[..len]);
        uart::write(b" ms]\r\n");

        // Wait until PC13 is high (button released)
        while gpio::read_input(GpioPin::PortC(gpio::PC13)) == 0 {
            uart::tick();
        }
        debounce(10);

        // Wait until PC13 is low (button pressed)
        while gpio::read_input(GpioPin::PortC(gpio::PC13)) != 0 {
            uart::tick();
        }
        debounce(10);
    }
}

/// Wait for specified milliseconds, servicing UART DMA
fn debounce(ms: u32) {
    let debounce_time = ticktimer::millis() + ms as u64;
    while ticktimer::millis() < debounce_time {
        uart::tick();
    }
}

/// Convert u64 to decimal string, return number of bytes written
///
/// We implement this manually instead of using format!() because this is
/// no_std code without allocator support. The format!() macro requires
/// heap allocation via String, which is not available in bare-metal
/// environments. This function writes directly to a pre-allocated buffer.
fn format_u64(mut value: u64, buf: &mut [u8]) -> usize {
    // Write digits in reverse order
    let mut len = 0;
    loop {
        buf[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
        if value == 0 {
            break;
        }
    }

    // Reverse the buffer to get correct digit order
    buf[..len].reverse();

    len
}
