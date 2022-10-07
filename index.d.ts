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
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): Document | DocumentFragment | Element | null
  remove(): void
  get ownerDocument(): Document | null
  get previousSibling(): Comment | DocType | Element | Text | null
  get previousElementSibling(): Element | null
  get nextSibling(): Comment | DocType | Element | Text | null
  get nextElementSibling(): Element | null
}
export class DocType {
  name: string
  publicId: string
  systemId: string
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): Document | DocumentFragment | Element | null
  remove(): void
  get ownerDocument(): Document | null
  get previousSibling(): Comment | DocType | Element | Text | null
  get previousElementSibling(): Element | null
  get nextSibling(): Comment | DocType | Element | Text | null
  get nextElementSibling(): Element | null
}
export class Document {
  get nodeName(): string
  get childNodes(): Array<Comment | DocType | Element | Text>
  get children(): Array<Element>
  appendChild(child: Comment | DocType | Element | Text): void
  removeElement(child: Element): void
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  get docType(): DocType | null
  get documentElement(): Element
  get head(): Element
  get body(): Element
  get nodeName(): string
  createElement(name: string): Element
  createTextNode(data: string): Text
}
export class DocumentFragment {
  get nodeName(): string
  get childNodes(): Array<Comment | DocType | Element | Text>
  get children(): Array<Element>
  appendChild(child: Comment | DocType | Element | Text): void
  removeElement(child: Element): void
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
}
export class Element {
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): Document | DocumentFragment | Element | null
  remove(): void
  get ownerDocument(): Document | null
  get previousSibling(): Comment | DocType | Element | Text | null
  get previousElementSibling(): Element | null
  get nextSibling(): Comment | DocType | Element | Text | null
  get nextElementSibling(): Element | null
  get childNodes(): Array<Comment | DocType | Element | Text>
  get children(): Array<Element>
  appendChild(child: Comment | DocType | Element | Text): void
  removeElement(child: Element): void
  getElementById(id: string): Element | null
  getElementsByClassName(className: string): Array<Element>
  getAttribute(name: string): string | null
  removeAttribute(name: string): void
  setAttribute(name: string, value: string): void
  hasAttribute(name: string): boolean
  get tagName(): string
  get innerHtml(): string
  get outerHtml(): string
  get className(): string
  set className(className: string)
  get id(): string
  set id(id: string)
}
export class Text {
  get nodeName(): string
  get parentElement(): Element | null
  get parentNode(): Document | DocumentFragment | Element | null
  remove(): void
  get ownerDocument(): Document | null
  get previousSibling(): Comment | DocType | Element | Text | null
  get previousElementSibling(): Element | null
  get nextSibling(): Comment | DocType | Element | Text | null
  get nextElementSibling(): Element | null
}
