//! V5 Inertial Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceImuReset(device: V5_DeviceT) {}
pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion) {}
pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude) {}
pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceImuTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32) {}
pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32) {}
