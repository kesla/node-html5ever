use super::StyleDeclaration;

macro_rules! impl_style_getters_and_setters {
    ({ $($field:ident),* }) => {
      paste::paste! {
        #[napi]
        impl StyleDeclaration {
          $(
            #[napi(getter)]
            pub fn [<get_ $field:snake>](&self) -> String {
              self.get_property_value(stringify!($field).into())
            }

            #[napi(setter)]
            pub fn [<set_ $field:snake>](&mut self, value: Option<String>) -> napi::Result<()> {
              if let Some(value) = value {
                self.set_property(stringify!($field).into(), value, None)
              } else {
                self.remove_property(stringify!($field).into())?;
                Ok(())
              }
            }
          )*
        }
      }
    };
  }

// these comes from https://raw.githubusercontent.com/microsoft/TypeScript/main/lib/lib.dom.d.ts
impl_style_getters_and_setters!({
  accentColor, alignContent, alignItems, alignSelf, alignmentBaseline, all,
  animation, animationDelay, animationDirection, animationDuration, animationFillMode, animationIterationCount,
  animationName, animationPlayState, animationTimingFunction, appearance, aspectRatio, backfaceVisibility,
  background, backgroundAttachment, backgroundBlendMode, backgroundClip, backgroundColor, backgroundImage,
  backgroundOrigin, backgroundPosition, backgroundPositionX, backgroundPositionY, backgroundRepeat, backgroundSize,
  baselineShift, blockSize, border, borderBlock, borderBlockColor, borderBlockEnd,
  borderBlockEndColor, borderBlockEndStyle, borderBlockEndWidth, borderBlockStart, borderBlockStartColor, borderBlockStartStyle,
  borderBlockStartWidth, borderBlockStyle, borderBlockWidth, borderBottom, borderBottomColor, borderBottomLeftRadius,
  borderBottomRightRadius, borderBottomStyle, borderBottomWidth, borderCollapse, borderColor, borderEndEndRadius,
  borderEndStartRadius, borderImage, borderImageOutset, borderImageRepeat, borderImageSlice, borderImageSource,
  borderImageWidth, borderInline, borderInlineColor, borderInlineEnd, borderInlineEndColor, borderInlineEndStyle,
  borderInlineEndWidth, borderInlineStart, borderInlineStartColor, borderInlineStartStyle, borderInlineStartWidth, borderInlineStyle,
  borderInlineWidth, borderLeft, borderLeftColor, borderLeftStyle, borderLeftWidth, borderRadius,
  borderRight, borderRightColor, borderRightStyle, borderRightWidth, borderSpacing, borderStartEndRadius,
  borderStartStartRadius, borderStyle, borderTop, borderTopColor, borderTopLeftRadius, borderTopRightRadius,
  borderTopStyle, borderTopWidth, borderWidth, bottom, boxShadow, boxSizing,
  breakAfter, breakBefore, breakInside, captionSide, caretColor, clear,
  clip, clipPath, clipRule, color, colorInterpolation, colorInterpolationFilters,
  colorScheme, columnCount, columnFill, columnGap, columnRule, columnRuleColor,
  columnRuleStyle, columnRuleWidth, columnSpan, columnWidth, columns, contain,
  content, counterIncrement, counterReset, counterSet,
  cursor, direction, display, dominantBaseline, emptyCells, fill,
  fillOpacity, fillRule, filter, flex, flexBasis, flexDirection,
  flexFlow, flexGrow, flexShrink, flexWrap, float, floodColor,
  floodOpacity, font, fontFamily, fontFeatureSettings, fontKerning, fontOpticalSizing,
  fontSize, fontSizeAdjust, fontStretch, fontStyle, fontSynthesis, fontVariant,
  fontVariantAlternates, fontVariantCaps, fontVariantEastAsian, fontVariantLigatures, fontVariantNumeric, fontVariantPosition,
  fontVariationSettings, fontWeight, gap, grid, gridArea, gridAutoColumns,
  gridAutoFlow, gridAutoRows, gridColumn, gridColumnEnd, gridColumnGap, gridColumnStart,
  gridGap, gridRow, gridRowEnd, gridRowGap, gridRowStart, gridTemplate,
  gridTemplateAreas, gridTemplateColumns, gridTemplateRows, height, hyphens, imageOrientation,
  imageRendering, inlineSize, inset, insetBlock, insetBlockEnd, insetBlockStart,
  insetInline, insetInlineEnd, insetInlineStart, isolation, justifyContent, justifyItems,
  justifySelf, left, letterSpacing, lightingColor, lineBreak, lineHeight,
  listStyle, listStyleImage, listStylePosition, listStyleType, margin, marginBlock,
  marginBlockEnd, marginBlockStart, marginBottom, marginInline, marginInlineEnd, marginInlineStart,
  marginLeft, marginRight, marginTop, marker, markerEnd, markerMid,
  markerStart, mask, maskClip, maskComposite, maskImage, maskMode,
  maskOrigin, maskPosition, maskRepeat, maskSize, maskType, maxBlockSize,
  maxHeight, maxInlineSize, maxWidth, minBlockSize, minHeight, minInlineSize,
  minWidth, mixBlendMode, objectFit, objectPosition, offset, offsetDistance,
  offsetPath, offsetRotate, opacity, order, orphans, outline,
  outlineColor, outlineOffset, outlineStyle, outlineWidth, overflow, overflowAnchor,
  overflowWrap, overflowX, overflowY, overscrollBehavior, overscrollBehaviorBlock, overscrollBehaviorInline,
  overscrollBehaviorX, overscrollBehaviorY, padding, paddingBlock, paddingBlockEnd, paddingBlockStart,
  paddingBottom, paddingInline, paddingInlineEnd, paddingInlineStart, paddingLeft, paddingRight,
  paddingTop, pageBreakAfter, pageBreakBefore, pageBreakInside, paintOrder, perspective,
  perspectiveOrigin, placeContent, placeItems, placeSelf, pointerEvents, position,
  printColorAdjust, quotes, resize, right, rotate, rowGap,
  rubyPosition, scale, scrollBehavior, scrollMargin, scrollMarginBlock, scrollMarginBlockEnd,
  scrollMarginBlockStart, scrollMarginBottom, scrollMarginInline, scrollMarginInlineEnd, scrollMarginInlineStart, scrollMarginLeft,
  scrollMarginRight, scrollMarginTop, scrollPadding, scrollPaddingBlock, scrollPaddingBlockEnd, scrollPaddingBlockStart,
  scrollPaddingBottom, scrollPaddingInline, scrollPaddingInlineEnd, scrollPaddingInlineStart, scrollPaddingLeft, scrollPaddingRight,
  scrollPaddingTop, scrollSnapAlign, scrollSnapStop, scrollSnapType, scrollbarGutter, shapeImageThreshold,
  shapeMargin, shapeOutside, shapeRendering, stopColor, stopOpacity, stroke,
  strokeDasharray, strokeDashoffset, strokeLinecap, strokeLinejoin, strokeMiterlimit, strokeOpacity,
  strokeWidth, tabSize, tableLayout, textAlign, textAlignLast, textAnchor,
  textCombineUpright, textDecoration, textDecorationColor, textDecorationLine, textDecorationSkipInk, textDecorationStyle,
  textDecorationThickness, textEmphasis, textEmphasisColor, textEmphasisPosition, textEmphasisStyle, textIndent,
  textOrientation, textOverflow, textRendering, textShadow, textTransform, textUnderlineOffset,
  textUnderlinePosition, top, touchAction, transform, transformBox, transformOrigin,
  transformStyle, transition, transitionDelay, transitionDuration, transitionProperty, transitionTimingFunction,
  translate, unicodeBidi, userSelect, verticalAlign, visibility, webkitAlignContent,
  webkitAlignItems, webkitAlignSelf, webkitAnimation, webkitAnimationDelay, webkitAnimationDirection, webkitAnimationDuration,
  webkitAnimationFillMode, webkitAnimationIterationCount, webkitAnimationName, webkitAnimationPlayState, webkitAnimationTimingFunction, webkitAppearance,
  webkitBackfaceVisibility, webkitBackgroundClip, webkitBackgroundOrigin, webkitBackgroundSize, webkitBorderBottomLeftRadius, webkitBorderBottomRightRadius,
  webkitBorderRadius, webkitBorderTopLeftRadius, webkitBorderTopRightRadius, webkitBoxAlign, webkitBoxFlex, webkitBoxOrdinalGroup,
  webkitBoxOrient, webkitBoxPack, webkitBoxShadow, webkitBoxSizing, webkitFilter, webkitFlex,
  webkitFlexBasis, webkitFlexDirection, webkitFlexFlow, webkitFlexGrow, webkitFlexShrink, webkitFlexWrap,
  webkitJustifyContent, webkitLineClamp, webkitMask, webkitMaskBoxImage, webkitMaskBoxImageOutset, webkitMaskBoxImageRepeat,
  webkitMaskBoxImageSlice, webkitMaskBoxImageSource, webkitMaskBoxImageWidth, webkitMaskClip, webkitMaskComposite, webkitMaskImage,
  webkitMaskOrigin, webkitMaskPosition, webkitMaskRepeat, webkitMaskSize, webkitOrder, webkitPerspective,
  webkitPerspectiveOrigin, webkitTextFillColor, webkitTextSizeAdjust, webkitTextStroke, webkitTextStrokeColor, webkitTextStrokeWidth,
  webkitTransform, webkitTransformOrigin, webkitTransformStyle, webkitTransition, webkitTransitionDelay, webkitTransitionDuration,
  webkitTransitionProperty, webkitTransitionTimingFunction, webkitUserSelect, whiteSpace, widows, width,
  willChange, wordBreak, wordSpacing, wordWrap, writingMode, zIndex
});
