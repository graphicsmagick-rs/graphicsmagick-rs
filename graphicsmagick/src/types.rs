//! Binding of GraphicsMagick Types.
//!
//! <http://www.graphicsmagick.org/api/types.html>

pub use graphicsmagick_sys::{AffineMatrix, PixelPacket, PointInfo, Quantum};

use num_enum::{FromPrimitive, IntoPrimitive};

/// <http://www.graphicsmagick.org/api/types.html#channeltype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ChannelType {
    /// Default
    UndefinedChannel = graphicsmagick_sys::ChannelType_UndefinedChannel,
    /// RGB Red channel
    RedChannel = graphicsmagick_sys::ChannelType_RedChannel,
    /// CMYK Cyan channel
    CyanChannel = graphicsmagick_sys::ChannelType_CyanChannel,
    /// RGB Green channel
    GreenChannel = graphicsmagick_sys::ChannelType_GreenChannel,
    /// CMYK Magenta channel
    MagentaChannel = graphicsmagick_sys::ChannelType_MagentaChannel,
    /// RGB Blue channel
    BlueChannel = graphicsmagick_sys::ChannelType_BlueChannel,
    /// CMYK Yellow channel
    YellowChannel = graphicsmagick_sys::ChannelType_YellowChannel,
    /// Opacity channel
    OpacityChannel = graphicsmagick_sys::ChannelType_OpacityChannel,
    /// CMYK Black (K) channel
    BlackChannel = graphicsmagick_sys::ChannelType_BlackChannel,
    /// Same as Opacity channel (deprecated)
    MatteChannel = graphicsmagick_sys::ChannelType_MatteChannel,
    /// Color channels
    AllChannels = graphicsmagick_sys::ChannelType_AllChannels,
    /// Color channels represent an intensity
    GrayChannel = graphicsmagick_sys::ChannelType_GrayChannel,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// FilterTypes is used to adjust the filter algorithm used when resizing images.
/// Different filters experience varying degrees of success with various images and can
/// take significantly different amounts of processing time. GraphicsMagick uses the
/// LanczosFilter by default since this filter has been shown to provide the best results for
/// most images in a reasonable amount of time. Other filter types (e.g. TriangleFilter) may
/// execute much faster but may show artifacts when the image is re-sized or around diagonal
/// lines. The only way to be sure is to test the filter with sample images.
///
/// <http://www.graphicsmagick.org/api/types.html#filtertypes>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum FilterTypes {
    /// Unset value.
    UndefinedFilter = graphicsmagick_sys::FilterTypes_UndefinedFilter,
    /// Point Filter
    PointFilter = graphicsmagick_sys::FilterTypes_PointFilter,
    /// Box Filter
    BoxFilter = graphicsmagick_sys::FilterTypes_BoxFilter,
    /// Triangle Filter
    TriangleFilter = graphicsmagick_sys::FilterTypes_TriangleFilter,
    /// Hermite Filter
    HermiteFilter = graphicsmagick_sys::FilterTypes_HermiteFilter,
    /// Hanning Filter
    HanningFilter = graphicsmagick_sys::FilterTypes_HanningFilter,
    /// Hamming Filter
    HammingFilter = graphicsmagick_sys::FilterTypes_HammingFilter,
    /// Blackman Filter
    BlackmanFilter = graphicsmagick_sys::FilterTypes_BlackmanFilter,
    /// Gaussian Filter
    GaussianFilter = graphicsmagick_sys::FilterTypes_GaussianFilter,
    /// Quadratic Filter
    QuadraticFilter = graphicsmagick_sys::FilterTypes_QuadraticFilter,
    /// Cubic Filter
    CubicFilter = graphicsmagick_sys::FilterTypes_CubicFilter,
    /// Catrom Filter
    CatromFilter = graphicsmagick_sys::FilterTypes_CatromFilter,
    /// Mitchell Filter
    MitchellFilter = graphicsmagick_sys::FilterTypes_MitchellFilter,
    /// Lanczos Filter
    LanczosFilter = graphicsmagick_sys::FilterTypes_LanczosFilter,
    /// Bessel Filter
    BesselFilter = graphicsmagick_sys::FilterTypes_BesselFilter,
    /// Sinc Filter
    SincFilter = graphicsmagick_sys::FilterTypes_SincFilter,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// CompositeOperator is used to select the image composition algorithm used to compose
