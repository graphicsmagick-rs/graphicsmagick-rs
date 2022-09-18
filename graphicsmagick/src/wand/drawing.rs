//! Binding of drawing_wand, Wand vector drawing interfaces.
//!
//! <http://www.graphicsmagick.org/wand/drawing_wand.html>

use crate::{
    types::{
        AffineMatrix, ClipPathUnits, DecorationType, FillRule, GravityType, LineCap, LineJoin,
        PaintMethod, StretchType, StyleType,
    },
    utils::assert_initialized,
    wand::pixel::PixelWand,
    MagickBoxSlice, MagickCString,
};
use graphicsmagick_sys::*;
use null_terminated_str::IntoNullTerminatedString;
use std::{
    os::raw::{c_double, c_uint, c_ulong},
    ptr::NonNull,
};

/// Wrapper of `graphicsmagick_sys::DrawingWand`.
#[derive(Debug)]
#[repr(transparent)]
pub struct DrawingWand {
    wand: NonNull<graphicsmagick_sys::DrawingWand>,
}

impl Drop for DrawingWand {
    fn drop(&mut self) {
        unsafe {
            MagickDestroyDrawingWand(self.wand.as_ptr());
        }
    }
}

impl Clone for DrawingWand {
    fn clone(&self) -> Self {
        DrawingWand {
            wand: NonNull::new(unsafe { MagickCloneDrawingWand(self.wand.as_ptr()) })
                .expect("MagickCloneDrawingWand returns NULL"),
        }
    }
}

impl Default for DrawingWand {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawingWand {
    pub fn new() -> Self {
        assert_initialized();

        let wand =
            NonNull::new(unsafe { MagickNewDrawingWand() }).expect("NewDrawingWand return NULL");

        DrawingWand { wand }
    }

    #[inline]
    pub fn wand(&self) -> *const graphicsmagick_sys::DrawingWand {
        self.wand.as_ptr() as *const _
    }

