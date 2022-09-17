/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum QuirksMode {
  Quirks = 0,
  LimitedQuirks = 1,
  NoQuirks = 2
}
export function parseDocument(html: string): Html5EverDom
export class Document {
  get docType(): DocType | null
  get nodeName(): string
}
export class DocType {
  name: string
  publicId: string
  systemId: string
}
export type Html5everDom = Html5EverDom
export class Html5EverDom {
  serialize(): string
  get quirksMode(): QuirksMode
  get document(): Document
}