/// a composite image with an image. By default, each of the composite image pixels are
/// replaced by the corresponding image tile pixel. Specify CompositeOperator to select
/// a different algorithm.
///
/// The image compositor requires a matte, or alpha channel in the image for some operations.
/// This extra channel usually defines a mask which represents a sort of a cookie-cutter for
/// the image. This is the case when matte is 255 (full coverage) for pixels inside the shape,
/// zero outside, and between zero and 255 on the boundary. For certain operations, if image
/// does not have a matte channel, it is initialized with 0 for any pixel matching in color
/// to pixel location (0,0), otherwise 255 (to work properly borderWidth must be 0).
///
/// <http://www.graphicsmagick.org/api/types.html#compositeoperator>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum CompositeOperator {
    /// Unset value.
    UndefinedCompositeOp = graphicsmagick_sys::CompositeOperator_UndefinedCompositeOp,
    /// The result is the union of the the two image shapes with the composite image obscuring
    /// image in the region of overlap.
    OverCompositeOp = graphicsmagick_sys::CompositeOperator_OverCompositeOp,
    /// The result is a simply composite image cut by the shape of image. None of the image
    /// data of image is included in the result.
    InCompositeOp = graphicsmagick_sys::CompositeOperator_InCompositeOp,
    /// The resulting image is composite image with the shape of image cut out.
    OutCompositeOp = graphicsmagick_sys::CompositeOperator_OutCompositeOp,
    /// The result is the same shape as image image, with composite image obscuring image
    /// there the image shapes overlap. Note that this differs from OverCompositeOp because
    /// the portion of composite image outside of image's shape does not appear in the result.
    AtopCompositeOp = graphicsmagick_sys::CompositeOperator_AtopCompositeOp,
    /// The result is the image data from both composite image and image that is outside
    /// the overlap region. The overlap region will be blank.
    XorCompositeOp = graphicsmagick_sys::CompositeOperator_XorCompositeOp,
    /// The result is just the sum of the image data. Output values are cropped to
    /// 255 (no over=low). This operation is independent of the matte channels.
    PlusCompositeOp = graphicsmagick_sys::CompositeOperator_PlusCompositeOp,
    /// The result of composite image - image, with overflow cropped to zero.
    /// The matte chanel is ignored (set to 255, full coverage).
    MinusCompositeOp = graphicsmagick_sys::CompositeOperator_MinusCompositeOp,
    /// The result of composite image + image, with overflow wrapping around (mod 256).
    AddCompositeOp = graphicsmagick_sys::CompositeOperator_AddCompositeOp,
    /// The result of composite image - image, with underflow wrapping around (mod 256).
    /// The add and subtract operators can be used to perform reversible transformations.
    SubtractCompositeOp = graphicsmagick_sys::CompositeOperator_SubtractCompositeOp,
    /// The result of abs(composite image - image). This is useful for comparing two very
    /// similar images.
    DifferenceCompositeOp = graphicsmagick_sys::CompositeOperator_DifferenceCompositeOp,
    ///
    MultiplyCompositeOp = graphicsmagick_sys::CompositeOperator_MultiplyCompositeOp,
    /// The result image shaded by composite image.
    BumpmapCompositeOp = graphicsmagick_sys::CompositeOperator_BumpmapCompositeOp,
    /// The resulting image is image replaced with composite image. Here the matte
    /// information is ignored.
    CopyCompositeOp = graphicsmagick_sys::CompositeOperator_CopyCompositeOp,
    /// The resulting image is the red layer in image replaced with the red layer in
    /// composite image. The other layers are copied untouched.
    CopyRedCompositeOp = graphicsmagick_sys::CompositeOperator_CopyRedCompositeOp,
    /// The resulting image is the green layer in image replaced with the green layer
    /// in composite image. The other layers are copied untouched.
    CopyGreenCompositeOp = graphicsmagick_sys::CompositeOperator_CopyGreenCompositeOp,
    /// The resulting image is the blue layer in image replaced with the blue layer in
    /// composite image. The other layers are copied untouched.
    CopyBlueCompositeOp = graphicsmagick_sys::CompositeOperator_CopyBlueCompositeOp,
    /// The resulting image is the matte layer in image replaced with the matte layer
    /// in composite image. The other layers are copied untouched.
    CopyOpacityCompositeOp = graphicsmagick_sys::CompositeOperator_CopyOpacityCompositeOp,
    /// Pixels in the region are set to Transparent.
    ClearCompositeOp = graphicsmagick_sys::CompositeOperator_ClearCompositeOp,
    ///
    DissolveCompositeOp = graphicsmagick_sys::CompositeOperator_DissolveCompositeOp,
    ///
    DisplaceCompositeOp = graphicsmagick_sys::CompositeOperator_DisplaceCompositeOp,
    /// Modulate brightness in HSL space.
    ModulateCompositeOp = graphicsmagick_sys::CompositeOperator_ModulateCompositeOp,
    ///
    ThresholdCompositeOp = graphicsmagick_sys::CompositeOperator_ThresholdCompositeOp,
    /// Do nothing at all.
    NoCompositeOp = graphicsmagick_sys::CompositeOperator_NoCompositeOp,
    ///
    DarkenCompositeOp = graphicsmagick_sys::CompositeOperator_DarkenCompositeOp,
    ///
    LightenCompositeOp = graphicsmagick_sys::CompositeOperator_LightenCompositeOp,
    /// Copy Hue channel (from HSL colorspace).
    HueCompositeOp = graphicsmagick_sys::CompositeOperator_HueCompositeOp,
    /// Copy Saturation channel (from HSL colorspace).
    SaturateCompositeOp = graphicsmagick_sys::CompositeOperator_SaturateCompositeOp,
    /// Copy Hue and Saturation channels (from HSL colorspace).
    ColorizeCompositeOp = graphicsmagick_sys::CompositeOperator_ColorizeCompositeOp,
    /// Copy Brightness channel (from HSL colorspace).
    LuminizeCompositeOp = graphicsmagick_sys::CompositeOperator_LuminizeCompositeOp,
    /// [Not yet implemented]
    ScreenCompositeOp = graphicsmagick_sys::CompositeOperator_ScreenCompositeOp,
    /// [Not yet implemented]
    OverlayCompositeOp = graphicsmagick_sys::CompositeOperator_OverlayCompositeOp,
    /// Copy the Cyan channel.
    CopyCyanCompositeOp = graphicsmagick_sys::CompositeOperator_CopyCyanCompositeOp,
    /// Copy the Magenta channel.
    CopyMagentaCompositeOp = graphicsmagick_sys::CompositeOperator_CopyMagentaCompositeOp,
    /// Copy the Yellow channel.
    CopyYellowCompositeOp = graphicsmagick_sys::CompositeOperator_CopyYellowCompositeOp,
    /// Copy the Black channel.
    CopyBlackCompositeOp = graphicsmagick_sys::CompositeOperator_CopyBlackCompositeOp,
    ///
    DivideCompositeOp = graphicsmagick_sys::CompositeOperator_DivideCompositeOp,
    ///
    HardLightCompositeOp = graphicsmagick_sys::CompositeOperator_HardLightCompositeOp,
    ///
    ExclusionCompositeOp = graphicsmagick_sys::CompositeOperator_ExclusionCompositeOp,
    ///
    ColorDodgeCompositeOp = graphicsmagick_sys::CompositeOperator_ColorDodgeCompositeOp,
    ///
    ColorBurnCompositeOp = graphicsmagick_sys::CompositeOperator_ColorBurnCompositeOp,
    ///
    SoftLightCompositeOp = graphicsmagick_sys::CompositeOperator_SoftLightCompositeOp,
    ///
    LinearBurnCompositeOp = graphicsmagick_sys::CompositeOperator_LinearBurnCompositeOp,
    ///
    LinearDodgeCompositeOp = graphicsmagick_sys::CompositeOperator_LinearDodgeCompositeOp,
    ///
    LinearLightCompositeOp = graphicsmagick_sys::CompositeOperator_LinearLightCompositeOp,
    ///
    VividLightCompositeOp = graphicsmagick_sys::CompositeOperator_VividLightCompositeOp,
    ///
    PinLightCompositeOp = graphicsmagick_sys::CompositeOperator_PinLightCompositeOp,
    ///
    HardMixCompositeOp = graphicsmagick_sys::CompositeOperator_HardMixCompositeOp,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// NoiseType is used as an argument to select the type of noise to be added to the image.
