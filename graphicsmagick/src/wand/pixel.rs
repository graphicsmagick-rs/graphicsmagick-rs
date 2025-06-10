//! Binding of pixel_wand, Wand pixel access interfaces.
//!
//! <http://www.graphicsmagick.org/wand/pixel_wand.html>

use crate::{
    types::*,
    utils::{MagickCString, assert_initialized},
};
use graphicsmagick_sys::*;
use null_terminated_str::IntoNullTerminatedString;
use std::{
    os::raw::{c_double, c_ulong},
    ptr::NonNull,
};

/// Wrapper of `graphicsmagick_sys::PixelWand`.
#[derive(Debug)]
#[repr(transparent)]
pub struct PixelWand {
    wand: NonNull<graphicsmagick_sys::PixelWand>,
}

impl PixelWand {
    pub fn new() -> Self {
        assert_initialized();

        let wand = NonNull::new(unsafe { NewPixelWand() }).expect("NewPixelWand return NULL");

        PixelWand { wand }
    }

    /// # Safety
    ///
    ///  * `wand` - must points to either NULL, or a valid allocation.
    #[inline]
    pub unsafe fn from_wand(wand: *mut graphicsmagick_sys::PixelWand) -> Option<PixelWand> {
        NonNull::new(wand).map(|wand| PixelWand { wand })
    }

    #[inline]
    pub fn wand(&self) -> *const graphicsmagick_sys::PixelWand {
        self.wand.as_ptr() as *const _
    }

    #[inline]
    pub fn wand_mut(&mut self) -> *mut graphicsmagick_sys::PixelWand {
        self.wand.as_ptr()
    }
}

impl Drop for PixelWand {
    fn drop(&mut self) {
        unsafe {
            DestroyPixelWand(self.wand.as_ptr());
        }
    }
}

impl Clone for PixelWand {
    fn clone(&self) -> Self {
        PixelWand {
            wand: NonNull::new(unsafe { ClonePixelWand(self.wand.as_ptr()) })
                .expect("ClonePixelWand returns NULL"),
        }
    }
}

