types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#channeltype](http://www.graphicsmagick.org/api/types.html#channeltype)
    ChannelType;

    /// Default
    (graphicsmagick_sys::ChannelType_UndefinedChannel, UndefinedChannel);
    /// RGB Red channel
    (graphicsmagick_sys::ChannelType_RedChannel, RedChannel);
    /// CMYK Cyan channel
    (graphicsmagick_sys::ChannelType_CyanChannel, CyanChannel);
    /// RGB Green channel
    (graphicsmagick_sys::ChannelType_GreenChannel, GreenChannel);
    /// CMYK Magenta channel
    (graphicsmagick_sys::ChannelType_MagentaChannel, MagentaChannel);
    /// RGB Blue channel
    (graphicsmagick_sys::ChannelType_BlueChannel, BlueChannel);
    /// CMYK Yellow channel
    (graphicsmagick_sys::ChannelType_YellowChannel, YellowChannel);
    /// Opacity channel
    (graphicsmagick_sys::ChannelType_OpacityChannel, OpacityChannel);
    /// CMYK Black (K) channel
    (graphicsmagick_sys::ChannelType_BlackChannel, BlackChannel);
    /// Same as Opacity channel (deprecated)
    (graphicsmagick_sys::ChannelType_MatteChannel, MatteChannel);
    /// Color channels
    (graphicsmagick_sys::ChannelType_AllChannels, AllChannels);
    /// Color channels represent an intensity
    (graphicsmagick_sys::ChannelType_GrayChannel, GrayChannel);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#filtertypes](http://www.graphicsmagick.org/api/types.html#filtertypes)
    ///
    /// FilterTypes is used to adjust the filter algorithm used when resizing images.
    /// Different filters experience varying degrees of success with various images and can
    /// take significantly different amounts of processing time. GraphicsMagick uses the
    /// LanczosFilter by default since this filter has been shown to provide the best results for
    /// most images in a reasonable amount of time. Other filter types (e.g. TriangleFilter) may
    /// execute much faster but may show artifacts when the image is re-sized or around diagonal
    /// lines. The only way to be sure is to test the filter with sample images.
    FilterTypes;

    /// Unset value.
    (graphicsmagick_sys::FilterTypes_UndefinedFilter, UndefinedFilter);
    /// Point Filter
    (graphicsmagick_sys::FilterTypes_PointFilter, PointFilter);
    /// Box Filter
    (graphicsmagick_sys::FilterTypes_BoxFilter, BoxFilter);
    /// Triangle Filter
    (graphicsmagick_sys::FilterTypes_TriangleFilter, TriangleFilter);
    /// Hermite Filter
    (graphicsmagick_sys::FilterTypes_HermiteFilter, HermiteFilter);
    /// Hanning Filter
    (graphicsmagick_sys::FilterTypes_HanningFilter, HanningFilter);
    /// Hamming Filter
    (graphicsmagick_sys::FilterTypes_HammingFilter, HammingFilter);
    /// Blackman Filter
    (graphicsmagick_sys::FilterTypes_BlackmanFilter, BlackmanFilter);
    /// Gaussian Filter
    (graphicsmagick_sys::FilterTypes_GaussianFilter, GaussianFilter);
    /// Quadratic Filter
    (graphicsmagick_sys::FilterTypes_QuadraticFilter, QuadraticFilter);
    /// Cubic Filter
    (graphicsmagick_sys::FilterTypes_CubicFilter, CubicFilter);
    /// Catrom Filter
    (graphicsmagick_sys::FilterTypes_CatromFilter, CatromFilter);
    /// Mitchell Filter
    (graphicsmagick_sys::FilterTypes_MitchellFilter, MitchellFilter);
    /// Lanczos Filter
    (graphicsmagick_sys::FilterTypes_LanczosFilter, LanczosFilter);
    /// Bessel Filter
    (graphicsmagick_sys::FilterTypes_BesselFilter, BesselFilter);
    /// Sinc Filter
    (graphicsmagick_sys::FilterTypes_SincFilter, SincFilter);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#compositeoperator](http://www.graphicsmagick.org/api/types.html#compositeoperator)
    ///
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
    CompositeOperator;

    /// Unset value.
    (graphicsmagick_sys::CompositeOperator_UndefinedCompositeOp, UndefinedCompositeOp);
    /// The result is the union of the the two image shapes with the composite image obscuring
    /// image in the region of overlap.
    (graphicsmagick_sys::CompositeOperator_OverCompositeOp, OverCompositeOp);
    /// The result is a simply composite image cut by the shape of image. None of the image
    /// data of image is included in the result.
    (graphicsmagick_sys::CompositeOperator_InCompositeOp, InCompositeOp);
    /// The resulting image is composite image with the shape of image cut out.
    (graphicsmagick_sys::CompositeOperator_OutCompositeOp, OutCompositeOp);
    /// The result is the same shape as image image, with composite image obscuring image
    /// there the image shapes overlap. Note that this differs from OverCompositeOp because
    /// the portion of composite image outside of image's shape does not appear in the result.
    (graphicsmagick_sys::CompositeOperator_AtopCompositeOp, AtopCompositeOp);
    /// The result is the image data from both composite image and image that is outside
    /// the overlap region. The overlap region will be blank.
    (graphicsmagick_sys::CompositeOperator_XorCompositeOp, XorCompositeOp);
    /// The result is just the sum of the image data. Output values are cropped to
    /// 255 (no overflow). This operation is independent of the matte channels.
    (graphicsmagick_sys::CompositeOperator_PlusCompositeOp, PlusCompositeOp);
    /// The result of composite image - image, with overflow cropped to zero.
    /// The matte chanel is ignored (set to 255, full coverage).
    (graphicsmagick_sys::CompositeOperator_MinusCompositeOp, MinusCompositeOp);
    /// The result of composite image + image, with overflow wrapping around (mod 256).
    (graphicsmagick_sys::CompositeOperator_AddCompositeOp, AddCompositeOp);
    /// The result of composite image - image, with underflow wrapping around (mod 256).
    /// The add and subtract operators can be used to perform reversible transformations.
    (graphicsmagick_sys::CompositeOperator_SubtractCompositeOp, SubtractCompositeOp);
    /// The result of abs(composite image - image). This is useful for comparing two very
    /// similar images.
    (graphicsmagick_sys::CompositeOperator_DifferenceCompositeOp, DifferenceCompositeOp);
    (graphicsmagick_sys::CompositeOperator_MultiplyCompositeOp, MultiplyCompositeOp);
    /// The result image shaded by composite image.
    (graphicsmagick_sys::CompositeOperator_BumpmapCompositeOp, BumpmapCompositeOp);
    /// The resulting image is image replaced with composite image. Here the matte
    /// information is ignored.
    (graphicsmagick_sys::CompositeOperator_CopyCompositeOp, CopyCompositeOp);
    /// The resulting image is the red layer in image replaced with the red layer in
    /// composite image. The other layers are copied untouched.
    (graphicsmagick_sys::CompositeOperator_CopyRedCompositeOp, CopyRedCompositeOp);
    /// The resulting image is the green layer in image replaced with the green layer
    /// in composite image. The other layers are copied untouched.
    (graphicsmagick_sys::CompositeOperator_CopyGreenCompositeOp, CopyGreenCompositeOp);
    /// The resulting image is the blue layer in image replaced with the blue layer in
    /// composite image. The other layers are copied untouched.
    (graphicsmagick_sys::CompositeOperator_CopyBlueCompositeOp, CopyBlueCompositeOp);
    /// The resulting image is the matte layer in image replaced with the matte layer
    /// in composite image. The other layers are copied untouched.
    (graphicsmagick_sys::CompositeOperator_CopyOpacityCompositeOp, CopyOpacityCompositeOp);
    /// Pixels in the region are set to Transparent.
    (graphicsmagick_sys::CompositeOperator_ClearCompositeOp, ClearCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DissolveCompositeOp, DissolveCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DisplaceCompositeOp, DisplaceCompositeOp);
    /// Modulate brightness in HSL space.
    (graphicsmagick_sys::CompositeOperator_ModulateCompositeOp, ModulateCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ThresholdCompositeOp, ThresholdCompositeOp);
    /// Do nothing at all.
    (graphicsmagick_sys::CompositeOperator_NoCompositeOp, NoCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DarkenCompositeOp, DarkenCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LightenCompositeOp, LightenCompositeOp);
    /// Copy Hue channel (from HSL colorspace).
    (graphicsmagick_sys::CompositeOperator_HueCompositeOp, HueCompositeOp);
    /// Copy Saturation channel (from HSL colorspace).
    (graphicsmagick_sys::CompositeOperator_SaturateCompositeOp, SaturateCompositeOp);
    /// Copy Hue and Saturation channels (from HSL colorspace).
    (graphicsmagick_sys::CompositeOperator_ColorizeCompositeOp, ColorizeCompositeOp);
    /// Copy Brightness channel (from HSL colorspace).
    (graphicsmagick_sys::CompositeOperator_LuminizeCompositeOp, LuminizeCompositeOp);
    /// [Not yet implemented]
    (graphicsmagick_sys::CompositeOperator_ScreenCompositeOp, ScreenCompositeOp);
    /// [Not yet implemented]
    (graphicsmagick_sys::CompositeOperator_OverlayCompositeOp, OverlayCompositeOp);
    /// Copy the Cyan channel.
    (graphicsmagick_sys::CompositeOperator_CopyCyanCompositeOp, CopyCyanCompositeOp);
    /// Copy the Magenta channel.
    (graphicsmagick_sys::CompositeOperator_CopyMagentaCompositeOp, CopyMagentaCompositeOp);
    /// Copy the Yellow channel.
    (graphicsmagick_sys::CompositeOperator_CopyYellowCompositeOp, CopyYellowCompositeOp);
    /// Copy the Black channel.
    (graphicsmagick_sys::CompositeOperator_CopyBlackCompositeOp, CopyBlackCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DivideCompositeOp, DivideCompositeOp);
    (graphicsmagick_sys::CompositeOperator_HardLightCompositeOp, HardLightCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ExclusionCompositeOp, ExclusionCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ColorDodgeCompositeOp, ColorDodgeCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ColorBurnCompositeOp, ColorBurnCompositeOp);
    (graphicsmagick_sys::CompositeOperator_SoftLightCompositeOp, SoftLightCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LinearBurnCompositeOp, LinearBurnCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LinearDodgeCompositeOp, LinearDodgeCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LinearLightCompositeOp, LinearLightCompositeOp);
    (graphicsmagick_sys::CompositeOperator_VividLightCompositeOp, VividLightCompositeOp);
    (graphicsmagick_sys::CompositeOperator_PinLightCompositeOp, PinLightCompositeOp);
    (graphicsmagick_sys::CompositeOperator_HardMixCompositeOp, HardMixCompositeOp);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#noisetype](http://www.graphicsmagick.org/api/types.html#noisetype)
    ///
    /// NoiseType is used as an argument to select the type of noise to be added to the image.
    NoiseType;

    /// Uniform noise
    (graphicsmagick_sys::NoiseType_UniformNoise, UniformNoise);
    /// Gaussian noise
    (graphicsmagick_sys::NoiseType_GaussianNoise, GaussianNoise);
    /// Multiplicative Gaussian noise
    (graphicsmagick_sys::NoiseType_MultiplicativeGaussianNoise, MultiplicativeGaussianNoise);
    /// Impulse noise
    (graphicsmagick_sys::NoiseType_ImpulseNoise, ImpulseNoise);
    /// Laplacian noise
    (graphicsmagick_sys::NoiseType_LaplacianNoise, LaplacianNoise);
    /// Poisson noise
    (graphicsmagick_sys::NoiseType_PoissonNoise, PoissonNoise);
    /// Random noise
    (graphicsmagick_sys::NoiseType_RandomNoise, RandomNoise);
    /// Undefined noise
    (graphicsmagick_sys::NoiseType_UndefinedNoise, UndefinedNoise);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#orientationtype](http://www.graphicsmagick.org/api/types.html#orientationtype)
    ///
    /// OrientationType specifies the orientation of the image. Useful for when the image is produced via a different ordinate system, the camera was turned on its side, or the page was scanned sideways.
    OrientationType;

    (graphicsmagick_sys::OrientationType_UndefinedOrientation, UndefinedOrientation);
    (graphicsmagick_sys::OrientationType_TopLeftOrientation, TopLeftOrientation);
    (graphicsmagick_sys::OrientationType_TopRightOrientation, TopRightOrientation);
    (graphicsmagick_sys::OrientationType_BottomRightOrientation, BottomRightOrientation);
    (graphicsmagick_sys::OrientationType_BottomLeftOrientation, BottomLeftOrientation);
    (graphicsmagick_sys::OrientationType_LeftTopOrientation, LeftTopOrientation);
    (graphicsmagick_sys::OrientationType_RightTopOrientation, RightTopOrientation);
    (graphicsmagick_sys::OrientationType_RightBottomOrientation, RightBottomOrientation);
    (graphicsmagick_sys::OrientationType_LeftBottomOrientation, LeftBottomOrientation);
}

