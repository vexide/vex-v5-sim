//! V5 Rotation Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceAbsEncReset(device: V5_DeviceT) {}

pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32) {}

pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool) {
    Default::default()
}

pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}

pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

pub fn vexDeviceAbsEncTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}

pub fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32) {}
