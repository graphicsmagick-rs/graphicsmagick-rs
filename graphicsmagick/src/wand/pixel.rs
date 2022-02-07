//! <http://www.graphicsmagick.org/wand/pixel_wand.html>
//!
//! Binding of pixel_wand, Wand pixel access interfaces.

use crate::utils::{assert_initialized, c_str_to_string, str_to_c_string};
use graphicsmagick_sys::*;
use std::{
    os::raw::{c_double, c_ulong},
    ptr::null_mut,
    str::Utf8Error,
};

/// Wrapper of `graphicsmagick_sys::PixelWand`.
pub struct PixelWand {
    wand: *mut graphicsmagick_sys::PixelWand,
}

impl PixelWand {
    pub fn new() -> Self {
        assert_initialized();

        let wand = unsafe { NewPixelWand() };
        assert_ne!(wand, null_mut(), "NewPixelWand return NULL");

        PixelWand { wand }
    }

    #[inline]
    pub fn from_wand(wand: *mut graphicsmagick_sys::PixelWand) -> Option<PixelWand> {
        if wand.is_null() {
            None
        } else {
            Some(PixelWand { wand })
        }
    }

    #[inline]
    pub(crate) fn from_wand_expect(wand: *mut graphicsmagick_sys::PixelWand) -> PixelWand {
        Self::from_wand(wand).expect("wand cant't be null")
    }

    #[inline]
    pub fn wand(&self) -> *const graphicsmagick_sys::PixelWand {
        self.wand
    }

    #[inline]
    pub fn wand_mut(&mut self) -> *mut graphicsmagick_sys::PixelWand {
        self.wand
    }
}

impl Drop for PixelWand {
    fn drop(&mut self) {
        unsafe {
            DestroyPixelWand(self.wand);
        }
    }
}

impl Clone for PixelWand {
    fn clone(&self) -> Self {
        PixelWand {
            wand: unsafe { ClonePixelWand(self.wand) },
        }
    }
}

