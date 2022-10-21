export type ChildNode = Comment | DocumentType | Element | Text;
export type ParentNode = Document | DocumentFragment | Element;
export class ClassList {
    item(index: number): string | null
    add(token: string): void
    remove(token: string): void
    toggle(token: string): boolean
    contains(token: string): boolean
    get length(): number
    get value(): string
    set value(value: string)
    [index: string]: string
}