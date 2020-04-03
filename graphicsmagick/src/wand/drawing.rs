use crate::utils::assert_initialized;
use graphicsmagick_sys::{
    MagickCloneDrawingWand, MagickDestroyDrawingWand, MagickDrawRectangle,
    MagickDrawRoundRectangle, MagickDrawSetFillOpacity, MagickNewDrawingWand,
};
use std::ptr::null_mut;

pub struct DrawingWand {
    pub(crate) wand: *mut graphicsmagick_sys::DrawingWand,
}

impl DrawingWand {
    pub fn new() -> Self {
        assert_initialized();

        let wand = unsafe { MagickNewDrawingWand() };
        assert_ne!(wand, null_mut(), "NewDrawingWand return NULL");

        DrawingWand { wand }
    }

    pub fn draw_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> &mut Self {
        unsafe { MagickDrawRectangle(self.wand, x1, y1, x2, y2) };
        self
    }

    pub fn draw_round_rectangle(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        rx: f64,
        ry: f64,
    ) -> &mut Self {
        unsafe { MagickDrawRoundRectangle(self.wand, x1, y1, x2, y2, rx, ry) };
        self
    }

    pub fn set_fill_opacity(&mut self, fill_opacity: f64) -> &mut Self {
        unsafe { MagickDrawSetFillOpacity(self.wand, fill_opacity) };
        self
    }
}

impl Drop for DrawingWand {
    fn drop(&mut self) {
        unsafe {
            MagickDestroyDrawingWand(self.wand);
        }
    }
}

impl Clone for DrawingWand {
    fn clone(&self) -> Self {
        DrawingWand {
            wand: unsafe { MagickCloneDrawingWand(self.wand) },
        }
    }
}
