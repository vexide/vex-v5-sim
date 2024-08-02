//! FFI Bindings for the XUARTPS UART driver.
//!
//! This driver supports the following features:
//!
//! - Dynamic data format (baud rate, data bits, stop bits, parity)
//! - Polled mode
//! - Interrupt driven mode
//! - Transmit and receive FIFOs (32 byte FIFO depth)
//! - Access to the external modem control lines
//!
//! ## Initialization & Configuration
//!
//! The XUartPs_Config structure is used by the driver to configure itself.
//! Fields inside this structure are properties of XUartPs based on its hardware
//! build.
//!
//! To support multiple runtime loading and initialization strategies employed
//! by various operating systems, the driver instance can be initialized in the
//! following way:
//!
//!   - `XUartPs_CfgInitialize(InstancePtr, CfgPtr, EffectiveAddr)` - Uses a
//!     configuration structure provided by the caller. If running in a system
//!     with address translation, the parameter EffectiveAddr should be the
//!     virtual address.
//!
//! ## Baud Rate
//!
//! The UART has an internal baud rate generator, which furnishes the baud rate
//! clock for both the receiver and the transmitter. The input clock frequency
//! can be either the master clock or the master clock divided by 8, configured
//! through the mode register.
//!
//! Accompanied with the baud rate divider register, the baud rate is determined
//! by:
//! ```c
//! baud_rate = input_clock / (bgen * (bdiv + 1)
//! ```
//! where bgen is the value of the baud rate generator, and bdiv is the value of
//! baud rate divider.
//!
//! ## Interrupts
//!
//! The FIFOs are not flushed when the driver is initialized, but a function is
//! provided to allow the user to reset the FIFOs if desired.
//!
//! The driver defaults to no interrupts at initialization such that interrupts
//! must be enabled if desired. An interrupt is generated for one of the
//! following conditions.
//!
//! - A change in the modem signals
//! - Data in the receive FIFO for a configuable time without receiver activity
//! - A parity error
//! - A framing error
//! - An overrun error
//! - Transmit FIFO is full
//! - Transmit FIFO is empty
//! - Receive FIFO is full
//! - Receive FIFO is empty
//! - Data in the receive FIFO equal to the receive threshold
//!
//! The application can control which interrupts are enabled using the
//! XUartPs_SetInterruptMask() function.
//!
//! In order to use interrupts, it is necessary for the user to connect the
//! driver interrupt handler, XUartPs_InterruptHandler(), to the interrupt
//! system of the application. A separate handler should be provided by the
//! application to communicate with the interrupt system, and conduct
//! application specific interrupt handling. An application registers its own
//! handler through the XUartPs_SetHandler() function.
//!
//! ## Data Transfer
//!
//! The functions, XUartPs_Send() and XUartPs_Recv(), are provided in the
//! driver to allow data to be sent and received. They can be used in either
//! polled or interrupt mode.
//!
//! ## Note
//!
//! The default configuration for the UART after initialization is:
//!
//! - 9,600 bps or XPAR_DFT_BAUDRATE if defined
//! - 8 data bits
//! - 1 stop bit
//! - no parity
//! - FIFO's are enabled with a receive threshold of 8 bytes
//! - The RX timeout is enabled with a timeout of 1 (4 char times)

#![allow(non_camel_case_types)]

// MARK: Constant Definitions

use core::{
    ffi::{c_char, c_void},
    ptr,
};

pub const UART1_BASE_ADDR: u32 = 0xE0001000;

/// The following constants indicate the max and min baud rates and these
/// numbers are based only on the testing that has been done. The hardware
/// is capable of other baud rates.
pub const XUARTPS_MAX_RATE: u32 = 6240000;
pub const XUARTPS_MIN_RATE: u32 = 110;

/// Default baud rate.
pub const XUARTPS_DFT_BAUDRATE: u32 = 115200;

/// Wait for 1 sec in worst case
pub const TIMEOUT_VAL: u32 = 1000000;

// UART Statuses

pub const XST_SUCCESS: i32 = 0;
pub const XST_UART_TEST_FAIL: i32 = 1054;
pub const XST_UART_BAUD_ERROR: i32 = 1055;

// Configuration options

