use crate::{
    error::Exception,
    initialize,
    types::{CompositeOperator, FilterType, ChannelType},
    utils::{assert_initialized, str_to_c_string},
    wand::{DrawingWand, PixelWand},
};
use graphicsmagick_sys::{
    size_t, CloneMagickWand, DestroyMagickWand, ExceptionInfo, ExceptionType, MagickCompositeImage,
    MagickCropImage, MagickDrawImage, MagickFlipImage, MagickFlopImage, MagickFree,
    MagickGetException, MagickGetImageFormat, MagickGetImageHeight, MagickGetImageWidth,
    MagickPass, MagickReadImage, MagickReadImageBlob, MagickRelinquishMemory, MagickResetIterator,
    MagickResizeImage, MagickRotateImage, MagickScaleImage, MagickSetImageFormat,
    MagickUnsharpMaskImage, MagickWriteImageBlob, NewMagickWand, MagickBlurImage,
    MagickColorizeImage, MagickFxImageChannel, MagickSetImageChannelDepth
};
use std::{
    ffi::CStr,
    mem::MaybeUninit,
    os::raw::{c_uint, c_void},
    ptr::{null, null_mut},
};

pub struct MagickWand<'a> {
    wand: *mut graphicsmagick_sys::MagickWand,
    blob: Option<&'a [u8]>,
}

impl<'a> MagickWand<'a> {
    pub fn new() -> Self {
        assert_initialized();

        let wand = unsafe { NewMagickWand() };
        assert_ne!(wand, null_mut(), "NewMagickWand return NULL");

        MagickWand { wand, blob: None }
    }

    pub fn read_image_blob(&mut self, blob: &'a [u8]) -> crate::Result<&mut Self> {
        self.blob = Some(blob);
        let length = blob.len() as size_t;
        let blob = blob.as_ptr();
        let status = unsafe { MagickReadImageBlob(self.wand, blob, length) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn resize_image(
        &mut self,
        columns: u64,
        rows: u64,
        filter: FilterType,
        blur: f64,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickResizeImage(self.wand, columns, rows, filter.into(), blur) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn set_image_format(&mut self, format: &str) -> crate::Result<&mut Self> {
        let format = str_to_c_string(format);
        let status = unsafe { MagickSetImageFormat(self.wand, format.as_ptr()) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn get_image_format(&mut self) -> crate::Result<String> {
        let format_ptr = unsafe { MagickGetImageFormat(self.wand) };
        let format = unsafe { CStr::from_ptr(format_ptr) }.to_str()?.to_string();
        unsafe { MagickFree(format_ptr as *mut c_void) };
        Ok(format)
    }

    #[inline]
    pub fn get_image_width(&mut self) -> u64 {
        (unsafe { MagickGetImageWidth(self.wand) }) as u64
    }

    #[inline]
    pub fn get_image_height(&mut self) -> u64 {
        (unsafe { MagickGetImageHeight(self.wand) }) as u64
    }

    pub fn draw_image(&mut self, drawing_wand: &mut DrawingWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickDrawImage(self.wand, drawing_wand.wand) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn composite_image(
        &mut self,
        composite_wand: &mut MagickWand<'_>,
        compose: CompositeOperator,
        x: i64,
        y: i64,
    ) -> crate::Result<&mut Self> {
        let status =
            unsafe { MagickCompositeImage(self.wand, composite_wand.wand, compose.into(), x, y) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn write_image_blob(&mut self) -> crate::Result<Vec<u8>> {
        unsafe { MagickResetIterator(self.wand) };
        let mut length: size_t = 0;
        let ptr = unsafe { MagickWriteImageBlob(self.wand, &mut length as *mut size_t) };
        let content = unsafe { Vec::from_raw_parts(ptr, length as usize, length as usize) };
        Ok(content)
    }

    pub fn crop_image(
        &mut self,
        width: u64,
        height: u64,
        x: i64,
        y: i64,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickCropImage(self.wand, width, height, x, y) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn read_image(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickReadImage(self.wand, filename.as_ptr()) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn scale_image(&mut self, columns: u64, rows: u64) -> crate::Result<&mut Self> {
        let status = unsafe { MagickScaleImage(self.wand, columns, rows) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn rotate_image(
        &mut self,
        background: &mut PixelWand,
        degrees: f64,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRotateImage(self.wand, background.wand, degrees) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn unsharp_mask_image(
        &mut self,
        radius: f64,
        sigma: f64,
        amount: f64,
        threshold: f64,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickUnsharpMaskImage(self.wand, radius, sigma, amount, threshold) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn flip_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickFlipImage(self.wand) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn flop_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickFlopImage(self.wand) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn blur_image(&mut self, radius: f64, sigma: f64) -> crate::Result<&mut Self> {
        let status = unsafe { MagickBlurImage(self.wand, radius, sigma) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn colorize_image(&mut self, colorize: &mut PixelWand, opacity: &mut PixelWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickColorizeImage(self.wand, colorize.wand, opacity.wand) };
        self.check_status(status)?;
        Ok(self)
    }

    pub fn fx_image_channel(&mut self, channel: ChannelType, expression: &str) -> crate::Result<MagickWand<'_>> {
        let expression = str_to_c_string(expression);
        let wand = unsafe { MagickFxImageChannel(self.wand, channel.into(), expression.as_ptr()) };
        Ok(MagickWand { wand, blob: None })
    }

    pub fn set_image_channel_depth(&mut self, channel: ChannelType, depth: u64) -> crate::Result<&mut Self>{
        let status = unsafe { MagickSetImageChannelDepth(self.wand, channel.into(), depth) };
        self.check_status(status)?;
        Ok(self)
    }

    #[inline]
    fn check_status(&self, status: c_uint) -> crate::Result<()> {
        if status == MagickPass {
            Ok(())
        } else {
            Err(self.get_exception())
        }
    }

    fn get_exception(&self) -> crate::Error {
        let mut severity: ExceptionType = 0;
        unsafe {
            let description_ptr =
                MagickGetException(self.wand, &mut severity as *mut ExceptionType);
            if description_ptr != null_mut() {
                let description = match CStr::from_ptr(description_ptr).to_str() {
                    Ok(description) => description,
                    Err(e) => return e.into(),
                };
                MagickRelinquishMemory(description_ptr as *mut c_void);
                Exception::new(severity.into(), description.to_string()).into()
            } else {
                Exception::new(0.into(), "".to_string()).into()
            }
        }
    }
}

impl Drop for MagickWand<'_> {
    fn drop(&mut self) {
        unsafe {
            DestroyMagickWand(self.wand);
        }
    }
}

impl Clone for MagickWand<'_> {
    fn clone(&self) -> Self {
        MagickWand {
            wand: unsafe { CloneMagickWand(self.wand) },
            blob: self.blob,
        }
    }
}