types_enum_block! {
    /// Pixel error metrics
    MetricType;

    (graphicsmagick_sys::MetricType_MeanAbsoluteErrorMetric, MeanAbsoluteErrorMetric);
    (graphicsmagick_sys::MetricType_MeanSquaredErrorMetric, MeanSquaredErrorMetric);
    (graphicsmagick_sys::MetricType_PeakAbsoluteErrorMetric, PeakAbsoluteErrorMetric);
    (graphicsmagick_sys::MetricType_PeakSignalToNoiseRatioMetric, PeakSignalToNoiseRatioMetric);
    (graphicsmagick_sys::MetricType_RootMeanSquaredErrorMetric, RootMeanSquaredErrorMetric);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#colorspacetype](http://www.graphicsmagick.org/api/types.html#colorspacetype)
    ///
    /// The ColorspaceType enumeration is used to specify the colorspace that quantization
    /// (color reduction and mapping) is done under or to specify the colorspace when encoding
    /// an output image. Colorspaces are ways of describing colors to fit the requirements of a
    /// particular application (e.g. Television, offset printing, color monitors). Color reduction,
    /// by default, takes place in the RGBColorspace. Empirical evidence suggests that distances
    /// in color spaces such as YUVColorspace or YIQColorspace correspond to perceptual color
    /// differences more closely han do distances in RGB space. These color spaces may give better
    /// results when color reducing an image. Refer to quantize for more details.
    //When encoding an output image, the colorspaces RGBColorspace, CMYKColorspace, and GRAYColorspace may be specified. The CMYKColorspace option is only applicable when writing TIFF, JPEG, and Adobe Photoshop bitmap (PSD) files.
    ColorspaceType;

    /// Unset value.
    (graphicsmagick_sys::ColorspaceType_UndefinedColorspace, UndefinedColorspace);
    /// Red, Green, Blue colorspace.
    (graphicsmagick_sys::ColorspaceType_RGBColorspace, RGBColorspace);
    /// Similar to Luma (Y) according to ITU-R 601
    (graphicsmagick_sys::ColorspaceType_GRAYColorspace, GRAYColorspace);
    /// Similar to Luma (Y) according to ITU-R 601
    (graphicsmagick_sys::ColorspaceType_TransparentColorspace, TransparentColorspace);
    (graphicsmagick_sys::ColorspaceType_OHTAColorspace, OHTAColorspace);
    /// CIE XYZ
    (graphicsmagick_sys::ColorspaceType_XYZColorspace, XYZColorspace);
    /// Kodak PhotoCD PhotoYCC
    (graphicsmagick_sys::ColorspaceType_YCCColorspace, YCCColorspace);
    (graphicsmagick_sys::ColorspaceType_YIQColorspace, YIQColorspace);
    (graphicsmagick_sys::ColorspaceType_YPbPrColorspace, YPbPrColorspace);
    /// YUV colorspace as used for computer video.
    (graphicsmagick_sys::ColorspaceType_YUVColorspace, YUVColorspace);
    /// Cyan, Magenta, Yellow, Black colorspace.
    (graphicsmagick_sys::ColorspaceType_CMYKColorspace, CMYKColorspace);
    /// Kodak PhotoCD sRGB
    (graphicsmagick_sys::ColorspaceType_sRGBColorspace, SRGBColorspace);
    /// Hue, saturation, luminosity
    (graphicsmagick_sys::ColorspaceType_HSLColorspace, HSLColorspace);
    /// Hue, whiteness, blackness
    (graphicsmagick_sys::ColorspaceType_HWBColorspace, HWBColorspace);
    /// ITU LAB
    (graphicsmagick_sys::ColorspaceType_LABColorspace, LABColorspace);
    /// RGB data with Cineon Log scaling, 2.048 density range
    (graphicsmagick_sys::ColorspaceType_CineonLogRGBColorspace, CineonLogRGBColorspace);
    /// Luma (Y) according to ITU-R 601
    (graphicsmagick_sys::ColorspaceType_Rec601LumaColorspace, Rec601LumaColorspace);
    /// YCbCr according to ITU-R 601
    (graphicsmagick_sys::ColorspaceType_Rec601YCbCrColorspace, Rec601YCbCrColorspace);
    /// Luma (Y) according to ITU-R 709
    (graphicsmagick_sys::ColorspaceType_Rec709LumaColorspace, Rec709LumaColorspace);
    /// YCbCr according to ITU-R 709
    (graphicsmagick_sys::ColorspaceType_Rec709YCbCrColorspace, Rec709YCbCrColorspace);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#compressiontype](http://www.graphicsmagick.org/api/types.html#compressiontype)
    ///
    /// CompressionType is used to express the desired compression type when encoding an image.
    /// Be aware that most image types only support a sub-set of the available compression types.
    /// If the compression type specified is incompatable with the image, GraphicsMagick selects
    /// a compression type compatable with the image type, which might be no compression at all.
    CompressionType;

    /// Unset value.
    (graphicsmagick_sys::CompressionType_UndefinedCompression, UndefinedCompression);
    /// No compression
    (graphicsmagick_sys::CompressionType_NoCompression, NoCompression);
    /// BZip (Burrows-Wheeler block-sorting text compression algorithm and Huffman coding)
    /// as used by bzip2 utilities
    (graphicsmagick_sys::CompressionType_BZipCompression, BZipCompression);
    /// CCITT Group 3 FAX compression
    (graphicsmagick_sys::CompressionType_FaxCompression, FaxCompression);
    (graphicsmagick_sys::CompressionType_Group3Compression, Group3Compression);
    /// CCITT Group 4 FAX compression (used only for TIFF)
    (graphicsmagick_sys::CompressionType_Group4Compression, Group4Compression);
    /// JPEG compression
    (graphicsmagick_sys::CompressionType_JPEGCompression, JPEGCompression);
    /// Lossless JPEG compression
    (graphicsmagick_sys::CompressionType_LosslessJPEGCompression, LosslessJPEGCompression);
    /// Lempel-Ziv-Welch (LZW) compression (caution, patented by Unisys)
    (graphicsmagick_sys::CompressionType_LZWCompression, LZWCompression);
    /// Run-Length encoded (RLE) compression
    (graphicsmagick_sys::CompressionType_RLECompression, RLECompression);
    /// Lempel-Ziv compression (LZ77) as used in PKZIP and GNU gzip.
    (graphicsmagick_sys::CompressionType_ZipCompression, ZipCompression);
    /// LZMA - Lempel-Ziv-Markov chain algorithm
    (graphicsmagick_sys::CompressionType_LZMACompression, LZMACompression);
    /// JPEG 2000 - ISO/IEC std 15444-1
    (graphicsmagick_sys::CompressionType_JPEG2000Compression, JPEG2000Compression);
    /// JBIG v1 - ISO/IEC std 11544 / ITU-T rec T.82
    (graphicsmagick_sys::CompressionType_JBIG1Compression, JBIG1Compression);
}

