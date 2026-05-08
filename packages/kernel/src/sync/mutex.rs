use std::{cell::SyncUnsafeCell, hint::spin_loop, sync::atomic::{AtomicBool, AtomicU32, Ordering}};
use critical_section::RestoreState;

pub use lock_api::MutexGuard;
pub type Mutex<T> = lock_api::Mutex<RawMutex, T>;

fn in_interrupt() -> bool {
    unsafe {
        let mut cpsr: u32;
        asm!("mrs {0}, cpsr", out(reg) cpsr);
        let is_system_mode = (cpsr & 0b11111) == 0b11111;
        !is_system_mode
    }
}

static MUTEX_COUNTER: AtomicU32 = AtomicU32::new(0);
struct GlobalMutexState(SyncUnsafeCell<RestoreState>);
/// safety: this variable should only be accessed in a critical section
static GLOBAL_MUTEX_STATE: SyncUnsafeCell<RestoreState> = SyncUnsafeCell::new(RestoreState::invalid());

struct MutexState(AtomicBool);
impl MutexState {
    const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    /// Returns true if the lock was acquired.
    fn try_lock(&self) -> bool {
        unsafe { 
            let state = critical_section::acquire();
            if self.0.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_ok() {
                if MUTEX_COUNTER.fetch_add(1, Ordering::AcqRel) == 0 {
                    // we're the first mutex to be locked, we need to save the RestoreState
                    // to be released later
                    *GLOBAL_MUTEX_STATE.get() = state;
                } else {
                    // another mutex has already entered a critical section before us so
                    // we need to release the nested critical section we entered
                    critical_section::release(state);
                }
                true
            } else {
                // the mutex is already locked so release the critical section we acquired
                critical_section::release(state);
                false
            }
        }
    }

    /// safety: this function must only be called after a successful call to try_lock
    unsafe fn unlock(&self) {
        unsafe { 
            if MUTEX_COUNTER.fetch_sub(1, Ordering::AcqRel) == 1 {
                // we are the last mutex to be unlocked so we need to unlock the critical section
                critical_section::release(GLOBAL_MUTEX_STATE.into());
            }
            self.0.store(false, Ordering::Release);
        }
        ()
    }
}

/// A raw mutex type built on top of the critical section.
pub struct RawMutex {
    state: MutexState,
}
impl RawMutex {
    /// Creates a new raw mutex.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            state: MutexState::new(),
        }
    }
}
unsafe impl lock_api::RawMutex for RawMutex {
    // Allow this because we can't get around it
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self::new();

    type GuardMarker = lock_api::GuardSend;

    fn lock(&self) {
        if self.state.try_lock() {
            ()
        } else if in_interrupt() {
            unreachable!("Deadlock in kernel detected!");
        } else {
            while !self.state.try_lock() {
                spin_loop()
            }
            ()
        }
    }

    fn try_lock(&self) -> bool {
        self.state.try_lock()
    }

    unsafe fn unlock(&self) {
        unsafe { self.state.unlock() }
    }
}
