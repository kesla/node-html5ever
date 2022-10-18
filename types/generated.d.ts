/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  quirksMode: QuirksMode
  errors: Array<string>
  constructor(html: string)
  static createDocumentFragment(html: string): DocumentFragment
  get document(): Document
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
}
export class Document {
  get nodeName(): string
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild(child: ChildNode): ChildNode
  removeChild(child: ChildNode): ChildNode
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
  get docType(): DocumentType | null
  get documentElement(): Element
  get head(): Element
  get body(): Element
  get nodeName(): string
  createElement(name: string): Element
  createTextNode(data: string): Text
  querySelectorAll(selectors: string): Array<Element>
}
export class DocumentFragment {
  get nodeName(): string
  get childNodes(): Array<ChildNode>
  get children(): Array<Element>
  append(childNodeOrText: ChildNode | string): void
  prepend(childNodeOrText: ChildNode | string): void
  appendChild(child: ChildNode): ChildNode
  removeChild(child: ChildNode): ChildNode
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
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
  appendChild(child: ChildNode): ChildNode
  removeChild(child: ChildNode): ChildNode
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  get firstChild(): ChildNode | null
  get firstElementChild(): Element | null
  get lastChild(): ChildNode | null
  get lastElementChild(): Element | null
  getAttribute(name: string): string | null
  removeAttribute(name: string): void
  setAttribute(name: string, value: string): void
  hasAttribute(name: string): boolean
  get tagName(): string
  get innerHTML(): string
  get outerHTML(): string
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
}