    #[inline]
    pub fn wand_mut(&mut self) -> *mut graphicsmagick_sys::DrawingWand {
        self.wand.as_ptr()
    }
}

impl DrawingWand {
    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawannotation>
    ///
    /// DrawAnnotation() draws text on the image.
    ///
    pub fn annotation<'a>(
        &mut self,
        x: c_double,
        y: c_double,
        text: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let text = text.into_null_terminated_string();
        unsafe { MagickDrawAnnotation(self.wand.as_ptr(), x, y, text.as_ptr().cast()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawaffine>
    ///
    /// DrawAffine() adjusts the current affine transformation matrix with
    ///
    /// the specified affine transformation matrix. Note that the current affine
    ///
    /// transform is adjusted rather than replaced.
    ///
    pub fn affine(&mut self, affine: &AffineMatrix) -> &mut Self {
        unsafe { MagickDrawAffine(self.wand.as_ptr(), affine) };
        self
    }

    // TODO Implement later.
    // /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawallocatewand>
    // ///
    // /// DrawAllocateWand() allocates an initial drawing wand which is an
    // ///
    // /// opaque handle required by the remaining drawing methods.
    // ///
    // pub fn allocate_wand(draw_info: &DrawInfo, image: &mut Image) -> Self {
    //     let status = unsafe { MagickDrawAllocateWand(draw_info, image) };
    //     todo!()
    // }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawarc>
    ///
    /// DrawArc() draws an arc falling within a specified bounding rectangle on the
    ///
    /// image.
    ///
    pub fn arc(
        &mut self,
        sx: c_double,
        sy: c_double,
        ex: c_double,
        ey: c_double,
        sd: c_double,
        ed: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawArc(self.wand.as_ptr(), sx, sy, ex, ey, sd, ed) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawbezier>
    ///
    /// DrawBezier() draws a bezier curve through a set of points on the image.
    ///
    pub fn bezier(&mut self, number_coordinates: c_ulong, coordinates: &PointInfo) -> &mut Self {
        unsafe { MagickDrawBezier(self.wand.as_ptr(), number_coordinates, coordinates) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawcircle>
    ///
    /// DrawCircle() draws a circle on the image.
    ///
    pub fn circle(&mut self, ox: c_double, oy: c_double, px: c_double, py: c_double) -> &mut Self {
        unsafe { MagickDrawCircle(self.wand.as_ptr(), ox, oy, px, py) };
        self
    }

    //    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawclearexception>
    //    ///
    //    /// DrawClearException() clears any existing exception from the drawing wand.
    //    ///
    //    pub fn clear_exception(&mut self) -> crate::Result<&mut Self> {
    //        let status = unsafe { MagickDrawClearException(self.wand.as_ptr()) };
    //    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetclippath>
    ///
    /// DrawGetClipPath() obtains the current clipping path ID. The value returned
    ///
    /// must be deallocated by the user when it is no longer needed.
    ///
    pub fn get_clip_path(&self) -> MagickCString {
        unsafe { MagickCString::new(MagickDrawGetClipPath(self.wand.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetclippath>
    ///
    /// DrawSetClipPath() associates a named clipping path with the image.  Only
    ///
    /// the areas drawn on by the clipping path will be modified as long as it
    ///
    /// remains in effect.
    ///
    pub fn set_clip_path<'a>(&mut self, clip_path: impl IntoNullTerminatedString<'a>) -> &mut Self {
        let clip_path = clip_path.into_null_terminated_string();
        unsafe { MagickDrawSetClipPath(self.wand.as_ptr(), clip_path.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetcliprule>
    ///
    /// DrawGetClipRule() returns the current polygon fill rule to be used by the
    ///
    /// clipping path.
    ///
    pub fn get_clip_rule(&self) -> FillRule {
        unsafe { MagickDrawGetClipRule(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetcliprule>
    ///
    /// DrawSetClipRule() set the polygon fill rule to be used by the clipping path.
    ///
    pub fn set_clip_rule(&mut self, fill_rule: FillRule) -> &mut Self {
        unsafe { MagickDrawSetClipRule(self.wand.as_ptr(), fill_rule.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetclipunits>
    ///
    /// DrawGetClipUnits() returns the interpretation of clip path units.
    ///
    pub fn get_clip_units(&self) -> ClipPathUnits {
        unsafe { MagickDrawGetClipUnits(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetclipunits>
    ///
    /// DrawSetClipUnits() sets the interpretation of clip path units.
    ///
    pub fn set_clip_units(&mut self, clip_units: ClipPathUnits) -> &mut Self {
        unsafe { MagickDrawSetClipUnits(self.wand.as_ptr(), clip_units.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawcolor>
    ///
    /// DrawColor() draws color on image using the current fill color, starting at
    ///
    /// specified position, and using specified paint method. The available paint
    ///
    /// methods are:
    ///
    /// PointMethod: Recolors the target pixel
    ///
    /// ReplaceMethod: Recolor any pixel that matches the target pixel.
    ///
    /// FloodfillMethod: Recolors target pixels and matching neighbors.
    ///
    /// FillToBorderMethod: Recolor target pixels and neighbors not matching
    ///
    /// ResetMethod: Recolor all pixels.
    ///
    pub fn color(&mut self, x: c_double, y: c_double, paint_method: PaintMethod) -> &mut Self {
        unsafe { MagickDrawColor(self.wand.as_ptr(), x, y, paint_method.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawcomment>
    ///
    /// DrawComment() adds a comment to a vector output stream.
    ///
    pub fn comment<'a>(&mut self, comment: impl IntoNullTerminatedString<'a>) -> &mut Self {
        let comment = comment.into_null_terminated_string();
        unsafe { MagickDrawComment(self.wand.as_ptr(), comment.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawellipse>
    ///
    /// DrawEllipse() draws an ellipse on the image.
    ///
    pub fn ellipse(
        &mut self,
        ox: c_double,
        oy: c_double,
        rx: c_double,
        ry: c_double,
        start: c_double,
        end: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawEllipse(self.wand.as_ptr(), ox, oy, rx, ry, start, end) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfillcolor>
    ///
    /// DrawGetFillColor() returns the fill color used for drawing filled objects.
    ///
    pub fn get_fill_color(&self) -> PixelWand {
        let mut pw = PixelWand::new();
        unsafe { MagickDrawGetFillColor(self.wand.as_ptr(), pw.wand_mut()) };
        pw
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfillcolor>
    ///
    /// DrawSetFillColor() sets the fill color to be used for drawing filled objects.
    ///
    pub fn set_fill_color(&mut self, fill_wand: &PixelWand) -> &mut Self {
        unsafe { MagickDrawSetFillColor(self.wand.as_ptr(), fill_wand.wand()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfillpatternurl>
    ///
    /// DrawSetFillPatternURL() sets the URL to use as a fill pattern for filling
    ///
    /// objects. Only local URLs (&quot;#identifier&quot;) are supported at this time. These
    ///
    /// local URLs are normally created by defining a named fill pattern with
    ///
    /// DrawPushPattern/DrawPopPattern.
    ///
    pub fn set_fill_pattern_url<'a>(
        &mut self,
        fill_url: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let fill_url = fill_url.into_null_terminated_string();
        unsafe { MagickDrawSetFillPatternURL(self.wand.as_ptr(), fill_url.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfillopacity>
    ///
    /// DrawGetFillOpacity() returns the opacity used when drawing using the fill
    ///
    /// color or fill texture.  Fully opaque is 1.0.
    ///
    pub fn get_fill_opacity(&self) -> c_double {
        unsafe { MagickDrawGetFillOpacity(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfillopacity>
    ///
    /// DrawSetFillOpacity() sets the opacity to use when drawing using the fill
    ///
    /// color or fill texture.  Fully opaque is 1.0.
    ///
    pub fn set_fill_opacity(&mut self, fill_opacity: c_double) -> &mut Self {
        unsafe { MagickDrawSetFillOpacity(self.wand.as_ptr(), fill_opacity) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfillrule>
    ///
    /// DrawGetFillRule() returns the fill rule used while drawing polygons.
    ///
    pub fn get_fill_rule(&self) -> FillRule {
        unsafe { MagickDrawGetFillRule(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfillrule>
    ///
    /// DrawSetFillRule() sets the fill rule to use while drawing polygons.
    ///
    pub fn set_fill_rule(&mut self, fill_rule: FillRule) -> &mut Self {
        unsafe { MagickDrawSetFillRule(self.wand.as_ptr(), fill_rule.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfont>
    ///
    /// DrawGetFont() returns a null-terminaged string specifying the font used
    ///
    /// when annotating with text. The value returned must be freed by the user
    ///
    /// when no longer needed.
    ///
    pub fn get_font(&self) -> MagickCString {
        unsafe { MagickCString::new(MagickDrawGetFont(self.wand.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfont>
    ///
    /// DrawSetFont() sets the fully-sepecified font to use when annotating with
    ///
    /// text.
    ///
    pub fn set_font<'a>(&mut self, font_name: impl IntoNullTerminatedString<'a>) -> &mut Self {
        let font_name = font_name.into_null_terminated_string();
        unsafe { MagickDrawSetFont(self.wand.as_ptr(), font_name.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfontfamily>
    ///
    /// DrawGetFontFamily() returns the font family to use when annotating with text.
    ///
    /// The value returned must be freed by the user when it is no longer needed.
    ///
    pub fn get_font_family(&self) -> MagickCString {
        unsafe { MagickCString::new(MagickDrawGetFontFamily(self.wand.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfontfamily>
    ///
    /// DrawSetFontFamily() sets the font family to use when annotating with text.
    ///
    pub fn set_font_family<'a>(
        &mut self,
        font_family: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let font_family = font_family.into_null_terminated_string();
        unsafe { MagickDrawSetFontFamily(self.wand.as_ptr(), font_family.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfontsize>
    ///
    /// DrawGetFontSize() returns the font pointsize used when annotating with text.
    ///
    pub fn get_font_size(&self) -> c_double {
        unsafe { MagickDrawGetFontSize(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfontsize>
    ///
    /// DrawSetFontSize() sets the font pointsize to use when annotating with text.
    ///
    pub fn set_font_size(&mut self, point_size: c_double) -> &mut Self {
        unsafe { MagickDrawSetFontSize(self.wand.as_ptr(), point_size) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfontstretch>
    ///
    /// DrawGetFontStretch() returns the font stretch used when annotating with text.
    ///
    pub fn get_font_stretch(&self) -> StretchType {
        unsafe { MagickDrawGetFontStretch(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfontstretch>
    ///
    /// DrawSetFontStretch() sets the font stretch to use when annotating with text.
    ///
    /// The AnyStretch enumeration acts as a wild-card &quot;don't care&quot; option.
    ///
    pub fn set_font_stretch(&mut self, font_stretch: StretchType) -> &mut Self {
        unsafe { MagickDrawSetFontStretch(self.wand.as_ptr(), font_stretch.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfontstyle>
    ///
    /// DrawGetFontStyle() returns the font style used when annotating with text.
    ///
    pub fn get_font_style(&self) -> StyleType {
        unsafe { MagickDrawGetFontStyle(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfontstyle>
    ///
    /// DrawSetFontStyle() sets the font style to use when annotating with text.
    ///
    /// The AnyStyle enumeration acts as a wild-card &quot;don't care&quot; option.
    ///
    pub fn set_font_style(&mut self, style: StyleType) -> &mut Self {
        unsafe { MagickDrawSetFontStyle(self.wand.as_ptr(), style.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetfontweight>
    ///
    /// DrawGetFontWeight() returns the font weight used when annotating with text.
    ///
    pub fn get_font_weight(&self) -> c_ulong {
        unsafe { MagickDrawGetFontWeight(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetfontweight>
    ///
    /// DrawSetFontWeight() sets the font weight to use when annotating with text.
    ///
    pub fn set_font_weight(&mut self, font_weight: c_ulong) -> &mut Self {
        unsafe { MagickDrawSetFontWeight(self.wand.as_ptr(), font_weight) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetgravity>
    ///
    /// DrawGetGravity() returns the text placement gravity used when annotating
    ///
    /// with text.
    ///
    pub fn get_gravity(&self) -> GravityType {
        unsafe { MagickDrawGetGravity(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetgravity>
    ///
    /// DrawSetGravity() sets the text placement gravity to use when annotating
    ///
    /// with text.
    ///
    pub fn set_gravity(&mut self, gravity: GravityType) -> &mut Self {
        unsafe { MagickDrawSetGravity(self.wand.as_ptr(), gravity.into()) };
        self
    }

    // TODO Implement later.
    // /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawcomposite>
    // ///
    // /// DrawComposite() composites an image onto the current image, using the
    // ///
    // /// specified composition operator, specified position, and at the specified
    // ///
    // /// size.
    // ///
    // pub fn composite(&mut self,  composite_operator: CompositeOperator,  x: c_double,  y: c_double,  width: c_double,  height: c_double, image: &Image) {
    // let status = unsafe { MagickDrawComposite(self.wand.as_ptr(),  composite_operator,  x,  y,  width,  height, image) };
    // }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawline>
    ///
    /// DrawLine() draws a line on the image using the current stroke color,
    ///
    /// stroke opacity, and stroke width.
    ///
    pub fn line(&mut self, sx: c_double, sy: c_double, ex: c_double, ey: c_double) -> &mut Self {
        unsafe { MagickDrawLine(self.wand.as_ptr(), sx, sy, ex, ey) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawmatte>
    ///
    /// DrawMatte() paints on the image's opacity channel in order to set effected
    ///
    /// pixels to transparent.
    ///
    /// to influence the opacity of pixels. The available paint
    ///
    /// methods are:
    ///
    /// PointMethod: Select the target pixel
    ///
    /// ReplaceMethod: Select any pixel that matches the target pixel.
    ///
    /// FloodfillMethod: Select the target pixel and matching neighbors.
    ///
    /// FillToBorderMethod: Select the target pixel and neighbors not matching
    ///
    /// border color.
    ///
    /// ResetMethod: Select all pixels.
    ///
    pub fn matte(&mut self, x: c_double, y: c_double, paint_method: PaintMethod) -> &mut Self {
        unsafe { MagickDrawMatte(self.wand.as_ptr(), x, y, paint_method.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathclose>
    ///
    /// DrawPathClose() adds a path element to the current path which closes the
    ///
    /// current subpath by drawing a straight line from the current point to the
    ///
    /// current subpath's most recent starting point (usually, the most recent
    ///
    /// moveto point).
    ///
    pub fn path_close(&mut self) -> &mut Self {
        unsafe { MagickDrawPathClose(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetoabsolute>
    ///
    /// DrawPathCurveToAbsolute() draws a cubic Bezier curve from the current
    ///
    /// point to (x,y) using (x1,y1) as the control point at the beginning of
    ///
    /// the curve and (x2,y2) as the control point at the end of the curve using
    ///
    /// absolute coordinates. At the end of the command, the new current point
    ///
    /// becomes the final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_absolute(
        &mut self,
        x1: c_double,
        y1: c_double,
        x2: c_double,
        y2: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToAbsolute(self.wand.as_ptr(), x1, y1, x2, y2, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetorelative>
    ///
    /// DrawPathCurveToRelative() draws a cubic Bezier curve from the current
    ///
    /// point to (x,y) using (x1,y1) as the control point at the beginning of
    ///
    /// the curve and (x2,y2) as the control point at the end of the curve using
    ///
    /// relative coordinates. At the end of the command, the new current point
    ///
    /// becomes the final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_relative(
        &mut self,
        x1: c_double,
        y1: c_double,
        x2: c_double,
        y2: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToRelative(self.wand.as_ptr(), x1, y1, x2, y2, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetoquadraticbezierabsolute>
    ///
    /// DrawPathCurveToQuadraticBezierAbsolute() draws a quadratic Bezier curve
    ///
    /// from the current point to (x,y) using (x1,y1) as the control point using
    ///
    /// absolute coordinates. At the end of the command, the new current point
    ///
    /// becomes the final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_quadratic_bezier_absolute(
        &mut self,
        x1: c_double,
        y1: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToQuadraticBezierAbsolute(self.wand.as_ptr(), x1, y1, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetoquadraticbezierrelative>
    ///
    /// DrawPathCurveToQuadraticBezierRelative() draws a quadratic Bezier curve
    ///
    /// from the current point to (x,y) using (x1,y1) as the control point using
    ///
    /// relative coordinates. At the end of the command, the new current point
    ///
    /// becomes the final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_quadratic_bezier_relative(
        &mut self,
        x1: c_double,
        y1: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToQuadraticBezierRelative(self.wand.as_ptr(), x1, y1, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetoquadraticbeziersmoothabsolute>
    ///
    /// DrawPathCurveToQuadraticBezierSmoothAbsolute() draws a quadratic
    ///
    /// Bezier curve (using absolute coordinates) from the current point to
    ///
    /// (x,y). The control point is assumed to be the reflection of the
    ///
    /// control point on the previous command relative to the current
    ///
    /// point. (If there is no previous command or if the previous command was
    ///
    /// not a DrawPathCurveToQuadraticBezierAbsolute,
    ///
    /// DrawPathCurveToQuadraticBezierRelative,
    ///
    /// DrawPathCurveToQuadraticBezierSmoothAbsolute or
    ///
    /// DrawPathCurveToQuadraticBezierSmoothRelative, assume the control point
    ///
    /// is coincident with the current point.). At the end of the command, the
    ///
    /// new current point becomes the final (x,y) coordinate pair used in the
    ///
    /// polybezier.
    ///
    pub fn path_curve_to_quadratic_bezier_smooth_absolute(
        &mut self,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToQuadraticBezierSmoothAbsolute(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetoquadraticbeziersmoothrelative>
    ///
    /// DrawPathCurveToQuadraticBezierSmoothRelative() draws a quadratic
    ///
    /// Bezier curve (using relative coordinates) from the current point to
    ///
    /// (x,y). The control point is assumed to be the reflection of the
    ///
    /// control point on the previous command relative to the current
    ///
    /// point. (If there is no previous command or if the previous command was
    ///
    /// not a DrawPathCurveToQuadraticBezierAbsolute,
    ///
    /// DrawPathCurveToQuadraticBezierRelative,
    ///
    /// DrawPathCurveToQuadraticBezierSmoothAbsolute or
    ///
    /// DrawPathCurveToQuadraticBezierSmoothRelative, assume the control point
    ///
    /// is coincident with the current point.). At the end of the command, the
    ///
    /// new current point becomes the final (x,y) coordinate pair used in the
    ///
    /// polybezier.
    ///
    pub fn path_curve_to_quadratic_bezier_smooth_relative(
        &mut self,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToQuadraticBezierSmoothRelative(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetosmoothabsolute>
    ///
    /// DrawPathCurveToSmoothAbsolute() draws a cubic Bezier curve from the
    ///
    /// current point to (x,y) using absolute coordinates. The first control
    ///
    /// point is assumed to be the reflection of the second control point on
    ///
    /// the previous command relative to the current point. (If there is no
    ///
    /// previous command or if the previous command was not an
    ///
    /// DrawPathCurveToAbsolute, DrawPathCurveToRelative,
    ///
    /// DrawPathCurveToSmoothAbsolute or DrawPathCurveToSmoothRelative, assume
    ///
    /// the first control point is coincident with the current point.) (x2,y2)
    ///
    /// is the second control point (i.e., the control point at the end of the
    ///
    /// curve). At the end of the command, the new current point becomes the
    ///
    /// final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_smooth_absolute(
        &mut self,
        _x2: c_double,
        y2: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToSmoothAbsolute(self.wand.as_ptr(), x, y2, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathcurvetosmoothrelative>
    ///
    /// DrawPathCurveToSmoothRelative() draws a cubic Bezier curve from the
    ///
    /// current point to (x,y) using relative coordinates. The first control
    ///
    /// point is assumed to be the reflection of the second control point on
    ///
    /// the previous command relative to the current point. (If there is no
    ///
    /// previous command or if the previous command was not an
    ///
    /// DrawPathCurveToAbsolute, DrawPathCurveToRelative,
    ///
    /// DrawPathCurveToSmoothAbsolute or DrawPathCurveToSmoothRelative, assume
    ///
    /// the first control point is coincident with the current point.) (x2,y2)
    ///
    /// is the second control point (i.e., the control point at the end of the
    ///
    /// curve). At the end of the command, the new current point becomes the
    ///
    /// final (x,y) coordinate pair used in the polybezier.
    ///
    pub fn path_curve_to_smooth_relative(
        &mut self,
        x2: c_double,
        y2: c_double,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawPathCurveToSmoothRelative(self.wand.as_ptr(), x2, y2, x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathellipticarcabsolute>
    ///
    /// DrawPathEllipticArcAbsolute() draws an elliptical arc from the current
    ///
    /// point to (x, y) using absolute coordinates. The size and orientation
    ///
    /// of the ellipse are defined by two radii (rx, ry) and an
    ///
    /// xAxisRotation, which indicates how the ellipse as a whole is rotated
    ///
    /// relative to the current coordinate system. The center (cx, cy) of the
    ///
    /// ellipse is calculated automatically to satisfy the constraints imposed
    ///
    /// by the other parameters. largeArcFlag and sweepFlag contribute to the
    ///
    /// automatic calculations and help determine how the arc is drawn. If
    ///
    /// largeArcFlag is true then draw the larger of the available arcs. If
    ///
    /// sweepFlag is true, then draw the arc matching a clock-wise rotation.
    ///
    pub fn path_elliptic_arc_absolute(
        &mut self,
        rx: c_double,
        ry: c_double,
        x_axis_rotation: c_double,
        large_arc_flag: c_uint,
        sweep_flag: c_uint,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe {
            MagickDrawPathEllipticArcAbsolute(
                self.wand.as_ptr(),
                rx,
                ry,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                x,
                y,
            )
        };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathellipticarcrelative>
    ///
    /// DrawPathEllipticArcRelative() draws an elliptical arc from the current
    ///
    /// point to (x, y) using relative coordinates. The size and orientation
    ///
    /// of the ellipse are defined by two radii (rx, ry) and an
    ///
    /// xAxisRotation, which indicates how the ellipse as a whole is rotated
    ///
    /// relative to the current coordinate system. The center (cx, cy) of the
    ///
    /// ellipse is calculated automatically to satisfy the constraints imposed
    ///
    /// by the other parameters. largeArcFlag and sweepFlag contribute to the
    ///
    /// automatic calculations and help determine how the arc is drawn. If
    ///
    /// largeArcFlag is true then draw the larger of the available arcs. If
    ///
    /// sweepFlag is true, then draw the arc matching a clock-wise rotation.
    ///
    pub fn path_elliptic_arc_relative(
        &mut self,
        rx: c_double,
        ry: c_double,
        x_axis_rotation: c_double,
        large_arc_flag: c_uint,
        sweep_flag: c_uint,
        x: c_double,
        y: c_double,
    ) -> &mut Self {
        unsafe {
            MagickDrawPathEllipticArcRelative(
                self.wand.as_ptr(),
                rx,
                ry,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                x,
                y,
            )
        };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathfinish>
    ///
    /// DrawPathFinish() terminates the current path.
    ///
    pub fn path_finish(&mut self) -> &mut Self {
        unsafe { MagickDrawPathFinish(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetoabsolute>
    ///
    /// DrawPathLineToAbsolute() draws a line path from the current point to the
    ///
    /// given coordinate using absolute coordinates. The coordinate then becomes
    ///
    /// the new current point.
    ///
    pub fn path_line_to_absolute(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToAbsolute(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetorelative>
    ///
    /// DrawPathLineToRelative() draws a line path from the current point to the
    ///
    /// given coordinate using relative coordinates. The coordinate then becomes
    ///
    /// the new current point.
    ///
    pub fn path_line_to_relative(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToRelative(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetohorizontalabsolute>
    ///
    /// DrawPathLineToHorizontalAbsolute() draws a horizontal line path from the
    ///
    /// current point to the target point using absolute coordinates.  The target
    ///
    /// point then becomes the new current point.
    ///
    pub fn path_line_to_horizontal_absolute(&mut self, x: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToHorizontalAbsolute(self.wand.as_ptr(), x) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetohorizontalrelative>
    ///
    /// DrawPathLineToHorizontalRelative() draws a horizontal line path from the
    ///
    /// current point to the target point using relative coordinates.  The target
    ///
    /// point then becomes the new current point.
    ///
    pub fn path_line_to_horizontal_relative(&mut self, x: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToHorizontalRelative(self.wand.as_ptr(), x) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetoverticalabsolute>
    ///
    /// DrawPathLineToVerticalAbsolute() draws a vertical line path from the
    ///
    /// current point to the target point using absolute coordinates.  The target
    ///
    /// point then becomes the new current point.
    ///
    pub fn path_line_to_vertical_absolute(&mut self, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToVerticalAbsolute(self.wand.as_ptr(), y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathlinetoverticalrelative>
    ///
    /// DrawPathLineToVerticalRelative() draws a vertical line path from the
    ///
    /// current point to the target point using relative coordinates.  The target
    ///
    /// point then becomes the new current point.
    ///
    pub fn path_line_to_vertical_relative(&mut self, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathLineToVerticalRelative(self.wand.as_ptr(), y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathmovetoabsolute>
    ///
    /// DrawPathMoveToAbsolute() starts a new sub-path at the given coordinate
    ///
    /// using absolute coordinates. The current point then becomes the
    ///
    /// specified coordinate.
    ///
    pub fn path_move_to_absolute(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathMoveToAbsolute(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathmovetorelative>
    ///
    /// DrawPathMoveToRelative() starts a new sub-path at the given coordinate
    ///
    /// using relative coordinates. The current point then becomes the
    ///
    /// specified coordinate.
    ///
    pub fn path_move_to_relative(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawPathMoveToRelative(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpathstart>
    ///
    /// DrawPathStart() declares the start of a path drawing list which is terminated
    ///
    /// by a matching DrawPathFinish() command. All other DrawPath commands must
    ///
    /// be enclosed between a DrawPathStart() and a DrawPathFinish() command. This
    ///
    /// is because path drawing commands are subordinate commands and they do not
    ///
    /// function by themselves.
    ///
    pub fn path_start(&mut self) -> &mut Self {
        unsafe { MagickDrawPathStart(self.wand.as_ptr()) };
        self
    }

    //    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpeekgraphiccontext>
    //    ///
    //    /// DrawPeekGraphicContext() returns the current graphic drawing_wand.
    //    ///
    //    pub fn peek_graphic_context(&self) -> &DrawInfo {
    //        let status = unsafe { MagickDrawPeekGraphicContext(self.wand.as_ptr()) };
    //        todo!()
    //    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpoint>
    ///
    /// DrawPoint() draws a point using the current stroke color and stroke
    ///
    /// thickness at the specified coordinates.
    ///
    pub fn point(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawPoint(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpolygon>
    ///
    /// DrawPolygon() draws a polygon using the current stroke, stroke width, and
    ///
    /// fill color or texture, using the specified array of coordinates.
    ///
    pub fn polygon(&mut self, number_coordinates: c_ulong, coordinates: &PointInfo) -> &mut Self {
        unsafe { MagickDrawPolygon(self.wand.as_ptr(), number_coordinates, coordinates) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpolyline>
    ///
    /// DrawPolyline() draws a polyline using the current stroke, stroke width, and
    ///
    /// fill color or texture, using the specified array of coordinates.
    ///
    pub fn polyline(&mut self, number_coordinates: c_ulong, coordinates: &PointInfo) -> &mut Self {
        unsafe { MagickDrawPolyline(self.wand.as_ptr(), number_coordinates, coordinates) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpopclippath>
    ///
    /// DrawPopClipPath() terminates a clip path definition.
    ///
    pub fn pop_clip_path(&mut self) -> &mut Self {
        unsafe { MagickDrawPopClipPath(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpopdefs>
    ///
    /// DrawPopDefs() terminates a definition list
    ///
    pub fn pop_defs(&mut self) -> &mut Self {
        unsafe { MagickDrawPopDefs(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpopgraphiccontext>
    ///
    /// DrawPopGraphicContext() destroys the current drawing_wand returning to the
    ///
    /// previously pushed drawing wand. Multiple drawing wand  may exist. It is an
    ///
    /// error to attempt to pop more drawing_wands than have been pushed, and it is
    ///
    /// proper form to pop all drawing_wands which have been pushed.
    ///
    pub fn pop_graphic_context(&mut self) -> &mut Self {
        unsafe { MagickDrawPopGraphicContext(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpoppattern>
    ///
    /// DrawPopPattern() terminates a pattern definition.
    ///
    pub fn pop_pattern(&mut self) -> &mut Self {
        unsafe { MagickDrawPopPattern(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpushclippath>
    ///
    /// DrawPushClipPath() starts a clip path definition which is comprized of
    ///
    /// any number of drawing commands and terminated by a DrawPopClipPath()
    ///
    /// command.
    ///
    pub fn push_clip_path<'a>(
        &mut self,
        clip_path_id: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let clip_path_id = clip_path_id.into_null_terminated_string();
        unsafe { MagickDrawPushClipPath(self.wand.as_ptr(), clip_path_id.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpushdefs>
    ///
    /// DrawPushDefs() indicates that commands up to a terminating DrawPopDefs()
    ///
    /// command create named elements (e.g. clip-paths, textures, etc.) which
    ///
    /// may safely be processed earlier for the sake of efficiency.
    ///
    pub fn push_defs(&mut self) -> &mut Self {
        unsafe { MagickDrawPushDefs(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpushgraphiccontext>
    ///
    /// DrawPushGraphicContext() clones the current drawing wand to create a
    ///
    /// new drawing wand. The original drawing drawing_wand(s) may be returned to
    ///
    /// by invoking DrawPopGraphicContext().  The drawing wands are stored on a
    ///
    /// drawing wand stack.  For every Pop there must have already been an
    ///
    /// equivalent Push.
    ///
    pub fn push_graphic_context(&mut self) -> &mut Self {
        unsafe { MagickDrawPushGraphicContext(self.wand.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawpushpattern>
    ///
    /// DrawPushPattern() indicates that subsequent commands up to a
    ///
    /// DrawPopPattern() command comprise the definition of a named pattern.
    ///
    /// The pattern space is assigned top left corner coordinates, a width
    ///
    /// and height, and becomes its own drawing space.  Anything which can
    ///
    /// be drawn may be used in a pattern definition.
    ///
    /// Named patterns may be used as stroke or brush definitions.
    ///
    pub fn push_pattern<'a>(
        &mut self,
        pattern_id: impl IntoNullTerminatedString<'a>,
        x: c_double,
        y: c_double,
        width: c_double,
        height: c_double,
    ) -> &mut Self {
        let pattern_id = pattern_id.into_null_terminated_string();
        unsafe {
            MagickDrawPushPattern(self.wand.as_ptr(), pattern_id.as_ptr(), x, y, width, height)
        };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawrectangle>
    ///
    /// DrawRectangle() draws a rectangle given two coordinates and using
    ///
    /// the current stroke, stroke width, and fill settings.
    ///
    pub fn rectangle(
        &mut self,
        x1: c_double,
        y1: c_double,
        x2: c_double,
        y2: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawRectangle(self.wand.as_ptr(), x1, y1, x2, y2) };
        self
    }

    //    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawrender>
    //    ///
    //    /// DrawRender() renders all preceding drawing commands onto the image.
    //    ///
    //    /// This function is deprecated.  Use MagickDrawImage() instead.
    //    ///
    //    pub fn render(&self) {
    //        unsafe { MagickDrawRender(self.wand.as_ptr()) };
    //    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawrotate>
    ///
    /// DrawRotate() applies the specified rotation to the current coordinate space.
    ///
    pub fn rotate(&mut self, degrees: c_double) -> &mut Self {
        unsafe { MagickDrawRotate(self.wand.as_ptr(), degrees) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawroundrectangle>
    ///
    /// DrawRoundRectangle() draws a rounted rectangle given two coordinates,
    ///
    /// x &amp; y corner radiuses and using the current stroke, stroke width,
    ///
    /// and fill settings.
    ///
    pub fn round_rectangle(
        &mut self,
        x1: c_double,
        y1: c_double,
        x2: c_double,
        y2: c_double,
        rx: c_double,
        ry: c_double,
    ) -> &mut Self {
        unsafe { MagickDrawRoundRectangle(self.wand.as_ptr(), x1, y1, x2, y2, rx, ry) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawscale>
    ///
    /// DrawScale() adjusts the scaling factor to apply in the horizontal and
    ///
    /// vertical directions to the current coordinate space.
    ///
    pub fn scale(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawScale(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawskewx>
    ///
    /// DrawSkewX() skews the current coordinate system in the horizontal
    ///
    /// direction.
    ///
    pub fn skew_x(&mut self, degrees: c_double) -> &mut Self {
        unsafe { MagickDrawSkewX(self.wand.as_ptr(), degrees) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawskewy>
    ///
    /// DrawSkewY() skews the current coordinate system in the vertical
    ///
    /// direction.
    ///
    pub fn skew_y(&mut self, degrees: c_double) -> &mut Self {
        unsafe { MagickDrawSkewY(self.wand.as_ptr(), degrees) };
        self
    }

    // This method has commented.
    // /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstopcolor>
    // ///
    // /// DrawSetStopColor() sets the stop color and offset for gradients
    // ///
    // pub fn set_stop_color(&mut self, stop_color: &PixelPacket, offset: c_double) {
    // // unsafe { MagickDrawSetStopColor(self.wand.as_ptr(), stop_color, offset) };
    // }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokecolor>
    ///
    /// DrawGetStrokeColor() returns the color used for stroking object outlines.
    ///
    pub fn get_stroke_color(&self) -> PixelWand {
        let mut pw = PixelWand::new();
        unsafe { MagickDrawGetStrokeColor(self.wand.as_ptr(), pw.wand_mut()) };
        pw
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokecolor>
    ///
    /// DrawSetStrokeColor() sets the color used for stroking object outlines.
    ///
    pub fn set_stroke_color(&mut self, stroke_wand: &PixelWand) -> &mut Self {
        unsafe { MagickDrawSetStrokeColor(self.wand.as_ptr(), stroke_wand.wand()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokepatternurl>
    ///
    /// DrawSetStrokePatternURL() sets the pattern used for stroking object outlines.
    ///
    pub fn set_stroke_pattern_url<'a>(
        &mut self,
        stroke_url: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let stroke_url = stroke_url.into_null_terminated_string();
        unsafe { MagickDrawSetStrokePatternURL(self.wand.as_ptr(), stroke_url.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokeantialias>
    ///
    /// DrawGetStrokeAntialias() returns the current stroke antialias setting.
    ///
    /// Stroked outlines are antialiased by default.  When antialiasing is disabled
    ///
    /// stroked pixels are thresholded to determine if the stroke color or
    ///
    /// underlying canvas color should be used.
    ///
    pub fn get_stroke_antialias(&self) -> c_uint {
        unsafe { MagickDrawGetStrokeAntialias(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokeantialias>
    ///
    /// DrawSetStrokeAntialias() controls whether stroked outlines are antialiased.
    ///
    /// Stroked outlines are antialiased by default.  When antialiasing is disabled
    ///
    /// stroked pixels are thresholded to determine if the stroke color or
    ///
    /// underlying canvas color should be used.
    ///
    pub fn set_stroke_antialias(&mut self, stroke_antialias: c_uint) -> &mut Self {
        unsafe { MagickDrawSetStrokeAntialias(self.wand.as_ptr(), stroke_antialias) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokedasharray>
    ///
    /// DrawGetStrokeDashArray() returns an array representing the pattern of
    ///
    /// dashes and gaps used to stroke paths (see DrawSetStrokeDashArray). The
    ///
    /// array must be freed once it is no longer required by the user.
    ///
    pub fn get_stroke_dash_array(&self) -> Option<MagickBoxSlice<c_double>> {
        let mut number_elements = 0;
        let a = unsafe { MagickDrawGetStrokeDashArray(self.wand.as_ptr(), &mut number_elements) };
        unsafe { MagickBoxSlice::new(a, number_elements.try_into().unwrap()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokedasharray>
    ///
    /// DrawSetStrokeDashArray() specifies the pattern of dashes and gaps used to
    ///
    /// stroke paths. The stroke dash array represents an array of numbers that
    ///
    /// specify the lengths of alternating dashes and gaps in pixels. If an odd
    ///
    /// number of values is provided, then the list of values is repeated to yield
    ///
    /// an even number of values. To remove an existing dash array, pass a zero
    ///
    /// number_elements argument and null dash_array.
    ///
    /// A typical stroke dash array might contain the members 5 3 2.
    ///
    pub fn set_stroke_dash_array(&mut self, dash: &[c_double]) -> &mut Self {
        unsafe {
            MagickDrawSetStrokeDashArray(self.wand.as_ptr(), dash.len() as c_ulong, dash.as_ptr())
        };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokedashoffset>
    ///
    /// DrawGetStrokeDashOffset() returns the offset into the dash pattern to
    ///
    /// start the dash.
    ///
    pub fn get_stroke_dash_offset(&self) -> c_double {
        unsafe { MagickDrawGetStrokeDashOffset(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokedashoffset>
    ///
    /// DrawSetStrokeDashOffset() specifies the offset into the dash pattern to
    ///
    /// start the dash.
    ///
    pub fn set_stroke_dash_offset(&mut self, dash_offset: c_double) -> &mut Self {
        unsafe { MagickDrawSetStrokeDashOffset(self.wand.as_ptr(), dash_offset) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokelinecap>
    ///
    /// DrawGetStrokeLineCap() returns the shape to be used at the end of
    ///
    /// open subpaths when they are stroked. Values of LineCap are
    ///
    /// UndefinedCap, ButtCap, RoundCap, and SquareCap.
    ///
    pub fn get_stroke_line_cap(&self) -> LineCap {
        unsafe { MagickDrawGetStrokeLineCap(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokelinecap>
    ///
    /// DrawSetStrokeLineCap() specifies the shape to be used at the end of
    ///
    /// open subpaths when they are stroked. Values of LineCap are
    ///
    /// UndefinedCap, ButtCap, RoundCap, and SquareCap.
    ///
    pub fn set_stroke_line_cap(&mut self, linecap: LineCap) -> &mut Self {
        unsafe { MagickDrawSetStrokeLineCap(self.wand.as_ptr(), linecap.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokelinejoin>
    ///
    /// DrawGetStrokeLineJoin() returns the shape to be used at the
    ///
    /// corners of paths (or other vector shapes) when they are
    ///
    /// stroked. Values of LineJoin are UndefinedJoin, MiterJoin, RoundJoin,
    ///
    /// and BevelJoin.
    ///
    pub fn get_stroke_line_join(&self) -> LineJoin {
        unsafe { MagickDrawGetStrokeLineJoin(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokelinejoin>
    ///
    /// DrawSetStrokeLineJoin() specifies the shape to be used at the
    ///
    /// corners of paths (or other vector shapes) when they are
    ///
    /// stroked. Values of LineJoin are UndefinedJoin, MiterJoin, RoundJoin,
    ///
    /// and BevelJoin.
    ///
    pub fn set_stroke_line_join(&mut self, linejoin: LineJoin) -> &mut Self {
        unsafe { MagickDrawSetStrokeLineJoin(self.wand.as_ptr(), linejoin.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokemiterlimit>
    ///
    /// DrawGetStrokeMiterLimit() returns the miter limit. When two line
    ///
    /// segments meet at a sharp angle and miter joins have been specified for
    ///
    /// 'lineJoin', it is possible for the miter to extend far beyond the
    ///
    /// thickness of the line stroking the path. The miterLimit' imposes a
    ///
    /// limit on the ratio of the miter length to the 'lineWidth'.
    ///
    pub fn get_stroke_miter_limit(&self) -> c_ulong {
        unsafe { MagickDrawGetStrokeMiterLimit(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokemiterlimit>
    ///
    /// DrawSetStrokeMiterLimit() specifies the miter limit. When two line
    ///
    /// segments meet at a sharp angle and miter joins have been specified for
    ///
    /// 'lineJoin', it is possible for the miter to extend far beyond the
    ///
    /// thickness of the line stroking the path. The miterLimit' imposes a
    ///
    /// limit on the ratio of the miter length to the 'lineWidth'.
    ///
    pub fn set_stroke_miter_limit(&mut self, miterlimit: c_ulong) -> &mut Self {
        unsafe { MagickDrawSetStrokeMiterLimit(self.wand.as_ptr(), miterlimit) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokeopacity>
    ///
    /// DrawGetStrokeOpacity() returns the opacity of stroked object outlines.
    ///
    pub fn get_stroke_opacity(&self) -> c_double {
        unsafe { MagickDrawGetStrokeOpacity(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokeopacity>
    ///
    /// DrawSetStrokeOpacity() specifies the opacity of stroked object outlines.
    ///
    pub fn set_stroke_opacity(&mut self, stroke_opacity: c_double) -> &mut Self {
        unsafe { MagickDrawSetStrokeOpacity(self.wand.as_ptr(), stroke_opacity) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgetstrokewidth>
    ///
    /// DrawGetStrokeWidth() returns the width of the stroke used to draw object
    ///
    /// outlines.
    ///
    pub fn get_stroke_width(&self) -> f64 {
        unsafe { MagickDrawGetStrokeWidth(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetstrokewidth>
    ///
    /// DrawSetStrokeWidth() sets the width of the stroke used to draw object
    ///
    /// outlines.
    ///
    pub fn set_stroke_width(&mut self, stroke_width: c_double) -> &mut Self {
        unsafe { MagickDrawSetStrokeWidth(self.wand.as_ptr(), stroke_width) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgettextantialias>
    ///
    /// DrawGetTextAntialias() returns the current text antialias setting, which
    ///
    /// determines whether text is antialiased.  Text is antialiased by default.
    ///
    pub fn get_text_antialias(&self) -> c_uint {
        unsafe { MagickDrawGetTextAntialias(self.wand.as_ptr()) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsettextantialias>
    ///
    /// DrawSetTextAntialias() controls whether text is antialiased.  Text is
    ///
    /// antialiased by default.
    ///
    pub fn set_text_antialias(&mut self, text_antialias: c_uint) -> &mut Self {
        unsafe { MagickDrawSetTextAntialias(self.wand.as_ptr(), text_antialias) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgettextdecoration>
    ///
    /// DrawGetTextDecoration() returns the decoration applied when annotating with
    ///
    /// text.
    ///
    pub fn get_text_decoration(&mut self) -> DecorationType {
        unsafe { MagickDrawGetTextDecoration(self.wand.as_ptr()) }.into()
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsettextdecoration>
    ///
    /// DrawSetTextDecoration() specifies a decoration to be applied when
    ///
    /// annotating with text.
    ///
    pub fn set_text_decoration(&mut self, decoration: DecorationType) -> &mut Self {
        unsafe { MagickDrawSetTextDecoration(self.wand.as_ptr(), decoration.into()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgettextencoding>
    ///
    /// DrawGetTextEncoding() returns a null-terminated string which specifies the
    ///
    /// code set used for text annotations. The string must be freed by the user
    ///
    /// once it is no longer required.
    ///
    pub fn get_text_encoding(&self) -> MagickCString {
        unsafe { MagickCString::new(MagickDrawGetTextEncoding(self.wand.as_ptr())) }
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsettextencoding>
    ///
    /// DrawSetTextEncoding() specifies specifies the code set to use for
    ///
    /// text annotations. The only character encoding which may be specified
    ///
    /// at this time is &quot;UTF-8&quot; for representing Unicode as a sequence of
    ///
    /// bytes. Specify an empty string to set text encoding to the system's
    ///
    /// default. Successful text annotation using Unicode may require fonts
    ///
    /// designed to support Unicode.
    ///
    pub fn set_text_encoding<'a>(
        &mut self,
        encoding: impl IntoNullTerminatedString<'a>,
    ) -> &mut Self {
        let encoding = encoding.into_null_terminated_string();
        unsafe { MagickDrawSetTextEncoding(self.wand.as_ptr(), encoding.as_ptr()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawgettextundercolor>
    ///
    /// DrawGetTextUnderColor() returns the color of a background rectangle
    ///
    /// to place under text annotations.
    ///
    pub fn get_text_under_color(&self) -> PixelWand {
        let mut under_color = PixelWand::new();
        unsafe { MagickDrawGetTextUnderColor(self.wand.as_ptr(), under_color.wand_mut()) };
        under_color
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsettextundercolor>
    ///
    /// DrawSetTextUnderColor() specifies the color of a background rectangle
    ///
    /// to place under text annotations.
    ///
    pub fn set_text_under_color(&mut self, under_wand: &PixelWand) -> &mut Self {
        unsafe { MagickDrawSetTextUnderColor(self.wand.as_ptr(), under_wand.wand()) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawtranslate>
    ///
    /// DrawTranslate() applies a translation to the current coordinate
    ///
    /// system which moves the coordinate system origin to the specified
    ///
    /// coordinate.
    ///
    pub fn translate(&mut self, x: c_double, y: c_double) -> &mut Self {
        unsafe { MagickDrawTranslate(self.wand.as_ptr(), x, y) };
        self
    }

    /// <http://www.graphicsmagick.org/wand/drawing_wand.html#drawsetviewbox>
    ///
    /// DrawSetViewbox() sets the overall canvas size to be recorded with the
    ///
    /// drawing vector data.  Usually this will be specified using the same
    ///
    /// size as the canvas image.  When the vector data is saved to SVG or MVG
    ///
    /// formats, the viewbox is use to specify the size of the canvas image that
    ///
    /// a viewer will render the vector data on.
    ///
    pub fn set_viewbox(&mut self, x1: c_ulong, y1: c_ulong, x2: c_ulong, y2: c_ulong) -> &mut Self {
        unsafe { MagickDrawSetViewbox(self.wand.as_ptr(), x1, y1, x2, y2) };
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        initialize,
        null_terminated_str::const_null_terminated_str,
        types::{
            AffineMatrix, ClipPathUnits, DecorationType, FillRule, GravityType, LineCap, LineJoin,
            PaintMethod, PointInfo, StretchType, StyleType,
        },
        wand::{DrawingWand, PixelWand},
    };

    fn new_logo_drawing_wand() -> DrawingWand {
        initialize();
        DrawingWand::new()
    }

    #[test]
    fn test_drawing_wand_annotation() {
        new_logo_drawing_wand().annotation(0., 0., "");
        new_logo_drawing_wand().annotation(0., 0., const_null_terminated_str!(""));
    }

    #[test]
    fn test_drawing_wand_affine() {
        let mut dw = new_logo_drawing_wand();
        dw.affine(&AffineMatrix {
            sx: 0.,
            rx: 0.,
            ry: 0.,
            sy: 0.,
            tx: 0.,
            ty: 0.,
        });
    }

    #[test]
    fn test_drawing_wand_arc() {
        let mut dw = new_logo_drawing_wand();
        dw.arc(0., 0., 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_bezier() {
        let mut dw = new_logo_drawing_wand();
        dw.bezier(0, &PointInfo { x: 0., y: 0. });
    }

    #[test]
    fn test_drawing_wand_circle() {
        let mut dw = new_logo_drawing_wand();
        dw.circle(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_get_clip_path() {
        let dw = new_logo_drawing_wand();
        dw.get_clip_path().to_str().unwrap();
    }

    #[test]
    fn test_drawing_wand_set_clip_path() {
        let mut dw = new_logo_drawing_wand();
        dw.set_clip_path("");
    }

    #[test]
    fn test_drawing_wand_set_clip_rule() {
        let mut dw = new_logo_drawing_wand();
        dw.set_clip_rule(FillRule::UndefinedRule);
    }

    #[test]
    fn test_drawing_wand_get_clip_units() {
        let dw = new_logo_drawing_wand();
        dw.get_clip_units();
    }

    #[test]
    fn test_drawing_wand_set_clip_units() {
        let mut dw = new_logo_drawing_wand();
        dw.set_clip_units(ClipPathUnits::ObjectBoundingBox);
    }

    #[test]
    fn test_drawing_wand_color() {
        let mut dw = new_logo_drawing_wand();
        dw.color(0., 0., PaintMethod::PointMethod);
    }

    #[test]
    fn test_drawing_wand_comment() {
        let mut dw = new_logo_drawing_wand();
        dw.comment("");
    }

    #[test]
    fn test_drawing_wand_ellipse() {
        let mut dw = new_logo_drawing_wand();
        dw.ellipse(0., 0., 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_get_fill_color() {
        let dw = new_logo_drawing_wand();
        dw.get_fill_color();
    }

    #[test]
    fn test_drawing_wand_set_fill_color() {
        let mut dw = new_logo_drawing_wand();
        dw.set_fill_color(&PixelWand::new());
    }

    #[test]
    fn test_drawing_wand_set_fill_pattern_url() {
        let mut dw = new_logo_drawing_wand();
        dw.set_fill_pattern_url("");
    }

    #[test]
    fn test_drawing_wand_get_fill_opacity() {
        let dw = new_logo_drawing_wand();
        dw.get_fill_opacity();
    }

    #[test]
    fn test_drawing_wand_set_fill_opacity() {
        let mut dw = new_logo_drawing_wand();
        dw.set_fill_opacity(0.);
    }

    #[test]
    fn test_drawing_wand_get_fill_rule() {
        let dw = new_logo_drawing_wand();
        dw.get_fill_rule();
    }

    #[test]
    fn test_drawing_wand_set_fill_rule() {
        let mut dw = new_logo_drawing_wand();
        dw.set_fill_rule(FillRule::UndefinedRule);
    }

    #[test]
    fn test_drawing_wand_get_font() {
        let dw = new_logo_drawing_wand();
        dw.get_font().to_str().unwrap();
    }

    #[test]
    fn test_drawing_wand_set_font() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font("");
    }

    #[test]
    fn test_drawing_wand_get_font_family() {
        let dw = new_logo_drawing_wand();
        dw.get_font_family().to_str().unwrap();
    }

    #[test]
    fn test_drawing_wand_set_font_family() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font_family("");
    }

    #[test]
    fn test_drawing_wand_get_font_size() {
        let dw = new_logo_drawing_wand();
        dw.get_font_size();
    }

    #[test]
    fn test_drawing_wand_set_font_size() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font_size(0.);
    }

    #[test]
    fn test_drawing_wand_get_font_stretch() {
        let dw = new_logo_drawing_wand();
        dw.get_font_stretch();
    }

    #[test]
    fn test_drawing_wand_set_font_stretch() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font_stretch(StretchType::NormalStretch);
    }

    #[test]
    fn test_drawing_wand_get_font_style() {
        let dw = new_logo_drawing_wand();
        dw.get_font_style();
    }

    #[test]
    fn test_drawing_wand_set_font_style() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font_style(StyleType::NormalStyle);
    }

    #[test]
    fn test_drawing_wand_get_font_weight() {
        let dw = new_logo_drawing_wand();
        dw.get_font_weight();
    }

    #[test]
    fn test_drawing_wand_set_font_weight() {
        let mut dw = new_logo_drawing_wand();
        dw.set_font_weight(0);
    }

    #[test]
    fn test_drawing_wand_get_gravity() {
        let dw = new_logo_drawing_wand();
        dw.get_gravity();
    }

    #[test]
    fn test_drawing_wand_set_gravity() {
        let mut dw = new_logo_drawing_wand();
        dw.set_gravity(GravityType::ForgetGravity);
    }

    #[test]
    fn test_drawing_wand_line() {
        let mut dw = new_logo_drawing_wand();
        dw.line(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_matte() {
        let mut dw = new_logo_drawing_wand();
        dw.matte(0., 0., PaintMethod::PointMethod);
    }

    #[test]
    fn test_drawing_wand_path_close() {
        let mut dw = new_logo_drawing_wand();
        dw.path_close();
    }

    #[test]
    fn test_drawing_wand_path_curve_to_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_absolute(0., 0., 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_relative(0., 0., 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_quadratic_bezier_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_quadratic_bezier_absolute(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_quadratic_bezier_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_quadratic_bezier_relative(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_quadratic_bezier_smooth_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_quadratic_bezier_smooth_absolute(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_quadratic_bezier_smooth_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_quadratic_bezier_smooth_relative(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_smooth_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_smooth_absolute(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_curve_to_smooth_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_curve_to_smooth_relative(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_elliptic_arc_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_elliptic_arc_absolute(0., 0., 0., 0, 0, 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_elliptic_arc_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_elliptic_arc_relative(0., 0., 0., 0, 0, 0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_finish() {
        let mut dw = new_logo_drawing_wand();
        dw.path_finish();
    }

    #[test]
    fn test_drawing_wand_path_line_to_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_absolute(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_line_to_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_relative(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_line_to_horizontal_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_horizontal_absolute(0.);
    }

    #[test]
    fn test_drawing_wand_path_line_to_horizontal_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_horizontal_relative(0.);
    }

    #[test]
    fn test_drawing_wand_path_line_to_vertical_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_vertical_absolute(0.);
    }

    #[test]
    fn test_drawing_wand_path_line_to_vertical_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_line_to_vertical_relative(0.);
    }

    #[test]
    fn test_drawing_wand_path_move_to_absolute() {
        let mut dw = new_logo_drawing_wand();
        dw.path_move_to_absolute(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_move_to_relative() {
        let mut dw = new_logo_drawing_wand();
        dw.path_move_to_relative(0., 0.);
    }

    #[test]
    fn test_drawing_wand_path_start() {
        let mut dw = new_logo_drawing_wand();
        dw.path_start();
    }

    #[test]
    fn test_drawing_wand_point() {
        let mut dw = new_logo_drawing_wand();
        dw.point(0., 0.);
    }

    #[test]
    fn test_drawing_wand_polygon() {
        let mut dw = new_logo_drawing_wand();
        dw.polygon(0, &PointInfo { x: 0., y: 0. });
    }

    #[test]
    fn test_drawing_wand_polyline() {
        let mut dw = new_logo_drawing_wand();
        dw.polyline(0, &PointInfo { x: 0., y: 0. });
    }

    #[test]
    fn test_drawing_wand_pop_clip_path() {
        let mut dw = new_logo_drawing_wand();
        dw.pop_clip_path();
    }

    #[test]
    fn test_drawing_wand_pop_defs() {
        let mut dw = new_logo_drawing_wand();
        dw.pop_defs();
    }

    #[test]
    fn test_drawing_wand_pop_graphic_context() {
        let mut dw = new_logo_drawing_wand();
        dw.pop_graphic_context();
    }

    #[test]
    fn test_drawing_wand_pop_pattern() {
        let mut dw = new_logo_drawing_wand();
        dw.pop_pattern();
    }

    #[test]
    fn test_drawing_wand_push_clip_path() {
        let mut dw = new_logo_drawing_wand();
        dw.push_clip_path("");
    }

    #[test]
    fn test_drawing_wand_push_defs() {
        let mut dw = new_logo_drawing_wand();
        dw.push_defs();
    }

    #[test]
    fn test_drawing_wand_push_graphic_context() {
        let mut dw = new_logo_drawing_wand();
        dw.push_graphic_context();
    }

    #[test]
    fn test_drawing_wand_push_pattern() {
        let mut dw = new_logo_drawing_wand();
        dw.push_pattern("", 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_rectangle() {
        let mut dw = new_logo_drawing_wand();
        dw.rectangle(0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_rotate() {
        let mut dw = new_logo_drawing_wand();
        dw.rotate(0.);
    }

    #[test]
    fn test_drawing_wand_round_rectangle() {
        let mut dw = new_logo_drawing_wand();
        dw.round_rectangle(0., 0., 0., 0., 0., 0.);
    }

    #[test]
    fn test_drawing_wand_scale() {
        let mut dw = new_logo_drawing_wand();
        dw.scale(0., 0.);
    }

    #[test]
    fn test_drawing_wand_skew_x() {
        let mut dw = new_logo_drawing_wand();
        dw.skew_x(0.);
    }

    #[test]
    fn test_drawing_wand_skew_y() {
        let mut dw = new_logo_drawing_wand();
        dw.skew_y(0.);
    }

    #[test]
    fn test_drawing_wand_get_stroke_color() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_color();
    }

    #[test]
    fn test_drawing_wand_set_stroke_color() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_color(&PixelWand::new());
    }

    #[test]
    fn test_drawing_wand_set_stroke_pattern_url() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_pattern_url("");
    }

    #[test]
    fn test_drawing_wand_get_stroke_antialias() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_antialias();
    }

    #[test]
    fn test_drawing_wand_set_stroke_antialias() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_antialias(0);
    }

    #[test]
    fn test_drawing_wand_get_stroke_dash_array() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_dash_array();
    }

    #[test]
    fn test_drawing_wand_set_stroke_dash_array() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_dash_array(&[]);
    }

    #[test]
    fn test_drawing_wand_get_stroke_dash_offset() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_dash_offset();
    }

    #[test]
    fn test_drawing_wand_set_stroke_dash_offset() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_dash_offset(0.);
    }

    #[test]
    fn test_drawing_wand_get_stroke_line_cap() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_line_cap();
    }

    #[test]
    fn test_drawing_wand_set_stroke_line_cap() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_line_cap(LineCap::UndefinedCap);
    }

    #[test]
    fn test_drawing_wand_get_stroke_line_join() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_line_join();
    }

    #[test]
    fn test_drawing_wand_set_stroke_line_join() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_line_join(LineJoin::UndefinedJoin);
    }

    #[test]
    fn test_drawing_wand_get_stroke_miter_limit() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_miter_limit();
    }

    #[test]
    fn test_drawing_wand_set_stroke_miter_limit() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_miter_limit(0);
    }

    #[test]
    fn test_drawing_wand_get_stroke_opacity() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_opacity();
    }

    #[test]
    fn test_drawing_wand_set_stroke_opacity() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_opacity(0.);
    }

    #[test]
    fn test_drawing_wand_get_stroke_width() {
        let dw = new_logo_drawing_wand();
        dw.get_stroke_width();
    }

    #[test]
    fn test_drawing_wand_set_stroke_width() {
        let mut dw = new_logo_drawing_wand();
        dw.set_stroke_width(0.);
    }

    #[test]
    fn test_drawing_wand_get_text_antialias() {
        let dw = new_logo_drawing_wand();
        dw.get_text_antialias();
    }

    #[test]
    fn test_drawing_wand_set_text_antialias() {
        let mut dw = new_logo_drawing_wand();
        dw.set_text_antialias(0);
    }

    #[test]
    fn test_drawing_wand_get_text_decoration() {
        let mut dw = new_logo_drawing_wand();
        dw.get_text_decoration();
    }

    #[test]
    fn test_drawing_wand_set_text_decoration() {
        let mut dw = new_logo_drawing_wand();
        dw.set_text_decoration(DecorationType::NoDecoration);
    }

    #[test]
    fn test_drawing_wand_get_text_encoding() {
        let dw = new_logo_drawing_wand();
        dw.get_text_encoding().to_str().unwrap();
    }

    #[test]
    fn test_drawing_wand_set_text_encoding() {
        let mut dw = new_logo_drawing_wand();
        dw.set_text_encoding("");
    }

    #[test]
    fn test_drawing_wand_get_text_under_color() {
        let dw = new_logo_drawing_wand();
        dw.get_text_under_color();
    }

    #[test]
    fn test_drawing_wand_set_text_under_color() {
        let mut dw = new_logo_drawing_wand();
        dw.set_text_under_color(&PixelWand::new());
    }

    #[test]
    fn test_drawing_wand_translate() {
        let mut dw = new_logo_drawing_wand();
        dw.translate(0., 0.);
    }

    #[test]
    fn test_drawing_wand_set_viewbox() {
        let mut dw = new_logo_drawing_wand();
        dw.set_viewbox(0, 0, 0, 0);
    }
}