/// Starts break transmission
pub const XUARTPS_OPTION_SET_BREAK: u32 = 0x0080;
/// Stops break transmission
pub const XUARTPS_OPTION_STOP_BREAK: u32 = 0x0040;
/// Reset the receive timeout
pub const XUARTPS_OPTION_RESET_TMOUT: u32 = 0x0020;
/// Reset the transmitter
pub const XUARTPS_OPTION_RESET_TX: u32 = 0x0010;
/// Reset the receiver
pub const XUARTPS_OPTION_RESET_RX: u32 = 0x0008;
/// Assert the RTS bit
pub const XUARTPS_OPTION_ASSERT_RTS: u32 = 0x0004;
/// Assert the DTR bit
pub const XUARTPS_OPTION_ASSERT_DTR: u32 = 0x0002;
/// Turn on flow control mode
pub const XUARTPS_OPTION_SET_FCM: u32 = 0x0001;

// Channel Operational Mode

/// Normal Mode
pub const XUARTPS_OPER_MODE_NORMAL: u8 = 0x00;
/// Auto Echo Mode
pub const XUARTPS_OPER_MODE_AUTO_ECHO: u8 = 0x01;
/// Local Loopback Mode
pub const XUARTPS_OPER_MODE_LOCAL_LOOP: u8 = 0x02;
/// Remote Loopback Mode
pub const XUARTPS_OPER_MODE_REMOTE_LOOP: u8 = 0x03;

// Data format values

/// 8 data bits
pub const XUARTPS_FORMAT_8_BITS: u32 = 0;
/// 7 data bits
pub const XUARTPS_FORMAT_7_BITS: u32 = 2;
/// 6 data bits
pub const XUARTPS_FORMAT_6_BITS: u32 = 3;

/// No parity
pub const XUARTPS_FORMAT_NO_PARITY: u32 = 4;
/// Mark parity
pub const XUARTPS_FORMAT_MARK_PARITY: u32 = 3;
/// Space parity
pub const XUARTPS_FORMAT_SPACE_PARITY: u32 = 2;
/// Odd parity
pub const XUARTPS_FORMAT_ODD_PARITY: u32 = 1;
/// Even parity
pub const XUARTPS_FORMAT_EVEN_PARITY: u32 = 0;

/// 2 stop bits
pub const XUARTPS_FORMAT_2_STOP_BIT: u32 = 2;
/// 1.5 stop bits
pub const XUARTPS_FORMAT_1_5_STOP_BIT: u32 = 1;
/// 1 stop bit
pub const XUARTPS_FORMAT_1_STOP_BIT: u32 = 0;

// Callback events

/// Data receiving done
pub const XUARTPS_EVENT_RECV_DATA: u32 = 1;
/// A receive timeout occurred
pub const XUARTPS_EVENT_RECV_TOUT: u32 = 2;
/// Data transmission done
pub const XUARTPS_EVENT_SENT_DATA: u32 = 3;
/// A receive error detected
pub const XUARTPS_EVENT_RECV_ERROR: u32 = 4;
/// Modem status changed
pub const XUARTPS_EVENT_MODEM: u32 = 5;
/// A receive parity, frame, break error detected
pub const XUARTPS_EVENT_PARE_FRAME_BRKE: u32 = 6;
/// A receive overrun error detected
pub const XUARTPS_EVENT_RECV_ORERR: u32 = 7;

// Register Map