types_enum_block! {
    /// DisposeType
    DisposeType;

    (graphicsmagick_sys::DisposeType_UndefinedDispose, UndefinedDispose);
    (graphicsmagick_sys::DisposeType_NoneDispose, NoneDispose);
    (graphicsmagick_sys::DisposeType_BackgroundDispose, BackgroundDispose);
    (graphicsmagick_sys::DisposeType_PreviousDispose, PreviousDispose);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#gravitytype](http://www.graphicsmagick.org/api/types.html#gravitytype)
    ///
    /// GravityType specifies positioning of an object (e.g. text, image) within a bounding
    /// region (e.g. an image). Gravity provides a convenient way to locate objects irrespective
    /// of the size of the bounding region, in other words, you don't need to provide absolute
    /// coordinates in order to position an object. A common default for gravity is
    /// NorthWestGravity.
    GravityType;

    /// Don't use gravity.
    (graphicsmagick_sys::GravityType_ForgetGravity, ForgetGravity);
    /// Position object at top-left of region.
    (graphicsmagick_sys::GravityType_NorthWestGravity, NorthWestGravity);
    /// Postiion object at top-center of region
    (graphicsmagick_sys::GravityType_NorthGravity, NorthGravity);
    /// Position object at top-right of region
    (graphicsmagick_sys::GravityType_NorthEastGravity, NorthEastGravity);
    /// Position object at left-center of region
    (graphicsmagick_sys::GravityType_WestGravity, WestGravity);
    /// Position object at center of region
    (graphicsmagick_sys::GravityType_CenterGravity, CenterGravity);
    /// Position object at right-center of region
    (graphicsmagick_sys::GravityType_EastGravity, EastGravity);
    /// Position object at left-bottom of region
    (graphicsmagick_sys::GravityType_SouthWestGravity, SouthWestGravity);
    /// Position object at bottom-center of region
    (graphicsmagick_sys::GravityType_SouthGravity, SouthGravity);
    /// Position object at bottom-right of region
    (graphicsmagick_sys::GravityType_SouthEastGravity, SouthEastGravity);
    (graphicsmagick_sys::GravityType_StaticGravity, StaticGravity);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#interlacetype](http://www.graphicsmagick.org/api/types.html#interlacetype)
    ///
    /// InterlaceType specifies the ordering of the red, green, and blue pixel information in
    /// the image. Interlacing is usually used to make image information available to the user
    /// faster by taking advantage of the space vs time tradeoff. For example, interlacing allows
    /// images on the Web to be recognizable sooner and satellite images to accumulate/render with
    /// image resolution increasing over time.
    ///
    /// Use LineInterlace or PlaneInterlace to create an interlaced GIF or progressive JPEG image.
    InterlaceType;

    /// Unset value.
    (graphicsmagick_sys::InterlaceType_UndefinedInterlace, UndefinedInterlace);
    /// Don't interlace image (RGBRGBRGBRGBRGBRGB...)
    (graphicsmagick_sys::InterlaceType_NoInterlace, NoInterlace);
    /// Use scanline interlacing (RRR...GGG...BBB...RRR...GGG...BBB...)
    (graphicsmagick_sys::InterlaceType_LineInterlace, LineInterlace);
    /// Use plane interlacing (RRRRRR...GGGGGG...BBBBBB...)
    (graphicsmagick_sys::InterlaceType_PlaneInterlace, PlaneInterlace);
    /// Similar to plane interlaing except that the different planes are saved to
    /// individual files (e.g. image.R, image.G, and image.B)
    (graphicsmagick_sys::InterlaceType_PartitionInterlace, PartitionInterlace);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#storagetype](http://www.graphicsmagick.org/api/types.html#storagetype)
    StorageType;

    (graphicsmagick_sys::StorageType_CharPixel, CharPixel);
    (graphicsmagick_sys::StorageType_ShortPixel, ShortPixel);
    (graphicsmagick_sys::StorageType_IntegerPixel, IntegerPixel);
    (graphicsmagick_sys::StorageType_LongPixel, LongPixel);
    (graphicsmagick_sys::StorageType_FloatPixel, FloatPixel);
    (graphicsmagick_sys::StorageType_DoublePixel, DoublePixel);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#renderingintent](http://www.graphicsmagick.org/api/types.html#renderingintent)
    ///
    /// Rendering intent is a concept defined by ICC Spec ICC.1:1998-09, "File Format for Color
    /// Profiles". GraphicsMagick uses RenderingIntent in order to support ICC Color Profiles.
    ///
    /// From the specification: "Rendering intent specifies the style of reproduction to be used
    /// during the evaluation of this profile in a sequence of profiles. It applies specifically
    /// to that profile in the sequence and not to the entire sequence. Typically, the user or
    /// application will set the rendering intent dynamically at runtime or embedding time."
    RenderingIntent;

    /// Unset value.
    (graphicsmagick_sys::RenderingIntent_UndefinedIntent, UndefinedIntent);
    /// A rendering intent that specifies the saturation of the pixels in the image is preserved
    /// perhaps at the expense of accuracy in hue and lightness.
    (graphicsmagick_sys::RenderingIntent_SaturationIntent, SaturationIntent);
    /// A rendering intent that specifies the full gamut of the image is compressed or expanded
    /// to fill the gamut of the destination device. Gray balance is preserved but colorimetric
    /// accuracy might not be preserved.
    (graphicsmagick_sys::RenderingIntent_PerceptualIntent, PerceptualIntent);
    /// Absolute colorimetric
    (graphicsmagick_sys::RenderingIntent_AbsoluteIntent, AbsoluteIntent);
    /// Relative colorimetric
    (graphicsmagick_sys::RenderingIntent_RelativeIntent, RelativeIntent);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#imagetype](http://www.graphicsmagick.org/api/types.html#imagetype)
    ///
    /// ImageType indicates the type classification of the image.
    ImageType;

    /// Unset value.
    (graphicsmagick_sys::ImageType_UndefinedType, UndefinedType);
    /// Monochrome image
    (graphicsmagick_sys::ImageType_BilevelType, BilevelType);
    /// Grayscale image
    (graphicsmagick_sys::ImageType_GrayscaleType, GrayscaleType);
    /// Grayscale image with opacity
    (graphicsmagick_sys::ImageType_GrayscaleMatteType, GrayscaleMatteType);
    /// Indexed color (palette) image
    (graphicsmagick_sys::ImageType_PaletteType, PaletteType);
    /// Indexed color (palette) image with opacity
    (graphicsmagick_sys::ImageType_PaletteMatteType, PaletteMatteType);
    /// Truecolor image
    (graphicsmagick_sys::ImageType_TrueColorType, TrueColorType);
    /// Truecolor image with opacity
    (graphicsmagick_sys::ImageType_TrueColorMatteType, TrueColorMatteType);
    /// Cyan/Yellow/Magenta/Black (CYMK) image
    (graphicsmagick_sys::ImageType_ColorSeparationType, ColorSeparationType);
    (graphicsmagick_sys::ImageType_ColorSeparationMatteType, ColorSeparationMatteType);
    (graphicsmagick_sys::ImageType_OptimizeType, OptimizeType);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#resolutiontype](http://www.graphicsmagick.org/api/types.html#resolutiontype)
    ///
    /// By default, GraphicsMagick defines resolutions in pixels per inch. ResolutionType provides a means to adjust this.
    ResolutionType;

    /// Unset value.
    (graphicsmagick_sys::ResolutionType_UndefinedResolution, UndefinedResolution);
    /// Density specifications are specified in units of pixels per inch (english units).
    (graphicsmagick_sys::ResolutionType_PixelsPerInchResolution, PixelsPerInchResolution);
    /// Density specifications are specified in units of pixels per centimeter (metric units).
    (graphicsmagick_sys::ResolutionType_PixelsPerCentimeterResolution, PixelsPerCentimeterResolution);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#virtualpixelmethod](http://www.graphicsmagick.org/api/types.html#virtualpixelmethod)
    ///
    /// Enum declaractions.
    VirtualPixelMethod;

    (graphicsmagick_sys::VirtualPixelMethod_UndefinedVirtualPixelMethod, UndefinedVirtualPixelMethod);
    (graphicsmagick_sys::VirtualPixelMethod_ConstantVirtualPixelMethod, ConstantVirtualPixelMethod);
    (graphicsmagick_sys::VirtualPixelMethod_EdgeVirtualPixelMethod, EdgeVirtualPixelMethod);
    (graphicsmagick_sys::VirtualPixelMethod_MirrorVirtualPixelMethod, MirrorVirtualPixelMethod);
    (graphicsmagick_sys::VirtualPixelMethod_TileVirtualPixelMethod, TileVirtualPixelMethod);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#resourcetype](http://www.graphicsmagick.org/api/types.html#resourcetype)
    ///
    /// Typedef declarations.
    ResourceType;

    /// Undefined value
    (graphicsmagick_sys::ResourceType_UndefinedResource, UndefinedResource);
    /// Pixel cache total disk space (Gigabytes)
    (graphicsmagick_sys::ResourceType_DiskResource, DiskResource);
    /// Pixel cache number of open files (Files)
    (graphicsmagick_sys::ResourceType_FileResource, FileResource);
    /// Pixel cache total file memory-mapping (Megabytes)
    (graphicsmagick_sys::ResourceType_MapResource, MapResource);
    /// Maximum pixel cache heap memory allocations (Megabytes)
    (graphicsmagick_sys::ResourceType_MemoryResource, MemoryResource);
    /// Maximum number of pixels in single image (Pixels)
    (graphicsmagick_sys::ResourceType_PixelsResource, PixelsResource);
    /// Maximum number of worker threads
    (graphicsmagick_sys::ResourceType_ThreadsResource, ThreadsResource);
    #[cfg(gm_v_1_3_21)]
    /// Maximum pixel width of an image (Pixels)
    (graphicsmagick_sys::ResourceType_WidthResource, WidthResource);
    #[cfg(gm_v_1_3_21)]
    /// Maximum pixel height of an image (Pixels)
    (graphicsmagick_sys::ResourceType_HeightResource, HeightResource);
}

