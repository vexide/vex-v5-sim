//! CTE Workcell Signal Tower

use vex_sdk::*;

pub fn vexDeviceLightTowerBlinkSet(
    device: V5_DeviceT,
    select: u8,
    mask: u8,
    onTime: i32,
    offTime: i32,
) {
}
pub fn vexDeviceLightTowerColorSet(device: V5_DeviceT, color_id: u32, value: u32) {}
pub fn vexDeviceLightTowerRgbGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceLightTowerRgbSet(device: V5_DeviceT, rgb_value: u32, xyw_value: u32) {}
pub fn vexDeviceLightTowerStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceLightTowerDebugGet(device: V5_DeviceT, id: i32) -> u32 {
    Default::default()
}
pub fn vexDeviceLightTowerXywGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
