//! Binding of magick_wand, Wand image processing interfaces.
//!
//! <http://www.graphicsmagick.org/wand/magick_wand.html>

use crate::{
    error::Exception,
    types::{
        ChannelType, ColorspaceType, CompositeOperator, CompressionType, DisposeType, FilterTypes,
        ImageType, InterlaceType, MetricType, MontageMode, NoiseType, PreviewType, RenderingIntent,
        ResolutionType, ResourceType, VirtualPixelMethod,
    },
    utils::{assert_initialized, c_arr_to_vec, str_to_c_string},
    wand::{DrawingWand, PixelWand},
    MagickCString,
};
use graphicsmagick_sys::*;
use std::{
    ffi::CStr,
    fmt,
    os::raw::{c_double, c_float, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void},
    ptr::null_mut,
    slice,
};

#[cfg(feature = "v1_3_26")]
use crate::types::OrientationType;

#[cfg(feature = "v1_3_22")]
use crate::types::GravityType;

/// # Safety
///
/// `STORAGE_TYPE` must match `TARGET`.
pub unsafe trait MagickWandExportTypeSealed {
    const STORAGE_TYPE: StorageType;

    /// Make using it in generic code easier.
    type Target: fmt::Debug + fmt::Display + Copy + Clone;
}

pub trait MagickWandExportType: MagickWandExportTypeSealed {}

macro_rules! def_magickwand_export_type {
    ($name:ident, $STORAGE_TYPE:expr, $Target:ty) => {
        pub struct $name;
        unsafe impl MagickWandExportTypeSealed for $name {
            const STORAGE_TYPE: StorageType = $STORAGE_TYPE;
            type Target = $Target;
        }
        impl MagickWandExportType for $name {}
    };
}

def_magickwand_export_type!(MagickWandExportCharPixel, StorageType_CharPixel, c_uchar);
def_magickwand_export_type!(MagickWandExportShortPixel, StorageType_ShortPixel, c_ushort);
def_magickwand_export_type!(
    MagickWandExportIntegerPixel,
    StorageType_IntegerPixel,
    c_uint
);
def_magickwand_export_type!(MagickWandExportLongPixel, StorageType_LongPixel, c_ulong);
def_magickwand_export_type!(MagickWandExportFloatPixel, StorageType_FloatPixel, c_float);
def_magickwand_export_type!(
    MagickWandExportDoublePixel,
    StorageType_DoublePixel,
    c_double
);

/// Wrapper of `graphicsmagick_sys::MagickWand`.
pub struct MagickWand<'a> {
    wand: *mut graphicsmagick_sys::MagickWand,
    blob: Option<&'a [u8]>,
}

impl<'a> MagickWand<'a> {
    /// Construct an empty MagickWand.
    ///
    /// # Panic
    ///
    /// Panic if not call [`crate::initialize`] first of all.
    ///
    pub fn new() -> Self {
        assert_initialized();

        let wand = unsafe { NewMagickWand() };
        assert_ne!(wand, null_mut(), "NewMagickWand return NULL");

        MagickWand { wand, blob: None }
    }

    #[inline]
    fn check_status(&mut self, status: c_uint) -> crate::Result<&mut Self> {
        if status == MagickPass {
            Ok(self)
        } else {
            Err(unsafe { self.get_error() })
        }
    }

    unsafe fn get_error(&mut self) -> crate::Error {
        let mut severity: ExceptionType = 0;

        let description_ptr = MagickGetException(self.wand, &mut severity as *mut ExceptionType);

        if description_ptr.is_null() {
            return Exception::new(0.into(), "Unknown exception".to_string()).into();
        }
        let description =
            slice::from_raw_parts(description_ptr as *const u8, libc::strlen(description_ptr));
        let description = match std::str::from_utf8(description) {
            Ok(description) => description.to_string(),
            Err(e) => return e.into(),
        };
        MagickRelinquishMemory(description_ptr as *mut c_void);
        Exception::new(severity.into(), description).into()
    }

