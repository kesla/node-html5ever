/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export function parse(html: string): Html5EverDom
export class Comment {
  get parentElement(): Element | null
  get parentNode(): Element | Document | null
}
export class DocType {
  name: string
  publicId: string
  systemId: string
  get parentElement(): Element | null
  get parentNode(): Element | Document | null
}
export class Document {
  get docType(): DocType | null
  get documentElement(): Element
  get head(): Element
  get body(): Element
  get nodeName(): string
  createElement(name: string): Element
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  quirksMode: QuirksMode
  errors: Array<string>
  get document(): Document
  serialize(): string
}
export class Element {
  get parentElement(): Element | null
  get parentNode(): Element | Document | null
  getAttribute(name: string): string | null
  setAttribute(name: string, value: string): void
  hasAttribute(name: string): boolean
  get nodeName(): string
  get tagName(): string
  get children(): Array<Element>
  get innerHtml(): string
  get outerHtml(): string
}
export class NodeList {
  get(index: number): Handle
}
export class Text {
  get parentElement(): Element | null
  get parentNode(): Element | Document | null
}