impl Default for PixelWand {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelWand {
    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblack](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetblack)
    ///
    /// PixelGetBlack() returns the normalized black color of the pixel wand.
    ///
    pub fn get_black(&self) -> c_double {
        unsafe { PixelGetBlack(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblackquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetblackquantum)
    ///
    /// PixelGetBlackQuantum() returns the black color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_black_quantum(&self) -> Quantum {
        unsafe { PixelGetBlackQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetblue](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetblue)
    ///
    /// PixelGetBlue(const) returns the normalized blue color of the pixel wand.
    ///
    pub fn get_blue(&self) -> c_double {
        unsafe { PixelGetBlue(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetbluequantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetbluequantum)
    ///
    /// PixelGetBlueQuantum(const ) returns the blue color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_blue_quantum(&self) -> Quantum {
        unsafe { PixelGetBlueQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcolorasstring](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetcolorasstring)
    ///
    /// PixelGetColorAsString() gets the color of the pixel wand.
    ///
    pub fn get_color_as_string(&mut self) -> Result<String, Utf8Error> {
        let c = unsafe { PixelGetColorAsString(self.wand) };
        c_str_to_string(c)
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcolorcount](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetcolorcount)
    ///
    /// PixelGetColorCount() returns the color count associated with this color.
    ///
    pub fn get_color_count(&self) -> c_ulong {
        unsafe { PixelGetColorCount(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcyan](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetcyan)
    ///
    /// PixelGetCyan() returns the normalized cyan color of the pixel wand.
    ///
    pub fn get_cyan(&self) -> c_double {
        unsafe { PixelGetCyan(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetcyanquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetcyanquantum)
    ///
    /// PixelGetCyanQuantum() returns the cyan color of the pixel wand.  The color
    ///
    /// is in the range of [0..MaxRGB]
    ///
    pub fn get_cyan_quantum(&self) -> Quantum {
        unsafe { PixelGetCyanQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetgreen](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetgreen)
    ///
    /// PixelGetGreen(const ) returns the normalized green color of the pixel wand.
    ///
    pub fn get_green(&self) -> c_double {
        unsafe { PixelGetGreen(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetgreenquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetgreenquantum)
    ///
    /// PixelGetGreenQuantum(const ) returns the green color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_green_quantum(&self) -> Quantum {
        unsafe { PixelGetGreenQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetmagenta](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetmagenta)
    ///
    /// PixelGetMagenta() returns the normalized magenta color of the pixel wand.
    ///
    pub fn get_magenta(&self) -> c_double {
        unsafe { PixelGetMagenta(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetmagentaquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetmagentaquantum)
    ///
    /// PixelGetMagentaQuantum() returns the magenta color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_magenta_quantum(&self) -> Quantum {
        unsafe { PixelGetMagentaQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetopacity](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetopacity)
    ///
    /// PixelGetOpacity(const ) returns the normalized opacity color of the pixel
    ///
    /// wand.
    ///
    pub fn get_opacity(&self) -> c_double {
        unsafe { PixelGetOpacity(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetopacityquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetopacityquantum)
    ///
    /// PixelGetOpacityQuantum(const ) returns the opacity color of the pixel wand.
    ///
    /// The color is in the range of [0..MaxRGB]
    ///
    pub fn get_opacity_quantum(&self) -> Quantum {
        unsafe { PixelGetOpacityQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetred](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetred)
    ///
    /// PixelGetRed(const ) returns the normalized red color of the pixel wand.
    ///
    pub fn get_red(&self) -> c_double {
        unsafe { PixelGetRed(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetredquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetredquantum)
    ///
    /// PixelGetRedQuantum(const ) returns the red color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_red_quantum(&self) -> Quantum {
        unsafe { PixelGetRedQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetyellow](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetyellow)
    ///
    /// PixelGetYellow() returns the normalized yellow color of the pixel wand.
    ///
    pub fn get_yellow(&self) -> c_double {
        unsafe { PixelGetYellow(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelgetyellowquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelgetyellowquantum)
    ///
    /// PixelGetYellowQuantum() returns the yellow color of the pixel wand.  The
    ///
    /// color is in the range of [0..MaxRGB]
    ///
    pub fn get_yellow_quantum(&self) -> Quantum {
        unsafe { PixelGetYellowQuantum(self.wand) }
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblack](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetblack)
    ///
    /// PixelSetBlack() sets the normalized black color of the pixel wand.
    ///
    pub fn set_black(&mut self, black: c_double) -> &mut Self {
        unsafe { PixelSetBlack(self.wand, black) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblackquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetblackquantum)
    ///
    /// PixelSetBlackQuantum() sets the black color of the pixel wand.  The color
    ///
    /// must be in the range of [0..MaxRGB]
    ///
    pub fn set_black_quantum(&mut self, black: Quantum) -> &mut Self {
        unsafe { PixelSetBlackQuantum(self.wand, black) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetblue](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetblue)
    ///
    /// PixelSetBlue() sets the normalized blue color of the pixel wand.
    ///
    pub fn set_blue(&mut self, blue: c_double) -> &mut Self {
        unsafe { PixelSetBlue(self.wand, blue) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetbluequantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetbluequantum)
    ///
    /// PixelSetBlueQuantum() sets the blue color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_blue_quantum(&mut self, blue: Quantum) -> &mut Self {
        unsafe { PixelSetBlueQuantum(self.wand, blue) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcolor](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetcolor)
    ///
    /// PixelSetColor() sets the color of the pixel wand with a string (e.g.
    ///
    /// &quot;blue&quot;, &quot;#0000ff&quot;, &quot;rgb(0,0,255)&quot;, etc.).
    ///
    pub fn set_color(&mut self, color: &str) -> &mut Self {
        let color = str_to_c_string(color);
        unsafe { PixelSetColor(self.wand, color.as_ptr()) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcolorcount](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetcolorcount)
    ///
    /// PixelSetColorCount() sets the color count of the pixel wand.
    ///
    pub fn set_color_count(&mut self, count: c_ulong) -> &mut Self {
        unsafe { PixelSetColorCount(self.wand, count) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcyan](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetcyan)
    ///
    /// PixelSetCyan() sets the normalized cyan color of the pixel wand.
    ///
    pub fn set_cyan(&mut self, cyan: c_double) -> &mut Self {
        unsafe { PixelSetCyan(self.wand, cyan) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetcyanquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetcyanquantum)
    ///
    /// PixelSetCyanQuantum() sets the cyan color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_cyan_quantum(&mut self, cyan: Quantum) -> &mut Self {
        unsafe { PixelSetCyanQuantum(self.wand, cyan) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetgreen](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetgreen)
    ///
    /// PixelSetGreen() sets the normalized green color of the pixel wand.
    ///
    pub fn set_green(&mut self, green: c_double) -> &mut Self {
        unsafe { PixelSetGreen(self.wand, green) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetgreenquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetgreenquantum)
    ///
    /// PixelSetGreenQuantum() sets the green color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_green_quantum(&mut self, green: Quantum) -> &mut Self {
        unsafe { PixelSetGreenQuantum(self.wand, green) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetmagenta](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetmagenta)
    ///
    /// PixelSetMagenta() sets the normalized magenta color of the pixel wand.
    ///
    pub fn set_magenta(&mut self, magenta: c_double) -> &mut Self {
        unsafe { PixelSetMagenta(self.wand, magenta) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetmagentaquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetmagentaquantum)
    ///
    /// PixelSetMagentaQuantum() sets the magenta color of the pixel wand.  The
    ///
    /// color must be in the range of [0..MaxRGB]
    ///
    pub fn set_magenta_quantum(&mut self, magenta: Quantum) -> &mut Self {
        unsafe { PixelSetMagentaQuantum(self.wand, magenta) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetopacity](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetopacity)
    ///
    /// PixelSetOpacity() sets the normalized opacity color of the pixel wand.
    ///
    pub fn set_opacity(&mut self, opacity: c_double) -> &mut Self {
        unsafe { PixelSetOpacity(self.wand, opacity) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetopacityquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetopacityquantum)
    ///
    /// PixelSetOpacityQuantum() sets the opacity color of the pixel wand.  The
    ///
    /// color must be in the range of [0..MaxRGB]
    ///
    pub fn set_opacity_quantum(&mut self, opacity: Quantum) -> &mut Self {
        unsafe { PixelSetOpacityQuantum(self.wand, opacity) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetquantumcolor](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetquantumcolor)
    ///
    /// PixelSetQuantumColor() sets the color of the pixel wand.
    ///
    pub fn set_quantum_color(&mut self, color: &mut PixelPacket) -> &mut Self {
        unsafe { PixelSetQuantumColor(self.wand, color as *mut PixelPacket) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetred](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetred)
    ///
    /// PixelSetRed() sets the normalized red color of the pixel wand.
    ///
    pub fn set_red(&mut self, red: c_double) -> &mut Self {
        unsafe { PixelSetRed(self.wand, red) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetredquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetredquantum)
    ///
    /// PixelSetRedQuantum() sets the red color of the pixel wand.  The color must
    ///
    /// be in the range of [0..MaxRGB]
    ///
    pub fn set_red_quantum(&mut self, red: Quantum) -> &mut Self {
        unsafe { PixelSetRedQuantum(self.wand, red) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetyellow](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetyellow)
    ///
    /// PixelSetYellow() sets the normalized yellow color of the pixel wand.
    ///
    pub fn set_yellow(&mut self, yellow: c_double) -> &mut Self {
        unsafe { PixelSetYellow(self.wand, yellow) };
        self
    }

    /// [http://www.graphicsmagick.org/wand/pixel_wand.html#pixelsetyellowquantum](http://www.graphicspixel.org/wand/pixel_wand.html#pixelsetyellowquantum)
    ///
    /// PixelSetYellowQuantum() sets the yellow color of the pixel wand.  The color
    ///
    /// must be in the range of [0..MaxRGB]
    ///
    pub fn set_yellow_quantum(&mut self, yellow: Quantum) -> &mut Self {
        unsafe { PixelSetYellowQuantum(self.wand, yellow) };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialize;

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
        pw.get_color_as_string().unwrap();
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