types_enum_block! {
    /// MontageMode
    MontageMode;

    (graphicsmagick_sys::MontageMode_UndefinedMode, UndefinedMode);
    (graphicsmagick_sys::MontageMode_FrameMode, FrameMode);
    (graphicsmagick_sys::MontageMode_UnframeMode, UnframeMode);
    (graphicsmagick_sys::MontageMode_ConcatenateMode, ConcatenateMode);
}

types_enum_block! {
    /// PreviewType
    PreviewType;

    (graphicsmagick_sys::PreviewType_UndefinedPreview, UndefinedPreview);
    (graphicsmagick_sys::PreviewType_RotatePreview, RotatePreview);
    (graphicsmagick_sys::PreviewType_ShearPreview, ShearPreview);
    (graphicsmagick_sys::PreviewType_RollPreview, RollPreview);
    (graphicsmagick_sys::PreviewType_HuePreview, HuePreview);
    (graphicsmagick_sys::PreviewType_SaturationPreview, SaturationPreview);
    (graphicsmagick_sys::PreviewType_BrightnessPreview, BrightnessPreview);
    (graphicsmagick_sys::PreviewType_GammaPreview, GammaPreview);
    (graphicsmagick_sys::PreviewType_SpiffPreview, SpiffPreview);
    (graphicsmagick_sys::PreviewType_DullPreview, DullPreview);
    (graphicsmagick_sys::PreviewType_GrayscalePreview, GrayscalePreview);
    (graphicsmagick_sys::PreviewType_QuantizePreview, QuantizePreview);
    (graphicsmagick_sys::PreviewType_DespecklePreview, DespecklePreview);
    (graphicsmagick_sys::PreviewType_ReduceNoisePreview, ReduceNoisePreview);
    (graphicsmagick_sys::PreviewType_AddNoisePreview, AddNoisePreview);
    (graphicsmagick_sys::PreviewType_SharpenPreview, SharpenPreview);
    (graphicsmagick_sys::PreviewType_BlurPreview, BlurPreview);
    (graphicsmagick_sys::PreviewType_ThresholdPreview, ThresholdPreview);
    (graphicsmagick_sys::PreviewType_EdgeDetectPreview, EdgeDetectPreview);
    (graphicsmagick_sys::PreviewType_SpreadPreview, SpreadPreview);
    (graphicsmagick_sys::PreviewType_SolarizePreview, SolarizePreview);
    (graphicsmagick_sys::PreviewType_ShadePreview, ShadePreview);
    (graphicsmagick_sys::PreviewType_RaisePreview, RaisePreview);
    (graphicsmagick_sys::PreviewType_SegmentPreview, SegmentPreview);
    (graphicsmagick_sys::PreviewType_SwirlPreview, SwirlPreview);
    (graphicsmagick_sys::PreviewType_ImplodePreview, ImplodePreview);
    (graphicsmagick_sys::PreviewType_WavePreview, WavePreview);
    (graphicsmagick_sys::PreviewType_OilPaintPreview, OilPaintPreview);
    (graphicsmagick_sys::PreviewType_CharcoalDrawingPreview, CharcoalDrawingPreview);
    (graphicsmagick_sys::PreviewType_JPEGPreview, JPEGPreview);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#fillrule](http://www.graphicsmagick.org/api/types.html#fillrule)
    FillRule;

    (graphicsmagick_sys::FillRule_UndefinedRule, UndefinedRule);
    (graphicsmagick_sys::FillRule_EvenOddRule, EvenOddRule);
    (graphicsmagick_sys::FillRule_NonZeroRule, NonZeroRule);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#clippathunits](http://www.graphicsmagick.org/api/types.html#clippathunits)
    ClipPathUnits;

    (graphicsmagick_sys::ClipPathUnits_UserSpace, UserSpace);
    (graphicsmagick_sys::ClipPathUnits_UserSpaceOnUse, UserSpaceOnUse);
    (graphicsmagick_sys::ClipPathUnits_ObjectBoundingBox, ObjectBoundingBox);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#paintmethod](http://www.graphicsmagick.org/api/types.html#paintmethod)
    ///
    /// PaintMethod specifies how pixel colors are to be replaced in the image. It is used to select
    /// the pixel-filling algorithm employed.
    PaintMethod;

    /// Replace pixel color at point.
    (graphicsmagick_sys::PaintMethod_PointMethod, PointMethod);
    /// Replace color for all image pixels matching color at point.
    (graphicsmagick_sys::PaintMethod_ReplaceMethod, ReplaceMethod);
    /// Replace color for pixels surrounding point until encountering pixel that fails to match color at point.
    (graphicsmagick_sys::PaintMethod_FloodfillMethod, FloodfillMethod);
    /// Replace color for pixels surrounding point until encountering pixels matching border color.
    (graphicsmagick_sys::PaintMethod_FillToBorderMethod, FillToBorderMethod);
    /// Replace colors for all pixels in image with pen color.
    (graphicsmagick_sys::PaintMethod_ResetMethod, ResetMethod);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#stretchtype](http://www.graphicsmagick.org/api/types.html#stretchtype)
    StretchType;

    (graphicsmagick_sys::StretchType_NormalStretch, NormalStretch);
    (graphicsmagick_sys::StretchType_UltraCondensedStretch, UltraCondensedStretch);
    (graphicsmagick_sys::StretchType_ExtraCondensedStretch, ExtraCondensedStretch);
    (graphicsmagick_sys::StretchType_CondensedStretch, CondensedStretch);
    (graphicsmagick_sys::StretchType_SemiCondensedStretch, SemiCondensedStretch);
    (graphicsmagick_sys::StretchType_SemiExpandedStretch, SemiExpandedStretch);
    (graphicsmagick_sys::StretchType_ExpandedStretch, ExpandedStretch);
    (graphicsmagick_sys::StretchType_ExtraExpandedStretch, ExtraExpandedStretch);
    (graphicsmagick_sys::StretchType_UltraExpandedStretch, UltraExpandedStretch);
    (graphicsmagick_sys::StretchType_AnyStretch, AnyStretch);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#styletype](http://www.graphicsmagick.org/api/types.html#styletype)
    StyleType;

    (graphicsmagick_sys::StyleType_NormalStyle, NormalStyle);
    (graphicsmagick_sys::StyleType_ItalicStyle, ItalicStyle);
    (graphicsmagick_sys::StyleType_ObliqueStyle, ObliqueStyle);
    (graphicsmagick_sys::StyleType_AnyStyle, AnyStyle);
}

