//! CPU Exception Vectors
//!
//! This module contains implementations of each ARM exception vector
//! required to handle exceptions (including interrupts) in `vectors.s`.
//!
//! This file does not include the `reset` vector (entrypoint), which can be
//! found in `main.rs`.
//!
//! In most cases, these functions are implemented from Xilinx's `asm_vectors.s`
//! file: <https://github.com/Xilinx/embeddedsw/blob/5688620af40994a0012ef5db3c873e1de3f20e9f/lib/bsp/standalone/src/arm/cortexa9/armcc/asm_vectors.s>

use core::arch::asm;

use vex_sdk::{
    vexSystemDataAbortInterrupt, vexSystemFIQInterrupt, vexSystemPrefetchAbortInterrupt,
};

use crate::utils::exit;

/// Undefined Instruction Vector
///
/// This function is jumped to when the CPU when pc runs into an undefined instruction. It
/// currently just saves the program state/registers and calls `UndefinedException` from libxil.
///
/// This exception occurs when the CPU's instruction pipelining encounters and attempts to
/// execute an instrction it does not recognize.
#[no_mangle]
#[naked]
pub extern "C" fn undefined_instruction() -> ! {
    unsafe {
        asm!(
            "
            stmdb sp!,{{r0-r3,r12,lr}} // state save from compiled code
            ldr r0, =UndefinedExceptionAddr
            sub r1, lr,#4
            str r1, [r0] // Address of instruction causing undefined exception
            bl UndefinedException // UndefinedException: call C function here
            ldmia sp!, {{r0-r3,r12,lr}} // state restore from compiled code

            movs pc, lr
            ",
            options(noreturn)
        )
    }
}

/// Software Interrupt Vector
///
/// This function is jumped to when the CPU recieves a software interrupt (SWI/SVC). It
/// currently just saves the program state/registers and calls `SWInterrupt` from libxil.
#[no_mangle]
#[naked]
pub extern "C" fn svc() -> ! {
    unsafe {
        asm!(
            "
            stmdb sp!,{{r0-r3,r12,lr}} // state save from compiled code
            tst	r0, #0x20 // check the T bit
            // ldrneh r0, [lr,#-2] // Thumb mode
            // bicne r0, r0, #0xff00 // Thumb mode
            ldreq r0, [lr,#-4] // ARM mode
            biceq r0, r0, #0xff000000 // ARM mode
            bl SWInterrupt // SWInterrupt: call C function here
            ldmia sp!,{{r0-r3,r12,lr}} // state restore from compiled code
            movs pc, lr // adjust return
            ",
            options(noreturn)
        )
    }
}

/// Prefetch Abort Vector
///
/// This function is jumped to by the CPU when a prefetch abort (PABT) occurs.
///
/// Prefetch aborts occur when instruction pipelining is unable to fetch an
/// instruction, then proceeds to attempt to execute at that location.
///
/// See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Prefetch-Abort-exception>
#[no_mangle]
pub extern "C" fn prefetch_abort() -> ! {
    // TODO: copy <https://github.com/Xilinx/embeddedsw/blob/5688620af40994a0012ef5db3c873e1de3f20e9f/lib/bsp/standalone/src/arm/cortexa9/armcc/asm_vectors.s#L133>
    // rather than just calling the jumptable function. This should first go through libxil, then libxil should
    // call vexSystemPrefetchAbortInterrupt.
    unsafe {
        vexSystemPrefetchAbortInterrupt();
    }
    exit(1);
}

/// Data Abort Vector
///
/// This function is jumped to by the CPU when a data abort occurs.
///
/// Data aborts typically occur as a result of illegal memory accesses.
///
/// See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Data-Abort-exception>
#[no_mangle]
pub extern "C" fn data_abort() -> ! {
    // TODO: copy <https://github.com/Xilinx/embeddedsw/blob/5688620af40994a0012ef5db3c873e1de3f20e9f/lib/bsp/standalone/src/arm/cortexa9/armcc/asm_vectors.s#L124>
    // rather than just calling the jumptable function. This should first go through libxil, then libxil should
    // call vexSystemDataAbortInterrupt.
    unsafe {
        vexSystemDataAbortInterrupt();
    }
    exit(1);
}

/// Interrupt Request Vector
///
/// This function is jumped to when the CPU recieves an IRQ. It currently just saves the
/// program state/registers and calls `IRQInterrupt` from libxil.
#[no_mangle]
#[naked]
pub extern "C" fn irq() -> ! {
    unsafe {
        asm!(
            "
                stmdb sp!,{{r0-r3,r12,lr}} // state save from compiled code
                vpush {{d0-d7}}
                vpush {{d16-d31}}
                vmrs r1, FPSCR
                push {{r1}}
                vmrs r1, FPEXC
                push {{r1}}
                bl IRQInterrupt // IRQ vector
                pop {{r1}}
                vmsr FPEXC, r1
                pop {{r1}}
                vmsr FPSCR, r1
                vpop {{d16-d31}}
                vpop {{d0-d7}}
                ldmia sp!,{{r0-r3,r12,lr}} // state restore from compiled code
                subs pc, lr, #4 // adjust return
            ",
            options(noreturn)
        )
    }
}

#[no_mangle]
pub extern "C" fn fiq() -> ! {
    // TODO: look at <https://github.com/Xilinx/embeddedsw/blob/5688620af40994a0012ef5db3c873e1de3f20e9f/lib/bsp/standalone/src/arm/cortexa9/armcc/asm_vectors.s#L82>
    //
    // This one's also a little weird since there's an FIQLoop symbol, where I have no idea
    // if or when its ever called or used here. Either way, FIQs aren't ever used on the V5.
    unsafe {
        vexSystemFIQInterrupt();
    }

    loop {
        core::hint::spin_loop();
    }
}