/// Control Register [8:0]
pub const XUARTPS_CR_OFFSET: u32 = 0x0000;
/// Mode Register [9:0]
pub const XUARTPS_MR_OFFSET: u32 = 0x0004;
/// Interrupt Enable [12:0]
pub const XUARTPS_IER_OFFSET: u32 = 0x0008;
/// Interrupt Disable [12:0]
pub const XUARTPS_IDR_OFFSET: u32 = 0x000C;
/// Interrupt Mask [12:0]
pub const XUARTPS_IMR_OFFSET: u32 = 0x0010;
/// Interrupt Status [12:0]
pub const XUARTPS_ISR_OFFSET: u32 = 0x0014;
/// Baud Rate Generator [15:0]
pub const XUARTPS_BAUDGEN_OFFSET: u32 = 0x0018;
/// RX Timeout [7:0]
pub const XUARTPS_RXTOUT_OFFSET: u32 = 0x001C;
/// RX FIFO Trigger Level [5:0]
pub const XUARTPS_RXWM_OFFSET: u32 = 0x0020;
/// Modem Control [5:0]
pub const XUARTPS_MODEMCR_OFFSET: u32 = 0x0024;
/// Modem Status [8:0]
pub const XUARTPS_MODEMSR_OFFSET: u32 = 0x0028;
/// Channel Status [14:0]
pub const XUARTPS_SR_OFFSET: u32 = 0x002C;
/// FIFO [7:0]
pub const XUARTPS_FIFO_OFFSET: u32 = 0x0030;
/// Baud Rate Divider [7:0]
pub const XUARTPS_BAUDDIV_OFFSET: u32 = 0x0034;
/// Flow Delay [5:0]
pub const XUARTPS_FLOWDEL_OFFSET: u32 = 0x0038;
/// TX FIFO Trigger Level [5:0]
pub const XUARTPS_TXWM_OFFSET: u32 = 0x0044;
/// RX FIFO Byte Status [11:0]
pub const XUARTPS_RXBS_OFFSET: u32 = 0x0048;

// Control Register

/// Stop transmission of break
pub const XUARTPS_CR_STOPBRK: u32 = 0x00000100;
/// Set break
pub const XUARTPS_CR_STARTBRK: u32 = 0x00000080;
/// RX timeout counter restart
pub const XUARTPS_CR_TORST: u32 = 0x00000040;
/// TX disabled.
pub const XUARTPS_CR_TX_DIS: u32 = 0x00000020;
/// TX enabled
pub const XUARTPS_CR_TX_EN: u32 = 0x00000010;
/// RX disabled.
pub const XUARTPS_CR_RX_DIS: u32 = 0x00000008;
/// RX enabled
pub const XUARTPS_CR_RX_EN: u32 = 0x00000004;
/// Enable/disable Mask
pub const XUARTPS_CR_EN_DIS_MASK: u32 = 0x0000003C;
/// TX logic reset
pub const XUARTPS_CR_TXRST: u32 = 0x00000002;
/// RX logic reset
pub const XUARTPS_CR_RXRST: u32 = 0x00000001;

// Mode Register

/// Input clock selection
pub const XUARTPS_MR_CCLK: u32 = 0x00000400;
/// Remote loopback mode
pub const XUARTPS_MR_CHMODE_R_LOOP: u32 = 0x00000300;
/// Local loopback mode
pub const XUARTPS_MR_CHMODE_L_LOOP: u32 = 0x00000200;
/// Auto echo mode
pub const XUARTPS_MR_CHMODE_ECHO: u32 = 0x00000100;
/// Normal mode
pub const XUARTPS_MR_CHMODE_NORM: u32 = 0x00000000;
/// Mode shift
pub const XUARTPS_MR_CHMODE_SHIFT: u32 = 8;
/// Mode mask
pub const XUARTPS_MR_CHMODE_MASK: u32 = 0x00000300;
/// 2 stop bits
pub const XUARTPS_MR_STOPMODE_2_BIT: u32 = 0x00000080;
/// 1.5 stop bits
pub const XUARTPS_MR_STOPMODE_1_5_BIT: u32 = 0x00000040;
/// 1 stop bit
pub const XUARTPS_MR_STOPMODE_1_BIT: u32 = 0x00000000;
/// Stop bits shift
pub const XUARTPS_MR_STOPMODE_SHIFT: u32 = 6;
/// Stop bits mask
pub const XUARTPS_MR_STOPMODE_MASK: u32 = 0x000000A0;
/// No parity mode
pub const XUARTPS_MR_PARITY_NONE: u32 = 0x00000020;
/// Mark parity mode
pub const XUARTPS_MR_PARITY_MARK: u32 = 0x00000018;
/// Space parity mode
pub const XUARTPS_MR_PARITY_SPACE: u32 = 0x00000010;
/// Odd parity mode
pub const XUARTPS_MR_PARITY_ODD: u32 = 0x00000008;
/// Even parity mode
pub const XUARTPS_MR_PARITY_EVEN: u32 = 0x00000000;
/// Parity setting shift
pub const XUARTPS_MR_PARITY_SHIFT: u32 = 3;
/// Parity mask
pub const XUARTPS_MR_PARITY_MASK: u32 = 0x00000038;
/// 6 bits data
pub const XUARTPS_MR_CHARLEN_6_BIT: u32 = 0x00000006;
/// 7 bits data
pub const XUARTPS_MR_CHARLEN_7_BIT: u32 = 0x00000004;
/// 8 bits data
pub const XUARTPS_MR_CHARLEN_8_BIT: u32 = 0x00000000;
/// Data Length shift
pub const XUARTPS_MR_CHARLEN_SHIFT: u32 = 1;
/// Data length mask
pub const XUARTPS_MR_CHARLEN_MASK: u32 = 0x00000006;
/// Input clock selection
pub const XUARTPS_MR_CLKSEL: u32 = 0x00000001;

