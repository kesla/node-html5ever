export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  errors: Array<string>
  constructor(html?: string | undefined | null)
  static createDocumentFragment(html: string, maybeQuirksMode?: QuirksMode | undefined | null): DocumentFragment
  get window(): Window
  get quirksMode(): QuirksMode
  serialize(): string
}
export class Comment {
  data: string
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get textContent(): string | null
  cloneNode(): this
}
export class Document {
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild<T extends ChildNode>(child: T): T
  insertBefore<T extends ChildNode>(new_node: T, reference_node: ChildNode): T
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
  normalize(): void
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get doctype(): DocumentType | null
  get documentElement(): Element
  get defaultView(): Window | null
  get head(): Element
  get body(): Element
  get textContent(): string | null
  createElement(name: string): Element
  createTextNode(data: string): Text
  createDocumentFragment(html?: string | undefined | null): DocumentFragment
}
export class DocumentFragment {
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild<T extends ChildNode>(child: T): T
  insertBefore<T extends ChildNode>(new_node: T, reference_node: ChildNode): T
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
  normalize(): void
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get textContent(): string | null
}
export class DocumentType {
  name: string
  publicId: string
  systemId: string
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get textContent(): string | null
  cloneNode(deep?: boolean | undefined | null): this
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
  add(token1?: string | undefined | null, token2?: string | undefined | null, token3?: string | undefined | null, token4?: string | undefined | null, token5?: string | undefined | null): void
  remove(token1?: string | undefined | null, token2?: string | undefined | null, token3?: string | undefined | null, token4?: string | undefined | null, token5?: string | undefined | null): void
  toggle(token: string, force?: boolean | undefined | null): boolean
  contains(token: string): boolean
  get length(): number
  get value(): string
  toString(): string
  set value(value: string)
}
export class Element {
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
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
  insertBefore<T extends ChildNode>(new_node: T, reference_node: ChildNode): T
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
  normalize(): void
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get attributes(): Array<Attr>
  getAttribute(name: string): string | null
  removeAttribute(name: string): void
  setAttribute(name: string, value: string): void
  hasAttribute(name: string): boolean
  get classList(): ClassList
  get style(): StyleDeclaration
  get tagName(): string
  get innerHTML(): string
  set innerHTML(html: string)
  get outerHTML(): string
  set outerHTML(html: string)
  get textContent(): string | null
  get className(): string
  set className(className: string)
  get id(): string
  set id(id: string)
  cloneNode(deep?: boolean | undefined | null): Element
  insertAdjacentElement(position: InsertPosition, element: Element): Element
  insertAdjacentHTML(position: InsertPosition, html: string): void
  insertAdjacentText(position: InsertPosition, text: string): void
  matches(selectors: string): boolean
}
export class Text {
  data: string
  get nodeName(): string
  get nodeType(): number
  get nodeValue(): string | null
  get parentElement(): Element | null
  get parentNode(): ParentNode | null
  get ownerDocument(): Document | null
  remove(): void
  get previousSibling(): ChildNode | null
  get previousElementSibling(): Element | null
  get nextSibling(): ChildNode | null
  get nextElementSibling(): Element | null
  get ATTRIBUTE_NODE(): number
  get CDATA_SECTION_NODE(): number
  get COMMENT_NODE(): number
  get DOCUMENT_FRAGMENT_NODE(): number
  get DOCUMENT_NODE(): number
  get DOCUMENT_TYPE_NODE(): number
  get ELEMENT_NODE(): number
  get PROCESSING_INSTRUCTION_NODE(): number
  get TEXT_NODE(): number
  get textContent(): string | null
  cloneNode(): this
}
export class Window {
  get document(): Document
}
export class StyleDeclaration {
  get accentColor(): string
  set accentColor(value?: string | undefined | null)
  get alignContent(): string
  set alignContent(value?: string | undefined | null)
  get alignItems(): string
  set alignItems(value?: string | undefined | null)
  get alignSelf(): string
  set alignSelf(value?: string | undefined | null)
  get alignmentBaseline(): string
  set alignmentBaseline(value?: string | undefined | null)
  get all(): string
  set all(value?: string | undefined | null)
  get animation(): string
  set animation(value?: string | undefined | null)
  get animationDelay(): string
  set animationDelay(value?: string | undefined | null)
  get animationDirection(): string
  set animationDirection(value?: string | undefined | null)
  get animationDuration(): string
  set animationDuration(value?: string | undefined | null)
  get animationFillMode(): string
  set animationFillMode(value?: string | undefined | null)
  get animationIterationCount(): string
  set animationIterationCount(value?: string | undefined | null)
  get animationName(): string
  set animationName(value?: string | undefined | null)
  get animationPlayState(): string
  set animationPlayState(value?: string | undefined | null)
  get animationTimingFunction(): string
  set animationTimingFunction(value?: string | undefined | null)
  get appearance(): string
  set appearance(value?: string | undefined | null)
  get aspectRatio(): string
  set aspectRatio(value?: string | undefined | null)
  get backfaceVisibility(): string
  set backfaceVisibility(value?: string | undefined | null)
  get background(): string
  set background(value?: string | undefined | null)
  get backgroundAttachment(): string
  set backgroundAttachment(value?: string | undefined | null)
  get backgroundBlendMode(): string
  set backgroundBlendMode(value?: string | undefined | null)
  get backgroundClip(): string
  set backgroundClip(value?: string | undefined | null)
  get backgroundColor(): string
  set backgroundColor(value?: string | undefined | null)
  get backgroundImage(): string
  set backgroundImage(value?: string | undefined | null)
  get backgroundOrigin(): string
  set backgroundOrigin(value?: string | undefined | null)
  get backgroundPosition(): string
  set backgroundPosition(value?: string | undefined | null)
  get backgroundPositionX(): string
  set backgroundPositionX(value?: string | undefined | null)
  get backgroundPositionY(): string
  set backgroundPositionY(value?: string | undefined | null)
  get backgroundRepeat(): string
  set backgroundRepeat(value?: string | undefined | null)
  get backgroundSize(): string
  set backgroundSize(value?: string | undefined | null)
  get baselineShift(): string
  set baselineShift(value?: string | undefined | null)
  get blockSize(): string
  set blockSize(value?: string | undefined | null)
  get border(): string
  set border(value?: string | undefined | null)
  get borderBlock(): string
  set borderBlock(value?: string | undefined | null)
  get borderBlockColor(): string
  set borderBlockColor(value?: string | undefined | null)
  get borderBlockEnd(): string
  set borderBlockEnd(value?: string | undefined | null)
  get borderBlockEndColor(): string
  set borderBlockEndColor(value?: string | undefined | null)
  get borderBlockEndStyle(): string
  set borderBlockEndStyle(value?: string | undefined | null)
  get borderBlockEndWidth(): string
  set borderBlockEndWidth(value?: string | undefined | null)
  get borderBlockStart(): string
  set borderBlockStart(value?: string | undefined | null)
  get borderBlockStartColor(): string
  set borderBlockStartColor(value?: string | undefined | null)
  get borderBlockStartStyle(): string
  set borderBlockStartStyle(value?: string | undefined | null)
  get borderBlockStartWidth(): string
  set borderBlockStartWidth(value?: string | undefined | null)
  get borderBlockStyle(): string
  set borderBlockStyle(value?: string | undefined | null)
  get borderBlockWidth(): string
  set borderBlockWidth(value?: string | undefined | null)
  get borderBottom(): string
  set borderBottom(value?: string | undefined | null)
  get borderBottomColor(): string
  set borderBottomColor(value?: string | undefined | null)
  get borderBottomLeftRadius(): string
  set borderBottomLeftRadius(value?: string | undefined | null)
  get borderBottomRightRadius(): string
  set borderBottomRightRadius(value?: string | undefined | null)
  get borderBottomStyle(): string
  set borderBottomStyle(value?: string | undefined | null)
  get borderBottomWidth(): string
  set borderBottomWidth(value?: string | undefined | null)
  get borderCollapse(): string
  set borderCollapse(value?: string | undefined | null)
  get borderColor(): string
  set borderColor(value?: string | undefined | null)
  get borderEndEndRadius(): string
  set borderEndEndRadius(value?: string | undefined | null)
  get borderEndStartRadius(): string
  set borderEndStartRadius(value?: string | undefined | null)
  get borderImage(): string
  set borderImage(value?: string | undefined | null)
  get borderImageOutset(): string
  set borderImageOutset(value?: string | undefined | null)
  get borderImageRepeat(): string
  set borderImageRepeat(value?: string | undefined | null)
  get borderImageSlice(): string
  set borderImageSlice(value?: string | undefined | null)
  get borderImageSource(): string
  set borderImageSource(value?: string | undefined | null)
  get borderImageWidth(): string
  set borderImageWidth(value?: string | undefined | null)
  get borderInline(): string
  set borderInline(value?: string | undefined | null)
  get borderInlineColor(): string
  set borderInlineColor(value?: string | undefined | null)
  get borderInlineEnd(): string
  set borderInlineEnd(value?: string | undefined | null)
  get borderInlineEndColor(): string
  set borderInlineEndColor(value?: string | undefined | null)
  get borderInlineEndStyle(): string
  set borderInlineEndStyle(value?: string | undefined | null)
  get borderInlineEndWidth(): string
  set borderInlineEndWidth(value?: string | undefined | null)
  get borderInlineStart(): string
  set borderInlineStart(value?: string | undefined | null)
  get borderInlineStartColor(): string
  set borderInlineStartColor(value?: string | undefined | null)
  get borderInlineStartStyle(): string
  set borderInlineStartStyle(value?: string | undefined | null)
  get borderInlineStartWidth(): string
  set borderInlineStartWidth(value?: string | undefined | null)
  get borderInlineStyle(): string
  set borderInlineStyle(value?: string | undefined | null)
  get borderInlineWidth(): string
  set borderInlineWidth(value?: string | undefined | null)
  get borderLeft(): string
  set borderLeft(value?: string | undefined | null)
  get borderLeftColor(): string
  set borderLeftColor(value?: string | undefined | null)
  get borderLeftStyle(): string
  set borderLeftStyle(value?: string | undefined | null)
  get borderLeftWidth(): string
  set borderLeftWidth(value?: string | undefined | null)
  get borderRadius(): string
  set borderRadius(value?: string | undefined | null)
  get borderRight(): string
  set borderRight(value?: string | undefined | null)
  get borderRightColor(): string
  set borderRightColor(value?: string | undefined | null)
  get borderRightStyle(): string
  set borderRightStyle(value?: string | undefined | null)
  get borderRightWidth(): string
  set borderRightWidth(value?: string | undefined | null)
  get borderSpacing(): string
  set borderSpacing(value?: string | undefined | null)
  get borderStartEndRadius(): string
  set borderStartEndRadius(value?: string | undefined | null)
  get borderStartStartRadius(): string
  set borderStartStartRadius(value?: string | undefined | null)
  get borderStyle(): string
  set borderStyle(value?: string | undefined | null)
  get borderTop(): string
  set borderTop(value?: string | undefined | null)
  get borderTopColor(): string
  set borderTopColor(value?: string | undefined | null)
  get borderTopLeftRadius(): string
  set borderTopLeftRadius(value?: string | undefined | null)
  get borderTopRightRadius(): string
  set borderTopRightRadius(value?: string | undefined | null)
  get borderTopStyle(): string
  set borderTopStyle(value?: string | undefined | null)
  get borderTopWidth(): string
  set borderTopWidth(value?: string | undefined | null)
  get borderWidth(): string
  set borderWidth(value?: string | undefined | null)
  get bottom(): string
  set bottom(value?: string | undefined | null)
  get boxShadow(): string
  set boxShadow(value?: string | undefined | null)
  get boxSizing(): string
  set boxSizing(value?: string | undefined | null)
  get breakAfter(): string
  set breakAfter(value?: string | undefined | null)
  get breakBefore(): string
  set breakBefore(value?: string | undefined | null)
  get breakInside(): string
  set breakInside(value?: string | undefined | null)
  get captionSide(): string
  set captionSide(value?: string | undefined | null)
  get caretColor(): string
  set caretColor(value?: string | undefined | null)
  get clear(): string
  set clear(value?: string | undefined | null)
  get clip(): string
  set clip(value?: string | undefined | null)
  get clipPath(): string
  set clipPath(value?: string | undefined | null)
  get clipRule(): string
  set clipRule(value?: string | undefined | null)
  get color(): string
  set color(value?: string | undefined | null)
  get colorInterpolation(): string
  set colorInterpolation(value?: string | undefined | null)
  get colorInterpolationFilters(): string
  set colorInterpolationFilters(value?: string | undefined | null)
  get colorScheme(): string
  set colorScheme(value?: string | undefined | null)
  get columnCount(): string
  set columnCount(value?: string | undefined | null)
  get columnFill(): string
  set columnFill(value?: string | undefined | null)
  get columnGap(): string
  set columnGap(value?: string | undefined | null)
  get columnRule(): string
  set columnRule(value?: string | undefined | null)
  get columnRuleColor(): string
  set columnRuleColor(value?: string | undefined | null)
  get columnRuleStyle(): string
  set columnRuleStyle(value?: string | undefined | null)
  get columnRuleWidth(): string
  set columnRuleWidth(value?: string | undefined | null)
  get columnSpan(): string
  set columnSpan(value?: string | undefined | null)
  get columnWidth(): string
  set columnWidth(value?: string | undefined | null)
  get columns(): string
  set columns(value?: string | undefined | null)
  get contain(): string
  set contain(value?: string | undefined | null)
  get content(): string
  set content(value?: string | undefined | null)
  get counterIncrement(): string
  set counterIncrement(value?: string | undefined | null)
  get counterReset(): string
  set counterReset(value?: string | undefined | null)
  get counterSet(): string
  set counterSet(value?: string | undefined | null)
  get cursor(): string
  set cursor(value?: string | undefined | null)
  get direction(): string
  set direction(value?: string | undefined | null)
  get display(): string
  set display(value?: string | undefined | null)
  get dominantBaseline(): string
  set dominantBaseline(value?: string | undefined | null)
  get emptyCells(): string
  set emptyCells(value?: string | undefined | null)
  get fill(): string
  set fill(value?: string | undefined | null)
  get fillOpacity(): string
  set fillOpacity(value?: string | undefined | null)
  get fillRule(): string
  set fillRule(value?: string | undefined | null)
  get filter(): string
  set filter(value?: string | undefined | null)
  get flex(): string
  set flex(value?: string | undefined | null)
  get flexBasis(): string
  set flexBasis(value?: string | undefined | null)
  get flexDirection(): string
  set flexDirection(value?: string | undefined | null)
  get flexFlow(): string
  set flexFlow(value?: string | undefined | null)
  get flexGrow(): string
  set flexGrow(value?: string | undefined | null)
  get flexShrink(): string
  set flexShrink(value?: string | undefined | null)
  get flexWrap(): string
  set flexWrap(value?: string | undefined | null)
  get float(): string
  set float(value?: string | undefined | null)
  get floodColor(): string
  set floodColor(value?: string | undefined | null)
  get floodOpacity(): string
  set floodOpacity(value?: string | undefined | null)
  get font(): string
  set font(value?: string | undefined | null)
  get fontFamily(): string
  set fontFamily(value?: string | undefined | null)
  get fontFeatureSettings(): string
  set fontFeatureSettings(value?: string | undefined | null)
  get fontKerning(): string
  set fontKerning(value?: string | undefined | null)
  get fontOpticalSizing(): string
  set fontOpticalSizing(value?: string | undefined | null)
  get fontSize(): string
  set fontSize(value?: string | undefined | null)
  get fontSizeAdjust(): string
  set fontSizeAdjust(value?: string | undefined | null)
  get fontStretch(): string
  set fontStretch(value?: string | undefined | null)
  get fontStyle(): string
  set fontStyle(value?: string | undefined | null)
  get fontSynthesis(): string
  set fontSynthesis(value?: string | undefined | null)
  get fontVariant(): string
  set fontVariant(value?: string | undefined | null)
  get fontVariantAlternates(): string
  set fontVariantAlternates(value?: string | undefined | null)
  get fontVariantCaps(): string
  set fontVariantCaps(value?: string | undefined | null)
  get fontVariantEastAsian(): string
  set fontVariantEastAsian(value?: string | undefined | null)
  get fontVariantLigatures(): string
  set fontVariantLigatures(value?: string | undefined | null)
  get fontVariantNumeric(): string
  set fontVariantNumeric(value?: string | undefined | null)
  get fontVariantPosition(): string
  set fontVariantPosition(value?: string | undefined | null)
  get fontVariationSettings(): string
  set fontVariationSettings(value?: string | undefined | null)
  get fontWeight(): string
  set fontWeight(value?: string | undefined | null)
  get gap(): string
  set gap(value?: string | undefined | null)
  get grid(): string
  set grid(value?: string | undefined | null)
  get gridArea(): string
  set gridArea(value?: string | undefined | null)
  get gridAutoColumns(): string
  set gridAutoColumns(value?: string | undefined | null)
  get gridAutoFlow(): string
  set gridAutoFlow(value?: string | undefined | null)
  get gridAutoRows(): string
  set gridAutoRows(value?: string | undefined | null)
  get gridColumn(): string
  set gridColumn(value?: string | undefined | null)
  get gridColumnEnd(): string
  set gridColumnEnd(value?: string | undefined | null)
  get gridColumnGap(): string
  set gridColumnGap(value?: string | undefined | null)
  get gridColumnStart(): string
  set gridColumnStart(value?: string | undefined | null)
  get gridGap(): string
  set gridGap(value?: string | undefined | null)
  get gridRow(): string
  set gridRow(value?: string | undefined | null)
  get gridRowEnd(): string
  set gridRowEnd(value?: string | undefined | null)
  get gridRowGap(): string
  set gridRowGap(value?: string | undefined | null)
  get gridRowStart(): string
  set gridRowStart(value?: string | undefined | null)
  get gridTemplate(): string
  set gridTemplate(value?: string | undefined | null)
  get gridTemplateAreas(): string
  set gridTemplateAreas(value?: string | undefined | null)
  get gridTemplateColumns(): string
  set gridTemplateColumns(value?: string | undefined | null)
  get gridTemplateRows(): string
  set gridTemplateRows(value?: string | undefined | null)
  get height(): string
  set height(value?: string | undefined | null)
  get hyphens(): string
  set hyphens(value?: string | undefined | null)
  get imageOrientation(): string
  set imageOrientation(value?: string | undefined | null)
  get imageRendering(): string
  set imageRendering(value?: string | undefined | null)
  get inlineSize(): string
  set inlineSize(value?: string | undefined | null)
  get inset(): string
  set inset(value?: string | undefined | null)
  get insetBlock(): string
  set insetBlock(value?: string | undefined | null)
  get insetBlockEnd(): string
  set insetBlockEnd(value?: string | undefined | null)
  get insetBlockStart(): string
  set insetBlockStart(value?: string | undefined | null)
  get insetInline(): string
  set insetInline(value?: string | undefined | null)
  get insetInlineEnd(): string
  set insetInlineEnd(value?: string | undefined | null)
  get insetInlineStart(): string
  set insetInlineStart(value?: string | undefined | null)
  get isolation(): string
  set isolation(value?: string | undefined | null)
  get justifyContent(): string
  set justifyContent(value?: string | undefined | null)
  get justifyItems(): string
  set justifyItems(value?: string | undefined | null)
  get justifySelf(): string
  set justifySelf(value?: string | undefined | null)
  get left(): string
  set left(value?: string | undefined | null)
  get letterSpacing(): string
  set letterSpacing(value?: string | undefined | null)
  get lightingColor(): string
  set lightingColor(value?: string | undefined | null)
  get lineBreak(): string
  set lineBreak(value?: string | undefined | null)
  get lineHeight(): string
  set lineHeight(value?: string | undefined | null)
  get listStyle(): string
  set listStyle(value?: string | undefined | null)
  get listStyleImage(): string
  set listStyleImage(value?: string | undefined | null)
  get listStylePosition(): string
  set listStylePosition(value?: string | undefined | null)
  get listStyleType(): string
  set listStyleType(value?: string | undefined | null)
  get margin(): string
  set margin(value?: string | undefined | null)
  get marginBlock(): string
  set marginBlock(value?: string | undefined | null)
  get marginBlockEnd(): string
  set marginBlockEnd(value?: string | undefined | null)
  get marginBlockStart(): string
  set marginBlockStart(value?: string | undefined | null)
  get marginBottom(): string
  set marginBottom(value?: string | undefined | null)
  get marginInline(): string
  set marginInline(value?: string | undefined | null)
  get marginInlineEnd(): string
  set marginInlineEnd(value?: string | undefined | null)
  get marginInlineStart(): string
  set marginInlineStart(value?: string | undefined | null)
  get marginLeft(): string
  set marginLeft(value?: string | undefined | null)
  get marginRight(): string
  set marginRight(value?: string | undefined | null)
  get marginTop(): string
  set marginTop(value?: string | undefined | null)
  get marker(): string
  set marker(value?: string | undefined | null)
  get markerEnd(): string
  set markerEnd(value?: string | undefined | null)
  get markerMid(): string
  set markerMid(value?: string | undefined | null)
  get markerStart(): string
  set markerStart(value?: string | undefined | null)
  get mask(): string
  set mask(value?: string | undefined | null)
  get maskClip(): string
  set maskClip(value?: string | undefined | null)
  get maskComposite(): string
  set maskComposite(value?: string | undefined | null)
  get maskImage(): string
  set maskImage(value?: string | undefined | null)
  get maskMode(): string
  set maskMode(value?: string | undefined | null)
  get maskOrigin(): string
  set maskOrigin(value?: string | undefined | null)
  get maskPosition(): string
  set maskPosition(value?: string | undefined | null)
  get maskRepeat(): string
  set maskRepeat(value?: string | undefined | null)
  get maskSize(): string
  set maskSize(value?: string | undefined | null)
  get maskType(): string
  set maskType(value?: string | undefined | null)
  get maxBlockSize(): string
  set maxBlockSize(value?: string | undefined | null)
  get maxHeight(): string
  set maxHeight(value?: string | undefined | null)
  get maxInlineSize(): string
  set maxInlineSize(value?: string | undefined | null)
  get maxWidth(): string
  set maxWidth(value?: string | undefined | null)
  get minBlockSize(): string
  set minBlockSize(value?: string | undefined | null)
  get minHeight(): string
  set minHeight(value?: string | undefined | null)
  get minInlineSize(): string
  set minInlineSize(value?: string | undefined | null)
  get minWidth(): string
  set minWidth(value?: string | undefined | null)
  get mixBlendMode(): string
  set mixBlendMode(value?: string | undefined | null)
  get objectFit(): string
  set objectFit(value?: string | undefined | null)
  get objectPosition(): string
  set objectPosition(value?: string | undefined | null)
  get offset(): string
  set offset(value?: string | undefined | null)
  get offsetDistance(): string
  set offsetDistance(value?: string | undefined | null)
  get offsetPath(): string
  set offsetPath(value?: string | undefined | null)
  get offsetRotate(): string
  set offsetRotate(value?: string | undefined | null)
  get opacity(): string
  set opacity(value?: string | undefined | null)
  get order(): string
  set order(value?: string | undefined | null)
  get orphans(): string
  set orphans(value?: string | undefined | null)
  get outline(): string
  set outline(value?: string | undefined | null)
  get outlineColor(): string
  set outlineColor(value?: string | undefined | null)
  get outlineOffset(): string
  set outlineOffset(value?: string | undefined | null)
  get outlineStyle(): string
  set outlineStyle(value?: string | undefined | null)
  get outlineWidth(): string
  set outlineWidth(value?: string | undefined | null)
  get overflow(): string
  set overflow(value?: string | undefined | null)
  get overflowAnchor(): string
  set overflowAnchor(value?: string | undefined | null)
  get overflowWrap(): string
  set overflowWrap(value?: string | undefined | null)
  get overflowX(): string
  set overflowX(value?: string | undefined | null)
  get overflowY(): string
  set overflowY(value?: string | undefined | null)
  get overscrollBehavior(): string
  set overscrollBehavior(value?: string | undefined | null)
  get overscrollBehaviorBlock(): string
  set overscrollBehaviorBlock(value?: string | undefined | null)
  get overscrollBehaviorInline(): string
  set overscrollBehaviorInline(value?: string | undefined | null)
  get overscrollBehaviorX(): string
  set overscrollBehaviorX(value?: string | undefined | null)
  get overscrollBehaviorY(): string
  set overscrollBehaviorY(value?: string | undefined | null)
  get padding(): string
  set padding(value?: string | undefined | null)
  get paddingBlock(): string
  set paddingBlock(value?: string | undefined | null)
  get paddingBlockEnd(): string
  set paddingBlockEnd(value?: string | undefined | null)
  get paddingBlockStart(): string
  set paddingBlockStart(value?: string | undefined | null)
  get paddingBottom(): string
  set paddingBottom(value?: string | undefined | null)
  get paddingInline(): string
  set paddingInline(value?: string | undefined | null)
  get paddingInlineEnd(): string
  set paddingInlineEnd(value?: string | undefined | null)
  get paddingInlineStart(): string
  set paddingInlineStart(value?: string | undefined | null)
  get paddingLeft(): string
  set paddingLeft(value?: string | undefined | null)
  get paddingRight(): string
  set paddingRight(value?: string | undefined | null)
  get paddingTop(): string
  set paddingTop(value?: string | undefined | null)
  get pageBreakAfter(): string
  set pageBreakAfter(value?: string | undefined | null)
  get pageBreakBefore(): string
  set pageBreakBefore(value?: string | undefined | null)
  get pageBreakInside(): string
  set pageBreakInside(value?: string | undefined | null)
  get paintOrder(): string
  set paintOrder(value?: string | undefined | null)
  get perspective(): string
  set perspective(value?: string | undefined | null)
  get perspectiveOrigin(): string
  set perspectiveOrigin(value?: string | undefined | null)
  get placeContent(): string
  set placeContent(value?: string | undefined | null)
  get placeItems(): string
  set placeItems(value?: string | undefined | null)
  get placeSelf(): string
  set placeSelf(value?: string | undefined | null)
  get pointerEvents(): string
  set pointerEvents(value?: string | undefined | null)
  get position(): string
  set position(value?: string | undefined | null)
  get printColorAdjust(): string
  set printColorAdjust(value?: string | undefined | null)
  get quotes(): string
  set quotes(value?: string | undefined | null)
  get resize(): string
  set resize(value?: string | undefined | null)
  get right(): string
  set right(value?: string | undefined | null)
  get rotate(): string
  set rotate(value?: string | undefined | null)
  get rowGap(): string
  set rowGap(value?: string | undefined | null)
  get rubyPosition(): string
  set rubyPosition(value?: string | undefined | null)
  get scale(): string
  set scale(value?: string | undefined | null)
  get scrollBehavior(): string
  set scrollBehavior(value?: string | undefined | null)
  get scrollMargin(): string
  set scrollMargin(value?: string | undefined | null)
  get scrollMarginBlock(): string
  set scrollMarginBlock(value?: string | undefined | null)
  get scrollMarginBlockEnd(): string
  set scrollMarginBlockEnd(value?: string | undefined | null)
  get scrollMarginBlockStart(): string
  set scrollMarginBlockStart(value?: string | undefined | null)
  get scrollMarginBottom(): string
  set scrollMarginBottom(value?: string | undefined | null)
  get scrollMarginInline(): string
  set scrollMarginInline(value?: string | undefined | null)
  get scrollMarginInlineEnd(): string
  set scrollMarginInlineEnd(value?: string | undefined | null)
  get scrollMarginInlineStart(): string
  set scrollMarginInlineStart(value?: string | undefined | null)
  get scrollMarginLeft(): string
  set scrollMarginLeft(value?: string | undefined | null)
  get scrollMarginRight(): string
  set scrollMarginRight(value?: string | undefined | null)
  get scrollMarginTop(): string
  set scrollMarginTop(value?: string | undefined | null)
  get scrollPadding(): string
  set scrollPadding(value?: string | undefined | null)
  get scrollPaddingBlock(): string
  set scrollPaddingBlock(value?: string | undefined | null)
  get scrollPaddingBlockEnd(): string
  set scrollPaddingBlockEnd(value?: string | undefined | null)
  get scrollPaddingBlockStart(): string
  set scrollPaddingBlockStart(value?: string | undefined | null)
  get scrollPaddingBottom(): string
  set scrollPaddingBottom(value?: string | undefined | null)
  get scrollPaddingInline(): string
  set scrollPaddingInline(value?: string | undefined | null)
  get scrollPaddingInlineEnd(): string
  set scrollPaddingInlineEnd(value?: string | undefined | null)
  get scrollPaddingInlineStart(): string
  set scrollPaddingInlineStart(value?: string | undefined | null)
  get scrollPaddingLeft(): string
  set scrollPaddingLeft(value?: string | undefined | null)
  get scrollPaddingRight(): string
  set scrollPaddingRight(value?: string | undefined | null)
  get scrollPaddingTop(): string
  set scrollPaddingTop(value?: string | undefined | null)
  get scrollSnapAlign(): string
  set scrollSnapAlign(value?: string | undefined | null)
  get scrollSnapStop(): string
  set scrollSnapStop(value?: string | undefined | null)
  get scrollSnapType(): string
  set scrollSnapType(value?: string | undefined | null)
  get scrollbarGutter(): string
  set scrollbarGutter(value?: string | undefined | null)
  get shapeImageThreshold(): string
  set shapeImageThreshold(value?: string | undefined | null)
  get shapeMargin(): string
  set shapeMargin(value?: string | undefined | null)
  get shapeOutside(): string
  set shapeOutside(value?: string | undefined | null)
  get shapeRendering(): string
  set shapeRendering(value?: string | undefined | null)
  get stopColor(): string
  set stopColor(value?: string | undefined | null)
  get stopOpacity(): string
  set stopOpacity(value?: string | undefined | null)
  get stroke(): string
  set stroke(value?: string | undefined | null)
  get strokeDasharray(): string
  set strokeDasharray(value?: string | undefined | null)
  get strokeDashoffset(): string
  set strokeDashoffset(value?: string | undefined | null)
  get strokeLinecap(): string
  set strokeLinecap(value?: string | undefined | null)
  get strokeLinejoin(): string
  set strokeLinejoin(value?: string | undefined | null)
  get strokeMiterlimit(): string
  set strokeMiterlimit(value?: string | undefined | null)
  get strokeOpacity(): string
  set strokeOpacity(value?: string | undefined | null)
  get strokeWidth(): string
  set strokeWidth(value?: string | undefined | null)
  get tabSize(): string
  set tabSize(value?: string | undefined | null)
  get tableLayout(): string
  set tableLayout(value?: string | undefined | null)
  get textAlign(): string
  set textAlign(value?: string | undefined | null)
  get textAlignLast(): string
  set textAlignLast(value?: string | undefined | null)
  get textAnchor(): string
  set textAnchor(value?: string | undefined | null)
  get textCombineUpright(): string
  set textCombineUpright(value?: string | undefined | null)
  get textDecoration(): string
  set textDecoration(value?: string | undefined | null)
  get textDecorationColor(): string
  set textDecorationColor(value?: string | undefined | null)
  get textDecorationLine(): string
  set textDecorationLine(value?: string | undefined | null)
  get textDecorationSkipInk(): string
  set textDecorationSkipInk(value?: string | undefined | null)
  get textDecorationStyle(): string
  set textDecorationStyle(value?: string | undefined | null)
  get textDecorationThickness(): string
  set textDecorationThickness(value?: string | undefined | null)
  get textEmphasis(): string
  set textEmphasis(value?: string | undefined | null)
  get textEmphasisColor(): string
  set textEmphasisColor(value?: string | undefined | null)
  get textEmphasisPosition(): string
  set textEmphasisPosition(value?: string | undefined | null)
  get textEmphasisStyle(): string
  set textEmphasisStyle(value?: string | undefined | null)
  get textIndent(): string
  set textIndent(value?: string | undefined | null)
  get textOrientation(): string
  set textOrientation(value?: string | undefined | null)
  get textOverflow(): string
  set textOverflow(value?: string | undefined | null)
  get textRendering(): string
  set textRendering(value?: string | undefined | null)
  get textShadow(): string
  set textShadow(value?: string | undefined | null)
  get textTransform(): string
  set textTransform(value?: string | undefined | null)
  get textUnderlineOffset(): string
  set textUnderlineOffset(value?: string | undefined | null)
  get textUnderlinePosition(): string
  set textUnderlinePosition(value?: string | undefined | null)
  get top(): string
  set top(value?: string | undefined | null)
  get touchAction(): string
  set touchAction(value?: string | undefined | null)
  get transform(): string
  set transform(value?: string | undefined | null)
  get transformBox(): string
  set transformBox(value?: string | undefined | null)
  get transformOrigin(): string
  set transformOrigin(value?: string | undefined | null)
  get transformStyle(): string
  set transformStyle(value?: string | undefined | null)
  get transition(): string
  set transition(value?: string | undefined | null)
  get transitionDelay(): string
  set transitionDelay(value?: string | undefined | null)
  get transitionDuration(): string
  set transitionDuration(value?: string | undefined | null)
  get transitionProperty(): string
  set transitionProperty(value?: string | undefined | null)
  get transitionTimingFunction(): string
  set transitionTimingFunction(value?: string | undefined | null)
  get translate(): string
  set translate(value?: string | undefined | null)
  get unicodeBidi(): string
  set unicodeBidi(value?: string | undefined | null)
  get userSelect(): string
  set userSelect(value?: string | undefined | null)
  get verticalAlign(): string
  set verticalAlign(value?: string | undefined | null)
  get visibility(): string
  set visibility(value?: string | undefined | null)
  get webkitAlignContent(): string
  set webkitAlignContent(value?: string | undefined | null)
  get webkitAlignItems(): string
  set webkitAlignItems(value?: string | undefined | null)
  get webkitAlignSelf(): string
  set webkitAlignSelf(value?: string | undefined | null)
  get webkitAnimation(): string
  set webkitAnimation(value?: string | undefined | null)
  get webkitAnimationDelay(): string
  set webkitAnimationDelay(value?: string | undefined | null)
  get webkitAnimationDirection(): string
  set webkitAnimationDirection(value?: string | undefined | null)
  get webkitAnimationDuration(): string
  set webkitAnimationDuration(value?: string | undefined | null)
  get webkitAnimationFillMode(): string
  set webkitAnimationFillMode(value?: string | undefined | null)
  get webkitAnimationIterationCount(): string
  set webkitAnimationIterationCount(value?: string | undefined | null)
  get webkitAnimationName(): string
  set webkitAnimationName(value?: string | undefined | null)
  get webkitAnimationPlayState(): string
  set webkitAnimationPlayState(value?: string | undefined | null)
  get webkitAnimationTimingFunction(): string
  set webkitAnimationTimingFunction(value?: string | undefined | null)
  get webkitAppearance(): string
  set webkitAppearance(value?: string | undefined | null)
  get webkitBackfaceVisibility(): string
  set webkitBackfaceVisibility(value?: string | undefined | null)
  get webkitBackgroundClip(): string
  set webkitBackgroundClip(value?: string | undefined | null)
  get webkitBackgroundOrigin(): string
  set webkitBackgroundOrigin(value?: string | undefined | null)
  get webkitBackgroundSize(): string
  set webkitBackgroundSize(value?: string | undefined | null)
  get webkitBorderBottomLeftRadius(): string
  set webkitBorderBottomLeftRadius(value?: string | undefined | null)
  get webkitBorderBottomRightRadius(): string
  set webkitBorderBottomRightRadius(value?: string | undefined | null)
  get webkitBorderRadius(): string
  set webkitBorderRadius(value?: string | undefined | null)
  get webkitBorderTopLeftRadius(): string
  set webkitBorderTopLeftRadius(value?: string | undefined | null)
  get webkitBorderTopRightRadius(): string
  set webkitBorderTopRightRadius(value?: string | undefined | null)
  get webkitBoxAlign(): string
  set webkitBoxAlign(value?: string | undefined | null)
  get webkitBoxFlex(): string
  set webkitBoxFlex(value?: string | undefined | null)
  get webkitBoxOrdinalGroup(): string
  set webkitBoxOrdinalGroup(value?: string | undefined | null)
  get webkitBoxOrient(): string
  set webkitBoxOrient(value?: string | undefined | null)
  get webkitBoxPack(): string
  set webkitBoxPack(value?: string | undefined | null)
  get webkitBoxShadow(): string
  set webkitBoxShadow(value?: string | undefined | null)
  get webkitBoxSizing(): string
  set webkitBoxSizing(value?: string | undefined | null)
  get webkitFilter(): string
  set webkitFilter(value?: string | undefined | null)
  get webkitFlex(): string
  set webkitFlex(value?: string | undefined | null)
  get webkitFlexBasis(): string
  set webkitFlexBasis(value?: string | undefined | null)
  get webkitFlexDirection(): string
  set webkitFlexDirection(value?: string | undefined | null)
  get webkitFlexFlow(): string
  set webkitFlexFlow(value?: string | undefined | null)
  get webkitFlexGrow(): string
  set webkitFlexGrow(value?: string | undefined | null)
  get webkitFlexShrink(): string
  set webkitFlexShrink(value?: string | undefined | null)
  get webkitFlexWrap(): string
  set webkitFlexWrap(value?: string | undefined | null)
  get webkitJustifyContent(): string
  set webkitJustifyContent(value?: string | undefined | null)
  get webkitLineClamp(): string
  set webkitLineClamp(value?: string | undefined | null)
  get webkitMask(): string
  set webkitMask(value?: string | undefined | null)
  get webkitMaskBoxImage(): string
  set webkitMaskBoxImage(value?: string | undefined | null)
  get webkitMaskBoxImageOutset(): string
  set webkitMaskBoxImageOutset(value?: string | undefined | null)
  get webkitMaskBoxImageRepeat(): string
  set webkitMaskBoxImageRepeat(value?: string | undefined | null)
  get webkitMaskBoxImageSlice(): string
  set webkitMaskBoxImageSlice(value?: string | undefined | null)
  get webkitMaskBoxImageSource(): string
  set webkitMaskBoxImageSource(value?: string | undefined | null)
  get webkitMaskBoxImageWidth(): string
  set webkitMaskBoxImageWidth(value?: string | undefined | null)
  get webkitMaskClip(): string
  set webkitMaskClip(value?: string | undefined | null)
  get webkitMaskComposite(): string
  set webkitMaskComposite(value?: string | undefined | null)
  get webkitMaskImage(): string
  set webkitMaskImage(value?: string | undefined | null)
  get webkitMaskOrigin(): string
  set webkitMaskOrigin(value?: string | undefined | null)
  get webkitMaskPosition(): string
  set webkitMaskPosition(value?: string | undefined | null)
  get webkitMaskRepeat(): string
  set webkitMaskRepeat(value?: string | undefined | null)
  get webkitMaskSize(): string
  set webkitMaskSize(value?: string | undefined | null)
  get webkitOrder(): string
  set webkitOrder(value?: string | undefined | null)
  get webkitPerspective(): string
  set webkitPerspective(value?: string | undefined | null)
  get webkitPerspectiveOrigin(): string
  set webkitPerspectiveOrigin(value?: string | undefined | null)
  get webkitTextFillColor(): string
  set webkitTextFillColor(value?: string | undefined | null)
  get webkitTextSizeAdjust(): string
  set webkitTextSizeAdjust(value?: string | undefined | null)
  get webkitTextStroke(): string
  set webkitTextStroke(value?: string | undefined | null)
  get webkitTextStrokeColor(): string
  set webkitTextStrokeColor(value?: string | undefined | null)
  get webkitTextStrokeWidth(): string
  set webkitTextStrokeWidth(value?: string | undefined | null)
  get webkitTransform(): string
  set webkitTransform(value?: string | undefined | null)
  get webkitTransformOrigin(): string
  set webkitTransformOrigin(value?: string | undefined | null)
  get webkitTransformStyle(): string
  set webkitTransformStyle(value?: string | undefined | null)
  get webkitTransition(): string
  set webkitTransition(value?: string | undefined | null)
  get webkitTransitionDelay(): string
  set webkitTransitionDelay(value?: string | undefined | null)
  get webkitTransitionDuration(): string
  set webkitTransitionDuration(value?: string | undefined | null)
  get webkitTransitionProperty(): string
  set webkitTransitionProperty(value?: string | undefined | null)
  get webkitTransitionTimingFunction(): string
  set webkitTransitionTimingFunction(value?: string | undefined | null)
  get webkitUserSelect(): string
  set webkitUserSelect(value?: string | undefined | null)
  get whiteSpace(): string
  set whiteSpace(value?: string | undefined | null)
  get widows(): string
  set widows(value?: string | undefined | null)
  get width(): string
  set width(value?: string | undefined | null)
  get willChange(): string
  set willChange(value?: string | undefined | null)
  get wordBreak(): string
  set wordBreak(value?: string | undefined | null)
  get wordSpacing(): string
  set wordSpacing(value?: string | undefined | null)
  get wordWrap(): string
  set wordWrap(value?: string | undefined | null)
  get writingMode(): string
  set writingMode(value?: string | undefined | null)
  get zIndex(): string
  set zIndex(value?: string | undefined | null)
  getPropertyValue(property: string): string
  getPropertyPriority(property: string): string
  removeProperty(property: string): string
  setProperty(property: string, value: string, priority?: string | undefined | null): void
  get cssText(): string
  set cssText(cssText: string)
  get cssFloat(): string
  set cssFloat(value: string)
  item(index: number): string | null
  get length(): number
}