impl Default for PixelWand {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelWand {
    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblack>
    ///
    /// PixelGetBlack() returns the normalized black color of the pixel wand.
    ///
    pub fn get_black(&self) -> c_double {
        unsafe { PixelGetBlack(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblackquantum>
    ///
    /// PixelGetBlackQuantum() returns the black color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_black_quantum(&self) -> Quantum {
        unsafe { PixelGetBlackQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblue>
    ///
    /// PixelGetBlue(const) returns the normalized blue color of the pixel wand.
    ///
    pub fn get_blue(&self) -> c_double {
        unsafe { PixelGetBlue(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetbluequantum>
    ///
    /// PixelGetBlueQuantum(const ) returns the blue color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_blue_quantum(&self) -> Quantum {
        unsafe { PixelGetBlueQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcolorasstring>
    ///
    /// PixelGetColorAsString() gets the color of the pixel wand.
    ///
    pub fn get_color_as_string(&mut self) -> MagickCString {
        unsafe { MagickCString::new(PixelGetColorAsString(self.wand.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcolorcount>
    ///
    /// PixelGetColorCount() returns the color count associated with this color.
    ///
    pub fn get_color_count(&self) -> c_ulong {
        unsafe { PixelGetColorCount(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcyan>
    ///
    /// PixelGetCyan() returns the normalized cyan color of the pixel wand.
    ///
    pub fn get_cyan(&self) -> c_double {
        unsafe { PixelGetCyan(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcyanquantum>
    ///
    /// PixelGetCyanQuantum() returns the cyan color of the pixel wand.  The color
    ///
    /// is in the range of [0..MaxRGB]
    ///
    pub fn get_cyan_quantum(&self) -> Quantum {
        unsafe { PixelGetCyanQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetgreen>
    ///
    /// PixelGetGreen(const ) returns the normalized green color of the pixel wand.
    ///
    pub fn get_green(&self) -> c_double {
        unsafe { PixelGetGreen(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetgreenquantum>
    ///
    /// PixelGetGreenQuantum(const ) returns the green color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_green_quantum(&self) -> Quantum {
        unsafe { PixelGetGreenQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetmagenta>
    ///
    /// PixelGetMagenta() returns the normalized magenta color of the pixel wand.
    ///
    pub fn get_magenta(&self) -> c_double {
        unsafe { PixelGetMagenta(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetmagentaquantum>
    ///
    /// PixelGetMagentaQuantum() returns the magenta color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_magenta_quantum(&self) -> Quantum {
        unsafe { PixelGetMagentaQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetopacity>
    ///
    /// PixelGetOpacity(const ) returns the normalized opacity color of the pixel
    ///
    /// wand.
    ///
    pub fn get_opacity(&self) -> c_double {
        unsafe { PixelGetOpacity(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetopacityquantum>
    ///
    /// PixelGetOpacityQuantum(const ) returns the opacity color of the pixel wand.
    ///
    /// The color is in the range of [0..MaxRGB]
    ///
    pub fn get_opacity_quantum(&self) -> Quantum {
        unsafe { PixelGetOpacityQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetred>
    ///
    /// PixelGetRed(const ) returns the normalized red color of the pixel wand.
    ///
    pub fn get_red(&self) -> c_double {
        unsafe { PixelGetRed(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetredquantum>
    ///
    /// PixelGetRedQuantum(const ) returns the red color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_red_quantum(&self) -> Quantum {
        unsafe { PixelGetRedQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetyellow>
    ///
    /// PixelGetYellow() returns the normalized yellow color of the pixel wand.
    ///
    pub fn get_yellow(&self) -> c_double {
        unsafe { PixelGetYellow(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetyellowquantum>
    ///
    /// PixelGetYellowQuantum() returns the yellow color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_yellow_quantum(&self) -> Quantum {
        unsafe { PixelGetYellowQuantum(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblack>
    ///
    /// PixelSetBlack() sets the normalized black color of the pixel wand.
    ///
    pub fn set_black(&mut self, black: c_double) -> &mut Self {
        unsafe { PixelSetBlack(self.wand.as_ptr(), black) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblackquantum>
    ///
    /// PixelSetBlackQuantum() sets the black color of the pixel wand.  The color
    ///
    /// must be in the range of [0..MaxRGB]
    ///
    pub fn set_black_quantum(&mut self, black: Quantum) -> &mut Self {
        unsafe { PixelSetBlackQuantum(self.wand.as_ptr(), black) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblue>
    ///
    /// PixelSetBlue() sets the normalized blue color of the pixel wand.
    ///
    pub fn set_blue(&mut self, blue: c_double) -> &mut Self {
        unsafe { PixelSetBlue(self.wand.as_ptr(), blue) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetbluequantum>
    ///
    /// PixelSetBlueQuantum() sets the blue color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_blue_quantum(&mut self, blue: Quantum) -> &mut Self {
        unsafe { PixelSetBlueQuantum(self.wand.as_ptr(), blue) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcolor>
    ///
    /// PixelSetColor() sets the color of the pixel wand with a string (e.g.
    ///
    /// &quot;blue&quot;, &quot;#0000ff&quot;, &quot;rgb(0,0,255)&quot;, etc.).
    ///
    pub fn set_color<'a>(&mut self, color: impl IntoNullTerminatedString<'a>) -> &mut Self {
        let color = color.into_null_terminated_string();
        unsafe { PixelSetColor(self.wand.as_ptr(), color.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcolorcount>
    ///
    /// PixelSetColorCount() sets the color count of the pixel wand.
    ///
    pub fn set_color_count(&mut self, count: c_ulong) -> &mut Self {
        unsafe { PixelSetColorCount(self.wand.as_ptr(), count) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcyan>
    ///
    /// PixelSetCyan() sets the normalized cyan color of the pixel wand.
    ///
    pub fn set_cyan(&mut self, cyan: c_double) -> &mut Self {
        unsafe { PixelSetCyan(self.wand.as_ptr(), cyan) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcyanquantum>
    ///
    /// PixelSetCyanQuantum() sets the cyan color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_cyan_quantum(&mut self, cyan: Quantum) -> &mut Self {
        unsafe { PixelSetCyanQuantum(self.wand.as_ptr(), cyan) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetgreen>
    ///
    /// PixelSetGreen() sets the normalized green color of the pixel wand.
    ///
    pub fn set_green(&mut self, green: c_double) -> &mut Self {
        unsafe { PixelSetGreen(self.wand.as_ptr(), green) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetgreenquantum>
    ///
    /// PixelSetGreenQuantum() sets the green color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_green_quantum(&mut self, green: Quantum) -> &mut Self {
        unsafe { PixelSetGreenQuantum(self.wand.as_ptr(), green) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetmagenta>
    ///
    /// PixelSetMagenta() sets the normalized magenta color of the pixel wand.
    ///
    pub fn set_magenta(&mut self, magenta: c_double) -> &mut Self {
        unsafe { PixelSetMagenta(self.wand.as_ptr(), magenta) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetmagentaquantum>
    ///
    /// PixelSetMagentaQuantum() sets the magenta color of the pixel wand.  The
    ///
    /// color must be in the range of [0..MaxRGB]
    ///
    pub fn set_magenta_quantum(&mut self, magenta: Quantum) -> &mut Self {
        unsafe { PixelSetMagentaQuantum(self.wand.as_ptr(), magenta) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetopacity>
    ///
    /// PixelSetOpacity() sets the normalized opacity color of the pixel wand.
    ///
    pub fn set_opacity(&mut self, opacity: c_double) -> &mut Self {
        unsafe { PixelSetOpacity(self.wand.as_ptr(), opacity) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetopacityquantum>
    ///
    /// PixelSetOpacityQuantum() sets the opacity color of the pixel wand.  The
    ///
    /// color must be in the range of [0..MaxRGB]
    ///
    pub fn set_opacity_quantum(&mut self, opacity: Quantum) -> &mut Self {
        unsafe { PixelSetOpacityQuantum(self.wand.as_ptr(), opacity) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetquantumcolor>
    ///
    /// PixelSetQuantumColor() sets the color of the pixel wand.
    ///
    pub fn set_quantum_color(&mut self, color: &mut PixelPacket) -> &mut Self {
        unsafe { PixelSetQuantumColor(self.wand.as_ptr(), color as *mut PixelPacket) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetred>
    ///
    /// PixelSetRed() sets the normalized red color of the pixel wand.
    ///
    pub fn set_red(&mut self, red: c_double) -> &mut Self {
        unsafe { PixelSetRed(self.wand.as_ptr(), red) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetredquantum>
    ///
    /// PixelSetRedQuantum() sets the red color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_red_quantum(&mut self, red: Quantum) -> &mut Self {
        unsafe { PixelSetRedQuantum(self.wand.as_ptr(), red) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetyellow>
    ///
    /// PixelSetYellow() sets the normalized yellow color of the pixel wand.
    ///
    pub fn set_yellow(&mut self, yellow: c_double) -> &mut Self {
        unsafe { PixelSetYellow(self.wand.as_ptr(), yellow) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetyellowquantum>
    ///
    /// PixelSetYellowQuantum() sets the yellow color of the pixel wand.  The color
    ///
    /// must be in the range of [0..MaxRGB]
    ///
    pub fn set_yellow_quantum(&mut self, yellow: Quantum) -> &mut Self {
        unsafe { PixelSetYellowQuantum(self.wand.as_ptr(), yellow) };
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{initialize, types::PixelPacket, wand::PixelWand};

    fn new_pixel_wand() -> PixelWand {
        initialize();
        PixelWand::new()
    }

    #[test]
    fn test_pixel_wand_get_black() {
        let pw = new_pixel_wand();
        pw.get_black();
    }

    #[test]
    fn test_pixel_wand_get_black_quantum() {
        let pw = new_pixel_wand();
        pw.get_black_quantum();
    }

    #[test]
    fn test_pixel_wand_get_blue() {
        let pw = new_pixel_wand();
        pw.get_blue();
    }

    #[test]
    fn test_pixel_wand_get_blue_quantum() {
        let pw = new_pixel_wand();
        pw.get_blue_quantum();
    }

    #[test]
    fn test_pixel_wand_get_color_as_string() {
        let mut pw = new_pixel_wand();
        pw.get_color_as_string().to_str().unwrap();
    }

    #[test]
    fn test_pixel_wand_get_color_count() {
        let pw = new_pixel_wand();
        pw.get_color_count();
    }

    #[test]
    fn test_pixel_wand_get_cyan() {
        let pw = new_pixel_wand();
        pw.get_cyan();
    }

    #[test]
    fn test_pixel_wand_get_cyan_quantum() {
        let pw = new_pixel_wand();
        pw.get_cyan_quantum();
    }

    #[test]
    fn test_pixel_wand_get_green() {
        let pw = new_pixel_wand();
        pw.get_green();
    }

    #[test]
    fn test_pixel_wand_get_green_quantum() {
        let pw = new_pixel_wand();
        pw.get_green_quantum();
    }

    #[test]
    fn test_pixel_wand_get_magenta() {
        let pw = new_pixel_wand();
        pw.get_magenta();
    }

    #[test]
    fn test_pixel_wand_get_magenta_quantum() {
        let pw = new_pixel_wand();
        pw.get_magenta_quantum();
    }

    #[test]
    fn test_pixel_wand_get_opacity() {
        let pw = new_pixel_wand();
        pw.get_opacity();
    }

    #[test]
    fn test_pixel_wand_get_opacity_quantum() {
        let pw = new_pixel_wand();
        pw.get_opacity_quantum();
    }

    #[test]
    fn test_pixel_wand_get_red() {
        let pw = new_pixel_wand();
        pw.get_red();
    }

    #[test]
    fn test_pixel_wand_get_red_quantum() {
        let pw = new_pixel_wand();
        pw.get_red_quantum();
    }

    #[test]
    fn test_pixel_wand_get_yellow() {
        let pw = new_pixel_wand();
        pw.get_yellow();
    }

    #[test]
    fn test_pixel_wand_get_yellow_quantum() {
        let pw = new_pixel_wand();
        pw.get_yellow_quantum();
    }

    #[test]
    fn test_pixel_wand_set_black() {
        let mut pw = new_pixel_wand();
        pw.set_black(0.);
    }

    #[test]
    fn test_pixel_wand_set_black_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_black_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_blue() {
        let mut pw = new_pixel_wand();
        pw.set_blue(0.);
    }

    #[test]
    fn test_pixel_wand_set_blue_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_blue_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_color() {
        let mut pw = new_pixel_wand();
        pw.set_color("");
    }

    #[test]
    fn test_pixel_wand_set_color_count() {
        let mut pw = new_pixel_wand();
        pw.set_color_count(0);
    }

    #[test]
    fn test_pixel_wand_set_cyan() {
        let mut pw = new_pixel_wand();
        pw.set_cyan(0.);
    }

    #[test]
    fn test_pixel_wand_set_cyan_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_cyan_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_green() {
        let mut pw = new_pixel_wand();
        pw.set_green(0.);
    }

    #[test]
    fn test_pixel_wand_set_green_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_green_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_magenta() {
        let mut pw = new_pixel_wand();
        pw.set_magenta(0.);
    }

    #[test]
    fn test_pixel_wand_set_magenta_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_magenta_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_opacity() {
        let mut pw = new_pixel_wand();
        pw.set_opacity(0.);
    }

    #[test]
    fn test_pixel_wand_set_opacity_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_opacity_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_quantum_color() {
        let mut pw = new_pixel_wand();
        pw.set_quantum_color(&mut PixelPacket {
            blue: 0,
            green: 0,
            red: 0,
            opacity: 0,
        });
    }

    #[test]
    fn test_pixel_wand_set_red() {
        let mut pw = new_pixel_wand();
        pw.set_red(0.);
    }

    #[test]
    fn test_pixel_wand_set_red_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_red_quantum(0);
    }

    #[test]
    fn test_pixel_wand_set_yellow() {
        let mut pw = new_pixel_wand();
        pw.set_yellow(0.);
    }

    #[test]
    fn test_pixel_wand_set_yellow_quantum() {
        let mut pw = new_pixel_wand();
        pw.set_yellow_quantum(0);
    }
}