// Interrupt Registers

pub const XUARTPS_IXR_RBRK: u32 = 0x00002000; // Rx FIFO break detect interrupt
pub const XUARTPS_IXR_TOVR: u32 = 0x00001000; // Tx FIFO Overflow interrupt
pub const XUARTPS_IXR_TNFUL: u32 = 0x00000800; // Tx FIFO Nearly Full interrupt
pub const XUARTPS_IXR_TTRIG: u32 = 0x00000400; // Tx Trig interrupt
pub const XUARTPS_IXR_DMS: u32 = 0x00000200; // Modem status change interrupt
pub const XUARTPS_IXR_TOUT: u32 = 0x00000100; // Timeout error interrupt
pub const XUARTPS_IXR_PARITY: u32 = 0x00000080; // Parity error interrupt
pub const XUARTPS_IXR_FRAMING: u32 = 0x00000040; // Framing error interrupt
pub const XUARTPS_IXR_OVER: u32 = 0x00000020; // Overrun error interrupt
pub const XUARTPS_IXR_TXFULL: u32 = 0x00000010; // TX FIFO full interrupt.
pub const XUARTPS_IXR_TXEMPTY: u32 = 0x00000008; // TX FIFO empty interrupt.
pub const XUARTPS_IXR_RXFULL: u32 = 0x00000004; // RX FIFO full interrupt.
pub const XUARTPS_IXR_RXEMPTY: u32 = 0x00000002; // RX FIFO empty interrupt.
pub const XUARTPS_IXR_RXOVR: u32 = 0x00000001; // RX FIFO trigger interrupt.
pub const XUARTPS_IXR_MASK: u32 = 0x00003FFF; // Valid bit mask

// Baud Rate Generator Register

/// Disable clock
pub const XUARTPS_BAUDGEN_DISABLE: u32 = 0x00000000;
/// Valid bits mask
pub const XUARTPS_BAUDGEN_MASK: u32 = 0x0000FFFF;
/// Reset value
pub const XUARTPS_BAUDGEN_RESET_VAL: u32 = 0x0000028B;

// Channel Status Register

/// TX FIFO Nearly Full Status
pub const XUARTPS_SR_TNFUL: u32 = 0x00004000;
/// TX FIFO Trigger Status
pub const XUARTPS_SR_TTRIG: u32 = 0x00002000;
/// RX FIFO fill over flow delay
pub const XUARTPS_SR_FLOWDEL: u32 = 0x00001000;
/// TX active
pub const XUARTPS_SR_TACTIVE: u32 = 0x00000800;
/// RX active
pub const XUARTPS_SR_RACTIVE: u32 = 0x00000400;
/// TX FIFO full
pub const XUARTPS_SR_TXFULL: u32 = 0x00000010;
/// TX FIFO empty
pub const XUARTPS_SR_TXEMPTY: u32 = 0x00000008;
/// RX FIFO full
pub const XUARTPS_SR_RXFULL: u32 = 0x00000004;
/// RX FIFO empty
pub const XUARTPS_SR_RXEMPTY: u32 = 0x00000002;
/// RX FIFO fill over trigger
pub const XUARTPS_SR_RXOVR: u32 = 0x00000001;

// MARK: Type Definitions