    #[inline]
    pub fn from_wand(wand: *mut graphicsmagick_sys::MagickWand) -> Option<MagickWand<'a>> {
        if wand.is_null() {
            None
        } else {
            Some(MagickWand { wand, blob: None })
        }
    }

    #[inline]
    pub fn wand(&self) -> *const graphicsmagick_sys::MagickWand {
        self.wand
    }

    #[inline]
    pub fn wand_mut(&mut self) -> *mut graphicsmagick_sys::MagickWand {
        self.wand
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

impl Default for MagickWand<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MagickWand<'a> {
    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickadaptivethresholdimage>
    ///
    /// MagickAdaptiveThresholdImage() selects an individual threshold for each pixel
    ///
    /// based on the range of intensity values in its local neighborhood.  This
    ///
    /// allows for thresholding of an image whose global intensity histogram
    ///
    /// doesn't contain distinctive peaks.
    ///
    /// # Panic
    ///
    ///  Panic if `width` or `height` is zero.
    ///
    pub fn adaptive_threshold_image(
        &mut self,
        width: c_ulong,
        height: c_ulong,
        offset: c_long,
    ) -> crate::Result<&mut Self> {
        assert_ne!(width, 0, "width must be positive");
        assert_ne!(height, 0, "height must be positive");
        let status = unsafe { MagickAdaptiveThresholdImage(self.wand, width, height, offset) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickaddimage>
    ///
    /// MagickAddImage() adds the specified images at the current image location.
    ///
    pub fn add_image(&mut self, add_wand: &MagickWand<'_>) -> crate::Result<&mut Self> {
        let status = unsafe { MagickAddImage(self.wand, add_wand.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickaddnoiseimage>
    ///
    /// MagickAddNoiseImage() adds random noise to the image.
    ///
    pub fn add_noise_image(&mut self, noise_type: NoiseType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickAddNoiseImage(self.wand, noise_type.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickaffinetransformimage>
    ///
    /// MagickAffineTransformImage() transforms an image as dictated by the affine
    ///
    /// matrix of the drawing wand.
    ///
    pub fn affine_transform_image(
        &mut self,
        drawing_wand: &DrawingWand,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickAffineTransformImage(self.wand, drawing_wand.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickannotateimage>
    ///
    /// MagickAnnotateImage() annotates an image with text.
    ///
    pub fn annotate_image(
        &mut self,
        drawing_wand: &DrawingWand,
        x: c_double,
        y: c_double,
        angle: c_double,
        text: &str,
    ) -> crate::Result<&mut Self> {
        let text = str_to_c_string(text);
        let status = unsafe {
            MagickAnnotateImage(self.wand, drawing_wand.wand(), x, y, angle, text.as_ptr())
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickanimateimages>
    ///
    /// MagickAnimateImages() animates an image or image sequence.
    ///
    pub fn animate_images(&mut self, server_name: &str) -> crate::Result<&mut Self> {
        let server_name = str_to_c_string(server_name);
        let status = unsafe { MagickAnimateImages(self.wand, server_name.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickappendimages>
    ///
    /// MagickAppendImages() append a set of images.
    ///
    pub fn append_images(&mut self, stack: c_uint) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickAppendImages(self.wand, stack) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickautoorientimage>
    ///
    /// MagickAutoOrientImage() adjusts the current image so that its orientation
    ///
    /// is suitable for viewing (i.e. top-left orientation).
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn auto_orient_image(
        &mut self,
        current_orientation: OrientationType,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickAutoOrientImage(self.wand, current_orientation.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickaverageimages>
    ///
    /// MagickAverageImages() average a set of images.
    ///
    pub fn average_images(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickAverageImages(self.wand) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickblackthresholdimage>
    ///
    /// MagickBlackThresholdImage() is like MagickThresholdImage() but  forces all
    ///
    /// pixels below the threshold into black while leaving all pixels above the
    ///
    /// threshold unchanged.
    ///
    pub fn black_threshold_image(&mut self, threshold: &PixelWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickBlackThresholdImage(self.wand, threshold.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickblurimage>
    ///
    /// MagickBlurImage() blurs an image.  We convolve the image with a Gaussian
    ///
    /// operator of the given radius and standard deviation (sigma).
    ///
    /// For reasonable results, the radius should be larger than sigma.  Use a
    ///
    /// radius of 0 and BlurImage() selects a suitable radius for you.
    ///
    pub fn blur_image(&mut self, radius: c_double, sigma: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickBlurImage(self.wand, radius, sigma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickborderimage>
    ///
    /// MagickBorderImage() surrounds the image with a border of the color defined
    ///
    /// by the bordercolor pixel wand.
    ///
    pub fn border_image(
        &mut self,
        border_color: &PixelWand,
        width: c_ulong,
        height: c_ulong,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickBorderImage(self.wand, border_color.wand(), width, height) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcdlimage>
    ///
    /// The MagickCdlImage() method applies ("bakes in") the ASC-CDL which is a
    ///
    /// format for the exchange of basic primary color grading information between
    ///
    /// equipment and software from different manufacturers. The format defines
    ///
    /// the math for three functions: slope, offset and power. Each function uses
    ///
    /// a number for the red, green, and blue color channels for a total of nine
    ///
    /// numbers comprising a single color decision. A tenth number for chrominance
    ///
    /// (saturation) has been proposed but is not yet standardized.
    ///
    /// The cdl argument string is comma delimited and is in the form (but
    ///
    /// without invervening spaces or line breaks):
    ///
    /// redslope, redoffset, redpower : greenslope, greenoffset, greenpower : blueslope, blueoffset, bluepower : saturation
    ///
    /// with the unity (no change) specification being:
    ///
    /// "1.0,0.0,1.0:1.0,0.0,1.0:1.0,0.0,1.0:0.0"
    ///
    /// See <http://en.wikipedia.org/wiki/ASC_CDL> for more information.
    ///
    pub fn cdl_image(&mut self, cdl: &str) -> crate::Result<&mut Self> {
        let cdl = str_to_c_string(cdl);
        let status = unsafe { MagickCdlImage(self.wand, cdl.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcharcoalimage>
    ///
    /// MagickCharcoalImage() simulates a charcoal drawing.
    ///
    pub fn charcoal_image(
        &mut self,
        radius: c_double,
        sigma: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickCharcoalImage(self.wand, radius, sigma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickchopimage>
    ///
    /// MagickChopImage() removes a region of an image and collapses the image to
    ///
    /// occupy the removed portion.
    ///
    pub fn chop_image(
        &mut self,
        width: c_ulong,
        height: c_ulong,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickChopImage(self.wand, width, height, x, y) };
        self.check_status(status)
    }

    // No usage.
    //    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickclearexception>
    //    ///
    //    /// MagickClearException() clears the last wand exception.
    //    ///
    //    pub fn clear_exception(&mut self) {
    //        let status = unsafe { MagickClearException(self.wand) };
    //    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickclipimage>
    ///
    /// MagickClipImage() clips along the first path from the 8BIM profile, if
    ///
    /// present.
    ///
    pub fn clip_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickClipImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickclippathimage>
    ///
    /// MagickClipPathImage() clips along the named paths from the 8BIM profile, if
    ///
    /// present. Later operations take effect inside the path.  Id may be a number
    ///
    /// if preceded with #, to work on a numbered path, e.g., "#1" to use the first
    ///
    /// path.
    ///
    pub fn clip_path_image(&mut self, path_name: &str, inside: bool) -> crate::Result<&mut Self> {
        let path_name = str_to_c_string(path_name);
        let status =
            unsafe { MagickClipPathImage(self.wand, path_name.as_ptr(), inside as c_uint) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcoalesceimages>
    ///
    /// MagickCoalesceImages() composites a set of images while respecting any page
    ///
    /// offsets and disposal methods.  GIF, MIFF, and MNG animation sequences
    ///
    /// typically start with an image background and each subsequent image
    ///
    /// varies in size and offset.  MagickCoalesceImages() returns a new sequence
    ///
    /// where each image in the sequence is the same size as the first and
    ///
    /// composited with the next image in the sequence.
    ///
    pub fn coalesce_images(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickCoalesceImages(self.wand) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcolorfloodfillimage>
    ///
    /// MagickColorFloodfillImage() changes the color value of any pixel that matches
    ///
    /// target and is an immediate neighbor.  If the method FillToBorderMethod is
    ///
    /// specified, the color value is changed for any neighbor pixel that does not
    ///
    /// match the bordercolor member of image.
    ///
    pub fn color_floodfill_image(
        &mut self,
        fill: &PixelWand,
        fuzz: c_double,
        border_color: &PixelWand,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickColorFloodfillImage(self.wand, fill.wand(), fuzz, border_color.wand(), x, y)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcolorizeimage>
    ///
    /// MagickColorizeImage() blends the fill color with each pixel in the image.
    ///
    pub fn colorize_image(
        &mut self,
        colorize: &PixelWand,
        opacity: &PixelWand,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickColorizeImage(self.wand, colorize.wand(), opacity.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcommentimage>
    ///
    /// MagickCommentImage() adds a comment to your image.
    ///
    pub fn comment_image(&mut self, comment: &str) -> crate::Result<&mut Self> {
        let comment = str_to_c_string(comment);
        let status = unsafe { MagickCommentImage(self.wand, comment.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcompareimagechannels>
    ///
    /// MagickCompareImageChannels() compares one or more image channels and
    ///
    /// returns the specified distortion metric.
    ///
    pub fn compare_image_channels(
        &mut self,
        reference: &MagickWand<'_>,
        channel: ChannelType,
        metric: MetricType,
        distortion: &mut c_double,
    ) -> Option<MagickWand<'_>> {
        let wand = unsafe {
            MagickCompareImageChannels(
                self.wand,
                reference.wand,
                channel.into(),
                metric.into(),
                distortion,
            )
        };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcompareimages>
    ///
    /// MagickCompareImage() compares one or more images and returns the specified
    ///
    /// distortion metric.
    ///
    pub fn compare_images(
        &mut self,
        reference: &MagickWand<'_>,
        metric: MetricType,
        distortion: &mut c_double,
    ) -> Option<MagickWand<'_>> {
        let wand =
            unsafe { MagickCompareImages(self.wand, reference.wand, metric.into(), distortion) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcompositeimage>
    ///
    /// MagickCompositeImage() composite one image onto another at the specified
    ///
    /// offset.
    ///
    pub fn composite_image(
        &mut self,
        composite_wand: &MagickWand<'_>,
        compose: CompositeOperator,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status =
            unsafe { MagickCompositeImage(self.wand, composite_wand.wand, compose.into(), x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcontrastimage>
    ///
    /// MagickContrastImage() enhances the intensity differences between the lighter
    ///
    /// and darker elements of the image.  Set sharpen to a value other than 0 to
    ///
    /// increase the image contrast otherwise the contrast is reduced.
    ///
    pub fn contrast_image(&mut self, sharpen: c_uint) -> crate::Result<&mut Self> {
        let status = unsafe { MagickContrastImage(self.wand, sharpen) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickconvolveimage>
    ///
    /// MagickConvolveImage() applies a custom convolution kernel to the image.
    ///
    pub fn convolve_image(&mut self, kernel: &[c_double]) -> crate::Result<&mut Self> {
        let status =
            unsafe { MagickConvolveImage(self.wand, kernel.len() as c_ulong, kernel.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcropimage>
    ///
    /// MagickCropImage() extracts a region of the image.
    ///
    pub fn crop_image(
        &mut self,
        width: c_ulong,
        height: c_ulong,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickCropImage(self.wand, width, height, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickcyclecolormapimage>
    ///
    /// MagickCycleColormapImage() displaces an image's colormap by a given number
    ///
    /// of positions.  If you cycle the colormap a number of times you can produce
    ///
    /// a psychodelic effect.
    ///
    pub fn cycle_colormap_image(&mut self, displace: c_long) -> crate::Result<&mut Self> {
        let status = unsafe { MagickCycleColormapImage(self.wand, displace) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdeconstructimages>
    ///
    /// MagickDeconstructImages() compares each image with the next in a sequence
    ///
    /// and returns the maximum bounding region of any pixel differences it
    ///
    /// discovers.
    ///
    pub fn deconstruct_images(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickDeconstructImages(self.wand) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdescribeimage>
    ///
    /// MagickDescribeImage()  describes an image by formatting its attributes
    ///
    /// to an allocated string which must be freed by the user.  Attributes
    ///
    /// include the image width, height, size, and others.  The string is
    ///
    /// similar to the output of 'identify -verbose'.
    ///
    pub fn describe_image(&mut self) -> MagickCString {
        unsafe { MagickCString::new(MagickDescribeImage(self.wand)) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdespeckleimage>
    ///
    /// MagickDespeckleImage() reduces the speckle noise in an image while
    ///
    /// perserving the edges of the original image.
    ///
    pub fn despeckle_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickDespeckleImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdisplayimage>
    ///
    /// MagickDisplayImage() displays an image.
    ///
    pub fn display_image(&mut self, server_name: &str) -> crate::Result<&mut Self> {
        let server_name = str_to_c_string(server_name);
        let status = unsafe { MagickDisplayImage(self.wand, server_name.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdisplayimages>
    ///
    /// MagickDisplayImages() displays an image or image sequence.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn display_images(&mut self, server_name: &str) -> crate::Result<&mut Self> {
        let server_name = str_to_c_string(server_name);
        let status = unsafe { MagickDisplayImages(self.wand, server_name.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickdrawimage>
    ///
    /// MagickDrawImage() draws vectors on the image as described by DrawingWand.
    ///
    pub fn draw_image(&mut self, drawing_wand: &DrawingWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickDrawImage(self.wand, drawing_wand.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickedgeimage>
    ///
    /// MagickEdgeImage() enhance edges within the image with a convolution filter
    ///
    /// of the given radius.  Use a radius of 0 and Edge() selects a suitable
    ///
    /// radius for you.
    ///
    pub fn edge_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickEdgeImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickembossimage>
    ///
    /// MagickEmbossImage() returns a grayscale image with a three-dimensional
    ///
    /// effect.  We convolve the image with a Gaussian operator of the given radius
    ///
    /// and standard deviation (sigma).  For reasonable results, radius should be
    ///
    /// larger than sigma.  Use a radius of 0 and Emboss() selects a suitable
    ///
    /// radius for you.
    ///
    pub fn emboss_image(&mut self, radius: c_double, sigma: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickEmbossImage(self.wand, radius, sigma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickenhanceimage>
    ///
    /// MagickEnhanceImage() applies a digital filter that improves the quality of a
    ///
    /// noisy image.
    ///
    pub fn enhance_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickEnhanceImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickequalizeimage>
    ///
    /// MagickEqualizeImage() equalizes the image histogram.
    ///
    pub fn equalize_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickEqualizeImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickextentimage>
    ///
    /// Use MagickExtentImage() to change the image dimensions as specified by
    ///
    /// geometry width and height.  The existing image content is composited at
    ///
    /// the position specified by geometry x and y using the image compose method.
    ///
    /// Existing image content which falls outside the bounds of the new image
    ///
    /// dimensions is discarded.
    ///
    pub fn extent_image(
        &mut self,
        width: size_t,
        height: size_t,
        x: ssize_t,
        y: ssize_t,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickExtentImage(self.wand, width, height, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickflattenimages>
    ///
    /// MagickFlattenImages() merges a sequence of images.  This is useful for
    ///
    /// combining Photoshop layers into a single image.
    ///
    pub fn flatten_images(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickFlattenImages(self.wand) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickflipimage>
    ///
    /// MagickFlipImage() creates a vertical mirror image by reflecting the pixels
    ///
    /// around the central x-axis.
    ///
    pub fn flip_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickFlipImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickflopimage>
    ///
    /// MagickFlopImage() creates a horizontal mirror image by reflecting the pixels
    ///
    /// around the central y-axis.
    ///
    pub fn flop_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickFlopImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickframeimage>
    ///
    /// MagickFrameImage() adds a simulated three-dimensional border around the
    ///
    /// image.  The width and height specify the border width of the vertical and
    ///
    /// horizontal sides of the frame.  The inner and outer bevels indicate the
    ///
    /// width of the inner and outer shadows of the frame.
    ///
    pub fn frame_image(
        &mut self,
        matte_color: &PixelWand,
        width: c_ulong,
        height: c_ulong,
        inner_bevel: c_long,
        outer_bevel: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickFrameImage(
                self.wand,
                matte_color.wand(),
                width,
                height,
                inner_bevel,
                outer_bevel,
            )
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickfximage>
    ///
    /// MagickFxImage() evaluate expression for each pixel in the image.
    ///
    pub fn fx_image(&mut self, expression: &str) -> Option<MagickWand<'_>> {
        let expression = str_to_c_string(expression);
        let wand = unsafe { MagickFxImage(self.wand, expression.as_ptr()) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickfximagechannel>
    ///
    /// MagickFxImageChannel() evaluate expression for each pixel in the specified
    ///
    /// channel.
    ///
    pub fn fx_image_channel(
        &mut self,
        channel: ChannelType,
        expression: &str,
    ) -> Option<MagickWand<'_>> {
        let expression = str_to_c_string(expression);
        let wand = unsafe { MagickFxImageChannel(self.wand, channel.into(), expression.as_ptr()) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgammaimage>
    ///
    /// Use MagickGammaImage() to gamma-correct an image.  The same image viewed on
    ///
    /// different devices will have perceptual differences in the way the
    ///
    /// image's intensities are represented on the screen.  Specify individual
    ///
    /// gamma levels for the red, green, and blue channels, or adjust all three
    ///
    /// with the gamma parameter.  Values typically range from 0.8 to 2.3.
    ///
    /// You can also reduce the influence of a particular channel with a gamma
    ///
    /// value of 0.
    ///
    pub fn gamma_image(&mut self, gamma: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickGammaImage(self.wand, gamma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgammaimagechannel>
    ///
    /// Use MagickGammaImageChannel() to gamma-correct a particular image channel.
    ///
    /// The same image viewed on different devices will have perceptual differences
    ///
    /// in the way the image's intensities are represented on the screen.  Specify
    ///
    /// individual gamma levels for the red, green, and blue channels, or adjust all
    ///
    /// three with the gamma parameter.  Values typically range from 0.8 to 2.3.
    ///
    /// You can also reduce the influence of a particular channel with a gamma
    ///
    /// value of 0.
    ///
    pub fn gamma_image_channel(
        &mut self,
        channel: ChannelType,
        gamma: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickGammaImageChannel(self.wand, channel.into(), gamma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetconfigureinfo>
    ///
    /// MagickGetConfigureInfo() returns ImageMagick configure attributes such as
    ///
    /// NAME, VERSION, LIB_VERSION, COPYRIGHT, etc.
    ///
    pub fn get_configure_info(&mut self, name: &str) -> MagickCString {
        let name = str_to_c_string(name);
        unsafe { MagickCString::new(MagickGetConfigureInfo(self.wand, name.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetcopyright>
    ///
    /// MagickGetCopyright() returns the ImageMagick API copyright as a string.
    ///
    pub fn get_copyright() -> &'static CStr {
        unsafe { CStr::from_ptr(MagickGetCopyright()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetfilename>
    ///
    /// MagickGetFilename() returns the filename associated with an image sequence.
    ///
    pub fn get_filename(&self) -> MagickCString {
        unsafe { MagickCString::new(MagickGetFilename(self.wand)) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgethomeurl>
    ///
    /// MagickGetHomeURL() returns the ImageMagick home URL.
    ///
    pub fn get_home_url() -> &'static CStr {
        unsafe { CStr::from_ptr(MagickGetHomeURL()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimage>
    ///
    /// MagickGetImage() clones the image at the current image index.
    ///
    pub fn get_image(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickGetImage(self.wand) };
        Self::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageattribute>
    ///
    /// MagickGetImageAttribute returns an image attribute as a string
    ///
    pub fn get_image_attribute(&mut self, name: &str) -> MagickCString {
        let name = str_to_c_string(name);
        unsafe { MagickCString::new(MagickGetImageAttribute(self.wand, name.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagebackgroundcolor>
    ///
    /// MagickGetImageBackgroundColor() returns the image background color.
    ///
    pub fn get_image_background_color(&mut self) -> crate::Result<PixelWand> {
        let mut background_color = PixelWand::new();
        let status =
            unsafe { MagickGetImageBackgroundColor(self.wand, background_color.wand_mut()) };
        self.check_status(status)?;
        Ok(background_color)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageblueprimary>
    ///
    /// MagickGetImageBluePrimary() returns the chromaticy blue primary point for the
    ///
    /// image.
    ///
    /// # Return
    ///
    /// (x, y)
    ///
    pub fn get_image_blue_primary(&mut self) -> crate::Result<(c_double, c_double)> {
        let mut x = 0.;
        let mut y = 0.;
        let status = unsafe { MagickGetImageBluePrimary(self.wand, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagebordercolor>
    ///
    /// MagickGetImageBorderColor() returns the image border color.
    ///
    pub fn get_image_border_color(&mut self) -> crate::Result<PixelWand> {
        let mut border_color = PixelWand::new();
        let status = unsafe { MagickGetImageBorderColor(self.wand, border_color.wand_mut()) };
        self.check_status(status)?;
        Ok(border_color)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageboundingbox>
    ///
    /// MagickGetImageBoundingBox() obtains the crop bounding box required to
    ///
    /// remove a solid-color border from the image.  Color quantums which differ
    ///
    /// less than the fuzz setting are considered to be the same.  If a border is
    ///
    /// not detected, then the the original image dimensions are returned.  The
    ///
    /// crop bounding box estimation uses the same algorithm as MagickTrimImage().
    ///
    /// # Return
    ///
    /// (width, height, x, y)
    ///
    pub fn get_image_bounding_box(
        &mut self,
        fuzz: c_double,
    ) -> crate::Result<(c_ulong, c_ulong, c_long, c_long)> {
        let mut width = 0;
        let mut height = 0;
        let mut x = 0;
        let mut y = 0;
        let status = unsafe {
            MagickGetImageBoundingBox(self.wand, fuzz, &mut width, &mut height, &mut x, &mut y)
        };
        self.check_status(status)?;
        Ok((width, height, x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagechanneldepth>
    ///
    /// MagickGetImageChannelDepth() gets the depth for a particular image channel.
    ///
    pub fn get_image_channel_depth(&mut self, channel: ChannelType) -> u64 {
        unsafe { MagickGetImageChannelDepth(self.wand, channel.into()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagechannelextrema>
    ///
    /// MagickGetImageChannelExtrema() gets the extrema for one or more image
    ///
    /// channels.
    ///
    /// # Return
    ///
    /// (minima, maxima)
    ///
    pub fn get_image_channel_extrema(
        &mut self,
        channel: ChannelType,
    ) -> crate::Result<(c_ulong, c_ulong)> {
        let mut minima = 0;
        let mut maxima = 0;
        let status = unsafe {
            MagickGetImageChannelExtrema(self.wand, channel.into(), &mut minima, &mut maxima)
        };
        self.check_status(status)?;
        Ok((minima, maxima))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagechannelmean>
    ///
    /// MagickGetImageChannelMean() gets the mean and standard deviation of one or
    ///
    /// more image channels.
    ///
    /// # Return
    ///
    /// (mean, standard_deviation)
    ///
    pub fn get_image_channel_mean(
        &mut self,
        channel: ChannelType,
    ) -> crate::Result<(c_double, c_double)> {
        let mut mean = 0.;
        let mut standard_deviation = 0.;
        let status = unsafe {
            MagickGetImageChannelMean(
                self.wand,
                channel.into(),
                &mut mean,
                &mut standard_deviation,
            )
        };
        self.check_status(status)?;
        Ok((mean, standard_deviation))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagecolormapcolor>
    ///
    /// MagickGetImageColormapColor() returns the color of the specified colormap
    ///
    /// index.
    ///
    pub fn get_image_colormap_color(&mut self, index: c_ulong) -> crate::Result<PixelWand> {
        let mut color = PixelWand::new();
        let status = unsafe { MagickGetImageColormapColor(self.wand, index, color.wand_mut()) };
        self.check_status(status)?;
        Ok(color)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagecolors>
    ///
    /// MagickGetImageColors() gets the number of unique colors in the image.
    ///
    pub fn get_image_colors(&mut self) -> u64 {
        unsafe { MagickGetImageColors(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagecolorspace>
    ///
    /// MagickGetImageColorspace() gets the image colorspace.
    ///
    pub fn get_image_colorspace(&mut self) -> ColorspaceType {
        unsafe { MagickGetImageColorspace(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagecompose>
    ///
    /// MagickGetImageCompose() returns the composite operator associated with the
    ///
    /// image.
    ///
    pub fn get_image_compose(&mut self) -> CompositeOperator {
        unsafe { MagickGetImageCompose(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagecompression>
    ///
    /// MagickGetImageCompression() gets the image compression.
    ///
    pub fn get_image_compression(&mut self) -> CompressionType {
        unsafe { MagickGetImageCompression(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagedelay>
    ///
    /// MagickGetImageDelay() gets the image delay.
    ///
    pub fn get_image_delay(&mut self) -> c_ulong {
        unsafe { MagickGetImageDelay(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagedepth>
    ///
    /// MagickGetImageDepth() gets the image depth.
    ///
    pub fn get_image_depth(&mut self) -> c_ulong {
        unsafe { MagickGetImageDepth(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageextrema>
    ///
    /// MagickGetImageExtrema() gets the extrema for the image.
    ///
    /// # Return
    ///
    /// (min, max)
    ///
    pub fn get_image_extrema(&mut self) -> crate::Result<(c_ulong, c_ulong)> {
        let mut min = 0;
        let mut max = 0;
        let status = unsafe { MagickGetImageExtrema(self.wand, &mut min, &mut max) };
        self.check_status(status)?;
        Ok((min, max))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagedispose>
    ///
    /// MagickGetImageDispose() gets the image disposal method.
    ///
    pub fn get_image_dispose(&mut self) -> DisposeType {
        unsafe { MagickGetImageDispose(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagefilename>
    ///
    /// MagickGetImageFilename() returns the filename of a particular image in a
    ///
    /// sequence.
    ///
    pub fn get_image_filename(&mut self) -> MagickCString {
        unsafe { MagickCString::new(MagickGetImageFilename(self.wand)) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageformat>
    ///
    /// MagickGetImageFormat() returns the format of a particular image in a
    ///
    /// sequence.
    ///
    pub fn get_image_format(&mut self) -> MagickCString {
        unsafe { MagickCString::new(MagickGetImageFormat(self.wand)) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagefuzz>
    ///
    /// MagickGetImageFuzz() returns the color comparison fuzz factor. Colors
    ///
    /// closer than the fuzz factor are considered to be the same when comparing
    ///
    /// colors.  Note that some other functions such as MagickColorFloodfillImage()
    ///
    /// implicitly set this value.
    ///
    pub fn get_image_fuzz(&mut self) -> c_double {
        unsafe { MagickGetImageFuzz(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagegamma>
    ///
    /// MagickGetImageGamma() gets the image gamma.
    ///
    pub fn get_image_gamma(&mut self) -> c_double {
        unsafe { MagickGetImageGamma(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagegravity>
    ///
    /// MagickGetImageGravity() gets the image gravity.
    ///
    #[cfg(feature = "v1_3_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_22")))]
    pub fn get_image_gravity(&mut self) -> GravityType {
        unsafe { MagickGetImageGravity(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagegreenprimary>
    ///
    /// MagickGetImageGreenPrimary() returns the chromaticy green primary point.
    ///
    /// # Return
    ///
    /// (x, y)
    ///
    pub fn get_image_green_primary(&mut self) -> crate::Result<(c_double, c_double)> {
        let mut x = 0.;
        let mut y = 0.;
        let status = unsafe { MagickGetImageGreenPrimary(self.wand, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageheight>
    ///
    /// MagickGetImageHeight() returns the image height.
    ///
    pub fn get_image_height(&mut self) -> c_ulong {
        unsafe { MagickGetImageHeight(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagehistogram>
    ///
    /// MagickGetImageHistogram() returns the image histogram as an array of
    ///
    /// PixelWand wands.
    ///
    pub fn get_image_histogram(&mut self) -> Option<Vec<PixelWand>> {
        let mut number_colors = 0;
        let wands = unsafe { MagickGetImageHistogram(self.wand, &mut number_colors) };
        c_arr_to_vec(wands, number_colors as usize, |wand| {
            PixelWand::from_wand_expect(unsafe { *wand })
        })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageindex>
    ///
    /// MagickGetImageIndex() returns the index of the current image.
    ///
    pub fn get_image_index(&mut self) -> usize {
        unsafe { MagickGetImageIndex(self.wand) as usize }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageinterlacescheme>
    ///
    /// MagickGetImageInterlaceScheme() gets the image interlace scheme.
    ///
    pub fn get_image_interlace_scheme(&mut self) -> InterlaceType {
        unsafe { MagickGetImageInterlaceScheme(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageiterations>
    ///
    /// MagickGetImageIterations() gets the image iterations.
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn get_image_iterations(&mut self) -> c_ulong {
        unsafe { MagickGetImageIterations(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagemattecolor>
    ///
    /// MagickGetImageMatteColor() returns the image matte color.
    ///
    pub fn get_image_matte_color(&mut self) -> crate::Result<PixelWand> {
        let mut matte_color = PixelWand::new();
        let status = unsafe { MagickGetImageMatteColor(self.wand, matte_color.wand_mut()) };
        self.check_status(status)?;
        Ok(matte_color)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageorientation>
    ///
    /// MagickGetImageOrientation() gets the image orientation type. May be one of:
    ///
    /// UndefinedOrientation    Image orientation not specified or error.
    ///
    /// TopLeftOrientation      Left to right and Top to bottom.
    ///
    /// TopRightOrientation     Right to left  and Top to bottom.
    ///
    /// BottomRightOrientation  Right to left and Bottom to top.
    ///
    /// BottomLeftOrientation   Left to right and Bottom to top.
    ///
    /// LeftTopOrientation      Top to bottom and Left to right.
    ///
    /// RightTopOrientation     Top to bottom and Right to left.
    ///
    /// RightBottomOrientation  Bottom to top and Right to left.
    ///
    /// LeftBottomOrientation   Bottom to top and Left to right.
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn get_image_orientation(&mut self) -> OrientationType {
        unsafe { MagickGetImageOrientation(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagepage>
    ///
    /// MagickGetImagePage() retrieves the image page size and offset used when
    ///
    /// placing (e.g. compositing) the image.
    ///
    /// # Return
    ///
    /// (width, height, x, y)
    ///
    pub fn get_image_page(&mut self) -> crate::Result<(c_ulong, c_ulong, c_long, c_long)> {
        let mut width = 0;
        let mut height = 0;
        let mut x = 0;
        let mut y = 0;
        let status =
            unsafe { MagickGetImagePage(self.wand, &mut width, &mut height, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((width, height, x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagepixels>
    ///
    /// MagickGetImagePixels() extracts pixel data from an image and returns it to
    /// you.
    ///
    /// The method returns False on success otherwise True if an error is
    /// encountered.
    ///
    /// The data is returned as char, short int, int, long, float, or double
    /// in the order specified by map.
    ///
    /// Suppose you want to extract the first scanline of a 640x480 image as
    /// character data in red-green-blue order:
    ///
    /// ```
    /// use graphicsmagick::{
    ///     wand::{MagickWand, MagickWandExportCharPixel},
    ///     initialize,
    /// };
    ///
    /// initialize();
    ///
    /// MagickWand::new()
    ///     .get_image_pixels::<MagickWandExportCharPixel>(0, 0, 640, 1, "RGB");
    /// ```
    ///
    pub fn get_image_pixels<ExportType: MagickWandExportType>(
        &mut self,
        x_offset: c_long,
        y_offset: c_long,
        columns: c_ulong,
        rows: c_ulong,
        map: &str,
    ) -> crate::Result<Vec<ExportType::Target>> {
        let size: usize = (columns * rows).try_into().unwrap();
        let len = size * map.len();

        let mut pixels = Vec::with_capacity(len);
        let map = str_to_c_string(map);
        let storage = ExportType::STORAGE_TYPE;

        let status = unsafe {
            MagickGetImagePixels(
                self.wand,
                x_offset,
                y_offset,
                columns,
                rows,
                map.as_ptr(),
                storage,
                pixels.spare_capacity_mut().as_mut_ptr() as *mut c_uchar,
            )
        };
        self.check_status(status)?;

        // Safety:
        //
        // MagickGetImagePixels succeeds, so it should have written
        // `len` bytes into the vec.
        unsafe {
            pixels.set_len(len);
        }

        Ok(pixels)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageprofile>
    ///
    /// MagickGetImageProfile() returns the named image profile.
    ///
    pub fn get_image_profile(&mut self, name: &str) -> MagickCString {
        let mut length = 0;
        let name = str_to_c_string(name);
        unsafe {
            MagickCString::new(MagickGetImageProfile(self.wand, name.as_ptr(), &mut length).cast())
        }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageredprimary>
    ///
    /// MagickGetImageRedPrimary() returns the chromaticy red primary point.
    ///
    /// # Return
    ///
    /// (x, y)
    ///
    pub fn get_image_red_primary(&mut self) -> crate::Result<(c_double, c_double)> {
        let mut x = 0.;
        let mut y = 0.;
        let status = unsafe { MagickGetImageRedPrimary(self.wand, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagerenderingintent>
    ///
    /// MagickGetImageRenderingIntent() gets the image rendering intent.
    ///
    pub fn get_image_rendering_intent(&mut self) -> RenderingIntent {
        unsafe { MagickGetImageRenderingIntent(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageresolution>
    ///
    /// MagickGetImageResolution() gets the image X &amp; Y resolution.
    ///
    /// # Return
    ///
    /// (x, y)
    ///
    pub fn get_image_resolution(&mut self) -> crate::Result<(c_double, c_double)> {
        let mut x = 0.;
        let mut y = 0.;
        let status = unsafe { MagickGetImageResolution(self.wand, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagescene>
    ///
    /// MagickGetImageScene() gets the image scene.
    ///
    pub fn get_image_scene(&mut self) -> c_ulong {
        unsafe { MagickGetImageScene(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagesignature>
    ///
    /// MagickGetImageSignature() generates an SHA-256 message digest for the image
    ///
    /// pixel stream.
    ///
    pub fn get_image_signature(&mut self) -> MagickCString {
        unsafe { MagickCString::new(MagickGetImageSignature(self.wand)) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagesize>
    ///
    /// MagickGetImageSize() returns the image size.
    ///
    pub fn get_image_size(&mut self) -> c_long {
        unsafe { MagickGetImageSize(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagetype>
    ///
    /// MagickGetImageType() gets the image type.
    ///
    pub fn get_image_type(&mut self) -> ImageType {
        unsafe { MagickGetImageType(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagesavedtype>
    ///
    /// MagickGetImageSavedType() gets the image type that will be used when the
    ///
    /// image is saved. This may be different to the current image type, returned
    ///
    /// by MagickGetImageType().
    ///
    pub fn get_image_saved_type(&mut self) -> ImageType {
        unsafe { MagickGetImageSavedType(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimageunits>
    ///
    /// MagickGetImageUnits() gets the image units of resolution.
    ///
    pub fn get_image_units(&mut self) -> ResolutionType {
        unsafe { MagickGetImageUnits(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagevirtualpixelmethod>
    ///
    /// MagickGetImageVirtualPixelMethod() returns the virtual pixel method for the
    ///
    /// sepcified image.
    ///
    pub fn get_image_virtual_pixel_method(&mut self) -> VirtualPixelMethod {
        unsafe { MagickGetImageVirtualPixelMethod(self.wand) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagewhitepoint>
    ///
    /// MagickGetImageWhitePoint() returns the chromaticy white point.
    ///
    /// # Return
    ///
    /// (x, y)
    ///
    pub fn get_image_white_point(&mut self) -> crate::Result<(c_double, c_double)> {
        let mut x = 0.;
        let mut y = 0.;
        let status = unsafe { MagickGetImageWhitePoint(self.wand, &mut x, &mut y) };
        self.check_status(status)?;
        Ok((x, y))
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetimagewidth>
    ///
    /// MagickGetImageWidth() returns the image width.
    ///
    pub fn get_image_width(&mut self) -> c_ulong {
        unsafe { MagickGetImageWidth(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetnumberimages>
    ///
    /// MagickGetNumberOfImages() returns the number of images associated with a
    ///
    /// magick wand.
    ///
    pub fn get_number_images(&mut self) -> c_ulong {
        unsafe { MagickGetNumberImages(self.wand) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetpackagename>
    ///
    /// MagickGetPackageName() returns the ImageMagick package name.
    ///
    pub fn get_package_name() -> &'static CStr {
        unsafe { CStr::from_ptr(MagickGetPackageName()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetquantumdepth>
    ///
    /// MagickGetQuantumDepth() returns the ImageMagick quantum depth.
    ///
    pub fn get_quantum_depth() -> (c_ulong, &'static CStr) {
        let mut depth = 0;
        let c = unsafe { MagickGetQuantumDepth(&mut depth) };
        (depth, unsafe { CStr::from_ptr(c) })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetreleasedate>
    ///
    /// MagickGetReleaseDate() returns the ImageMagick release date.
    ///
    pub fn get_release_date() -> &'static CStr {
        unsafe { CStr::from_ptr(MagickGetReleaseDate()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetresourcelimit>
    ///
    /// MagickGetResourceLimit() returns the the specified resource in megabytes.
    ///
    pub fn get_resource_limit(r#type: ResourceType) -> u64 {
        unsafe { MagickGetResourceLimit(r#type.into()) }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetsamplingfactors>
    ///
    /// MagickGetSamplingFactors() gets the horizontal and vertical sampling factor.
    ///
    pub fn get_sampling_factors(&mut self) -> Option<Vec<c_double>> {
        let mut number_factors = 0;
        let a = unsafe { MagickGetSamplingFactors(self.wand, &mut number_factors) };
        c_arr_to_vec(a, number_factors as usize, |p| unsafe { *p })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetsize>
    ///
    /// MagickGetSize() returns the size associated with the magick wand.
    ///
    /// # Return
    ///
    /// (columns, rows)
    ///
    pub fn get_size(&self) -> (c_ulong, c_ulong) {
        let mut columns = 0;
        let mut rows = 0;
        unsafe { MagickGetSize(self.wand, &mut columns, &mut rows) };
        (columns, rows)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickgetversion>
    ///
    /// MagickGetVersion() returns the ImageMagick API version as a string and
    ///
    /// as a number.
    ///
    /// # Return
    ///
    /// (MagickLibVersion, MagickVersion)
    ///
    pub fn get_version() -> (c_ulong, &'static CStr) {
        let mut version = 0;
        let c = unsafe { MagickGetVersion(&mut version) };
        (version, unsafe { CStr::from_ptr(c) })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickhaldclutimage>
    ///
    /// The MagickHaldClutImage() method apply a color lookup table (Hald CLUT) to
    ///
    /// the image.  The fundamental principle of the Hald CLUT algorithm is that
    ///
    /// application of an identity CLUT causes no change to the input image,
    ///
    /// but an identity CLUT image which has had its colors transformed in
    ///
    /// some way (e.g. in Adobe Photoshop) may be used to implement an identical
    ///
    /// transform on any other image.
    ///
    /// The minimum CLUT level is 2, and the maximum depends on available memory
    ///
    /// (largest successfully tested is 24).  A CLUT image is required to have equal
    ///
    /// width and height. A CLUT of level 8 is an image of dimension 512x512, a CLUT
    ///
    /// of level 16 is an image of dimension 4096x4096.  Interpolation is used so
    ///
    /// extremely large CLUT images are not required.
    ///
    /// GraphicsMagick provides an 'identity' coder which may be used to generate
    ///
    /// identity HLUTs.  For example, reading from "identity:8" creates an identity
    ///
    /// CLUT of order 8.
    ///
    /// The Hald CLUT algorithm has been developed by Eskil Steenberg as described
    ///
    /// at <http://www.quelsolaar.com/technology/clut.html>, and was adapted for
    ///
    /// GraphicsMagick by Clément Follet with support from Cédric Lejeune of
    ///
    /// Workflowers.
    ///
    pub fn hald_clut_image(&mut self, clut_wand: &MagickWand<'_>) -> crate::Result<&mut Self> {
        let status = unsafe { MagickHaldClutImage(self.wand, clut_wand.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickhascolormap>
    ///
    /// MagickHasColormap() returns True if the check was successful with the
    ///
    /// colormap parameter set to a boolean value indicating whether the current
    ///
    /// wand image uses a color map or not. Returns False if there are no wand
    ///
    /// images available.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn has_colormap(&mut self) -> crate::Result<bool> {
        let mut colormap = 0;
        let status = unsafe { MagickHasColormap(self.wand, &mut colormap) };
        self.check_status(status)?;
        Ok(colormap != 0)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickhasnextimage>
    ///
    /// MagickHasNextImage() returns True if the wand has more images when
    ///
    /// traversing the list in the forward direction
    ///
    pub fn has_next_image(&mut self) -> bool {
        (unsafe { MagickHasNextImage(self.wand) }) == 1
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickhaspreviousimage>
    ///
    /// MagickHasPreviousImage() returns True if the wand has more images when
    ///
    /// traversing the list in the reverse direction
    ///
    pub fn has_previous_image(&mut self) -> bool {
        (unsafe { MagickHasPreviousImage(self.wand) }) == 0
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickimplodeimage>
    ///
    /// MagickImplodeImage() creates a new image that is a copy of an existing
    ///
    /// one with the image pixels "implode" by the specified percentage.  It
    ///
    /// allocates the memory necessary for the new Image structure and returns a
    ///
    /// pointer to the new image.
    ///
    pub fn implode_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickImplodeImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickisgrayimage>
    ///
    /// MagickIsGrayImage() returns True if the check was successful with the
    ///
    /// grayimage parameter set to a boolean value indicating whether the current
    ///
    /// wand image is a gray-scale image or not. Returns False if there was
    ///
    /// an error.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn is_gray_image(&mut self) -> crate::Result<bool> {
        let mut gray_image = 0;
        let status = unsafe { MagickIsGrayImage(self.wand, &mut gray_image) };
        self.check_status(status)?;
        Ok(gray_image == 0)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickismonochromeimage>
    ///
    /// MagickIsMonochromeImage() returns True if the check was successful with the
    ///
    /// monochrome parameter set to a boolean value indicating whether the current
    ///
    /// wand image is a monochrome image or not. Returns False if there was
    ///
    /// an error.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn is_monochrome_image(&mut self) -> crate::Result<bool> {
        let mut monochrome = 0;
        let status = unsafe { MagickIsMonochromeImage(self.wand, &mut monochrome) };
        self.check_status(status)?;
        Ok(monochrome == 0)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickisopaqueimage>
    ///
    /// MagickIsOpaqueImage() returns True if the check was successful with the
    ///
    /// opaque parameter set to a boolean value indicating whether the current
    ///
    /// wand image is entirely opaque or not. Returns False if there was
    ///
    /// an error.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn is_opaque_image(&mut self) -> crate::Result<bool> {
        let mut opaque = 0;
        let status = unsafe { MagickIsOpaqueImage(self.wand, &mut opaque) };
        self.check_status(status)?;
        Ok(opaque == 0)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickispaletteimage>
    ///
    /// MagickIsPaletteImage() returns True if the check was successful with the
    ///
    /// palette parameter set to a boolean value indicating whether the current
    ///
    /// wand image is an image with 256 unique colors or less. Returns False if
    ///
    /// there was an error. Note that a palette image does not necessarily use a
    ///
    /// colormap. See MagickHasColormap() if needing to determine whether a
    ///
    /// colormap is in use.
    ///
    #[cfg(feature = "v1_3_29")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_29")))]
    pub fn is_palette_image(&mut self) -> crate::Result<bool> {
        let mut palette = 0;
        let status = unsafe { MagickIsPaletteImage(self.wand, &mut palette) };
        self.check_status(status)?;
        Ok(palette == 0)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicklabelimage>
    ///
    /// MagickLabelImage() adds a label to your image.
    ///
    pub fn label_image(&mut self, label: &str) -> crate::Result<&mut Self> {
        let label = str_to_c_string(label);
        let status = unsafe { MagickLabelImage(self.wand, label.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicklevelimage>
    ///
    /// MagickLevelImage() adjusts the levels of an image by scaling the colors
    ///
    /// falling between specified white and black points to the full available
    ///
    /// quantum range. The parameters provided represent the black, mid, and white
    ///
    /// points. The black point specifies the darkest color in the image. Colors
    ///
    /// darker than the black point are set to zero. Mid point specifies a gamma
    ///
    /// correction to apply to the image.  White point specifies the lightest color
    ///
    /// in the image. Colors brighter than the white point are set to the maximum
    ///
    /// quantum value.
    ///
    pub fn level_image(
        &mut self,
        black_point: c_double,
        gamma: c_double,
        white_point: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickLevelImage(self.wand, black_point, gamma, white_point) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicklevelimagechannel>
    ///
    /// MagickLevelImageChannel() adjusts the levels of the specified channel of
    ///
    /// the reference image by scaling the colors falling between specified white
    ///
    /// and black points to the full available quantum range. The parameters
    ///
    /// provided represent the black, mid, and white points. The black point
    ///
    /// specifies the darkest color in the image. Colors darker than the black
    ///
    /// point are set to zero.  Mid point specifies a gamma correction to apply
    ///
    /// to the image.  White point specifies the lightest color in the image.
    ///
    /// Colors brighter than the white point are set to the maximum quantum value.
    ///
    pub fn level_image_channel(
        &mut self,
        channel: ChannelType,
        black_point: c_double,
        gamma: c_double,
        white_point: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickLevelImageChannel(self.wand, channel.into(), black_point, gamma, white_point)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmagnifyimage>
    ///
    /// MagickMagnifyImage() is a convenience method that scales an image
    ///
    /// proportionally to twice its original size.
    ///
    pub fn magnify_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickMagnifyImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmapimage>
    ///
    /// MagickMapImage() replaces the colors of an image with the closest color
    ///
    /// from a reference image.
    ///
    pub fn map_image(
        &mut self,
        map_wand: &MagickWand<'_>,
        dither: c_uint,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickMapImage(self.wand, map_wand.wand(), dither) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmattefloodfillimage>
    ///
    /// MagickMatteFloodfillImage() changes the transparency value of any pixel that
    ///
    /// matches target and is an immediate neighbor.  If the method
    ///
    /// FillToBorderMethod is specified, the transparency value is changed for any
    ///
    /// neighbor pixel that does not match the bordercolor member of image.
    ///
    pub fn matte_floodfill_image(
        &mut self,
        opacity: Quantum,
        fuzz: c_double,
        border_color: &PixelWand,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickMatteFloodfillImage(self.wand, opacity, fuzz, border_color.wand(), x, y)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmedianfilterimage>
    ///
    /// MagickMedianFilterImage() applies a digital filter that improves the quality
    ///
    /// of a noisy image.  Each pixel is replaced by the median in a set of
    ///
    /// neighboring pixels as defined by radius.
    ///
    pub fn median_filter_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickMedianFilterImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickminifyimage>
    ///
    /// MagickMinifyImage() is a convenience method that scales an image
    ///
    /// proportionally to one-half its original size
    ///
    pub fn minify_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickMinifyImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmodulateimage>
    ///
    /// MagickModulateImage() lets you control the brightness, saturation, and hue
    ///
    /// of an image.
    ///
    pub fn modulate_image(
        &mut self,
        brightness: c_double,
        saturation: c_double,
        hue: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickModulateImage(self.wand, brightness, saturation, hue) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmontageimage>
    ///
    /// Use MagickMontageImage() to create a composite image by combining several
    ///
    /// separate images. The images are tiled on the composite image with the name
    ///
    /// of the image optionally appearing just below the individual tile.
    ///
    pub fn montage_image(
        &mut self,
        drawing_wand: &DrawingWand,
        tile_geometry: &str,
        thumbnail_geometry: &str,
        mode: MontageMode,
        frame: &str,
    ) -> Option<MagickWand<'_>> {
        let tile_geometry = str_to_c_string(tile_geometry);
        let thumbnail_geometry = str_to_c_string(thumbnail_geometry);
        let frame = str_to_c_string(frame);
        let wand = unsafe {
            MagickMontageImage(
                self.wand,
                drawing_wand.wand(),
                tile_geometry.as_ptr(),
                thumbnail_geometry.as_ptr(),
                mode.into(),
                frame.as_ptr(),
            )
        };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmorphimages>
    ///
    /// MagickMorphImages() method morphs a set of images.  Both the image pixels
    ///
    /// and size are linearly interpolated to give the appearance of a
    ///
    /// meta-morphosis from one image to the next.
    ///
    pub fn morph_images(&mut self, number_frames: c_ulong) -> MagickWand<'_> {
        let _status = unsafe { MagickMorphImages(self.wand, number_frames) };
        todo!()
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmosaicimages>
    ///
    /// MagickMosaicImages() inlays an image sequence to form a single coherent
    ///
    /// picture.  It returns a wand with each image in the sequence composited at
    ///
    /// the location defined by the page offset of the image.
    ///
    pub fn mosaic_images(&mut self) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickMosaicImages(self.wand) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickmotionblurimage>
    ///
    /// MagickMotionBlurImage() simulates motion blur.  We convolve the image with a
    ///
    /// Gaussian operator of the given radius and standard deviation (sigma).
    ///
    /// For reasonable results, radius should be larger than sigma.  Use a
    ///
    /// radius of 0 and MotionBlurImage() selects a suitable radius for you.
    ///
    /// Angle gives the angle of the blurring motion.
    ///
    pub fn motion_blur_image(
        &mut self,
        radius: c_double,
        sigma: c_double,
        angle: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickMotionBlurImage(self.wand, radius, sigma, angle) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicknegateimage>
    ///
    /// MagickNegateImage() negates the colors in the reference image.  The
    ///
    /// Grayscale option means that only grayscale values within the image are
    ///
    /// negated.
    ///
    /// You can also reduce the influence of a particular channel with a gamma
    ///
    /// value of 0.
    ///
    pub fn negate_image(&mut self, gray: c_uint) -> crate::Result<&mut Self> {
        let status = unsafe { MagickNegateImage(self.wand, gray) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicknegateimagechannel>
    ///
    /// MagickNegateImageChannel() negates the colors in the specified channel of the
    ///
    /// reference image.  The Grayscale option means that only grayscale values
    ///
    /// within the image are negated.  Note that the Grayscale option has no
    ///
    /// effect for GraphicsMagick.
    ///
    /// You can also reduce the influence of a particular channel with a gamma
    ///
    /// value of 0.
    ///
    pub fn negate_image_channel(
        &mut self,
        channel: ChannelType,
        gray: c_uint,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickNegateImageChannel(self.wand, channel.into(), gray) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicknextimage>
    ///
    /// MagickNextImage() associates the next image in the image list with a magick
    ///
    /// wand.  True is returned if the Wand iterated to a next image, or False is
    ///
    /// returned if the wand did not iterate to a next image.
    ///
    pub fn next_image(&mut self) -> bool {
        (unsafe { MagickNextImage(self.wand) }) == 1
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicknormalizeimage>
    ///
    /// MagickNormalizeImage() enhances the contrast of a color image by adjusting
    ///
    /// the pixels color to span the entire range of colors available
    ///
    /// You can also reduce the influence of a particular channel with a gamma
    ///
    /// value of 0.
    ///
    pub fn normalize_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickNormalizeImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickoilpaintimage>
    ///
    /// MagickOilPaintImage() applies a special effect filter that simulates an oil
    ///
    /// painting.  Each pixel is replaced by the most frequent color occurring
    ///
    /// in a circular region defined by radius.
    ///
    pub fn oil_paint_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickOilPaintImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickopaqueimage>
    ///
    /// MagickOpaqueImage() changes any pixel that matches color with the color
    ///
    /// defined by fill.
    ///
    pub fn opaque_image(
        &mut self,
        target: &PixelWand,
        fill: &PixelWand,
        fuzz: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickOpaqueImage(self.wand, target.wand(), fill.wand(), fuzz) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickpingimage>
    ///
    /// MagickPingImage() is like MagickReadImage() except the only valid
    ///
    /// information returned is the image width, height, size, and format.  It
    ///
    /// is designed to efficiently obtain this information from a file without
    ///
    /// reading the entire image sequence into memory.
    ///
    pub fn ping_image(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickPingImage(self.wand, filename.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickpreviewimages>
    ///
    /// MagickPreviewImages() tiles 9 thumbnails of the specified image with an
    ///
    /// image processing operation applied at varying strengths.  This is helpful
    ///
    /// to quickly pin-point an appropriate parameter for an image processing
    ///
    /// operation.
    ///
    pub fn preview_images(&mut self, preview: PreviewType) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickPreviewImages(self.wand, preview.into()) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickpreviousimage>
    ///
    /// MagickPreviousImage() selects the previous image associated with a magick
    ///
    /// wand.
    ///
    pub fn previous_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickPreviousImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickprofileimage>
    ///
    /// Use MagickProfileImage() to add or remove a ICC, IPTC, or generic profile
    ///
    /// from an image.  If the profile is NULL, it is removed from the image
    ///
    /// otherwise added.  Use a name of '*' and a profile of NULL to remove all
    ///
    /// profiles from the image.
    ///
    pub fn profile_image(
        &mut self,
        name: &str,
        profile: &str,
        length: size_t,
    ) -> crate::Result<&mut Self> {
        let name = str_to_c_string(name);
        let profile = str_to_c_string(profile);
        let status = unsafe {
            MagickProfileImage(self.wand, name.as_ptr(), profile.as_ptr().cast(), length)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickquantizeimage>
    ///
    /// MagickQuantizeImage() analyzes the colors within a reference image and
    ///
    /// chooses a fixed number of colors to represent the image.  The goal of the
    ///
    /// algorithm is to minimize the color difference between the input and output
    ///
    /// image while minimizing the processing time.
    ///
    /// # Panics
    ///
    /// Panic if colorspace is UndefinedColorspace.
    ///
    pub fn quantize_image(
        &mut self,
        number_colors: c_ulong,
        colorspace: ColorspaceType,
        tree_depth: c_ulong,
        dither: c_uint,
        measure_error: c_uint,
    ) -> crate::Result<&mut Self> {
        assert_ne!(
            colorspace,
            ColorspaceType::UndefinedColorspace,
            "colorspace cant be undefined"
        );
        let status = unsafe {
            MagickQuantizeImage(
                self.wand,
                number_colors,
                colorspace.into(),
                tree_depth,
                dither,
                measure_error,
            )
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickquantizeimages>
    ///
    /// MagickQuantizeImage() analyzes the colors within a sequence of images and
    ///
    /// chooses a fixed number of colors to represent the image.  The goal of the
    ///
    /// algorithm is to minimize the color difference between the input and output
    ///
    /// image while minimizing the processing time.
    ///
    /// # Panics
    ///
    /// Panic if colorspace is UndefinedColorspace.
    ///
    pub fn quantize_images(
        &mut self,
        number_colors: c_ulong,
        colorspace: ColorspaceType,
        tree_depth: c_ulong,
        dither: c_uint,
        measure_error: c_uint,
    ) -> crate::Result<&mut Self> {
        assert_ne!(
            colorspace,
            ColorspaceType::UndefinedColorspace,
            "colorspace cant be undefined"
        );
        let status = unsafe {
            MagickQuantizeImages(
                self.wand,
                number_colors,
                colorspace.into(),
                tree_depth,
                dither,
                measure_error,
            )
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickqueryfontmetrics>
    ///
    /// MagickQueryFontMetrics() returns a 7 element array representing the
    ///
    /// following font metrics:Element Description
    ///
    /// 0 character width
    ///
    /// 1 character height
    ///
    /// 2 ascender
    ///
    /// 3 descender
    ///
    /// 4 text width
    ///
    /// 5 text height
    ///
    /// 6 maximum horizontal advance
    ///
    pub fn query_font_metrics(
        &mut self,
        drawing_wand: &DrawingWand,
        text: &str,
    ) -> crate::Result<[f64; 7]> {
        let text = str_to_c_string(text);
        let ds = unsafe { MagickQueryFontMetrics(self.wand, drawing_wand.wand(), text.as_ptr()) };
        if ds.is_null() {
            return Err(unsafe { self.get_error() });
        }
        let arr: [f64; 7] = unsafe {
            [
                *(ds.add(0)),
                *(ds.add(1)),
                *(ds.add(2)),
                *(ds.add(3)),
                *(ds.add(4)),
                *(ds.add(5)),
                *(ds.add(6)),
            ]
        };
        unsafe {
            MagickFree(ds.cast());
        }
        Ok(arr)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickqueryfonts>
    ///
    /// MagickQueryFonts() returns any font that match the specified pattern.
    ///
    pub fn query_fonts(pattern: &str) -> Option<Vec<MagickCString>> {
        let pattern = str_to_c_string(pattern);
        let mut number_fonts = 0;
        let a = unsafe { MagickQueryFonts(pattern.as_ptr(), &mut number_fonts) };
        c_arr_to_vec(a, number_fonts as usize, |s| unsafe {
            MagickCString::new(*s)
        })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickqueryformats>
    ///
    /// MagickQueryFormats() returns any image formats that match the specified
    ///
    /// pattern.
    ///
    pub fn query_formats(pattern: &str) -> Option<Vec<MagickCString>> {
        let pattern = str_to_c_string(pattern);
        let mut number_formats = 0;
        let a = unsafe { MagickQueryFormats(pattern.as_ptr(), &mut number_formats) };
        c_arr_to_vec(a, number_formats as usize, |s| unsafe {
            MagickCString::new(*s)
        })
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickradialblurimage>
    ///
    /// MagickRadialBlurImage() radial blurs an image.
    ///
    pub fn radial_blur_image(&mut self, angle: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRadialBlurImage(self.wand, angle) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickraiseimage>
    ///
    /// MagickRaiseImage() creates a simulated three-dimensional button-like effect
    ///
    /// by lightening and darkening the edges of the image.  Members width and
    ///
    /// height of raise_info define the width of the vertical and horizontal
    ///
    /// edge of the effect.
    ///
    pub fn raise_image(
        &mut self,
        width: c_ulong,
        height: c_ulong,
        x: c_long,
        y: c_long,
        raise_flag: c_uint,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRaiseImage(self.wand, width, height, x, y, raise_flag) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickreadimage>
    ///
    /// MagickReadImage() reads an image or image sequence.
    ///
    pub fn read_image(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickReadImage(self.wand, filename.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickreadimageblob>
    ///
    /// MagickReadImageBlob() reads an image or image sequence from a blob.
    ///
    pub fn read_image_blob(&mut self, blob: &'a [u8]) -> crate::Result<&mut Self> {
        self.blob = Some(blob);
        let length = blob.len() as size_t;
        let blob = blob.as_ptr();
        let status = unsafe { MagickReadImageBlob(self.wand, blob, length) };
        self.check_status(status)
    }

    // Not Need
    //    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickreadimagefile>
    //    ///
    //    /// MagickReadImageFile() reads an image or image sequence from an open file
    //    ///
    //    /// descriptor.
    //    ///
    //    pub fn read_image_file(&mut self, file: &mut File) -> crate::Result<&mut Self> {
    //        // let status = unsafe { MagickReadImageFile(self.wand, file) };
    //    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickreducenoiseimage>
    ///
    /// MagickReduceNoiseImage() smooths the contours of an image while still
    ///
    /// preserving edge information.  The algorithm works by replacing each pixel
    ///
    /// with its neighbor closest in value.  A neighbor is defined by radius.  Use
    ///
    /// a radius of 0 and ReduceNoise() selects a suitable radius for you.
    ///
    pub fn reduce_noise_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickReduceNoiseImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickremoveimage>
    ///
    /// MagickRemoveImage() removes an image from the image list.
    ///
    pub fn remove_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRemoveImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickremoveimageoption>
    ///
    /// MagickRemoveImageOption() removes an image format-specific option from the
    ///
    /// the image (.e.g MagickRemoveImageOption(wand,"jpeg","preserve-settings").
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn remove_image_option(&mut self, format: &str, key: &str) -> crate::Result<&mut Self> {
        let format = str_to_c_string(format);
        let key = str_to_c_string(key);
        let status = unsafe { MagickRemoveImageOption(self.wand, format.as_ptr(), key.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickremoveimageprofile>
    ///
    /// MagickRemoveImageProfile() removes the named image profile and returns it.
    ///
    pub fn remove_image_profile(&mut self, name: &str) -> MagickCString {
        let name = str_to_c_string(name);
        let mut length = 0;
        unsafe {
            MagickCString::new(
                MagickRemoveImageProfile(self.wand, name.as_ptr(), &mut length).cast(),
            )
        }
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickresetiterator>
    ///
    /// MagickReset() resets the wand iterator.  Use it in conjunction
    ///
    /// with MagickNextImage() to iterate over all the images in a wand
    ///
    /// container.
    ///
    pub fn reset_iterator(&mut self) {
        unsafe { MagickResetIterator(self.wand) };
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickresampleimage>
    ///
    /// MagickResampleImage() resample image to desired resolution.
    ///
    /// Bessel   Blackman   Box
    ///
    /// Catrom   Cubic      Gaussian
    ///
    /// Hanning  Hermite    Lanczos
    ///
    /// Mitchell Point      Quandratic
    ///
    /// Sinc     Triangle
    ///
    /// Most of the filters are FIR (finite impulse response), however, Bessel,
    ///
    /// Gaussian, and Sinc are IIR (infinite impulse response).  Bessel and Sinc
    ///
    /// are windowed (brought down to zero) with the Blackman filter.
    ///
    pub fn resample_image(
        &mut self,
        x_resolution: c_double,
        y_resolution: c_double,
        filter: FilterTypes,
        blur: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickResampleImage(self.wand, x_resolution, y_resolution, filter.into(), blur)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickresizeimage>
    ///
    /// MagickResizeImage() scales an image to the desired dimensions with one of
    ///
    /// these filters:
    ///
    /// Bessel   Blackman   Box
    ///
    /// Catrom   Cubic      Gaussian
    ///
    /// Hanning  Hermite    Lanczos
    ///
    /// Mitchell Point      Quandratic
    ///
    /// Sinc     Triangle
    ///
    /// Most of the filters are FIR (finite impulse response), however, Bessel,
    ///
    /// Gaussian, and Sinc are IIR (infinite impulse response).  Bessel and Sinc
    ///
    /// are windowed (brought down to zero) with the Blackman filter.
    ///
    pub fn resize_image(
        &mut self,
        columns: c_ulong,
        rows: c_ulong,
        filter: FilterTypes,
        blur: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickResizeImage(self.wand, columns, rows, filter.into(), blur) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickrollimage>
    ///
    /// MagickRollImage() offsets an image as defined by x_offset and y_offset.
    ///
    pub fn roll_image(&mut self, x_offset: c_long, y_offset: c_long) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRollImage(self.wand, x_offset, y_offset) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickrotateimage>
    ///
    /// MagickRotateImage() rotates an image the specified number of degrees. Empty
    ///
    /// triangles left over from rotating the image are filled with the
    ///
    /// background color.
    ///
    pub fn rotate_image(
        &mut self,
        background: &PixelWand,
        degrees: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickRotateImage(self.wand, background.wand(), degrees) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksampleimage>
    ///
    /// MagickSampleImage() scales an image to the desired dimensions with pixel
    ///
    /// sampling.  Unlike other scaling methods, this method does not introduce
    ///
    /// any additional color into the scaled image.
    ///
    pub fn sample_image(&mut self, columns: c_ulong, rows: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSampleImage(self.wand, columns, rows) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickscaleimage>
    ///
    /// MagickScaleImage() scales the size of an image to the given dimensions.
    ///
    pub fn scale_image(&mut self, columns: c_ulong, rows: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickScaleImage(self.wand, columns, rows) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickseparateimagechannel>
    ///
    /// MagickChannelImage() separates a channel from the image and returns a
    ///
    /// grayscale image.  A channel is a particular color component of each pixel
    ///
    /// in the image.
    ///
    pub fn separate_image_channel(&mut self, channel: ChannelType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSeparateImageChannel(self.wand, channel.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetcompressionquality>
    ///
    /// MagickSetCompressionQuality() sets the image quality factor, which
    ///
    /// determines compression options when saving the file.
    ///
    /// For the JPEG and MPEG image formats, quality is 0 (lowest image
    ///
    /// quality and highest compression) to 100 (best quality but least
    ///
    /// effective compression).  The default quality is 75.  Use the
    ///
    /// -sampling-factor option to specify the factors for chroma
    ///
    /// downsampling.  To use the same quality value as that found by the
    ///
    /// JPEG decoder, use the -define jpeg:preserve-settings flag.
    ///
    /// For the MIFF image format, and the TIFF format while using ZIP
    ///
    /// compression, quality/10 is the zlib compres- sion level, which is 0
    ///
    /// (worst but fastest compression) to 9 (best but slowest). It has no
    ///
    /// effect on the image appearance, since the compression is always
    ///
    /// lossless.
    ///
    /// For the JPEG-2000 image format, quality is mapped using a non-linear
    ///
    /// equation to the compression ratio required by the Jasper library.
    ///
    /// This non-linear equation is intended to loosely approximate the
    ///
    /// quality provided by the JPEG v1 format.  The default quality value 75
    ///
    /// results in a request for 16:1 compression. The quality value 100
    ///
    /// results in a request for non-lossy compres- sion.
    ///
    /// For the MNG and PNG image formats, the quality value sets the zlib
    ///
    /// compression level (quality / 10) and filter-type (quality % 10).
    ///
    /// Compression levels range from 0 (fastest compression) to 100 (best
    ///
    /// but slowest).  For compression level 0, the Huffman-only strategy is
    ///
    /// used, which is fastest but not necessarily the worst compression.  If
    ///
    /// filter-type is 4 or less, the specified filter-type is used for all
    ///
    /// scanlines:
    ///
    /// none
    ///
    /// sub
    ///
    /// up
    ///
    /// average
    ///
    /// Paeth
    ///
    /// If filter-type is 5, adaptive filtering is used when quality is
    ///
    /// greater than 50 and the image does not have a color map, otherwise no
    ///
    /// filtering is used.
    ///
    /// If filter-type is 6, adaptive filtering with minimum-
    ///
    /// sum-of-absolute-values is used.
    ///
    /// Only if the output is MNG, if filter-type is 7, the LOCO color
    ///
    /// transformation and adaptive filtering with
    ///
    /// minimum-sum-of-absolute-values are used.
    ///
    /// The default is quality is 75, which means nearly the best compression
    ///
    /// with adaptive filtering.  The quality setting has no effect on the
    ///
    /// appearance of PNG and MNG images, since the compression is always
    ///
    /// lossless.
    ///
    /// For further information, see the PNG specification.
    ///
    /// When writing a JNG image with transparency, two quality values are
    ///
    /// required, one for the main image and one for the grayscale image that
    ///
    /// conveys the opacity channel.  These are written as a single integer
    ///
    /// equal to the main image quality plus 1000 times the opacity quality.
    ///
    /// For example, if you want to use quality 75 for the main image and
    ///
    /// quality 90 to compress the opacity data, use -quality 90075.
    ///
    /// For the PNM family of formats (PNM, PGM, and PPM) specify a quality
    ///
    /// factor of zero in order to obtain the ASCII variant of the
    ///
    /// format. Note that -compress none used to be used to trigger ASCII
    ///
    /// output but provided the opposite result of what was expected as
    ///
    /// compared with other formats.
    ///
    pub fn set_compression_quality(&mut self, quality: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetCompressionQuality(self.wand, quality) };
        self.check_status(status)
    }

    // TODO Detect version to implement.
    //     /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetdepth>
    //     ///
    //     /// MagickSetDepth() sets the sample depth to be used when reading from a
    //     ///
    //     /// raw image or a format which requires that the depth be specified in
    //     ///
    //     /// advance by the user.
    //     ///
    //     pub fn set_depth(&mut self,  depth: size_t) -> crate::Result<&mut Self> {
    //         let status = unsafe { MagickSetDepth(self.wand,  depth) };
    //         self.check_status(status)
    //     }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetfilename>
    ///
    /// MagickSetFilename() sets the filename before you read or write an image file.
    ///
    pub fn set_filename(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickSetFilename(self.wand, filename.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetformat>
    ///
    /// MagickSetFormat() sets the file or blob format (e.g. "BMP") to be used
    ///
    /// when a file or blob is read.  Usually this is not necessary because
    ///
    /// GraphicsMagick is able to auto-detect the format based on the file
    ///
    /// header (or the file extension), but some formats do not use a unique
    ///
    /// header or the selection may be ambigious. Use MagickSetImageFormat()
    ///
    /// to set the format to be used when a file or blob is to be written.
    ///
    pub fn set_format(&mut self, format: &str) -> crate::Result<&mut Self> {
        let format = str_to_c_string(format);
        let status = unsafe { MagickSetFormat(self.wand, format.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimage>
    ///
    /// MagickSetImage() replaces the last image returned by MagickSetImageIndex(),
    ///
    /// MagickNextImage(), MagickPreviousImage() with the images from the specified
    ///
    /// wand.
    ///
    pub fn set_image(&mut self, set_wand: &MagickWand<'_>) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImage(self.wand, set_wand.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageattribute>
    ///
    /// MagickSetImageAttribute sets an image attribute
    ///
    pub fn set_image_attribute(&mut self, name: &str, value: &str) -> crate::Result<&mut Self> {
        let name = str_to_c_string(name);
        let value = str_to_c_string(value);
        let status = unsafe { MagickSetImageAttribute(self.wand, name.as_ptr(), value.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagebackgroundcolor>
    ///
    /// MagickSetImageBackgroundColor() sets the image background color.
    ///
    pub fn set_image_background_color(
        &mut self,
        background: &PixelWand,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageBackgroundColor(self.wand, background.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageblueprimary>
    ///
    /// MagickSetImageBluePrimary() sets the image chromaticity blue primary point.
    ///
    pub fn set_image_blue_primary(&mut self, x: c_double, y: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageBluePrimary(self.wand, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagebordercolor>
    ///
    /// MagickSetImageBorderColor() sets the image border color.
    ///
    pub fn set_image_border_color(&mut self, border: &PixelWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageBorderColor(self.wand, border.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagecolormapcolor>
    ///
    /// MagickSetImageColormapColor() sets the color of the specified colormap
    ///
    /// index.
    ///
    pub fn set_image_colormap_color(
        &mut self,
        index: c_ulong,
        color: &PixelWand,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageColormapColor(self.wand, index, color.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagecolorspace>
    ///
    /// MagickSetImageColorspace() sets the image colorspace.
    ///
    /// # Panics
    ///
    /// Panic if colorspace is UndefinedColorspace
    ///
    pub fn set_image_colorspace(&mut self, colorspace: ColorspaceType) -> crate::Result<&mut Self> {
        assert_ne!(
            colorspace,
            ColorspaceType::UndefinedColorspace,
            "colorspace cant be undefined"
        );
        let status = unsafe { MagickSetImageColorspace(self.wand, colorspace.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagecompose>
    ///
    /// MagickSetImageCompose() sets the image composite operator, useful for
    ///
    /// specifying how to composite the image thumbnail when using the
    ///
    /// MagickMontageImage() method.
    ///
    pub fn set_image_compose(&mut self, compose: CompositeOperator) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageCompose(self.wand, compose.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagecompression>
    ///
    /// MagickSetImageCompression() sets the image compression.
    ///
    pub fn set_image_compression(
        &mut self,
        compression: CompressionType,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageCompression(self.wand, compression.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagedelay>
    ///
    /// MagickSetImageDelay() sets the image delay.
    ///
    pub fn set_image_delay(&mut self, delay: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageDelay(self.wand, delay) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagechanneldepth>
    ///
    /// MagickSetImageChannelDepth() sets the depth of a particular image channel.
    ///
    pub fn set_image_channel_depth(
        &mut self,
        channel: ChannelType,
        depth: c_ulong,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageChannelDepth(self.wand, channel.into(), depth) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagedepth>
    ///
    /// MagickSetImageDepth() sets the image depth.
    ///
    pub fn set_image_depth(&mut self, depth: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageDepth(self.wand, depth) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagedispose>
    ///
    /// MagickSetImageDispose() sets the image disposal method.
    ///
    pub fn set_image_dispose(&mut self, dispose: DisposeType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageDispose(self.wand, dispose.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagefilename>
    ///
    /// MagickSetImageFilename() sets the filename of a particular image in a
    ///
    /// sequence.
    ///
    pub fn set_image_filename(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickSetImageFilename(self.wand, filename.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageformat>
    ///
    /// MagickSetImageFormat() sets the format of a particular image in a
    ///
    /// sequence.  The format is designated by a magick string (e.g. "GIF").
    ///
    pub fn set_image_format(&mut self, format: &str) -> crate::Result<&mut Self> {
        let format = str_to_c_string(format);
        let status = unsafe { MagickSetImageFormat(self.wand, format.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagefuzz>
    ///
    /// MagickSetImageFuzz() sets the color comparison fuzz factor.  Colors
    ///
    /// closer than the fuzz factor are considered to be the same when comparing
    ///
    /// colors.  Note that some other functions such as MagickColorFloodfillImage()
    ///
    /// implicitly set this value.
    ///
    pub fn set_image_fuzz(&mut self, fuzz: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageFuzz(self.wand, fuzz) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagegamma>
    ///
    /// MagickSetImageGamma() sets the image gamma.
    ///
    pub fn set_image_gamma(&mut self, gamma: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageGamma(self.wand, gamma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagegravity>
    ///
    /// MagickSetImageGravity() sets the image gravity.  This is used
    ///
    /// when evaluating regions defined by a geometry and the image
    ///
    /// dimensions.  It may be used in conjunction with operations which
    ///
    /// use a geometry parameter to adjust the x, y parameters of the
    ///
    /// final operation. Gravity is used in composition to determine where
    ///
    /// the image should be placed within the defined geometry region.
    ///
    /// It may be used with montage to effect placement of the image within
    ///
    /// the tile.
    ///
    #[cfg(feature = "v1_3_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_22")))]
    pub fn set_image_gravity(&mut self, gravity: GravityType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageGravity(self.wand, gravity.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagegreenprimary>
    ///
    /// MagickSetImageGreenPrimary() sets the image chromaticity green primary
    ///
    /// point.
    ///
    pub fn set_image_green_primary(
        &mut self,
        x: c_double,
        y: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageGreenPrimary(self.wand, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageindex>
    ///
    /// MagickSetImageIndex() set the current image to the position of the list
    ///
    /// specified with the index parameter.
    ///
    pub fn set_image_index(&mut self, index: c_long) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageIndex(self.wand, index) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageinterlacescheme>
    ///
    /// MagickSetImageInterlaceScheme() sets the image interlace scheme.  Please
    ///
    /// use SetInterlaceScheme() instead to change the interlace scheme used when
    ///
    /// writing the image.
    ///
    pub fn set_image_interlace_scheme(
        &mut self,
        interlace_scheme: InterlaceType,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageInterlaceScheme(self.wand, interlace_scheme.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageiterations>
    ///
    /// MagickSetImageIterations() sets the image iterations.
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn set_image_iterations(&mut self, iterations: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageIterations(self.wand, iterations) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagemattecolor>
    ///
    /// MagickSetImageMatteColor() sets the image matte color.
    ///
    pub fn set_image_matte_color(&mut self, matte: &PixelWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageMatteColor(self.wand, matte.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageoption>
    ///
    /// MagickSetImageOption() associates one or options with a particular image
    ///
    /// format (.e.g MagickSetImageOption(wand,"jpeg","preserve-settings","true").
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn set_image_option(
        &mut self,
        format: &str,
        key: &str,
        value: &str,
    ) -> crate::Result<&mut Self> {
        let format = str_to_c_string(format);
        let key = str_to_c_string(key);
        let value = str_to_c_string(value);
        let status = unsafe {
            MagickSetImageOption(self.wand, format.as_ptr(), key.as_ptr(), value.as_ptr())
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageorientation>
    ///
    /// MagickSetImageOrientation() sets the internal image orientation type.
    ///
    /// The EXIF orientation tag will be updated if present.
    ///
    #[cfg(feature = "v1_3_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_26")))]
    pub fn set_image_orientation(
        &mut self,
        new_orientation: OrientationType,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageOrientation(self.wand, new_orientation.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagepage>
    ///
    /// MagickSetImagePage() sets the image page size and offset used when
    ///
    /// placing (e.g. compositing) the image.  Pass all zeros for the
    ///
    /// default placement.
    ///
    pub fn set_image_page(
        &mut self,
        width: c_ulong,
        height: c_ulong,
        x: c_long,
        y: c_long,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImagePage(self.wand, width, height, x, y) };
        self.check_status(status)
    }

    // TODO As get_image_pixels
    //    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagepixels>
    //    ///
    //    /// MagickSetImagePixels() accepts pixel data and stores it in the image at the
    //    ///
    //    /// location you specify.  The method returns False on success otherwise True
    //    ///
    //    /// if an error is encountered.  The pixel data can be either char, short int,
    //    ///
    //    /// int, long, float, or double in the order specified by map.
    //    ///
    //    /// Suppose your want want to upload the first scanline of a 640x480 image from
    //    ///
    //    /// character data in red-green-blue order:
    //    ///
    //    /// MagickSetImagePixels(wand,0,0,0,640,1,"RGB",CharPixel,pixels);
    //    ///
    //    pub fn set_image_pixels(
    //        &mut self,
    //        x_offset: c_long,
    //        y_offset: c_long,
    //        columns: c_ulong,
    //        rows: c_ulong,
    //        map: &str,
    //        storage: StorageType,
    //        pixels: &mut c_uchar,
    //    ) -> crate::Result<&mut Self> {
    //        // let status = unsafe { MagickSetImagePixels(self.wand,  x_offset,  y_offset,  columns,  rows, map,  storage, pixels) };
    //    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageprofile>
    ///
    /// MagickSetImageProfile() adds a named profile to the magick wand.  If a
    ///
    /// profile with the same name already exists, it is replaced.  This method
    ///
    /// differs from the MagickProfileImage() method in that it does not apply any
    ///
    /// CMS color profiles.
    ///
    pub fn set_image_profile(
        &mut self,
        name: &str,
        profile: &str,
        length: c_ulong,
    ) -> crate::Result<&mut Self> {
        let name = str_to_c_string(name);
        let profile = str_to_c_string(profile);
        let status = unsafe {
            MagickSetImageProfile(self.wand, name.as_ptr(), profile.as_ptr().cast(), length)
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageredprimary>
    ///
    /// MagickSetImageRedPrimary() sets the image chromaticity red primary point.
    ///
    pub fn set_image_red_primary(&mut self, x: c_double, y: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageRedPrimary(self.wand, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagerenderingintent>
    ///
    /// MagickSetImageRenderingIntent() sets the image rendering intent.
    ///
    pub fn set_image_rendering_intent(
        &mut self,
        rendering_intent: RenderingIntent,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageRenderingIntent(self.wand, rendering_intent.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageresolution>
    ///
    /// MagickSetImageResolution() sets the image resolution.
    ///
    pub fn set_image_resolution(
        &mut self,
        x_resolution: c_double,
        y_resolution: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageResolution(self.wand, x_resolution, y_resolution) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagescene>
    ///
    /// MagickSetImageScene() sets the image scene.
    ///
    pub fn set_image_scene(&mut self, scene: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageScene(self.wand, scene) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagetype>
    ///
    /// MagickSetImageType() sets the image type.
    ///
    pub fn set_image_type(&mut self, image_type: ImageType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageType(self.wand, image_type.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagesavedtype>
    ///
    /// MagickSetImageSavedType() sets the image type that will be used when the
    ///
    /// image is saved.
    ///
    pub fn set_image_saved_type(&mut self, image_type: ImageType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageSavedType(self.wand, image_type.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimageunits>
    ///
    /// MagickSetImageUnits() sets the image units of resolution.
    ///
    pub fn set_image_units(&mut self, units: ResolutionType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageUnits(self.wand, units.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagevirtualpixelmethod>
    ///
    /// MagickSetImageVirtualPixelMethod() sets the image virtual pixel method.
    ///
    pub fn set_image_virtual_pixel_method(
        &mut self,
        method: VirtualPixelMethod,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageVirtualPixelMethod(self.wand, method.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetinterlacescheme>
    ///
    /// MagickSetInterlaceScheme() sets the interlace scheme used when writing
    ///
    /// the image.
    ///
    pub fn set_interlace_scheme(
        &mut self,
        interlace_scheme: InterlaceType,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetInterlaceScheme(self.wand, interlace_scheme.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetresolution>
    ///
    /// MagickSetResolution() sets the resolution (density) of the magick wand.
    ///
    /// Set it before you read an EPS, PDF, or Postscript file in order to
    ///
    /// influence the size of the returned image, or after an image has already
    ///
    /// been created to influence the rendered image size when used with
    ///
    /// typesetting software.
    ///
    /// Also see MagickSetResolutionUnits() which specifies the units to use for
    ///
    /// the image resolution.
    ///
    pub fn set_resolution(
        &mut self,
        x_resolution: c_double,
        y_resolution: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetResolution(self.wand, x_resolution, y_resolution) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetresolutionunits>
    ///
    /// MagickSetResolutionUnits() sets the resolution units of the magick wand.
    ///
    /// It should be used in conjunction with MagickSetResolution().
    ///
    /// This method works both before and after an image has been read.
    ///
    /// Also see MagickSetImageUnits() which specifies the units which apply to
    ///
    /// the image resolution setting after an image has been read.
    ///
    pub fn set_resolution_units(&mut self, units: ResolutionType) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetResolutionUnits(self.wand, units.into()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetresourcelimit>
    ///
    /// MagickSetResourceLimit() sets the limit for a particular resource in
    ///
    /// megabytes.
    ///
    pub fn set_resource_limit(r#type: ResourceType, limit: c_ulong) -> bool {
        let status = unsafe { MagickSetResourceLimit(r#type.into(), limit) };
        status != MagickFail
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetsamplingfactors>
    ///
    /// MagickSetSamplingFactors() sets the image sampling factors.
    ///
    pub fn set_sampling_factors(&mut self, factors: &[c_double]) -> crate::Result<&mut Self> {
        let status = unsafe {
            MagickSetSamplingFactors(self.wand, factors.len() as c_ulong, factors.as_ptr())
        };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetsize>
    ///
    /// MagickSetSize() sets the size of the magick wand.  Set it before you
    ///
    /// read a raw image format such as RGB, GRAY, or CMYK.
    ///
    pub fn set_size(&mut self, columns: c_ulong, rows: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetSize(self.wand, columns, rows) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetimagewhitepoint>
    ///
    /// MagickSetImageWhitePoint() sets the image chromaticity white point.
    ///
    pub fn set_image_white_point(&mut self, x: c_double, y: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSetImageWhitePoint(self.wand, x, y) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksetpassphrase>
    ///
    /// MagickSetPassphrase() sets the passphrase.
    ///
    pub fn set_passphrase(&mut self, passphrase: &str) -> crate::Result<&mut Self> {
        let passphrase = str_to_c_string(passphrase);
        let status = unsafe { MagickSetPassphrase(self.wand, passphrase.as_ptr()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksharpenimage>
    ///
    /// MagickSharpenImage() sharpens an image.  We convolve the image with a Gaussian
    ///
    /// operator of the given radius and standard deviation (sigma).
    ///
    /// For reasonable results, the radius should be larger than sigma.  Use a
    ///
    /// radius of 0 and SharpenImage() selects a suitable radius for you.
    ///
    pub fn sharpen_image(&mut self, radius: c_double, sigma: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSharpenImage(self.wand, radius, sigma) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickshaveimage>
    ///
    /// MagickShaveImage() shaves pixels from the image edges.  It allocates the
    ///
    /// memory necessary for the new Image structure and returns a pointer to the
    ///
    /// new image.
    ///
    pub fn shave_image(&mut self, columns: c_ulong, rows: c_ulong) -> crate::Result<&mut Self> {
        let status = unsafe { MagickShaveImage(self.wand, columns, rows) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickshearimage>
    ///
    /// MagickShearImage() slides one edge of an image along the X or Y axis,
    ///
    /// creating a parallelogram.  An X direction shear slides an edge along the X
    ///
    /// axis, while a Y direction shear slides an edge along the Y axis.  The amount
    ///
    /// of the shear is controlled by a shear angle.  For X direction shears, x_shear
    ///
    /// is measured relative to the Y axis, and similarly, for Y direction shears
    ///
    /// y_shear is measured relative to the X axis.  Empty triangles left over from
    ///
    /// shearing the image are filled with the background color.
    ///
    pub fn shear_image(
        &mut self,
        background: &PixelWand,
        x_shear: c_double,
        y_shear: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickShearImage(self.wand, background.wand(), x_shear, y_shear) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksolarizeimage>
    ///
    /// MagickSolarizeImage() applies a special effect to the image, similar to the
    ///
    /// effect achieved in a photo darkroom by selectively exposing areas of photo
    ///
    /// sensitive paper to light.  Threshold ranges from 0 to MaxRGB and is a
    ///
    /// measure of the extent of the solarization.
    ///
    pub fn solarize_image(&mut self, threshold: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSolarizeImage(self.wand, threshold) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickspreadimage>
    ///
    /// MagickSpreadImage() is a special effects method that randomly displaces each
    ///
    /// pixel in a block defined by the radius parameter.
    ///
    pub fn spread_image(&mut self, radius: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSpreadImage(self.wand, radius) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicksteganoimage>
    ///
    /// Use MagickSteganoImage() to hide a digital watermark within the image.
    ///
    /// Recover the hidden watermark later to prove that the authenticity of
    ///
    /// an image.  Offset defines the start position within the image to hide
    ///
    /// the watermark.
    ///
    pub fn stegano_image(
        &mut self,
        watermark_wand: &MagickWand<'_>,
        offset: c_long,
    ) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickSteganoImage(self.wand, watermark_wand.wand(), offset) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickstereoimage>
    ///
    /// MagickStereoImage() composites two images and produces a single image that
    ///
    /// is the composite of a left and right image of a stereo pair
    ///
    pub fn stereo_image(&mut self, offset_wand: &MagickWand<'_>) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickStereoImage(self.wand, offset_wand.wand()) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickstripimage>
    ///
    /// MagickStripImage() removes all profiles and text attributes from the image.
    ///
    pub fn strip_image(&mut self) -> crate::Result<&mut Self> {
        let status = unsafe { MagickStripImage(self.wand) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickswirlimage>
    ///
    /// MagickSwirlImage() swirls the pixels about the center of the image, where
    ///
    /// degrees indicates the sweep of the arc through which each pixel is moved.
    ///
    /// You get a more dramatic effect as the degrees move from 1 to 360.
    ///
    pub fn swirl_image(&mut self, degrees: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickSwirlImage(self.wand, degrees) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicktextureimage>
    ///
    /// MagickTextureImage() repeatedly tiles the texture image across and down the
    ///
    /// image canvas.
    ///
    pub fn texture_image(&mut self, texture_wand: &MagickWand<'_>) -> Option<MagickWand<'_>> {
        let wand = unsafe { MagickTextureImage(self.wand, texture_wand.wand()) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickthresholdimage>
    ///
    /// MagickThresholdImage() changes the value of individual pixels based on
    ///
    /// the intensity of each pixel compared to threshold.  The result is a
    ///
    /// high-contrast, two color image.
    ///
    pub fn threshold_image(&mut self, threshold: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickThresholdImage(self.wand, threshold) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickthresholdimagechannel>
    ///
    /// MagickThresholdImageChannel() changes the value of individual pixel
    ///
    /// component based on the intensity of each pixel compared to threshold.  The
    ///
    /// result is a high-contrast, two color image.
    ///
    pub fn threshold_image_channel(
        &mut self,
        channel: ChannelType,
        threshold: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickThresholdImageChannel(self.wand, channel.into(), threshold) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicktintimage>
    ///
    /// MagickTintImage() applies a color vector to each pixel in the image.  The
    ///
    /// length of the vector is 0 for black and white and at its maximum for the
    ///
    /// midtones.  The vector weighting function is
    ///
    /// f(x)=(1-(4.0*((x-0.5)*(x-0.5)))).
    ///
    pub fn tint_image(
        &mut self,
        tint: &PixelWand,
        opacity: &PixelWand,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickTintImage(self.wand, tint.wand(), opacity.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicktransformimage>
    ///
    /// MagickTransformImage() is a convenience method that behaves like
    ///
    /// MagickResizeImage() or MagickCropImage() but accepts scaling and/or cropping
    ///
    /// information as a region geometry specification.  If the operation fails, the
    ///
    /// original image handle is returned.
    ///
    pub fn transform_image(&mut self, crop: &str, geometry: &str) -> Option<MagickWand<'_>> {
        let crop = str_to_c_string(crop);
        let geometry = str_to_c_string(geometry);
        let wand = unsafe { MagickTransformImage(self.wand, crop.as_ptr(), geometry.as_ptr()) };
        MagickWand::from_wand(wand)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicktransparentimage>
    ///
    /// MagickTransparentImage() changes any pixel that matches color with the color
    ///
    /// defined by fill.
    ///
    pub fn transparent_image(
        &mut self,
        target: &PixelWand,
        opacity: Quantum,
        fuzz: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickTransparentImage(self.wand, target.wand(), opacity, fuzz) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magicktrimimage>
    ///
    /// MagickTrimImage() remove edges that are the background color from the image.
    ///
    pub fn trim_image(&mut self, fuzz: c_double) -> crate::Result<&mut Self> {
        let status = unsafe { MagickTrimImage(self.wand, fuzz) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickunsharpmaskimage>
    ///
    /// MagickUnsharpMaskImage() sharpens an image.  We convolve the image with a
    ///
    /// Gaussian operator of the given radius and standard deviation (sigma).
    ///
    /// For reasonable results, radius should be larger than sigma.  Use a radius
    ///
    /// of 0 and UnsharpMaskImage() selects a suitable radius for you.
    ///
    pub fn unsharp_mask_image(
        &mut self,
        radius: c_double,
        sigma: c_double,
        amount: c_double,
        threshold: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickUnsharpMaskImage(self.wand, radius, sigma, amount, threshold) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwaveimage>
    ///
    /// MagickWaveImage()  creates a "ripple" effect in the image by shifting
    ///
    /// the pixels vertically along a sine wave whose amplitude and wavelength
    ///
    /// is specified by the given parameters.
    ///
    pub fn wave_image(
        &mut self,
        amplitude: c_double,
        wave_length: c_double,
    ) -> crate::Result<&mut Self> {
        let status = unsafe { MagickWaveImage(self.wand, amplitude, wave_length) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwhitethresholdimage>
    ///
    /// MagickWhiteThresholdImage() is like ThresholdImage() but  forces all pixels
    ///
    /// above the threshold into white while leaving all pixels below the threshold
    ///
    /// unchanged.
    ///
    pub fn white_threshold_image(&mut self, threshold: &PixelWand) -> crate::Result<&mut Self> {
        let status = unsafe { MagickWhiteThresholdImage(self.wand, threshold.wand()) };
        self.check_status(status)
    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwriteimage>
    ///
    /// MagickWriteImage() writes an image.
    ///
    pub fn write_image(&mut self, filename: &str) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickWriteImage(self.wand, filename.as_ptr()) };
        self.check_status(status)
    }

    // Not need
    //    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwriteimagesfile>
    //    ///
    //    /// MagickWriteImagesFile() writes an image or image sequence to a stdio
    //    ///
    //    /// FILE handle.  This may be used to append an encoded image to an already
    //    ///
    //    /// existing appended image sequence if the file seek position is at the end
    //    ///
    //    /// of an existing file.
    //    ///
    //    pub fn write_images_file(
    //        &mut self,
    //        file: &mut File,
    //        adjoin: c_uint,
    //    ) -> crate::Result<&mut Self> {
    //        // let status = unsafe { MagickWriteImagesFile(self.wand, file,  adjoin) };
    //    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwriteimageblob>
    ///
    /// MagickWriteImageBlob() implements direct to memory image formats.  It
    ///
    /// returns the image as a blob (a formatted "file" in memory) and its
    ///
    /// length, starting from the current position in the image sequence.
    ///
    /// Use MagickSetImageFormat() to set the format to write to the blob
    ///
    /// (GIF, JPEG,  PNG, etc.).
    ///
    /// Use MagickResetIterator() on the wand if it is desired to write
    ///
    /// a sequence from the beginning and the iterator is not currently
    ///
    /// at the beginning.
    ///
    pub fn write_image_blob(&mut self) -> Option<Vec<u8>> {
        let mut length = 0;
        let ptr = unsafe { MagickWriteImageBlob(self.wand, &mut length) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { Vec::from_raw_parts(ptr, length as usize, length as usize) })
    }

    // Not need
    //    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwriteimagefile>
    //    ///
    //    /// MagickWriteImageFile() writes an image to an open file descriptor.
    //    ///
    //    pub fn write_image_file(&mut self, file: &mut File) -> crate::Result<&mut Self> {
    //        // let status = unsafe { MagickWriteImageFile(self.wand, file) };
    //    }

    /// <http://www.graphicsmagick.org/wand/magick_wand.html#magickwriteimages>
    ///
    /// MagickWriteImages() writes an image or image sequence.  If the wand
    ///
    /// represents an image sequence, then it is written starting at the first
    ///
    /// frame in the sequence.
    ///
    pub fn write_images(&mut self, filename: &str, adjoin: c_uint) -> crate::Result<&mut Self> {
        let filename = str_to_c_string(filename);
        let status = unsafe { MagickWriteImages(self.wand, filename.as_ptr(), adjoin) };
        self.check_status(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initialize,
        tests::{logo_path, logo_unicode_path},
    };
    use std::{fs::File, io::Read};

    fn new_magick_wand<'a>() -> MagickWand<'a> {
        initialize();
        MagickWand::new()
    }

    fn new_logo_magick_wand<'a>() -> MagickWand<'a> {
        let mut mw = new_magick_wand();
        mw.read_image(&logo_unicode_path()).unwrap();
        mw
    }

    #[test]
    fn test_magick_wand_new() {
        let mw = new_magick_wand();
        assert_eq!(mw.blob, None);
    }

    #[test]
    fn test_magick_wand_read_image() {
        let mut mw = new_magick_wand();
        mw.read_image(&logo_unicode_path()).unwrap();
        assert_eq!((mw.get_image_width(), mw.get_image_height()), (311, 177));
    }

    #[test]
    fn test_magick_wand_read_image_blob() {
        let mut file = File::open(&logo_path()).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();

        let mut mw = new_magick_wand();
        mw.read_image_blob(&content).unwrap();
    }

    #[test]
    fn test_magick_wand_read_image_blob_failed() {
        let content = b"....";
        let mut mw = new_magick_wand();
        assert!(mw.read_image_blob(content).is_err());
    }

    #[test]
    #[should_panic]
    fn test_magick_wand_adaptive_threshold_image_with_zero_width() {
        let mut mw = new_logo_magick_wand();
        mw.adaptive_threshold_image(0, 1, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_magick_wand_adaptive_threshold_image_with_zero_height() {
        let mut mw = new_logo_magick_wand();
        mw.adaptive_threshold_image(1, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_adaptive_threshold_image() {
        let mut mw = new_logo_magick_wand();
        mw.adaptive_threshold_image(1, 1, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_add_image() {
        let mut mw = new_logo_magick_wand();
        let mw2 = new_logo_magick_wand();
        mw.add_image(&mw2).unwrap();
    }

    #[test]
    fn test_magick_wand_add_noise_image() {
        let mut mw = new_logo_magick_wand();
        mw.add_noise_image(NoiseType::GaussianNoise).unwrap();
    }

    #[test]
    fn test_magick_wand_affine_transform_image() {
        let mut mw = new_logo_magick_wand();
        mw.affine_transform_image(&DrawingWand::new()).unwrap();
    }

    #[test]
    fn test_magick_wand_annotate_image() {
        let mut mw = new_logo_magick_wand();
        let _ = mw.annotate_image(&DrawingWand::new(), 0., 0., 0., "Hello 你好！");
    }

    #[test]
    #[ignore] // Ignore because it is a gui method.
    fn test_magick_wand_animate_images() {
        let mut mw = new_logo_magick_wand();
        mw.animate_images("Hello 你好！").unwrap();
    }

    #[test]
    fn test_magick_wand_append_images() {
        let mut mw = new_logo_magick_wand();
        mw.append_images(0).unwrap();
        mw.append_images(10).unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_auto_orient_image() {
        let mut mw = new_logo_magick_wand();
        mw.auto_orient_image(OrientationType::BottomLeftOrientation)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_average_images() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.average_images().is_none());
    }

    #[test]
    fn test_magick_wand_black_threshold_image() {
        let mut mw = new_logo_magick_wand();
        mw.black_threshold_image(&PixelWand::new()).unwrap();
    }

    #[test]
    fn test_magick_wand_blur_image() {
        let mut mw = new_logo_magick_wand();
        mw.blur_image(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_border_image() {
        let mut mw = new_logo_magick_wand();
        mw.border_image(&PixelWand::new(), 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_cdl_image() {
        let mut mw = new_logo_magick_wand();
        mw.cdl_image("").unwrap();
        mw.cdl_image("1.0,0.0,1.0:1.0,0.0,1.0:1.0,0.0,1.0:0.0")
            .unwrap();
    }

    #[test]
    fn test_magick_wand_charcoal_image() {
        let mut mw = new_logo_magick_wand();
        mw.charcoal_image(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_chop_image() {
        let mut mw = new_logo_magick_wand();
        mw.chop_image(0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_clip_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.clip_image().is_err());
    }

    #[test]
    fn test_magick_wand_clip_path_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.clip_path_image("", false).is_err());
        assert!(mw.clip_path_image("#1", true).is_err());
    }

    #[test]
    fn test_magick_wand_coalesce_images() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.coalesce_images().is_none());
    }

    #[test]
    fn test_magick_wand_color_flood_fill_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw
            .color_floodfill_image(&PixelWand::new(), 0., &PixelWand::new(), 0, 0)
            .is_err());
    }

    #[test]
    fn test_magick_wand_colorize_image() {
        let mut mw = new_logo_magick_wand();
        mw.colorize_image(&PixelWand::new(), &PixelWand::new())
            .unwrap();
    }

    #[test]
    fn test_magick_wand_comment_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.comment_image("").is_err());
    }

    #[test]
    fn test_magick_wand_compare_image_channels() {
        let mut mw = new_logo_magick_wand();
        mw.compare_image_channels(
            &new_magick_wand(),
            ChannelType::AllChannels,
            MetricType::MeanAbsoluteErrorMetric,
            &mut 0.,
        );
    }

    #[test]
    fn test_magick_wand_compare_images() {
        let mut mw = new_logo_magick_wand();
        mw.compare_images(
            &new_magick_wand(),
            MetricType::MeanAbsoluteErrorMetric,
            &mut 0.,
        );
    }

    #[test]
    fn test_magick_wand_composite_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw
            .composite_image(
                &new_magick_wand(),
                CompositeOperator::ClearCompositeOp,
                0,
                0,
            )
            .is_err())
    }

    #[test]
    fn test_magick_wand_contrast_image() {
        let mut mw = new_logo_magick_wand();
        mw.contrast_image(0).unwrap();
    }

    #[test]
    fn test_magick_wand_convolve_image() {
        let mut mw = new_logo_magick_wand();
        mw.convolve_image(&[0.]).unwrap();
        assert!(mw.convolve_image(&[0., 0.]).is_err());
    }

    #[test]
    fn test_magick_wand_crop_image() {
        let mut mw = new_logo_magick_wand();
        mw.crop_image(0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_cycle_colormap_image() {
        let mut mw = new_logo_magick_wand();
        mw.cycle_colormap_image(0).unwrap();
    }

    #[test]
    fn test_magick_wand_deconstruct_images() {
        let mut mw = new_logo_magick_wand();
        mw.deconstruct_images();
    }

    #[test]
    fn test_magick_wand_describe_image() {
        let mut mw = new_logo_magick_wand();
        mw.describe_image().to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_despeckle_image() {
        let mut mw = new_logo_magick_wand();
        mw.despeckle_image().unwrap();
    }

    #[test]
    #[ignore] // Ignore because it is a gui method.
    fn test_magick_wand_display_image() {
        let mut mw = new_logo_magick_wand();
        mw.display_image("").unwrap();
    }

    #[test]
    #[ignore] // Ignore because it is a gui method.
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_display_images() {
        let mut mw = new_logo_magick_wand();
        mw.display_images("").unwrap();
    }

    #[test]
    fn test_magick_wand_draw_image() {
        let mut mw = new_logo_magick_wand();
        let mut dw = DrawingWand::new();
        dw.point(1., 1.);
        mw.draw_image(&dw).unwrap();
    }

    #[test]
    fn test_magick_wand_edge_image() {
        let mut mw = new_logo_magick_wand();
        mw.edge_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_emboss_image() {
        let mut mw = new_logo_magick_wand();
        mw.emboss_image(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_enhance_image() {
        let mut mw = new_logo_magick_wand();
        mw.enhance_image().unwrap();
    }

    #[test]
    fn test_magick_wand_equalize_image() {
        let mut mw = new_logo_magick_wand();
        mw.equalize_image().unwrap();
    }

    #[test]
    fn test_magick_wand_extent_image() {
        let mut mw = new_logo_magick_wand();
        mw.extent_image(0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_flatten_images() {
        let mut mw = new_logo_magick_wand();
        mw.flatten_images().unwrap();
    }

    #[test]
    fn test_magick_wand_flip_image() {
        let mut mw = new_logo_magick_wand();
        mw.flip_image().unwrap();
    }

    #[test]
    fn test_magick_wand_flop_image() {
        let mut mw = new_logo_magick_wand();
        mw.flop_image().unwrap();
    }

    #[test]
    fn test_magick_wand_frame_image() {
        let mut mw = new_logo_magick_wand();
        mw.frame_image(&PixelWand::new(), 0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_fx_image() {
        let mut mw = new_logo_magick_wand();
        mw.fx_image("");
    }

    #[test]
    fn test_magick_wand_fx_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.fx_image_channel(ChannelType::AllChannels, "");
    }

    #[test]
    fn test_magick_wand_gamma_image() {
        let mut mw = new_logo_magick_wand();
        mw.gamma_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_gamma_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.gamma_image_channel(ChannelType::AllChannels, 0.)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_get_configure_info() {
        let mut mw = new_logo_magick_wand();
        mw.get_configure_info("VERSION").to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_copyright() {
        MagickWand::get_copyright();
    }

    #[test]
    fn test_magick_wand_get_filename() {
        let mw = new_logo_magick_wand();
        mw.get_filename().to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_home_url() {
        let _mw = new_logo_magick_wand();
        MagickWand::get_home_url();
    }

    #[test]
    fn test_magick_wand_get_image() {
        let mut mw = new_logo_magick_wand();
        mw.get_image();
    }

    #[test]
    fn test_magick_wand_get_image_attribute() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_attribute("").to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_background_color() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_background_color().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_blue_primary() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_blue_primary().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_border_color() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_border_color().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_bounding_box() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_bounding_box(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_channel_depth() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_channel_depth(ChannelType::AllChannels);
    }

    #[test]
    fn test_magick_wand_get_image_channel_extrema() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_channel_extrema(ChannelType::AllChannels)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_channel_mean() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_channel_mean(ChannelType::AllChannels).unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_colormap_color() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.get_image_colormap_color(0).is_err());
        assert!(mw.get_image_colormap_color(1).is_err());
    }

    #[test]
    fn test_magick_wand_get_image_colors() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_colors();
    }

    #[test]
    fn test_magick_wand_get_image_colorspace() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_colorspace();
    }

    #[test]
    fn test_magick_wand_get_image_compose() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_compose();
    }

    #[test]
    fn test_magick_wand_get_image_compression() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_compression();
    }

    #[test]
    fn test_magick_wand_get_image_delay() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_delay();
    }

    #[test]
    fn test_magick_wand_get_image_depth() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_depth();
    }

    #[test]
    fn test_magick_wand_get_image_extrema() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_extrema().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_dispose() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_dispose();
    }

    #[test]
    fn test_magick_wand_get_image_filename() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_filename().to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_format() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_format().to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_fuzz() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_fuzz();
    }

    #[test]
    fn test_magick_wand_get_image_gamma() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_gamma();
    }

    #[test]
    #[cfg(feature = "v1_3_22")]
    fn test_magick_wand_get_image_gravity() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_gravity();
    }

    #[test]
    fn test_magick_wand_get_image_green_primary() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_green_primary().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_height() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_height();
    }

    #[test]
    fn test_magick_wand_get_image_histogram() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_histogram().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_index() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_index();
    }

    #[test]
    fn test_magick_wand_get_image_interlace_scheme() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_interlace_scheme();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_get_image_iterations() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_iterations();
    }

    #[test]
    fn test_magick_wand_get_image_matte_color() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_matte_color().unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_get_image_orientation() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_orientation();
    }

    #[test]
    fn test_magick_wand_get_image_page() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_page().unwrap();
    }

    fn test_magick_wand_get_image_pixels_inner<ExportType: MagickWandExportType>() {
        let mut mw = new_logo_magick_wand();

        let pixels = mw
            .get_image_pixels::<ExportType>(0, 0, 0, 0, "RGBA")
            .unwrap();

        assert!(pixels.is_empty());

        let pixels = mw
            .get_image_pixels::<ExportType>(0, 0, 10, 10, "RGBA")
            .unwrap();

        assert!(!pixels.is_empty());
    }

    #[test]
    fn test_magick_wand_get_image_pixels() {
        test_magick_wand_get_image_pixels_inner::<MagickWandExportCharPixel>();
        test_magick_wand_get_image_pixels_inner::<MagickWandExportShortPixel>();
        test_magick_wand_get_image_pixels_inner::<MagickWandExportIntegerPixel>();
        test_magick_wand_get_image_pixels_inner::<MagickWandExportLongPixel>();
        test_magick_wand_get_image_pixels_inner::<MagickWandExportFloatPixel>();
        test_magick_wand_get_image_pixels_inner::<MagickWandExportDoublePixel>();
    }

    #[test]
    fn test_magick_wand_get_image_profile() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_profile("ICM").to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_red_primary() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_red_primary().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_rendering_intent() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_rendering_intent();
    }

    #[test]
    fn test_magick_wand_get_image_resolution() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_resolution().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_scene() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_scene();
    }

    #[test]
    fn test_magick_wand_get_image_signature() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_signature().to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_size() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_size();
    }

    #[test]
    fn test_magick_wand_get_image_type() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_type();
    }

    #[test]
    fn test_magick_wand_get_image_saved_type() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_saved_type();
    }

    #[test]
    fn test_magick_wand_get_image_units() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_units();
    }

    #[test]
    fn test_magick_wand_get_image_virtual_pixel_method() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_virtual_pixel_method();
    }

    #[test]
    fn test_magick_wand_get_image_white_point() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_white_point().unwrap();
    }

    #[test]
    fn test_magick_wand_get_image_width() {
        let mut mw = new_logo_magick_wand();
        mw.get_image_width();
    }

    #[test]
    fn test_magick_wand_get_number_images() {
        let mut mw = new_logo_magick_wand();
        mw.get_number_images();
    }

    #[test]
    fn test_magick_wand_get_package_name() {
        MagickWand::get_package_name();
    }

    #[test]
    fn test_magick_wand_get_quantum_depth() {
        MagickWand::get_quantum_depth();
    }

    #[test]
    fn test_magick_wand_get_release_date() {
        MagickWand::get_release_date();
    }

    #[test]
    fn test_magick_wand_get_resource_limit() {
        MagickWand::get_resource_limit(ResourceType::UndefinedResource);
    }

    #[test]
    fn test_magick_wand_get_sampling_factors() {
        let mut mw = new_logo_magick_wand();
        mw.get_sampling_factors();
    }

    #[test]
    fn test_magick_wand_get_size() {
        let mw = new_logo_magick_wand();
        mw.get_size();
    }

    #[test]
    fn test_magick_wand_get_version() {
        MagickWand::get_version();
    }

    #[test]
    fn test_magick_wand_hald_clut_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.hald_clut_image(&new_magick_wand()).is_err());
        assert!(mw.hald_clut_image(&new_logo_magick_wand()).is_err());
    }

    #[test]
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_has_colormap() {
        let mut mw = new_logo_magick_wand();
        mw.has_colormap().unwrap();
    }

    #[test]
    fn test_magick_wand_has_next_image() {
        let mut mw = new_logo_magick_wand();
        mw.has_next_image();
    }

    #[test]
    fn test_magick_wand_has_previous_image() {
        let mut mw = new_logo_magick_wand();
        mw.has_previous_image();
    }

    #[test]
    fn test_magick_wand_implode_image() {
        let mut mw = new_logo_magick_wand();
        mw.implode_image(0.).unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_is_gray_image() {
        let mut mw = new_logo_magick_wand();
        mw.is_gray_image().unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_is_monochrome_image() {
        let mut mw = new_logo_magick_wand();
        mw.is_monochrome_image().unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_is_opaque_image() {
        let mut mw = new_logo_magick_wand();
        mw.is_opaque_image().unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_29")]
    fn test_magick_wand_is_palette_image() {
        let mut mw = new_logo_magick_wand();
        mw.is_palette_image().unwrap();
    }

    #[test]
    fn test_magick_wand_label_image() {
        let mut mw = new_logo_magick_wand();
        mw.label_image("x").unwrap();
    }

    #[test]
    fn test_magick_wand_level_image() {
        let mut mw = new_logo_magick_wand();
        mw.level_image(0., 0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_level_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.level_image_channel(ChannelType::AllChannels, 0., 0., 0.)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_magnify_image() {
        let mut mw = new_logo_magick_wand();
        mw.magnify_image().unwrap();
    }

    #[test]
    fn test_magick_wand_map_image() {
        let mut mw = new_logo_magick_wand();
        let _ = mw.map_image(&new_logo_magick_wand(), 0);
    }

    #[test]
    fn test_magick_wand_matte_flood_fill_image() {
        let mut mw = new_logo_magick_wand();
        mw.matte_floodfill_image(1, 0., &PixelWand::new(), 0, 0)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_median_filter_image() {
        let mut mw = new_logo_magick_wand();
        mw.median_filter_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_minify_image() {
        let mut mw = new_logo_magick_wand();
        mw.minify_image().unwrap();
    }

    #[test]
    fn test_magick_wand_modulate_image() {
        let mut mw = new_logo_magick_wand();
        mw.modulate_image(0., 0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_montage_image() {
        let mut mw = new_logo_magick_wand();
        mw.montage_image(&DrawingWand::new(), "", "", MontageMode::UndefinedMode, "")
            .unwrap();
    }

    #[test]
    fn test_magick_wand_mosaic_images() {
        let mut mw = new_logo_magick_wand();
        mw.mosaic_images().unwrap();
    }

    #[test]
    fn test_magick_wand_motion_blur_image() {
        let mut mw = new_logo_magick_wand();
        mw.motion_blur_image(0., 1., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_negate_image() {
        let mut mw = new_logo_magick_wand();
        mw.negate_image(0).unwrap();
    }

    #[test]
    fn test_magick_wand_negate_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.negate_image_channel(ChannelType::AllChannels, 0)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_next_image() {
        let mut mw = new_logo_magick_wand();
        assert!(!mw.next_image());
    }

    #[test]
    fn test_magick_wand_normalize_image() {
        let mut mw = new_logo_magick_wand();
        mw.normalize_image().unwrap();
    }

    #[test]
    fn test_magick_wand_oil_paint_image() {
        let mut mw = new_logo_magick_wand();
        mw.oil_paint_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_opaque_image() {
        let mut mw = new_logo_magick_wand();
        mw.opaque_image(&PixelWand::new(), &PixelWand::new(), 0.)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_ping_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.ping_image("").is_err());
    }

    #[test]
    fn test_magick_wand_preview_images() {
        let mut mw = new_logo_magick_wand();
        mw.preview_images(PreviewType::UndefinedPreview);
    }

    #[test]
    fn test_magick_wand_previous_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.previous_image().is_err());
    }

    #[test]
    fn test_magick_wand_profile_image() {
        let mut mw = new_logo_magick_wand();
        mw.profile_image("", "", 0).unwrap();
    }

    #[test]
    fn test_magick_wand_quantize_image() {
        let mut mw = new_logo_magick_wand();
        mw.quantize_image(0, ColorspaceType::CineonLogRGBColorspace, 0, 0, 0)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_quantize_images() {
        let mut mw = new_logo_magick_wand();
        mw.quantize_images(0, ColorspaceType::CineonLogRGBColorspace, 0, 0, 0)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_query_font_metrics() {
        let mut mw = new_logo_magick_wand();
        let _ = mw.query_font_metrics(&DrawingWand::new(), "");
    }

    #[test]
    fn test_magick_wand_query_fonts() {
        MagickWand::query_fonts("").unwrap();
    }

    #[test]
    fn test_magick_wand_query_formats() {
        MagickWand::query_formats("").unwrap();
    }

    #[test]
    fn test_magick_wand_radial_blur_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.radial_blur_image(0.).is_err());
    }

    #[test]
    fn test_magick_wand_raise_image() {
        let mut mw = new_logo_magick_wand();
        mw.raise_image(0, 0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_reduce_noise_image() {
        let mut mw = new_logo_magick_wand();
        mw.reduce_noise_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_remove_image() {
        let mut mw = new_logo_magick_wand();
        mw.remove_image().unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_remove_image_option() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.remove_image_option("", "").is_err());
    }

    #[test]
    fn test_magick_wand_remove_image_profile() {
        let mut mw = new_logo_magick_wand();
        mw.remove_image_profile("").to_str().unwrap();
    }

    #[test]
    fn test_magick_wand_reset_iterator() {
        let mut mw = new_logo_magick_wand();
        mw.reset_iterator();
    }

    #[test]
    fn test_magick_wand_resample_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw
            .resample_image(0., 0., FilterTypes::UndefinedFilter, 0.)
            .is_err());
    }

    #[test]
    fn test_magick_wand_resize_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw
            .resize_image(0, 0, FilterTypes::UndefinedFilter, 0.)
            .is_err());
    }

    #[test]
    fn test_magick_wand_roll_image() {
        let mut mw = new_logo_magick_wand();
        mw.roll_image(0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_rotate_image() {
        let mut mw = new_logo_magick_wand();
        mw.rotate_image(&PixelWand::new(), 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_sample_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.sample_image(0, 0).is_err());
    }

    #[test]
    fn test_magick_wand_scale_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.scale_image(0, 0).is_err());
    }

    #[test]
    fn test_magick_wand_separate_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.separate_image_channel(ChannelType::UndefinedChannel)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_compression_quality() {
        let mut mw = new_logo_magick_wand();
        mw.set_compression_quality(0).unwrap();
    }

    //    #[test]
    //    fn test_magick_wand_set_depth() {
    //        let mut mw = new_logo_magick_wand();
    //        mw.set_depth(
    //            0
    //        ).unwrap();
    //    }

    #[test]
    fn test_magick_wand_set_filename() {
        let mut mw = new_logo_magick_wand();
        mw.set_filename("").unwrap();
    }

    #[test]
    fn test_magick_wand_set_format() {
        let mut mw = new_logo_magick_wand();
        mw.set_format("").unwrap();
    }

    #[test]
    fn test_magick_wand_set_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.set_image(&new_magick_wand()).is_err());
    }

    #[test]
    fn test_magick_wand_set_image_attribute() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.set_image_attribute("", "").is_err());
    }

    #[test]
    fn test_magick_wand_set_image_background_color() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_background_color(&PixelWand::new()).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_blue_primary() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_blue_primary(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_border_color() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_border_color(&PixelWand::new()).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_colormap_color() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.set_image_colormap_color(0, &PixelWand::new()).is_err());
    }

    #[test]
    fn test_magick_wand_set_image_colorspace() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_colorspace(ColorspaceType::CineonLogRGBColorspace)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_compose() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_compose(CompositeOperator::UndefinedCompositeOp)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_compression() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_compression(CompressionType::UndefinedCompression)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_delay() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_delay(0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_channel_depth() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_channel_depth(ChannelType::UndefinedChannel, 0)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_depth() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_depth(0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_dispose() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_dispose(DisposeType::UndefinedDispose).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_filename() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_filename("").unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_format() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_format("").unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_fuzz() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_fuzz(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_gamma() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_gamma(0.).unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_22")]
    fn test_magick_wand_set_image_gravity() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_gravity(GravityType::ForgetGravity).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_green_primary() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_green_primary(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_index() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_index(0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_interlace_scheme() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_interlace_scheme(InterlaceType::UndefinedInterlace)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_set_image_iterations() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_iterations(0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_matte_color() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_matte_color(&PixelWand::new()).unwrap();
    }

    #[test]
    #[cfg(feature = "v1_3_26")]
    fn test_magick_wand_set_image_option() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_option("", "", "").unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_page() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_page(0, 0, 0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_profile() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_profile("", "", 0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_red_primary() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_red_primary(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_rendering_intent() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_rendering_intent(RenderingIntent::UndefinedIntent)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_resolution() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_resolution(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_scene() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_scene(0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_type() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_type(ImageType::UndefinedType).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_saved_type() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_saved_type(ImageType::UndefinedType).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_units() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_units(ResolutionType::UndefinedResolution)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_virtual_pixel_method() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_virtual_pixel_method(VirtualPixelMethod::UndefinedVirtualPixelMethod)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_interlace_scheme() {
        let mut mw = new_logo_magick_wand();
        mw.set_interlace_scheme(InterlaceType::UndefinedInterlace)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_resolution() {
        let mut mw = new_logo_magick_wand();
        mw.set_resolution(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_resolution_units() {
        let mut mw = new_logo_magick_wand();
        mw.set_resolution_units(ResolutionType::UndefinedResolution)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_set_resource_limit() {
        MagickWand::set_resource_limit(ResourceType::UndefinedResource, 0);
    }

    #[test]
    fn test_magick_wand_set_sampling_factors() {
        let mut mw = new_logo_magick_wand();
        mw.set_sampling_factors(&[]).unwrap();
    }

    #[test]
    fn test_magick_wand_set_size() {
        let mut mw = new_logo_magick_wand();
        mw.set_size(0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_set_image_white_point() {
        let mut mw = new_logo_magick_wand();
        mw.set_image_white_point(0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_set_passphrase() {
        let mut mw = new_logo_magick_wand();
        mw.set_passphrase("").unwrap();
    }

    #[test]
    fn test_magick_wand_sharpen_image() {
        let mut mw = new_logo_magick_wand();
        mw.sharpen_image(0., 1.).unwrap();
    }

    #[test]
    fn test_magick_wand_shave_image() {
        let mut mw = new_logo_magick_wand();
        mw.shave_image(0, 0).unwrap();
    }

    #[test]
    fn test_magick_wand_shear_image() {
        let mut mw = new_logo_magick_wand();
        mw.shear_image(&PixelWand::new(), 0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_solarize_image() {
        let mut mw = new_logo_magick_wand();
        mw.solarize_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_spread_image() {
        let mut mw = new_logo_magick_wand();
        mw.spread_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_stegano_image() {
        let mut mw = new_logo_magick_wand();
        mw.stegano_image(&new_magick_wand(), 0);
    }

    #[test]
    fn test_magick_wand_stereo_image() {
        let mut mw = new_logo_magick_wand();
        mw.stereo_image(&new_magick_wand());
    }

    #[test]
    fn test_magick_wand_strip_image() {
        let mut mw = new_logo_magick_wand();
        mw.strip_image().unwrap();
    }

    #[test]
    fn test_magick_wand_swirl_image() {
        let mut mw = new_logo_magick_wand();
        mw.swirl_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_texture_image() {
        let mut mw = new_logo_magick_wand();
        mw.texture_image(&new_logo_magick_wand()).unwrap();
    }

    #[test]
    fn test_magick_wand_threshold_image() {
        let mut mw = new_logo_magick_wand();
        mw.threshold_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_threshold_image_channel() {
        let mut mw = new_logo_magick_wand();
        mw.threshold_image_channel(ChannelType::UndefinedChannel, 0.)
            .unwrap();
    }

    #[test]
    fn test_magick_wand_tint_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.tint_image(&PixelWand::new(), &PixelWand::new()).is_err());
    }

    #[test]
    fn test_magick_wand_transform_image() {
        let mut mw = new_logo_magick_wand();
        mw.transform_image("", "").unwrap();
    }

    #[test]
    fn test_magick_wand_transparent_image() {
        let mut mw = new_logo_magick_wand();
        mw.transparent_image(&PixelWand::new(), 0, 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_trim_image() {
        let mut mw = new_logo_magick_wand();
        mw.trim_image(0.).unwrap();
    }

    #[test]
    fn test_magick_wand_unsharp_mask_image() {
        let mut mw = new_logo_magick_wand();
        mw.unsharp_mask_image(0., 0., 0., 0.).unwrap();
    }

    #[test]
    fn test_magick_wand_wave_image() {
        let mut mw = new_logo_magick_wand();
        let _ = mw.wave_image(0., 0.);
    }

    #[test]
    fn test_magick_wand_white_threshold_image() {
        let mut mw = new_logo_magick_wand();
        mw.white_threshold_image(&PixelWand::new()).unwrap();
    }

    #[test]
    fn test_magick_wand_write_image() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.write_image("").is_err());
    }

    #[test]
    fn test_magick_wand_write_image_blob() {
        let mut mw = new_logo_magick_wand();
        mw.write_image_blob().unwrap();
    }

    #[test]
    fn test_magick_wand_write_images() {
        let mut mw = new_logo_magick_wand();
        assert!(mw.write_images("", 0).is_err());
    }
}
