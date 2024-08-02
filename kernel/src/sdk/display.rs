//! Brain Display

use core::ffi::{c_char, VaList};

use vex_sdk::*;

pub fn vexDisplayForegroundColor(col: u32) {}
pub fn vexDisplayBackgroundColor(col: u32) {}
pub fn vexDisplayErase() {}
pub fn vexDisplayScroll(nStartLine: i32, nLines: i32) {}
pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32) {}
pub fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32) {}
pub fn vexDisplayPixelSet(x: u32, y: u32) {}
pub fn vexDisplayPixelClear(x: u32, y: u32) {}
pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {}
pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {}
pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {}
pub fn vexDisplayTextSize(n: u32, d: u32) {}
pub fn vexDisplayFontNamedSet(pFontName: *const c_char) {}
pub fn vexDisplayForegroundColorGet() -> u32 {
    Default::default()
}
pub fn vexDisplayBackgroundColorGet() -> u32 {
    Default::default()
}
pub fn vexDisplayStringWidthGet(pString: *const c_char) -> i32 {
    Default::default()
}
pub fn vexDisplayStringHeightGet(pString: *const c_char) -> i32 {
    Default::default()
}
pub fn vexDisplayPenSizeSet(width: u32) {}
pub fn vexDisplayPenSizeGet() -> u32 {
    Default::default()
}
pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) {}
pub fn vexDisplayDoubleBufferDisable() {}
pub fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32 {
    Default::default()
}
pub fn vexImagePngRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
    ibuflen: u32,
) -> u32 {
    Default::default()
}
pub fn vexDisplayVPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVSmallStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {
}
pub fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {
}
