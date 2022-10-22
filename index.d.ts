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
  get style(): object
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
