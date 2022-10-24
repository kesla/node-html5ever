export type ChildNode = Comment | DocumentType | Element | Text;
export type ParentNode = Document | DocumentFragment | Element;
export type ClassList = import("./types/generated").ClassList & {
    [index: string]: string
};export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  errors: Array<string>
  constructor(html: string)
  static createDocumentFragment(html: string): DocumentFragment
  get document(): Document
  get quirksMode(): QuirksMode
  serialize(): string
}
export class Comment {
  data: string
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get textContent(): string | null
}
export class Document {
  get nodeName(): string
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild<T extends ChildNode>(child: T): T
  removeChild<T extends ChildNode>(child: T): T
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  getElementsByTagName(qualifiedName: string): Array<Element>
  querySelector(selectors: string): Element | null
  querySelectorAll(selectors: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
  get docType(): DocumentType | null
  get documentElement(): Element
  get head(): Element
  get body(): Element
  get textContent(): string | null
  createElement(name: string): Element
  createTextNode(data: string): Text
}
export class DocumentFragment {
  get nodeName(): string
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild<T extends ChildNode>(child: T): T
  removeChild<T extends ChildNode>(child: T): T
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  getElementsByTagName(qualifiedName: string): Array<Element>
  querySelector(selectors: string): Element | null
  querySelectorAll(selectors: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
  get textContent(): string | null
}
export class DocumentType {
  name: string
  publicId: string
  systemId: string
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get textContent(): string | null
}
export class Attr {
  get localName(): string
  get name(): string
  get namespaceUri(): string
  get ownerDocument(): Document | null
  get ownerElement(): Element
  get prefix(): string | null
  get value(): string
}
export class ClassList {
  item(index: number): string | null
  add(token: string): void
  remove(token: string): void
  toggle(token: string): boolean
  contains(token: string): boolean
  get length(): number
  get value(): string
  set value(value: string)
}
export class Element {
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild<T extends ChildNode>(child: T): T
  removeChild<T extends ChildNode>(child: T): T
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  getElementsByTagName(qualifiedName: string): Array<Element>
  querySelector(selectors: string): Element | null
  querySelectorAll(selectors: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
  get attributes(): Array<Attr>
  getAttribute(name: string): string | null
  removeAttribute(name: string): void
  setAttribute(name: string, value: string): void
  hasAttribute(name: string): boolean
  get classList(): ClassList
  get style(): StyleDeclaration
  get tagName(): string
  get innerHTML(): string
  get outerHTML(): string
  get textContent(): string | null
  get className(): string
  set className(className: string)
  get id(): string
  set id(id: string)
}
export class Text {
  data: string
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get textContent(): string | null
}
export class StyleDeclaration {
  get accentColor(): string
  set accentColor(value: string)
  get alignContent(): string
  set alignContent(value: string)
  get alignItems(): string
  set alignItems(value: string)
  get alignSelf(): string
  set alignSelf(value: string)
  get alignmentBaseline(): string
  set alignmentBaseline(value: string)
  get all(): string
  set all(value: string)
  get animation(): string
  set animation(value: string)
  get animationDelay(): string
  set animationDelay(value: string)
  get animationDirection(): string
  set animationDirection(value: string)
  get animationDuration(): string
  set animationDuration(value: string)
  get animationFillMode(): string
  set animationFillMode(value: string)
  get animationIterationCount(): string
  set animationIterationCount(value: string)
  get animationName(): string
  set animationName(value: string)
  get animationPlayState(): string
  set animationPlayState(value: string)
  get animationTimingFunction(): string
  set animationTimingFunction(value: string)
  get appearance(): string
  set appearance(value: string)
  get aspectRatio(): string
  set aspectRatio(value: string)
  get backfaceVisibility(): string
  set backfaceVisibility(value: string)
  get background(): string
  set background(value: string)
  get backgroundAttachment(): string
  set backgroundAttachment(value: string)
  get backgroundBlendMode(): string
  set backgroundBlendMode(value: string)
  get backgroundClip(): string
  set backgroundClip(value: string)
  get backgroundColor(): string
  set backgroundColor(value: string)
  get backgroundImage(): string
  set backgroundImage(value: string)
  get backgroundOrigin(): string
  set backgroundOrigin(value: string)
  get backgroundPosition(): string
  set backgroundPosition(value: string)
  get backgroundPositionX(): string
  set backgroundPositionX(value: string)
  get backgroundPositionY(): string
  set backgroundPositionY(value: string)
  get backgroundRepeat(): string
  set backgroundRepeat(value: string)
  get backgroundSize(): string
  set backgroundSize(value: string)
  get baselineShift(): string
  set baselineShift(value: string)
  get blockSize(): string
  set blockSize(value: string)
  get border(): string
  set border(value: string)
  get borderBlock(): string
  set borderBlock(value: string)
  get borderBlockColor(): string
  set borderBlockColor(value: string)
  get borderBlockEnd(): string
  set borderBlockEnd(value: string)
  get borderBlockEndColor(): string
  set borderBlockEndColor(value: string)
  get borderBlockEndStyle(): string
  set borderBlockEndStyle(value: string)
  get borderBlockEndWidth(): string
  set borderBlockEndWidth(value: string)
  get borderBlockStart(): string
  set borderBlockStart(value: string)
  get borderBlockStartColor(): string
  set borderBlockStartColor(value: string)
  get borderBlockStartStyle(): string
  set borderBlockStartStyle(value: string)
  get borderBlockStartWidth(): string
  set borderBlockStartWidth(value: string)
  get borderBlockStyle(): string
  set borderBlockStyle(value: string)
  get borderBlockWidth(): string
  set borderBlockWidth(value: string)
  get borderBottom(): string
  set borderBottom(value: string)
  get borderBottomColor(): string
  set borderBottomColor(value: string)
  get borderBottomLeftRadius(): string
  set borderBottomLeftRadius(value: string)
  get borderBottomRightRadius(): string
  set borderBottomRightRadius(value: string)
  get borderBottomStyle(): string
  set borderBottomStyle(value: string)
  get borderBottomWidth(): string
  set borderBottomWidth(value: string)
  get borderCollapse(): string
  set borderCollapse(value: string)
  get borderColor(): string
  set borderColor(value: string)
  get borderEndEndRadius(): string
  set borderEndEndRadius(value: string)
  get borderEndStartRadius(): string
  set borderEndStartRadius(value: string)
  get borderImage(): string
  set borderImage(value: string)
  get borderImageOutset(): string
  set borderImageOutset(value: string)
  get borderImageRepeat(): string
  set borderImageRepeat(value: string)
  get borderImageSlice(): string
  set borderImageSlice(value: string)
  get borderImageSource(): string
  set borderImageSource(value: string)
  get borderImageWidth(): string
  set borderImageWidth(value: string)
  get borderInline(): string
  set borderInline(value: string)
  get borderInlineColor(): string
  set borderInlineColor(value: string)
  get borderInlineEnd(): string
  set borderInlineEnd(value: string)
  get borderInlineEndColor(): string
  set borderInlineEndColor(value: string)
  get borderInlineEndStyle(): string
  set borderInlineEndStyle(value: string)
  get borderInlineEndWidth(): string
  set borderInlineEndWidth(value: string)
  get borderInlineStart(): string
  set borderInlineStart(value: string)
  get borderInlineStartColor(): string
  set borderInlineStartColor(value: string)
  get borderInlineStartStyle(): string
  set borderInlineStartStyle(value: string)
  get borderInlineStartWidth(): string
  set borderInlineStartWidth(value: string)
  get borderInlineStyle(): string
  set borderInlineStyle(value: string)
  get borderInlineWidth(): string
  set borderInlineWidth(value: string)
  get borderLeft(): string
  set borderLeft(value: string)
  get borderLeftColor(): string
  set borderLeftColor(value: string)
  get borderLeftStyle(): string
  set borderLeftStyle(value: string)
  get borderLeftWidth(): string
  set borderLeftWidth(value: string)
  get borderRadius(): string
  set borderRadius(value: string)
  get borderRight(): string
  set borderRight(value: string)
  get borderRightColor(): string
  set borderRightColor(value: string)
  get borderRightStyle(): string
  set borderRightStyle(value: string)
  get borderRightWidth(): string
  set borderRightWidth(value: string)
  get borderSpacing(): string
  set borderSpacing(value: string)
  get borderStartEndRadius(): string
  set borderStartEndRadius(value: string)
  get borderStartStartRadius(): string
  set borderStartStartRadius(value: string)
  get borderStyle(): string
  set borderStyle(value: string)
  get borderTop(): string
  set borderTop(value: string)
  get borderTopColor(): string
  set borderTopColor(value: string)
  get borderTopLeftRadius(): string
  set borderTopLeftRadius(value: string)
  get borderTopRightRadius(): string
  set borderTopRightRadius(value: string)
  get borderTopStyle(): string
  set borderTopStyle(value: string)
  get borderTopWidth(): string
  set borderTopWidth(value: string)
  get borderWidth(): string
  set borderWidth(value: string)
  get bottom(): string
  set bottom(value: string)
  get boxShadow(): string
  set boxShadow(value: string)
  get boxSizing(): string
  set boxSizing(value: string)
  get breakAfter(): string
  set breakAfter(value: string)
  get breakBefore(): string
  set breakBefore(value: string)
  get breakInside(): string
  set breakInside(value: string)
  get captionSide(): string
  set captionSide(value: string)
  get caretColor(): string
  set caretColor(value: string)
  get clear(): string
  set clear(value: string)
  get clip(): string
  set clip(value: string)
  get clipPath(): string
  set clipPath(value: string)
  get clipRule(): string
  set clipRule(value: string)
  get color(): string
  set color(value: string)
  get colorInterpolation(): string
  set colorInterpolation(value: string)
  get colorInterpolationFilters(): string
  set colorInterpolationFilters(value: string)
  get colorScheme(): string
  set colorScheme(value: string)
  get columnCount(): string
  set columnCount(value: string)
  get columnFill(): string
  set columnFill(value: string)
  get columnGap(): string
  set columnGap(value: string)
  get columnRule(): string
  set columnRule(value: string)
  get columnRuleColor(): string
  set columnRuleColor(value: string)
  get columnRuleStyle(): string
  set columnRuleStyle(value: string)
  get columnRuleWidth(): string
  set columnRuleWidth(value: string)
  get columnSpan(): string
  set columnSpan(value: string)
  get columnWidth(): string
  set columnWidth(value: string)
  get columns(): string
  set columns(value: string)
  get contain(): string
  set contain(value: string)
  get content(): string
  set content(value: string)
  get counterIncrement(): string
  set counterIncrement(value: string)
  get counterReset(): string
  set counterReset(value: string)
  get counterSet(): string
  set counterSet(value: string)
  get cursor(): string
  set cursor(value: string)
  get direction(): string
  set direction(value: string)
  get display(): string
  set display(value: string)
  get dominantBaseline(): string
  set dominantBaseline(value: string)
  get emptyCells(): string
  set emptyCells(value: string)
  get fill(): string
  set fill(value: string)
  get fillOpacity(): string
  set fillOpacity(value: string)
  get fillRule(): string
  set fillRule(value: string)
  get filter(): string
  set filter(value: string)
  get flex(): string
  set flex(value: string)
  get flexBasis(): string
  set flexBasis(value: string)
  get flexDirection(): string
  set flexDirection(value: string)
  get flexFlow(): string
  set flexFlow(value: string)
  get flexGrow(): string
  set flexGrow(value: string)
  get flexShrink(): string
  set flexShrink(value: string)
  get flexWrap(): string
  set flexWrap(value: string)
  get float(): string
  set float(value: string)
  get floodColor(): string
  set floodColor(value: string)
  get floodOpacity(): string
  set floodOpacity(value: string)
  get font(): string
  set font(value: string)
  get fontFamily(): string
  set fontFamily(value: string)
  get fontFeatureSettings(): string
  set fontFeatureSettings(value: string)
  get fontKerning(): string
  set fontKerning(value: string)
  get fontOpticalSizing(): string
  set fontOpticalSizing(value: string)
  get fontSize(): string
  set fontSize(value: string)
  get fontSizeAdjust(): string
  set fontSizeAdjust(value: string)
  get fontStretch(): string
  set fontStretch(value: string)
  get fontStyle(): string
  set fontStyle(value: string)
  get fontSynthesis(): string
  set fontSynthesis(value: string)
  get fontVariant(): string
  set fontVariant(value: string)
  get fontVariantAlternates(): string
  set fontVariantAlternates(value: string)
  get fontVariantCaps(): string
  set fontVariantCaps(value: string)
  get fontVariantEastAsian(): string
  set fontVariantEastAsian(value: string)
  get fontVariantLigatures(): string
  set fontVariantLigatures(value: string)
  get fontVariantNumeric(): string
  set fontVariantNumeric(value: string)
  get fontVariantPosition(): string
  set fontVariantPosition(value: string)
  get fontVariationSettings(): string
  set fontVariationSettings(value: string)
  get fontWeight(): string
  set fontWeight(value: string)
  get gap(): string
  set gap(value: string)
  get grid(): string
  set grid(value: string)
  get gridArea(): string
  set gridArea(value: string)
  get gridAutoColumns(): string
  set gridAutoColumns(value: string)
  get gridAutoFlow(): string
  set gridAutoFlow(value: string)
  get gridAutoRows(): string
  set gridAutoRows(value: string)
  get gridColumn(): string
  set gridColumn(value: string)
  get gridColumnEnd(): string
  set gridColumnEnd(value: string)
  get gridColumnGap(): string
  set gridColumnGap(value: string)
  get gridColumnStart(): string
  set gridColumnStart(value: string)
  get gridGap(): string
  set gridGap(value: string)
  get gridRow(): string
  set gridRow(value: string)
  get gridRowEnd(): string
  set gridRowEnd(value: string)
  get gridRowGap(): string
  set gridRowGap(value: string)
  get gridRowStart(): string
  set gridRowStart(value: string)
  get gridTemplate(): string
  set gridTemplate(value: string)
  get gridTemplateAreas(): string
  set gridTemplateAreas(value: string)
  get gridTemplateColumns(): string
  set gridTemplateColumns(value: string)
  get gridTemplateRows(): string
  set gridTemplateRows(value: string)
  get height(): string
  set height(value: string)
  get hyphens(): string
  set hyphens(value: string)
  get imageOrientation(): string
  set imageOrientation(value: string)
  get imageRendering(): string
  set imageRendering(value: string)
  get inlineSize(): string
  set inlineSize(value: string)
  get inset(): string
  set inset(value: string)
  get insetBlock(): string
  set insetBlock(value: string)
  get insetBlockEnd(): string
  set insetBlockEnd(value: string)
  get insetBlockStart(): string
  set insetBlockStart(value: string)
  get insetInline(): string
  set insetInline(value: string)
  get insetInlineEnd(): string
  set insetInlineEnd(value: string)
  get insetInlineStart(): string
  set insetInlineStart(value: string)
  get isolation(): string
  set isolation(value: string)
  get justifyContent(): string
  set justifyContent(value: string)
  get justifyItems(): string
  set justifyItems(value: string)
  get justifySelf(): string
  set justifySelf(value: string)
  get left(): string
  set left(value: string)
  get letterSpacing(): string
  set letterSpacing(value: string)
  get lightingColor(): string
  set lightingColor(value: string)
  get lineBreak(): string
  set lineBreak(value: string)
  get lineHeight(): string
  set lineHeight(value: string)
  get listStyle(): string
  set listStyle(value: string)
  get listStyleImage(): string
  set listStyleImage(value: string)
  get listStylePosition(): string
  set listStylePosition(value: string)
  get listStyleType(): string
  set listStyleType(value: string)
  get margin(): string
  set margin(value: string)
  get marginBlock(): string
  set marginBlock(value: string)
  get marginBlockEnd(): string
  set marginBlockEnd(value: string)
  get marginBlockStart(): string
  set marginBlockStart(value: string)
  get marginBottom(): string
  set marginBottom(value: string)
  get marginInline(): string
  set marginInline(value: string)
  get marginInlineEnd(): string
  set marginInlineEnd(value: string)
  get marginInlineStart(): string
  set marginInlineStart(value: string)
  get marginLeft(): string
  set marginLeft(value: string)
  get marginRight(): string
  set marginRight(value: string)
  get marginTop(): string
  set marginTop(value: string)
  get marker(): string
  set marker(value: string)
  get markerEnd(): string
  set markerEnd(value: string)
  get markerMid(): string
  set markerMid(value: string)
  get markerStart(): string
  set markerStart(value: string)
  get mask(): string
  set mask(value: string)
  get maskClip(): string
  set maskClip(value: string)
  get maskComposite(): string
  set maskComposite(value: string)
  get maskImage(): string
  set maskImage(value: string)
  get maskMode(): string
  set maskMode(value: string)
  get maskOrigin(): string
  set maskOrigin(value: string)
  get maskPosition(): string
  set maskPosition(value: string)
  get maskRepeat(): string
  set maskRepeat(value: string)
  get maskSize(): string
  set maskSize(value: string)
  get maskType(): string
  set maskType(value: string)
  get maxBlockSize(): string
  set maxBlockSize(value: string)
  get maxHeight(): string
  set maxHeight(value: string)
  get maxInlineSize(): string
  set maxInlineSize(value: string)
  get maxWidth(): string
  set maxWidth(value: string)
  get minBlockSize(): string
  set minBlockSize(value: string)
  get minHeight(): string
  set minHeight(value: string)
  get minInlineSize(): string
  set minInlineSize(value: string)
  get minWidth(): string
  set minWidth(value: string)
  get mixBlendMode(): string
  set mixBlendMode(value: string)
  get objectFit(): string
  set objectFit(value: string)
  get objectPosition(): string
  set objectPosition(value: string)
  get offset(): string
  set offset(value: string)
  get offsetDistance(): string
  set offsetDistance(value: string)
  get offsetPath(): string
  set offsetPath(value: string)
  get offsetRotate(): string
  set offsetRotate(value: string)
  get opacity(): string
  set opacity(value: string)
  get order(): string
  set order(value: string)
  get orphans(): string
  set orphans(value: string)
  get outline(): string
  set outline(value: string)
  get outlineColor(): string
  set outlineColor(value: string)
  get outlineOffset(): string
  set outlineOffset(value: string)
  get outlineStyle(): string
  set outlineStyle(value: string)
  get outlineWidth(): string
  set outlineWidth(value: string)
  get overflow(): string
  set overflow(value: string)
  get overflowAnchor(): string
  set overflowAnchor(value: string)
  get overflowWrap(): string
  set overflowWrap(value: string)
  get overflowX(): string
  set overflowX(value: string)
  get overflowY(): string
  set overflowY(value: string)
  get overscrollBehavior(): string
  set overscrollBehavior(value: string)
  get overscrollBehaviorBlock(): string
  set overscrollBehaviorBlock(value: string)
  get overscrollBehaviorInline(): string
  set overscrollBehaviorInline(value: string)
  get overscrollBehaviorX(): string
  set overscrollBehaviorX(value: string)
  get overscrollBehaviorY(): string
  set overscrollBehaviorY(value: string)
  get padding(): string
  set padding(value: string)
  get paddingBlock(): string
  set paddingBlock(value: string)
  get paddingBlockEnd(): string
  set paddingBlockEnd(value: string)
  get paddingBlockStart(): string
  set paddingBlockStart(value: string)
  get paddingBottom(): string
  set paddingBottom(value: string)
  get paddingInline(): string
  set paddingInline(value: string)
  get paddingInlineEnd(): string
  set paddingInlineEnd(value: string)
  get paddingInlineStart(): string
  set paddingInlineStart(value: string)
  get paddingLeft(): string
  set paddingLeft(value: string)
  get paddingRight(): string
  set paddingRight(value: string)
  get paddingTop(): string
  set paddingTop(value: string)
  get pageBreakAfter(): string
  set pageBreakAfter(value: string)
  get pageBreakBefore(): string
  set pageBreakBefore(value: string)
  get pageBreakInside(): string
  set pageBreakInside(value: string)
  get paintOrder(): string
  set paintOrder(value: string)
  get perspective(): string
  set perspective(value: string)
  get perspectiveOrigin(): string
  set perspectiveOrigin(value: string)
  get placeContent(): string
  set placeContent(value: string)
  get placeItems(): string
  set placeItems(value: string)
  get placeSelf(): string
  set placeSelf(value: string)
  get pointerEvents(): string
  set pointerEvents(value: string)
  get position(): string
  set position(value: string)
  get printColorAdjust(): string
  set printColorAdjust(value: string)
  get quotes(): string
  set quotes(value: string)
  get resize(): string
  set resize(value: string)
  get right(): string
  set right(value: string)
  get rotate(): string
  set rotate(value: string)
  get rowGap(): string
  set rowGap(value: string)
  get rubyPosition(): string
  set rubyPosition(value: string)
  get scale(): string
  set scale(value: string)
  get scrollBehavior(): string
  set scrollBehavior(value: string)
  get scrollMargin(): string
  set scrollMargin(value: string)
  get scrollMarginBlock(): string
  set scrollMarginBlock(value: string)
  get scrollMarginBlockEnd(): string
  set scrollMarginBlockEnd(value: string)
  get scrollMarginBlockStart(): string
  set scrollMarginBlockStart(value: string)
  get scrollMarginBottom(): string
  set scrollMarginBottom(value: string)
  get scrollMarginInline(): string
  set scrollMarginInline(value: string)
  get scrollMarginInlineEnd(): string
  set scrollMarginInlineEnd(value: string)
  get scrollMarginInlineStart(): string
  set scrollMarginInlineStart(value: string)
  get scrollMarginLeft(): string
  set scrollMarginLeft(value: string)
  get scrollMarginRight(): string
  set scrollMarginRight(value: string)
  get scrollMarginTop(): string
  set scrollMarginTop(value: string)
  get scrollPadding(): string
  set scrollPadding(value: string)
  get scrollPaddingBlock(): string
  set scrollPaddingBlock(value: string)
  get scrollPaddingBlockEnd(): string
  set scrollPaddingBlockEnd(value: string)
  get scrollPaddingBlockStart(): string
  set scrollPaddingBlockStart(value: string)
  get scrollPaddingBottom(): string
  set scrollPaddingBottom(value: string)
  get scrollPaddingInline(): string
  set scrollPaddingInline(value: string)
  get scrollPaddingInlineEnd(): string
  set scrollPaddingInlineEnd(value: string)
  get scrollPaddingInlineStart(): string
  set scrollPaddingInlineStart(value: string)
  get scrollPaddingLeft(): string
  set scrollPaddingLeft(value: string)
  get scrollPaddingRight(): string
  set scrollPaddingRight(value: string)
  get scrollPaddingTop(): string
  set scrollPaddingTop(value: string)
  get scrollSnapAlign(): string
  set scrollSnapAlign(value: string)
  get scrollSnapStop(): string
  set scrollSnapStop(value: string)
  get scrollSnapType(): string
  set scrollSnapType(value: string)
  get scrollbarGutter(): string
  set scrollbarGutter(value: string)
  get shapeImageThreshold(): string
  set shapeImageThreshold(value: string)
  get shapeMargin(): string
  set shapeMargin(value: string)
  get shapeOutside(): string
  set shapeOutside(value: string)
  get shapeRendering(): string
  set shapeRendering(value: string)
  get stopColor(): string
  set stopColor(value: string)
  get stopOpacity(): string
  set stopOpacity(value: string)
  get stroke(): string
  set stroke(value: string)
  get strokeDasharray(): string
  set strokeDasharray(value: string)
  get strokeDashoffset(): string
  set strokeDashoffset(value: string)
  get strokeLinecap(): string
  set strokeLinecap(value: string)
  get strokeLinejoin(): string
  set strokeLinejoin(value: string)
  get strokeMiterlimit(): string
  set strokeMiterlimit(value: string)
  get strokeOpacity(): string
  set strokeOpacity(value: string)
  get strokeWidth(): string
  set strokeWidth(value: string)
  get tabSize(): string
  set tabSize(value: string)
  get tableLayout(): string
  set tableLayout(value: string)
  get textAlign(): string
  set textAlign(value: string)
  get textAlignLast(): string
  set textAlignLast(value: string)
  get textAnchor(): string
  set textAnchor(value: string)
  get textCombineUpright(): string
  set textCombineUpright(value: string)
  get textDecoration(): string
  set textDecoration(value: string)
  get textDecorationColor(): string
  set textDecorationColor(value: string)
  get textDecorationLine(): string
  set textDecorationLine(value: string)
  get textDecorationSkipInk(): string
  set textDecorationSkipInk(value: string)
  get textDecorationStyle(): string
  set textDecorationStyle(value: string)
  get textDecorationThickness(): string
  set textDecorationThickness(value: string)
  get textEmphasis(): string
  set textEmphasis(value: string)
  get textEmphasisColor(): string
  set textEmphasisColor(value: string)
  get textEmphasisPosition(): string
  set textEmphasisPosition(value: string)
  get textEmphasisStyle(): string
  set textEmphasisStyle(value: string)
  get textIndent(): string
  set textIndent(value: string)
  get textOrientation(): string
  set textOrientation(value: string)
  get textOverflow(): string
  set textOverflow(value: string)
  get textRendering(): string
  set textRendering(value: string)
  get textShadow(): string
  set textShadow(value: string)
  get textTransform(): string
  set textTransform(value: string)
  get textUnderlineOffset(): string
  set textUnderlineOffset(value: string)
  get textUnderlinePosition(): string
  set textUnderlinePosition(value: string)
  get top(): string
  set top(value: string)
  get touchAction(): string
  set touchAction(value: string)
  get transform(): string
  set transform(value: string)
  get transformBox(): string
  set transformBox(value: string)
  get transformOrigin(): string
  set transformOrigin(value: string)
  get transformStyle(): string
  set transformStyle(value: string)
  get transition(): string
  set transition(value: string)
  get transitionDelay(): string
  set transitionDelay(value: string)
  get transitionDuration(): string
  set transitionDuration(value: string)
  get transitionProperty(): string
  set transitionProperty(value: string)
  get transitionTimingFunction(): string
  set transitionTimingFunction(value: string)
  get translate(): string
  set translate(value: string)
  get unicodeBidi(): string
  set unicodeBidi(value: string)
  get userSelect(): string
  set userSelect(value: string)
  get verticalAlign(): string
  set verticalAlign(value: string)
  get visibility(): string
  set visibility(value: string)
  get webkitAlignContent(): string
  set webkitAlignContent(value: string)
  get webkitAlignItems(): string
  set webkitAlignItems(value: string)
  get webkitAlignSelf(): string
  set webkitAlignSelf(value: string)
  get webkitAnimation(): string
  set webkitAnimation(value: string)
  get webkitAnimationDelay(): string
  set webkitAnimationDelay(value: string)
  get webkitAnimationDirection(): string
  set webkitAnimationDirection(value: string)
  get webkitAnimationDuration(): string
  set webkitAnimationDuration(value: string)
  get webkitAnimationFillMode(): string
  set webkitAnimationFillMode(value: string)
  get webkitAnimationIterationCount(): string
  set webkitAnimationIterationCount(value: string)
  get webkitAnimationName(): string
  set webkitAnimationName(value: string)
  get webkitAnimationPlayState(): string
  set webkitAnimationPlayState(value: string)
  get webkitAnimationTimingFunction(): string
  set webkitAnimationTimingFunction(value: string)
  get webkitAppearance(): string
  set webkitAppearance(value: string)
  get webkitBackfaceVisibility(): string
  set webkitBackfaceVisibility(value: string)
  get webkitBackgroundClip(): string
  set webkitBackgroundClip(value: string)
  get webkitBackgroundOrigin(): string
  set webkitBackgroundOrigin(value: string)
  get webkitBackgroundSize(): string
  set webkitBackgroundSize(value: string)
  get webkitBorderBottomLeftRadius(): string
  set webkitBorderBottomLeftRadius(value: string)
  get webkitBorderBottomRightRadius(): string
  set webkitBorderBottomRightRadius(value: string)
  get webkitBorderRadius(): string
  set webkitBorderRadius(value: string)
  get webkitBorderTopLeftRadius(): string
  set webkitBorderTopLeftRadius(value: string)
  get webkitBorderTopRightRadius(): string
  set webkitBorderTopRightRadius(value: string)
  get webkitBoxAlign(): string
  set webkitBoxAlign(value: string)
  get webkitBoxFlex(): string
  set webkitBoxFlex(value: string)
  get webkitBoxOrdinalGroup(): string
  set webkitBoxOrdinalGroup(value: string)
  get webkitBoxOrient(): string
  set webkitBoxOrient(value: string)
  get webkitBoxPack(): string
  set webkitBoxPack(value: string)
  get webkitBoxShadow(): string
  set webkitBoxShadow(value: string)
  get webkitBoxSizing(): string
  set webkitBoxSizing(value: string)
  get webkitFilter(): string
  set webkitFilter(value: string)
  get webkitFlex(): string
  set webkitFlex(value: string)
  get webkitFlexBasis(): string
  set webkitFlexBasis(value: string)
  get webkitFlexDirection(): string
  set webkitFlexDirection(value: string)
  get webkitFlexFlow(): string
  set webkitFlexFlow(value: string)
  get webkitFlexGrow(): string
  set webkitFlexGrow(value: string)
  get webkitFlexShrink(): string
  set webkitFlexShrink(value: string)
  get webkitFlexWrap(): string
  set webkitFlexWrap(value: string)
  get webkitJustifyContent(): string
  set webkitJustifyContent(value: string)
  get webkitLineClamp(): string
  set webkitLineClamp(value: string)
  get webkitMask(): string
  set webkitMask(value: string)
  get webkitMaskBoxImage(): string
  set webkitMaskBoxImage(value: string)
  get webkitMaskBoxImageOutset(): string
  set webkitMaskBoxImageOutset(value: string)
  get webkitMaskBoxImageRepeat(): string
  set webkitMaskBoxImageRepeat(value: string)
  get webkitMaskBoxImageSlice(): string
  set webkitMaskBoxImageSlice(value: string)
  get webkitMaskBoxImageSource(): string
  set webkitMaskBoxImageSource(value: string)
  get webkitMaskBoxImageWidth(): string
  set webkitMaskBoxImageWidth(value: string)
  get webkitMaskClip(): string
  set webkitMaskClip(value: string)
  get webkitMaskComposite(): string
  set webkitMaskComposite(value: string)
  get webkitMaskImage(): string
  set webkitMaskImage(value: string)
  get webkitMaskOrigin(): string
  set webkitMaskOrigin(value: string)
  get webkitMaskPosition(): string
  set webkitMaskPosition(value: string)
  get webkitMaskRepeat(): string
  set webkitMaskRepeat(value: string)
  get webkitMaskSize(): string
  set webkitMaskSize(value: string)
  get webkitOrder(): string
  set webkitOrder(value: string)
  get webkitPerspective(): string
  set webkitPerspective(value: string)
  get webkitPerspectiveOrigin(): string
  set webkitPerspectiveOrigin(value: string)
  get webkitTextFillColor(): string
  set webkitTextFillColor(value: string)
  get webkitTextSizeAdjust(): string
  set webkitTextSizeAdjust(value: string)
  get webkitTextStroke(): string
  set webkitTextStroke(value: string)
  get webkitTextStrokeColor(): string
  set webkitTextStrokeColor(value: string)
  get webkitTextStrokeWidth(): string
  set webkitTextStrokeWidth(value: string)
  get webkitTransform(): string
  set webkitTransform(value: string)
  get webkitTransformOrigin(): string
  set webkitTransformOrigin(value: string)
  get webkitTransformStyle(): string
  set webkitTransformStyle(value: string)
  get webkitTransition(): string
  set webkitTransition(value: string)
  get webkitTransitionDelay(): string
  set webkitTransitionDelay(value: string)
  get webkitTransitionDuration(): string
  set webkitTransitionDuration(value: string)
  get webkitTransitionProperty(): string
  set webkitTransitionProperty(value: string)
  get webkitTransitionTimingFunction(): string
  set webkitTransitionTimingFunction(value: string)
  get webkitUserSelect(): string
  set webkitUserSelect(value: string)
  get whiteSpace(): string
  set whiteSpace(value: string)
  get widows(): string
  set widows(value: string)
  get width(): string
  set width(value: string)
  get willChange(): string
  set willChange(value: string)
  get wordBreak(): string
  set wordBreak(value: string)
  get wordSpacing(): string
  set wordSpacing(value: string)
  get wordWrap(): string
  set wordWrap(value: string)
  get writingMode(): string
  set writingMode(value: string)
  get zIndex(): string
  set zIndex(value: string)
  getPropertyValue(property: string): string
  getPropertyPriority(property: string): string
  removeProperty(property: string): string
  setProperty(property: string, value: string, priority?: string | undefined | null): void
  get cssText(): string
  set cssText(cssText: string)
  get cssFloat(): string
  set cssFloat(value: string)
  get length(): number
}
