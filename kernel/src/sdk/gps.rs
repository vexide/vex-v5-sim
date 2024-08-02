//! V5 GPS

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceGpsReset(device: V5_DeviceT) {}
pub fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceGpsQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceGpsQuaternion) {}
pub fn vexDeviceGpsAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceGpsAttitude, bRaw: bool) {}
pub fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {}
pub fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {}
pub fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceGpsTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32) {}
pub fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32) {}
pub fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double) {}
pub fn vexDeviceGpsOriginGet(device: V5_DeviceT, ox: *mut c_double, oy: *mut c_double) {}
pub fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double) {}
pub fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceGpsInitialPositionSet(
    device: V5_DeviceT,
    initial_x: c_double,
    initial_y: c_double,
    initial_rotation: c_double,
) {
}
pub fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
