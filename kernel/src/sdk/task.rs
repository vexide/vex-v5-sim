//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

use log::trace;

use super::SERIAL;

/// Adds a new simple task to the task scheduler.
pub fn vexTaskAdd(
    callback: unsafe extern "C" fn() -> c_int,
    interval: c_int,
    label: *const c_char,
) {
}

/// Gets a tasks's callback function and internal ID.
pub fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void {
    core::ptr::null_mut()
}

/// Yields execution away from the current task for a given number of milliseconds.
pub fn vexTaskSleep(time: u32) {}

/// Returns the maximum number of threads that are supported by the VEXos
/// task scheduler.
///
/// Real V5 enviornments set this at 128, but since we don't currently emulate the
/// VEXcode task scheduler, we just return 0.
pub fn vexTaskHardwareConcurrency() -> i32 {
    0
}

/// Unknown use; on the partner SDK this is aliased to vexTasksRun,
/// but this real version on the jumptable isn't understood well.
pub fn vexBackgroundProcessing() {}

/// Ticks's the (non-existent in this environment) internal VEXos task
/// scheduler
///
/// This more importantly handles many device reads and flushes
/// serial.
pub fn vexTasksRun() {
    // trace!("vexTasksRun");
    SERIAL.flush().unwrap();
}
