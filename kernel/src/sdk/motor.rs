//! V5 Smart Motor

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32) {}
pub fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode) {}
pub fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode {
    Default::default()
}
pub fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32) {}
pub fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32) {}
pub fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool) {}
pub fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits) {}
pub fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits {
    Default::default()
}
pub fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode) {}
pub fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode {
    Default::default()
}
pub fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double) {}
pub fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut u32) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorPositionReset(device: V5_DeviceT) {}
pub fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double) {}
pub fn vexDeviceMotorAbsoluteTargetSet(device: V5_DeviceT, position: c_double, veloctiy: i32) {}
pub fn vexDeviceMotorRelativeTargetSet(device: V5_DeviceT, position: c_double, velocity: i32) {}
pub fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32) {}
pub fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset) {}
pub fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset {
    Default::default()
}
pub fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32) {}
pub fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32) {}
pub fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {}
pub fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {}
pub fn vexDeviceMotorExternalProfileSet(device: V5_DeviceT, position: c_double, velocity: i32) {}