///
/// <http://www.graphicsmagick.org/api/types.html#noisetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum NoiseType {
    /// Uniform noise
    UniformNoise = graphicsmagick_sys::NoiseType_UniformNoise,
    /// Gaussian noise
    GaussianNoise = graphicsmagick_sys::NoiseType_GaussianNoise,
    /// Multiplicative Gaussian noise
    MultiplicativeGaussianNoise = graphicsmagick_sys::NoiseType_MultiplicativeGaussianNoise,
    /// Impulse noise
    ImpulseNoise = graphicsmagick_sys::NoiseType_ImpulseNoise,
    /// Laplacian noise
    LaplacianNoise = graphicsmagick_sys::NoiseType_LaplacianNoise,
    /// Poisson noise
    PoissonNoise = graphicsmagick_sys::NoiseType_PoissonNoise,
    /// Random noise
    RandomNoise = graphicsmagick_sys::NoiseType_RandomNoise,
    /// Undefined noise
    UndefinedNoise = graphicsmagick_sys::NoiseType_UndefinedNoise,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// OrientationType specifies the orientation of the image. Useful for when the image is produced via a different ordinate system, the camera was turned on its side, or the page was scanned sideways.
///
/// <http://www.graphicsmagick.org/api/types.html#orientationtype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum OrientationType {
    ///
    UndefinedOrientation = graphicsmagick_sys::OrientationType_UndefinedOrientation,
    ///
    TopLeftOrientation = graphicsmagick_sys::OrientationType_TopLeftOrientation,
    ///
    TopRightOrientation = graphicsmagick_sys::OrientationType_TopRightOrientation,
    ///
    BottomRightOrientation = graphicsmagick_sys::OrientationType_BottomRightOrientation,
    ///
    BottomLeftOrientation = graphicsmagick_sys::OrientationType_BottomLeftOrientation,
    ///
    LeftTopOrientation = graphicsmagick_sys::OrientationType_LeftTopOrientation,
    ///
    RightTopOrientation = graphicsmagick_sys::OrientationType_RightTopOrientation,
    ///
    RightBottomOrientation = graphicsmagick_sys::OrientationType_RightBottomOrientation,
    ///
    LeftBottomOrientation = graphicsmagick_sys::OrientationType_LeftBottomOrientation,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// Pixel error metrics
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum MetricType {
    ///
    MeanAbsoluteErrorMetric = graphicsmagick_sys::MetricType_MeanAbsoluteErrorMetric,
    ///
    MeanSquaredErrorMetric = graphicsmagick_sys::MetricType_MeanSquaredErrorMetric,
    ///
    PeakAbsoluteErrorMetric = graphicsmagick_sys::MetricType_PeakAbsoluteErrorMetric,
    ///
    PeakSignalToNoiseRatioMetric = graphicsmagick_sys::MetricType_PeakSignalToNoiseRatioMetric,
    ///
    RootMeanSquaredErrorMetric = graphicsmagick_sys::MetricType_RootMeanSquaredErrorMetric,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// The ColorspaceType enumeration is used to specify the colorspace that quantization
/// (color reduction and mapping) is done under or to specify the colorspace when encoding
/// an output image. Colorspaces are ways of describing colors to fit the requirements of a
/// particular application (e.g. Television, offset printing, color monitors). Color reduction,
/// by default, takes place in the RGBColorspace. Empirical evidence suggests that distances
/// in color spaces such as YUVColorspace or YIQColorspace correspond to perceptual color
/// differences more closely han do distances in RGB space. These color spaces may give better
/// results when color reducing an image. Refer to quantize for more details.
/// When encoding an output image, the colorspaces RGBColorspace, CMYKColorspace, and GRAYColorspace may be specified. The CMYKColorspace option is only applicable when writing TIFF, JPEG, and Adobe Photoshop bitmap (PSD) files.
///
/// <http://www.graphicsmagick.org/api/types.html#colorspacetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ColorspaceType {
    /// Unset value.
    UndefinedColorspace = graphicsmagick_sys::ColorspaceType_UndefinedColorspace,
    /// Red, Green, Blue colorspace.
    RGBColorspace = graphicsmagick_sys::ColorspaceType_RGBColorspace,
    /// Similar to Luma (Y) according to ITU-R 601
    GRAYColorspace = graphicsmagick_sys::ColorspaceType_GRAYColorspace,
    /// Similar to Luma (Y) according to ITU-R 601
    TransparentColorspace = graphicsmagick_sys::ColorspaceType_TransparentColorspace,
    ///
    OHTAColorspace = graphicsmagick_sys::ColorspaceType_OHTAColorspace,
    /// CIE XYZ
    XYZColorspace = graphicsmagick_sys::ColorspaceType_XYZColorspace,
    /// Kodak PhotoCD PhotoYCC
    YCCColorspace = graphicsmagick_sys::ColorspaceType_YCCColorspace,
    ///
    YIQColorspace = graphicsmagick_sys::ColorspaceType_YIQColorspace,
    ///
    YPbPrColorspace = graphicsmagick_sys::ColorspaceType_YPbPrColorspace,
    /// YUV colorspace as used for computer video.
    YUVColorspace = graphicsmagick_sys::ColorspaceType_YUVColorspace,
    /// Cyan, Magenta, Yellow, Black colorspace.
    CMYKColorspace = graphicsmagick_sys::ColorspaceType_CMYKColorspace,
    /// Kodak PhotoCD sRGB
    SRGBColorspace = graphicsmagick_sys::ColorspaceType_sRGBColorspace,
    /// Hue, saturation, luminosity
    HSLColorspace = graphicsmagick_sys::ColorspaceType_HSLColorspace,
    /// Hue, whiteness, blackness
    HWBColorspace = graphicsmagick_sys::ColorspaceType_HWBColorspace,
    /// ITU LAB
    LABColorspace = graphicsmagick_sys::ColorspaceType_LABColorspace,
    /// RGB data with Cineon Log scaling, 2.048 density range
    CineonLogRGBColorspace = graphicsmagick_sys::ColorspaceType_CineonLogRGBColorspace,
    /// Luma (Y) according to ITU-R 601
    Rec601LumaColorspace = graphicsmagick_sys::ColorspaceType_Rec601LumaColorspace,
    /// YCbCr according to ITU-R 601
    Rec601YCbCrColorspace = graphicsmagick_sys::ColorspaceType_Rec601YCbCrColorspace,
    /// Luma (Y) according to ITU-R 709
    Rec709LumaColorspace = graphicsmagick_sys::ColorspaceType_Rec709LumaColorspace,
    /// YCbCr according to ITU-R 709
    Rec709YCbCrColorspace = graphicsmagick_sys::ColorspaceType_Rec709YCbCrColorspace,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// CompressionType is used to express the desired compression type when encoding an image.
/// Be aware that most image types only support a sub-set of the available compression types.
/// If the compression type specified is incompatable with the image, GraphicsMagick selects
/// a compression type compatable with the image type, which might be no compression at all.
///
/// <http://www.graphicsmagick.org/api/types.html#compressiontype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum CompressionType {
    /// Unset value.
    UndefinedCompression = graphicsmagick_sys::CompressionType_UndefinedCompression,
    /// No compression
    NoCompression = graphicsmagick_sys::CompressionType_NoCompression,
    /// BZip (Burrows-Wheeler block-sorting text compression algorithm and Huffman coding)
    /// as used by bzip2 utilities
    BZipCompression = graphicsmagick_sys::CompressionType_BZipCompression,
    /// CCITT Group 3 FAX compression.
    ///
    /// Alias: Group3Compression
    FaxCompression = graphicsmagick_sys::CompressionType_FaxCompression,
    /// CCITT Group 4 FAX compression (used only for TIFF)
    Group4Compression = graphicsmagick_sys::CompressionType_Group4Compression,
    /// JPEG compression
    JPEGCompression = graphicsmagick_sys::CompressionType_JPEGCompression,
    /// Lossless JPEG compression
    LosslessJPEGCompression = graphicsmagick_sys::CompressionType_LosslessJPEGCompression,
    /// Lempel-Ziv-Welch (LZW) compression (caution, patented by Unisys)
    LZWCompression = graphicsmagick_sys::CompressionType_LZWCompression,
    /// Run-Length encoded (RLE) compression
    RLECompression = graphicsmagick_sys::CompressionType_RLECompression,
    /// Lempel-Ziv compression (LZ77) as used in PKZIP and GNU gzip.
    ZipCompression = graphicsmagick_sys::CompressionType_ZipCompression,
    /// LZMA - Lempel-Ziv-Markov chain algorithm
    LZMACompression = graphicsmagick_sys::CompressionType_LZMACompression,
    /// JPEG 2000 - ISO/IEC std 15444-1
    JPEG2000Compression = graphicsmagick_sys::CompressionType_JPEG2000Compression,
    /// JBIG v1 - ISO/IEC std 11544 / ITU-T rec T.82
    JBIG1Compression = graphicsmagick_sys::CompressionType_JBIG1Compression,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// DisposeType
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum DisposeType {
    ///
    UndefinedDispose = graphicsmagick_sys::DisposeType_UndefinedDispose,
    ///
    NoneDispose = graphicsmagick_sys::DisposeType_NoneDispose,
    ///
    BackgroundDispose = graphicsmagick_sys::DisposeType_BackgroundDispose,
    ///
    PreviousDispose = graphicsmagick_sys::DisposeType_PreviousDispose,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// GravityType specifies positioning of an object (e.g. text, image) within a bounding
/// region (e.g. an image). Gravity provides a convenient way to locate objects irrespective
/// of the size of the bounding region, in other words, you don't need to provide absolute
/// coordinates in order to position an object. A common default for gravity is
/// NorthWestGravity.
///
/// <http://www.graphicsmagick.org/api/types.html#gravitytype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum GravityType {
    /// Don't use gravity.
    ForgetGravity = graphicsmagick_sys::GravityType_ForgetGravity,
    /// Position object at top-left of region.
    NorthWestGravity = graphicsmagick_sys::GravityType_NorthWestGravity,
    /// Postiion object at top-center of region
    NorthGravity = graphicsmagick_sys::GravityType_NorthGravity,
    /// Position object at top-right of region
    NorthEastGravity = graphicsmagick_sys::GravityType_NorthEastGravity,
    /// Position object at left-center of region
    WestGravity = graphicsmagick_sys::GravityType_WestGravity,
    /// Position object at center of region
    CenterGravity = graphicsmagick_sys::GravityType_CenterGravity,
    /// Position object at right-center of region
    EastGravity = graphicsmagick_sys::GravityType_EastGravity,
    /// Position object at left-bottom of region
    SouthWestGravity = graphicsmagick_sys::GravityType_SouthWestGravity,
    /// Position object at bottom-center of region
    SouthGravity = graphicsmagick_sys::GravityType_SouthGravity,
    /// Position object at bottom-right of region
    SouthEastGravity = graphicsmagick_sys::GravityType_SouthEastGravity,
    ///
    StaticGravity = graphicsmagick_sys::GravityType_StaticGravity,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// InterlaceType specifies the ordering of the red, green, and blue pixel information in
/// the image. Interlacing is usually used to make image information available to the user
/// faster by taking advantage of the space vs time tradeoff. For example, interlacing allows
/// images on the Web to be recognizable sooner and satellite images to accumulate/render with
/// image resolution increasing over time.
///
/// Use LineInterlace or PlaneInterlace to create an interlaced GIF or progressive JPEG image.
///
/// <http://www.graphicsmagick.org/api/types.html#interlacetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum InterlaceType {
    /// Unset value.
    UndefinedInterlace = graphicsmagick_sys::InterlaceType_UndefinedInterlace,
    /// Don't interlace image (RGBRGBRGBRGBRGBRGB...)
    NoInterlace = graphicsmagick_sys::InterlaceType_NoInterlace,
    /// Use scanline interlacing (RRR...GGG...BBB...RRR...GGG...BBB...)
    LineInterlace = graphicsmagick_sys::InterlaceType_LineInterlace,
    /// Use plane interlacing (RRRRRR...GGGGGG...BBBBBB...)
    PlaneInterlace = graphicsmagick_sys::InterlaceType_PlaneInterlace,
    /// Similar to plane interlaing except that the different planes are saved to
    /// individual files (e.g. image.R, image.G, and image.B)
    PartitionInterlace = graphicsmagick_sys::InterlaceType_PartitionInterlace,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#storagetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum StorageType {
    ///
    CharPixel = graphicsmagick_sys::StorageType_CharPixel,
    ///
    ShortPixel = graphicsmagick_sys::StorageType_ShortPixel,
    ///
    IntegerPixel = graphicsmagick_sys::StorageType_IntegerPixel,
    ///
    LongPixel = graphicsmagick_sys::StorageType_LongPixel,
    ///
    FloatPixel = graphicsmagick_sys::StorageType_FloatPixel,
    ///
    DoublePixel = graphicsmagick_sys::StorageType_DoublePixel,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// Rendering intent is a concept defined by ICC Spec ICC.1:1998-09, "File Format for Color
/// Profiles". GraphicsMagick uses RenderingIntent in order to support ICC Color Profiles.
///
/// From the specification: "Rendering intent specifies the style of reproduction to be used
/// during the evaluation of this profile in a sequence of profiles. It applies specifically
/// to that profile in the sequence and not to the entire sequence. Typically, the user or
/// application will set the rendering intent dynamically at runtime or embedding time."
///
/// <http://www.graphicsmagick.org/api/types.html#renderingintent>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum RenderingIntent {
    /// Unset value.
    UndefinedIntent = graphicsmagick_sys::RenderingIntent_UndefinedIntent,
    /// A rendering intent that specifies the saturation of the pixels in the image is preserved
    /// perhaps at the expense of accuracy in hue and lightness.
    SaturationIntent = graphicsmagick_sys::RenderingIntent_SaturationIntent,
    /// A rendering intent that specifies the full gamut of the image is compressed or expanded
    /// to fill the gamut of the destination device. Gray balance is preserved but colorimetric
    /// accuracy might not be preserved.
    PerceptualIntent = graphicsmagick_sys::RenderingIntent_PerceptualIntent,
    /// Absolute colorimetric
    AbsoluteIntent = graphicsmagick_sys::RenderingIntent_AbsoluteIntent,
    /// Relative colorimetric
    RelativeIntent = graphicsmagick_sys::RenderingIntent_RelativeIntent,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// ImageType indicates the type classification of the image.
///
/// <http://www.graphicsmagick.org/api/types.html#imagetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ImageType {
    /// Unset value.
    UndefinedType = graphicsmagick_sys::ImageType_UndefinedType,
    /// Monochrome image
    BilevelType = graphicsmagick_sys::ImageType_BilevelType,
    /// Grayscale image
    GrayscaleType = graphicsmagick_sys::ImageType_GrayscaleType,
    /// Grayscale image with opacity
    GrayscaleMatteType = graphicsmagick_sys::ImageType_GrayscaleMatteType,
    /// Indexed color (palette) image
    PaletteType = graphicsmagick_sys::ImageType_PaletteType,
    /// Indexed color (palette) image with opacity
    PaletteMatteType = graphicsmagick_sys::ImageType_PaletteMatteType,
    /// Truecolor image
    TrueColorType = graphicsmagick_sys::ImageType_TrueColorType,
    /// Truecolor image with opacity
    TrueColorMatteType = graphicsmagick_sys::ImageType_TrueColorMatteType,
    /// Cyan/Yellow/Magenta/Black (CYMK) image
    ColorSeparationType = graphicsmagick_sys::ImageType_ColorSeparationType,
    ///
    ColorSeparationMatteType = graphicsmagick_sys::ImageType_ColorSeparationMatteType,
    ///
    OptimizeType = graphicsmagick_sys::ImageType_OptimizeType,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// By default, GraphicsMagick defines resolutions in pixels per inch. ResolutionType provides a means to adjust this.
/// <http://www.graphicsmagick.org/api/types.html#resolutiontype>
///
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ResolutionType {
    /// Unset value.
    UndefinedResolution = graphicsmagick_sys::ResolutionType_UndefinedResolution,
    /// Density specifications are specified in units of pixels per inch (english units).
    PixelsPerInchResolution = graphicsmagick_sys::ResolutionType_PixelsPerInchResolution,
    /// Density specifications are specified in units of pixels per centimeter (metric units).
    PixelsPerCentimeterResolution =
        graphicsmagick_sys::ResolutionType_PixelsPerCentimeterResolution,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// Enum declaractions.
///
/// <http://www.graphicsmagick.org/api/types.html#virtualpixelmethod>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum VirtualPixelMethod {
    ///
    UndefinedVirtualPixelMethod =
        graphicsmagick_sys::VirtualPixelMethod_UndefinedVirtualPixelMethod,
    ///
    ConstantVirtualPixelMethod = graphicsmagick_sys::VirtualPixelMethod_ConstantVirtualPixelMethod,
    ///
    EdgeVirtualPixelMethod = graphicsmagick_sys::VirtualPixelMethod_EdgeVirtualPixelMethod,
    ///
    MirrorVirtualPixelMethod = graphicsmagick_sys::VirtualPixelMethod_MirrorVirtualPixelMethod,
    ///
    TileVirtualPixelMethod = graphicsmagick_sys::VirtualPixelMethod_TileVirtualPixelMethod,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// Typedef declarations.
///
/// <http://www.graphicsmagick.org/api/types.html#resourcetype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ResourceType {
    /// Undefined value
    UndefinedResource = graphicsmagick_sys::ResourceType_UndefinedResource,
    /// Pixel cache total disk space (Gigabytes)
    DiskResource = graphicsmagick_sys::ResourceType_DiskResource,
    /// Pixel cache number of open files (Files)
    FileResource = graphicsmagick_sys::ResourceType_FileResource,
    /// Pixel cache total file memory-mapping (Megabytes)
    MapResource = graphicsmagick_sys::ResourceType_MapResource,
    /// Maximum pixel cache heap memory allocations (Megabytes)
    MemoryResource = graphicsmagick_sys::ResourceType_MemoryResource,
    /// Maximum number of pixels in single image (Pixels)
    PixelsResource = graphicsmagick_sys::ResourceType_PixelsResource,
    /// Maximum number of worker threads
    ThreadsResource = graphicsmagick_sys::ResourceType_ThreadsResource,
    #[cfg(feature = "v1_3_21")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_21")))]
    /// Maximum pixel width of an image (Pixels)
    WidthResource = graphicsmagick_sys::ResourceType_WidthResource,
    #[cfg(feature = "v1_3_21")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_3_21")))]
    /// Maximum pixel height of an image (Pixels)
    HeightResource = graphicsmagick_sys::ResourceType_HeightResource,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// MontageMode
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum MontageMode {
    ///
    UndefinedMode = graphicsmagick_sys::MontageMode_UndefinedMode,
    ///
    FrameMode = graphicsmagick_sys::MontageMode_FrameMode,
    ///
    UnframeMode = graphicsmagick_sys::MontageMode_UnframeMode,
    ///
    ConcatenateMode = graphicsmagick_sys::MontageMode_ConcatenateMode,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// PreviewType
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum PreviewType {
    ///
    UndefinedPreview = graphicsmagick_sys::PreviewType_UndefinedPreview,
    ///
    RotatePreview = graphicsmagick_sys::PreviewType_RotatePreview,
    ///
    ShearPreview = graphicsmagick_sys::PreviewType_ShearPreview,
    ///
    RollPreview = graphicsmagick_sys::PreviewType_RollPreview,
    ///
    HuePreview = graphicsmagick_sys::PreviewType_HuePreview,
    ///
    SaturationPreview = graphicsmagick_sys::PreviewType_SaturationPreview,
    ///
    BrightnessPreview = graphicsmagick_sys::PreviewType_BrightnessPreview,
    ///
    GammaPreview = graphicsmagick_sys::PreviewType_GammaPreview,
    ///
    SpiffPreview = graphicsmagick_sys::PreviewType_SpiffPreview,
    ///
    DullPreview = graphicsmagick_sys::PreviewType_DullPreview,
    ///
    GrayscalePreview = graphicsmagick_sys::PreviewType_GrayscalePreview,
    ///
    QuantizePreview = graphicsmagick_sys::PreviewType_QuantizePreview,
    ///
    DespecklePreview = graphicsmagick_sys::PreviewType_DespecklePreview,
    ///
    ReduceNoisePreview = graphicsmagick_sys::PreviewType_ReduceNoisePreview,
    ///
    AddNoisePreview = graphicsmagick_sys::PreviewType_AddNoisePreview,
    ///
    SharpenPreview = graphicsmagick_sys::PreviewType_SharpenPreview,
    ///
    BlurPreview = graphicsmagick_sys::PreviewType_BlurPreview,
    ///
    ThresholdPreview = graphicsmagick_sys::PreviewType_ThresholdPreview,
    ///
    EdgeDetectPreview = graphicsmagick_sys::PreviewType_EdgeDetectPreview,
    ///
    SpreadPreview = graphicsmagick_sys::PreviewType_SpreadPreview,
    ///
    SolarizePreview = graphicsmagick_sys::PreviewType_SolarizePreview,
    ///
    ShadePreview = graphicsmagick_sys::PreviewType_ShadePreview,
    ///
    RaisePreview = graphicsmagick_sys::PreviewType_RaisePreview,
    ///
    SegmentPreview = graphicsmagick_sys::PreviewType_SegmentPreview,
    ///
    SwirlPreview = graphicsmagick_sys::PreviewType_SwirlPreview,
    ///
    ImplodePreview = graphicsmagick_sys::PreviewType_ImplodePreview,
    ///
    WavePreview = graphicsmagick_sys::PreviewType_WavePreview,
    ///
    OilPaintPreview = graphicsmagick_sys::PreviewType_OilPaintPreview,
    ///
    CharcoalDrawingPreview = graphicsmagick_sys::PreviewType_CharcoalDrawingPreview,
    ///
    JPEGPreview = graphicsmagick_sys::PreviewType_JPEGPreview,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#fillrule>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum FillRule {
    ///
    UndefinedRule = graphicsmagick_sys::FillRule_UndefinedRule,
    ///
    EvenOddRule = graphicsmagick_sys::FillRule_EvenOddRule,
    ///
    NonZeroRule = graphicsmagick_sys::FillRule_NonZeroRule,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#clippathunits>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum ClipPathUnits {
    ///
    UserSpace = graphicsmagick_sys::ClipPathUnits_UserSpace,
    ///
    UserSpaceOnUse = graphicsmagick_sys::ClipPathUnits_UserSpaceOnUse,
    ///
    ObjectBoundingBox = graphicsmagick_sys::ClipPathUnits_ObjectBoundingBox,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// PaintMethod specifies how pixel colors are to be replaced in the image. It is used to select
/// the pixel-filling algorithm employed.
///
/// <http://www.graphicsmagick.org/api/types.html#paintmethod>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum PaintMethod {
    /// Replace pixel color at point.
    PointMethod = graphicsmagick_sys::PaintMethod_PointMethod,
    /// Replace color for all image pixels matching color at point.
    ReplaceMethod = graphicsmagick_sys::PaintMethod_ReplaceMethod,
    /// Replace color for pixels surrounding point until encountering pixel that fails to match color at point.
    FloodfillMethod = graphicsmagick_sys::PaintMethod_FloodfillMethod,
    /// Replace color for pixels surrounding point until encountering pixels matching border color.
    FillToBorderMethod = graphicsmagick_sys::PaintMethod_FillToBorderMethod,
    /// Replace colors for all pixels in image with pen color.
    ResetMethod = graphicsmagick_sys::PaintMethod_ResetMethod,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#stretchtype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum StretchType {
    ///
    NormalStretch = graphicsmagick_sys::StretchType_NormalStretch,
    ///
    UltraCondensedStretch = graphicsmagick_sys::StretchType_UltraCondensedStretch,
    ///
    ExtraCondensedStretch = graphicsmagick_sys::StretchType_ExtraCondensedStretch,
    ///
    CondensedStretch = graphicsmagick_sys::StretchType_CondensedStretch,
    ///
    SemiCondensedStretch = graphicsmagick_sys::StretchType_SemiCondensedStretch,
    ///
    SemiExpandedStretch = graphicsmagick_sys::StretchType_SemiExpandedStretch,
    ///
    ExpandedStretch = graphicsmagick_sys::StretchType_ExpandedStretch,
    ///
    ExtraExpandedStretch = graphicsmagick_sys::StretchType_ExtraExpandedStretch,
    ///
    UltraExpandedStretch = graphicsmagick_sys::StretchType_UltraExpandedStretch,
    ///
    AnyStretch = graphicsmagick_sys::StretchType_AnyStretch,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#styletype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum StyleType {
    ///
    NormalStyle = graphicsmagick_sys::StyleType_NormalStyle,
    ///
    ItalicStyle = graphicsmagick_sys::StyleType_ItalicStyle,
    ///
    ObliqueStyle = graphicsmagick_sys::StyleType_ObliqueStyle,
    ///
    AnyStyle = graphicsmagick_sys::StyleType_AnyStyle,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// LineCap
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum LineCap {
    ///
    UndefinedCap = graphicsmagick_sys::LineCap_UndefinedCap,
    ///
    ButtCap = graphicsmagick_sys::LineCap_ButtCap,
    ///
    RoundCap = graphicsmagick_sys::LineCap_RoundCap,
    ///
    SquareCap = graphicsmagick_sys::LineCap_SquareCap,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// LineJoin
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum LineJoin {
    ///
    UndefinedJoin = graphicsmagick_sys::LineJoin_UndefinedJoin,
    ///
    MiterJoin = graphicsmagick_sys::LineJoin_MiterJoin,
    ///
    RoundJoin = graphicsmagick_sys::LineJoin_RoundJoin,
    ///
    BevelJoin = graphicsmagick_sys::LineJoin_BevelJoin,

    #[num_enum(default)]
    Unknown = u32::MAX,
}

/// <http://www.graphicsmagick.org/api/types.html#decorationtype>
#[derive(Debug, Eq, PartialEq, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u32)]
pub enum DecorationType {
    ///
    NoDecoration = graphicsmagick_sys::DecorationType_NoDecoration,
    ///
    UnderlineDecoration = graphicsmagick_sys::DecorationType_UnderlineDecoration,
    ///
    OverlineDecoration = graphicsmagick_sys::DecorationType_OverlineDecoration,
    ///
    LineThroughDecoration = graphicsmagick_sys::DecorationType_LineThroughDecoration,

    #[num_enum(default)]
    Unknown = u32::MAX,
}
