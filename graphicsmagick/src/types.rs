c_enum_block! {
    FilterType;

    (graphicsmagick_sys::FilterTypes_UndefinedFilter, UndefinedFilter);
    (graphicsmagick_sys::FilterTypes_PointFilter, PointFilter);
    (graphicsmagick_sys::FilterTypes_BoxFilter, BoxFilter);
    (graphicsmagick_sys::FilterTypes_TriangleFilter, TriangleFilter);
    (graphicsmagick_sys::FilterTypes_HermiteFilter, HermiteFilter);
    (graphicsmagick_sys::FilterTypes_HanningFilter, HanningFilter);
    (graphicsmagick_sys::FilterTypes_HammingFilter, HammingFilter);
    (graphicsmagick_sys::FilterTypes_BlackmanFilter, BlackmanFilter);
    (graphicsmagick_sys::FilterTypes_GaussianFilter, GaussianFilter);
    (graphicsmagick_sys::FilterTypes_QuadraticFilter, QuadraticFilter);
    (graphicsmagick_sys::FilterTypes_CubicFilter, CubicFilter);
    (graphicsmagick_sys::FilterTypes_CatromFilter, CatromFilter);
    (graphicsmagick_sys::FilterTypes_MitchellFilter, MitchellFilter);
    (graphicsmagick_sys::FilterTypes_LanczosFilter, LanczosFilter);
    (graphicsmagick_sys::FilterTypes_BesselFilter, BesselFilter);
    (graphicsmagick_sys::FilterTypes_SincFilter, SincFilter);
}

c_enum_block! {
    CompositeOperator;

    (graphicsmagick_sys::CompositeOperator_UndefinedCompositeOp, UndefinedCompositeOp);
    (graphicsmagick_sys::CompositeOperator_OverCompositeOp, OverCompositeOp);
    (graphicsmagick_sys::CompositeOperator_InCompositeOp, InCompositeOp);
    (graphicsmagick_sys::CompositeOperator_OutCompositeOp, OutCompositeOp);
    (graphicsmagick_sys::CompositeOperator_AtopCompositeOp, AtopCompositeOp);
    (graphicsmagick_sys::CompositeOperator_XorCompositeOp, XorCompositeOp);
    (graphicsmagick_sys::CompositeOperator_PlusCompositeOp, PlusCompositeOp);
    (graphicsmagick_sys::CompositeOperator_MinusCompositeOp, MinusCompositeOp);
    (graphicsmagick_sys::CompositeOperator_AddCompositeOp, AddCompositeOp);
    (graphicsmagick_sys::CompositeOperator_SubtractCompositeOp, SubtractCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DifferenceCompositeOp, DifferenceCompositeOp);
    (graphicsmagick_sys::CompositeOperator_MultiplyCompositeOp, MultiplyCompositeOp);
    (graphicsmagick_sys::CompositeOperator_BumpmapCompositeOp, BumpmapCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyCompositeOp, CopyCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyRedCompositeOp, CopyRedCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyGreenCompositeOp, CopyGreenCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyBlueCompositeOp, CopyBlueCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyOpacityCompositeOp, CopyOpacityCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ClearCompositeOp, ClearCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DissolveCompositeOp, DissolveCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DisplaceCompositeOp, DisplaceCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ModulateCompositeOp, ModulateCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ThresholdCompositeOp, ThresholdCompositeOp);
    (graphicsmagick_sys::CompositeOperator_NoCompositeOp, NoCompositeOp);
    (graphicsmagick_sys::CompositeOperator_DarkenCompositeOp, DarkenCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LightenCompositeOp, LightenCompositeOp);
    (graphicsmagick_sys::CompositeOperator_HueCompositeOp, HueCompositeOp);
    (graphicsmagick_sys::CompositeOperator_SaturateCompositeOp, SaturateCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ColorizeCompositeOp, ColorizeCompositeOp);
    (graphicsmagick_sys::CompositeOperator_LuminizeCompositeOp, LuminizeCompositeOp);
    (graphicsmagick_sys::CompositeOperator_ScreenCompositeOp, ScreenCompositeOp);
    (graphicsmagick_sys::CompositeOperator_OverlayCompositeOp, OverlayCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyCyanCompositeOp, CopyCyanCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyMagentaCompositeOp, CopyMagentaCompositeOp);
    (graphicsmagick_sys::CompositeOperator_CopyYellowCompositeOp, CopyYellowCompositeOp);
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

c_enum_block! {
    ChannelType;

    (graphicsmagick_sys::ChannelType_UndefinedChannel, UndefinedChannel);
    (graphicsmagick_sys::ChannelType_RedChannel, RedChannel);
    (graphicsmagick_sys::ChannelType_CyanChannel, CyanChannel);
    (graphicsmagick_sys::ChannelType_GreenChannel, GreenChannel);
    (graphicsmagick_sys::ChannelType_MagentaChannel, MagentaChannel);
    (graphicsmagick_sys::ChannelType_BlueChannel, BlueChannel);
    (graphicsmagick_sys::ChannelType_YellowChannel, YellowChannel);
    (graphicsmagick_sys::ChannelType_OpacityChannel, OpacityChannel);
    (graphicsmagick_sys::ChannelType_BlackChannel, BlackChannel);
    (graphicsmagick_sys::ChannelType_MatteChannel, MatteChannel);
    (graphicsmagick_sys::ChannelType_AllChannels, AllChannels);
    (graphicsmagick_sys::ChannelType_GrayChannel, GrayChannel);
}
