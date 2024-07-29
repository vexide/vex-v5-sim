use log::info;

pub mod logging;
pub mod panic;
pub mod uart;

pub fn exit(code: i32) -> ! {
    info!("Exiting with code {}", code);
    todo!("Exit (code {})", code);
}