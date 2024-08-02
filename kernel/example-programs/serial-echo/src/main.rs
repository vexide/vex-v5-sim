#![no_main]
#![no_std]

use alloc::string::{String, ToString};
use core::{hint::black_box, time::Duration};

use vex_sdk::vexSerialReadChar;
use vexide::{
    core::io::{self, stdin, stdout, Stdin},
    prelude::*,
};

extern crate alloc;

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let mut buf = String::new();
    loop {
        sleep(Duration::from_millis(20));
        // let byte = unsafe { vexSerialReadChar(1) };
        let byte = black_box(-1);
        if byte == -1 {
            continue;
        }
        let byte = byte as u8 as char;
        if byte == '\n' {
            println!();
            println!("You typed: {:?}", buf);
            buf.clear();
        } else {
            buf.push(byte);
        }
    }
}