/// This typedef contains configuration information for the device.
#[repr(C)]
pub struct XUartPs_Config {
    pub Name: *const c_char,
    /// Base address of device (IPIF)
    pub BaseAddress: u32,
    /// Input clock frequency
    pub InputClockHz: u32,
    /// Specifies whether modem pins are connected to MIO or FMIO
    pub ModemPinsConnected: i32,
}

/// Keep track of state information about a data buffer in the interrupt mode.
#[repr(C)]
pub struct XUartPsBuffer {
    pub NextBytePtr: *mut u8,
    pub RequestedBytes: u32,
    pub RemainingBytes: u32,
}

/// Keep track of data format setting of a device.
#[repr(C)]
pub struct XUartPsFormat {
    /// In bps, ie 1200
    pub BaudRate: u32,
    /// Number of data bits
    pub DataBits: u32,
    /// Parity
    pub Parity: u32,
    /// Number of stop bits
    pub StopBits: u32,
}

/// This data type defines a handler that an application defines to communicate
/// with interrupt system to retrieve state information about an application.
///
/// # Parameters
///
/// - `CallBackRef` is a callback reference passed in by the upper layer when
///   setting the handler, and is passed back to the upper layer when the handler
///   is called. It is used to find the device driver instance.
/// - `Event` contains one of the event constants indicating events that have
///   occurred.
/// - `EventData` contains the number of bytes sent or received at the time of
///   the call for send and receive events and contains the modem status for
///   modem events.
pub type XUartPs_Handler = extern "C" fn(CallBackRef: *mut c_void, Event: u32, EventData: u32);

/// The XUartPs driver instance data structure. A pointer to an instance data
/// structure is passed around by functions to refer to a specific driver
/// instance.
#[repr(C)]
pub struct XUartPs {
    /// Configuration data structure
    pub Config: XUartPs_Config,
    /// Input clock frequency
    pub InputClockHz: u32,
    /// Device is initialized and ready
    pub IsReady: u32,
    /// Current baud rate
    pub BaudRate: u32,
    pub SendBuffer: XUartPsBuffer,
    pub ReceiveBuffer: XUartPsBuffer,
    pub Handler: Option<XUartPs_Handler>,
    /// Callback reference for event handler
    pub CallBackRef: *mut c_void,
    pub Platform: u32,
    pub is_rxbs_error: u8,
}

