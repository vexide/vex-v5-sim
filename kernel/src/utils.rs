use log::info;

pub fn exit(code: i32) -> ! {
    info!("Exiting with code {}", code);
    todo!("Exit (code {})", code);
}
