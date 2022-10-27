export type ChildNode = Comment | DocumentType | Element | Text;
export type ParentNode = Document | DocumentFragment | Element;
export type ClassList = import("./types/generated").ClassList & {
    [index: string]: string;
};
export type StyleDeclaration = import("./types/generated").StyleDeclaration & {
    [index: string]: string;
};
