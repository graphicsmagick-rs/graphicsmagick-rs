use crate::utils::assert_initialized;
use graphicsmagick_sys::{ClonePixelWand, DestroyPixelWand, NewPixelWand, PixelSetBlack,
                         PixelSetOpacity,
};
use std::ptr::null_mut;

pub struct PixelWand {
    pub(crate) wand: *mut graphicsmagick_sys::PixelWand,
}

impl PixelWand {
    pub fn new() -> Self {
        assert_initialized();

        let wand = unsafe { NewPixelWand() };
        assert_ne!(wand, null_mut(), "NewPixelWand return NULL");

        PixelWand { wand }
    }

    pub fn set_black(&mut self, black: f64) -> &mut Self {
        unsafe { PixelSetBlack(self.wand, black) };
        self
    }

    pub fn set_opacity(&mut self, opacity: f64) -> &mut Self {
        unsafe { PixelSetOpacity(self.wand, opacity)  };
        self
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