types_enum_block! {
    /// LineCap
    LineCap;

    (graphicsmagick_sys::LineCap_UndefinedCap, UndefinedCap);
    (graphicsmagick_sys::LineCap_ButtCap, ButtCap);
    (graphicsmagick_sys::LineCap_RoundCap, RoundCap);
    (graphicsmagick_sys::LineCap_SquareCap, SquareCap);
}

types_enum_block! {
    /// LineJoin
    LineJoin;

    (graphicsmagick_sys::LineJoin_UndefinedJoin, UndefinedJoin);
    (graphicsmagick_sys::LineJoin_MiterJoin, MiterJoin);
    (graphicsmagick_sys::LineJoin_RoundJoin, RoundJoin);
    (graphicsmagick_sys::LineJoin_BevelJoin, BevelJoin);
}

types_enum_block! {
    /// [http://www.graphicsmagick.org/api/types.html#decorationtype](http://www.graphicsmagick.org/api/types.html#decorationtype)
    DecorationType;

    (graphicsmagick_sys::DecorationType_NoDecoration, NoDecoration);
    (graphicsmagick_sys::DecorationType_UnderlineDecoration, UnderlineDecoration);
    (graphicsmagick_sys::DecorationType_OverlineDecoration, OverlineDecoration);
    (graphicsmagick_sys::DecorationType_LineThroughDecoration, LineThroughDecoration);
}
