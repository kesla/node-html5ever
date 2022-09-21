/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export function parseDocument(html: string): Html5EverDom
export class DocType {
  name: string
  publicId: string
  systemId: string
  get parentNode(): Element | Document | null
}
export class Document {
  get docType(): DocType | null
  get documentElement(): Element
  get head(): Element
  get body(): Element
  get nodeName(): string
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  quirksMode: QuirksMode
  errors: Array<string>
  get document(): Document
  serialize(): string
}
export class Element {
  getAttribute(key: string): string | null
  get nodeName(): string
  get tagName(): string
  get children(): Array<Element>
  get outerHtml(): string
  get parentNode(): Element | Document | null
}
export class NodeList {
  get(index: number): Node
}
export class Text {
  get parentNode(): Element | Document | null
}
