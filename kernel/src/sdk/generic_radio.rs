//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use vex_sdk::*;

pub fn vexDeviceGenericRadioConnection(
    device: V5_DeviceT,
    link_id: *mut c_char,
    r#type: c_int,
    ov: bool,
) {
}
pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericRadioTransmit(device: V5_DeviceT, data: *const u8, size: u16) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceGenericRadioReceive(device: V5_DeviceT, data: *mut u8, size: u16) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool {
    Default::default()
}