impl XUartPs {
    pub const unsafe fn zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// MARK: Macros (Inline Functions) Definitions

/// Get the UART Channel Status Register.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
///
/// # Returns
///
/// The value read from the register.
#[inline]
pub unsafe fn XUartPs_GetChannelStatus(InstancePtr: *mut XUartPs) -> u32 {
    unsafe {
        ptr::read_volatile((((*InstancePtr).Config.BaseAddress) + XUARTPS_SR_OFFSET) as *const u32)
    }
}

/// Get the UART Mode Control Register.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
///
/// # Returns
///
/// The value read from the register.
#[inline]
pub unsafe fn XUartPs_GetModeControl(InstancePtr: *mut XUartPs) -> u32 {
    unsafe {
        ptr::read_volatile((((*InstancePtr).Config.BaseAddress) + XUARTPS_CR_OFFSET) as *const u32)
    }
}

/// Set the UART Mode Control Register.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
/// - `RegisterValue` is the value to be written to the register.
///
/// # Returns
///
/// The value to write to the register.
#[inline]
pub unsafe fn XUartPs_SetModeControl(InstancePtr: *mut XUartPs, RegisterValue: u32) {
    // Docs say that RegisterValue is u16 but seems wrong
    unsafe {
        ptr::write_volatile(
            (((*InstancePtr).Config.BaseAddress) + XUARTPS_CR_OFFSET) as *mut u32,
            RegisterValue,
        );
    }
}

/// Enable the transmitter and receiver of the UART.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
///
/// # Returns
///
/// None.
#[inline]
pub unsafe fn XUartPs_EnableUart(InstancePtr: *mut XUartPs) {
    unsafe {
        let reg_value = XUartPs_GetModeControl(InstancePtr) & (!XUARTPS_CR_EN_DIS_MASK)
            | (XUARTPS_CR_RX_EN | XUARTPS_CR_TX_EN);
        XUartPs_SetModeControl(InstancePtr, reg_value);
    }
}

/// Disable the transmitter and receiver of the UART.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
///
/// # Returns
///
/// None.
#[inline]
pub unsafe fn XUartPs_DisableUart(InstancePtr: *mut XUartPs) {
    unsafe {
        let reg_value = XUartPs_GetModeControl(InstancePtr) & (!XUARTPS_CR_EN_DIS_MASK)
            | (XUARTPS_CR_RX_DIS | XUARTPS_CR_TX_DIS);
        XUartPs_SetModeControl(InstancePtr, reg_value);
    }
}

/// Determine if the transmitter FIFO is empty.
///
/// # Parameters
///
/// - `InstancePtr` is a pointer to the XUartPs instance.
///
/// # Returns
///
/// - TRUE if a byte can be sent
/// - FALSE if the Transmitter Fifo is not empty
#[inline]
pub unsafe fn XUartPs_IsTransmitEmpty(InstancePtr: *mut XUartPs) -> bool {
    unsafe { (XUartPs_GetChannelStatus(InstancePtr) & XUARTPS_SR_TXEMPTY) == XUARTPS_SR_TXEMPTY }
}

// MARK: Function Prototypes

extern "C" {
    pub fn XUartPs_LookupConfig(BaseAddress: u32) -> *mut XUartPs_Config;
    pub fn XUartPs_CfgInitialize(
        InstancePtr: *mut XUartPs,
        Config: *mut XUartPs_Config,
        EffectiveAddr: u32,
    ) -> i32;
    pub fn XUartPs_SetBaudRate(InstancePtr: *mut XUartPs, BaudRate: u32) -> i32;
    pub fn XUartPs_Send(InstancePtr: *mut XUartPs, BufferPtr: *const u8, NumBytes: u32) -> u32;
    pub fn XUartPs_Recv(InstancePtr: *mut XUartPs, BufferPtr: *mut u8, NumBytes: u32) -> u32;
    pub fn XUartPs_SetOptions(InstancePtr: *mut XUartPs, Options: u16);
    pub fn XUartPs_GetOptions(InstancePtr: *mut XUartPs) -> u16;
    pub fn XUartPs_SetFifoThreshold(InstancePtr: *mut XUartPs, TriggerLevel: u8);
    pub fn XUartPs_GetFifoThreshold(InstancePtr: *mut XUartPs) -> u8;
    pub fn XUartPs_GetModemStatus(InstancePtr: *mut XUartPs) -> u16;
    pub fn XUartPs_IsSending(InstancePtr: *mut XUartPs) -> u32;
    pub fn XUartPs_GetOperMode(InstancePtr: *mut XUartPs) -> u8;
    pub fn XUartPs_SetOperMode(InstancePtr: *mut XUartPs, OperationMode: u8);
    pub fn XUartPs_GetFlowDelay(InstancePtr: *mut XUartPs) -> u8;
    pub fn XUartPs_SetFlowDelay(InstancePtr: *mut XUartPs, FlowDelayValue: u8);
    pub fn XUartPs_GetRecvTimeout(InstancePtr: *mut XUartPs) -> u8;
    pub fn XUartPs_SetRecvTimeout(InstancePtr: *mut XUartPs, RecvTimeout: u8);
    pub fn XUartPs_SetDataFormat(InstancePtr: *mut XUartPs, FormatPtr: *mut XUartPsFormat) -> i32;
    pub fn XUartPs_GetDataFormat(InstancePtr: *mut XUartPs, FormatPtr: *mut XUartPsFormat);
    pub fn XUartPs_GetInterruptMask(InstancePtr: *mut XUartPs) -> u32;
    pub fn XUartPs_SetInterruptMask(InstancePtr: *mut XUartPs, Mask: u32);
    pub fn XUartPs_InterruptHandler(InstancePtr: *mut XUartPs);
    pub fn XUartPs_SetHandler(
        InstancePtr: *mut XUartPs,
        FuncPtr: XUartPs_Handler,
        CallBackRef: *mut c_void,
    );
    pub fn XUartPs_SelfTest(InstancePtr: *mut XUartPs) -> i32;
}
