import tap from "tap";

import { Html5EverDom, QuirksMode, Element, Document, DocumentFragment, Comment, Text, DocumentType } from "../index.js";

/**
 *
 * @param {String} name
 * @param {(t:Tap.Test) => void} fn
 */
const test = (name, fn) => {
  tap.test(name, (t) => {
    fn(t);
    t.end();
  });
};

/**
 *
 * @param {String} name
 * @param {(t:Tap.Test) => void} fn
 */
test.only = (name, fn) => {
  tap.only(name, (t) => {
    fn(t);
    t.end();
  });
};

test("parse works", (t) => {
  let dom = new Html5EverDom("<html></html>");
  t.ok(dom);
  t.matchSnapshot(dom, "dom");
  t.matchSnapshot(dom.serialize(), ".serialize()");
  t.matchSnapshot(dom.document, ".document");
  t.strictSame(dom.document.nodeName, "#document");
  t.strictSame(dom.quirksMode, QuirksMode.Quirks, "Correct quirks mode");
  t.strictSame(dom.document.docType, null, ".document.docType is not set");

  t.strictSame(
    dom.document instanceof Document,
    true,
    ".document is a Document"
  );
});

test("doc type / Quirks mode", (t) => {
  let dom = new Html5EverDom("<!DOCTYPE html><html></html>");
  t.ok(dom);
  t.strictSame(dom.quirksMode, QuirksMode.NoQuirks, "Correct quircks mode");
  t.ok(dom.document.docType, ".document.docType is truthy");
  t.strictSame(dom.document.docType?.name, "html");
  t.strictSame(dom.document.docType?.publicId, "");
  t.strictSame(dom.document.docType?.systemId, "");
  t.matchSnapshot(dom.serialize(), ".serialize()");

  let dom2 = new Html5EverDom(`
    <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
  `);
  t.ok(dom2.document.docType, ".document.docType is truthy");
  t.strictSame(dom2.document.docType?.name, "html");
  t.strictSame(
    dom2.document.docType?.publicId,
    "-//W3C//DTD HTML 4.01 Transitional//EN"
  );
  t.strictSame(
    dom2.document.docType?.systemId,
    "http://www.w3.org/TR/html4/loose.dtd"
  );
});

test(".document is initiated once", (t) => {
  let dom = new Html5EverDom("");
  let document1 = dom.document;
  let document2 = dom.document;
  t.strictSame(document1, document2);
});

test("element", (t) => {
  let dom = new Html5EverDom(
    `<!DOCTYPE html>
    <html id="main">
      <body class="foo"><div>Body content</div></body>
    </html>`
  );
  let document = dom.document;

  let { documentElement: html, head, body } = document;
  let { documentElement: html2, head: head2, body: body2 } = document;

  t.strictSame(html.tagName, "HTML");
  t.strictSame(html.getAttribute("id"), "main");
  t.strictSame(body.tagName, "BODY");
  t.strictSame(body.nodeName, "BODY");
  t.strictSame(body.getAttribute("class"), "foo");

  t.strictSame(html instanceof Element, true);
  t.strictSame(body instanceof Element, true);

  t.matchSnapshot(html.outerHTML, "html.outerHTML");
  t.matchSnapshot(html.innerHTML, "html.innerHTML");
  t.matchSnapshot(body.outerHTML, "body.outerHTML");
  t.matchSnapshot(body.innerHTML, "body.innerHTML");

  t.strictSame(html, html2);
  t.strictSame(body, body2);
  t.strictSame(head, head2);

  t.strictSame(html.children[0], head);
  t.strictSame(html.children[1], body);

  t.strictSame(head.parentNode, html);
  t.strictSame(head.parentElement, head.parentElement);
  t.strictSame(head.parentElement, html);

  t.strictSame(body.parentNode, html);
  t.strictSame(body.parentElement, html);

  let div = body.children[0];
  t.strictSame(div.tagName, "DIV");
  t.strictSame(div.parentNode, body);
  t.strictSame(div.parentElement, body);

  t.strictSame(html.parentNode, document);
  t.strictSame(html.parentElement, null);
});

test("comment", (t) => {
  let dom = new Html5EverDom("<!-- Hello, world -->");

  t.matchSnapshot(dom.serialize(), "Comment dom");
});

test("createElement + set attributes", (t) => {
  let dom = new Html5EverDom("");

  let element = dom.document.createElement("div");

  t.matchSnapshot(element.outerHTML, "empty div");
  t.strictSame(element.parentElement, null);
  t.strictSame(element.parentNode, null);

  t.strictSame(element.hasAttribute("foo"), false);
  t.strictSame(element.getAttribute("foo"), null);

  element.setAttribute("foo", "bar");
  t.strictSame(element.hasAttribute("foo"), true);
  t.strictSame(element.getAttribute("foo"), "bar");
  t.matchSnapshot(element.outerHTML, 'foo="bar"');

  element.setAttribute("foo", "baz");
  t.strictSame(element.hasAttribute("foo"), true);
  t.strictSame(element.getAttribute("foo"), "baz");
  t.matchSnapshot(element.outerHTML, 'foo="baz"');

  element.setAttribute("hello", "world");

  element.removeAttribute("foo");
  t.strictSame(element.hasAttribute("foo"), false);
  t.strictSame(element.getAttribute("foo"), null);
  t.strictSame(element.getAttribute("hello"), "world");
  t.matchSnapshot(element.outerHTML, "attribute foo removed, hello added");
});

test("Text node", (t) => {
  let dom = new Html5EverDom("");
  let text = dom.document.createTextNode("Hello, world");
  t.strictSame(text.parentElement, null);
  t.strictSame(text.parentNode, null);
  t.strictSame(text.ownerDocument, null);

  let text2 = dom.document.body.appendChild(text);
  t.strictSame(text2, text, "text2 is text");

  t.matchSnapshot(dom.serialize(), "Text node in body");
  t.strictSame(text.parentElement, dom.document.body);
  t.strictSame(text.parentNode, dom.document.body);
  t.strictSame(text.ownerDocument, dom.document);
});

test("basic appendChild & remove", (t) => {
  let { document } = new Html5EverDom("");

  let body = document.body;
  let child = document.createElement("div");
  body.appendChild(child);

  t.matchSnapshot(body.outerHTML, "body.outerHTML");

  t.ok(child.parentElement, "child.parentElement is truthy");
  t.strictSame(child.parentElement, body, "child.parentElement is body");

  t.ok(child.parentNode, "child.parentNode is truthy");
  t.strictSame(child.parentNode, body, "child.parentNode is body");

  t.strictSame(body.children[0], child, "body.children[0] is child");

  child.remove();

  t.strictSame(child.parentElement, null, "child.parentElement is null");
  t.strictSame(child.parentNode, null, "child.parentNode is null");
  t.strictSame(body.children.length, 0, "body.children.length is 0");
  t.matchSnapshot(body.outerHTML, "body.outerHTML after remove");
});

test("basic appendChild & removeChild", (t) => {
  let { document } = new Html5EverDom("");

  let body = document.body;
  let child = document.createElement("div");
  body.appendChild(child);

  t.matchSnapshot(body.outerHTML, "body.outerHTML");

  t.ok(child.parentElement, "child.parentElement is truthy");
  t.strictSame(child.parentElement, body, "child.parentElement is body");

  t.ok(child.parentNode, "child.parentNode is truthy");
  t.strictSame(child.parentNode, body, "child.parentNode is body");

  t.strictSame(body.children[0], child, "body.children[0] is child");

  let child2 = body.removeChild(child);

  t.strictSame(child2, child, "child2 is child");
  t.strictSame(child.parentElement, null, "child.parentElement is null");
  t.strictSame(child.parentNode, null, "child.parentNode is null");
  t.strictSame(body.children.length, 0, "body.children.length is 0");
  t.matchSnapshot(body.outerHTML, "body.outerHTML after remove");
});

test("appendChild() remove element from previous parent", (t) => {
  let { document } = new Html5EverDom(`
    <div id="parent1">
      <div id="child1"></div>
      <div id="child2"></div>
    </div>
    <div id="parent2"></div>
  `);
  let parent1 = document.getElementById("parent1");
  let parent2 = document.getElementById("parent2");
  let child1 = document.getElementById("child1");
  let child2 = document.getElementById("child2");
  if (!parent1 || !parent2 || !child1 || !child2) {
    throw new Error("missing element");
  }

  parent2.appendChild(child1);

  t.strictSame(
    child1.parentElement,
    parent2,
    "child1.parentElement is parent2"
  );
  t.strictSame(parent1.children.length, 1, "parent1.children.length is 1");
  t.strictSame(parent2.children.length, 1, "parent2.children.length is 1");
  t.strictSame(
    child1.nextElementSibling,
    null,
    "child1.nextElementSibling is null"
  );
  t.strictSame(child1.previousElementSibling, null),
    "child1.previousElementSibling is null";
  t.strictSame(
    child2.nextElementSibling,
    null,
    "child2.nextElementSibling is null"
  );
  t.strictSame(
    child2.previousElementSibling,
    null,
    "child2.previousElementSibling is null"
  );
});

test('.append works w both strings and elements', (t) => {
  let { document } = new Html5EverDom("");
  let body = document.body;
  body.append("hello"/*, "world" */);
  t.ok(body.firstChild instanceof Text, "body.firstChild is a Text node");
  t.matchSnapshot(body.outerHTML, "body.outerHTML");
  const div = document.createElement("div");
  body.append(div);
  t.strictSame(div, body.lastChild, "div is body.lastChild");
  t.matchSnapshot(body.outerHTML, "body.outerHTML");
})

test('.prepend works w both strings and elements', (t) => {
  let { document } = new Html5EverDom("");
  let body = document.body;
  body.prepend("hello"/*, "world" */);
  t.ok(body.firstChild instanceof Text, "body.firstChild is a Text node");
  t.matchSnapshot(body.outerHTML, "body.outerHTML");
  const div = document.createElement("div");
  body.prepend(div);
  t.strictSame(div, body.firstChild, "div is body.firstChild");
  t.matchSnapshot(body.outerHTML, "body.outerHTML");
})

test("Element.id & Element.className", (t) => {
  let { document } = new Html5EverDom("");

  let div = document.createElement("div");
  t.strictSame(div.id, "");
  t.strictSame(div.className, "");

  div.id = "foo";
  div.className = "bar baz";

  t.matchSnapshot(div.outerHTML, "div.outerHTML");

  t.strictSame(div.id, "foo");
  t.strictSame(div.className, "bar baz");
});

test("Element.getElementById + Element.getElementsByClassName", (t) => {
  let { document } = new Html5EverDom(`
    <div id="foo">
      <div id="bar" class="baz">First</div>
    </div>
    <div class="baz">Second</div>
  `);

  let div = document.getElementById("foo");
  t.strictSame(div?.id, "foo");

  let bar = document.getElementById("bar");
  t.strictSame(bar?.id, "bar");

  let baz = document.getElementsByClassName("baz");
  t.strictSame(baz.length, 2);
  t.strictSame(baz[0].id, "bar");
  t.strictSame(baz[0].className, "baz");
  t.strictSame(baz[1].id, "");
  t.strictSame(baz[0].className, "baz");
});

test("previousSibling & nextSibling", (t) => {
  let { document } = new Html5EverDom(
    `
    <div id="foo"></div><div id="bar"></div>
    `.trim()
  );

  let foo = document.getElementById("foo");
  let bar = document.getElementById("bar");

  t.strictSame(foo?.previousSibling, null);
  t.strictSame(foo?.previousElementSibling, null);
  t.strictSame(foo?.nextSibling, bar);
  t.strictSame(foo?.nextElementSibling, bar);

  t.strictSame(bar?.previousSibling, foo);
  t.strictSame(bar?.previousElementSibling, foo);
  t.strictSame(bar?.nextSibling, null);
  t.strictSame(bar?.nextElementSibling, null);
});

test("Instance of", (t) => {
  let fragment = Html5EverDom.createDocumentFragment("text<div></div><!-- comment -->");

  t.ok(fragment instanceof DocumentFragment);
  t.ok(fragment.childNodes[0] instanceof Text);
  t.ok(fragment.childNodes[1] instanceof Element);
  t.ok(fragment.childNodes[2] instanceof Comment);

  let { document } = new Html5EverDom("<!DOCTYPE html>");
  t.ok(document instanceof Document);
  t.ok(document.body instanceof Element);
  t.ok(document.head instanceof Element);
  t.ok(document.documentElement instanceof Element);
  t.ok(document.docType instanceof DocumentType);
})

test(".firstChild & .lastChild", (t) => {
  let { document } = new Html5EverDom(
    `<div id="foo"></div>
    <div id="bar">First text<div id="hello"></div><div id="world"></div>End text</div>
    `.trim()
  );

  let foo = document.getElementById("foo");
  let bar = document.getElementById("bar");

  t.strictSame(foo?.firstChild, null);
  t.strictSame(foo?.lastChild, null);
  t.strictSame(foo?.firstElementChild, null);
  t.strictSame(foo?.lastElementChild, null);

  t.strictSame(bar?.firstChild, bar?.childNodes[0]);
  t.strictSame(bar?.lastChild, bar?.childNodes[bar.childNodes.length - 1]);
  t.strictSame(bar?.firstElementChild, bar?.children[0]);
  t.strictSame(bar?.lastElementChild, bar?.children[bar.children.length - 1]);
});

test('.removeChild errors if the node is not a child', (t) => {
  let { document } = new Html5EverDom(
    `<div id="foo"></div><div id="bar"></div>`
  );

  const foo = document.getElementById("foo");
  const bar = document.getElementById("bar");

  if (!foo || !bar) {
    throw new Error("missing element");
  }

  t.throws(() => {
    foo.removeChild(bar);
  });
})

test('basic querySelectorAll', (t) => {
  let { document } = new Html5EverDom(`
    <div id="foo">
      <div id="bar" class="baz">First</div>
    </div>
    <div class="baz">Second</div>
  `);

  let div = document.querySelectorAll("div");
  t.strictSame(div.length, 3);
  t.strictSame(div[0].id, "foo");
  t.strictSame(div[1].id, "bar");
  t.strictSame(div[2].id, "");
});

test('ClassList', (t) => {
  let { document } = new Html5EverDom(`
    <div id="foo" class="bar baz"></div>
    `);

  const div = document.getElementById("foo");

  if (!div) {
    throw new Error("missing element");
  }

  t.strictSame(div.classList.length, 2);
  t.ok(div.classList.contains("bar"));
  t.ok(div.classList.contains("baz"));
  t.notOk(div.classList.contains("qux"));
  t.strictSame(div.getAttribute("class"), "bar baz");
  t.strictSame(div.classList.value, "bar baz");
  t.strictSame(div.className, "bar baz");

  div.classList.add("qux");
  div.classList.add("qux");
  t.strictSame(div.classList.length, 3);
  t.ok(div.classList.contains("bar"));
  t.ok(div.classList.contains("baz"));
  t.ok(div.classList.contains("qux"));
  t.strictSame(div.getAttribute("class"), "bar baz qux");
  t.strictSame(div.classList.value, "bar baz qux");
  t.strictSame(div.className, "bar baz qux");

  div.classList.remove("qux");
  div.classList.remove("qux");
  t.strictSame(div.classList.length, 2);
  t.ok(div.classList.contains("bar"));
  t.ok(div.classList.contains("baz"));
  t.notOk(div.classList.contains("qux"));
  t.strictSame(div.getAttribute("class"), "bar baz");
  t.strictSame(div.classList.value, "bar baz");
  t.strictSame(div.className, "bar baz");

  t.ok(div.classList.toggle("qux"));
  t.strictSame(div.classList.length, 3);
  t.ok(div.classList.contains("bar"));
  t.ok(div.classList.contains("baz"));
  t.ok(div.classList.contains("qux"));
  t.strictSame(div.getAttribute("class"), "bar baz qux");
  t.strictSame(div.classList.value, "bar baz qux");
  t.strictSame(div.className, "bar baz qux");

  t.notOk(div.classList.toggle("qux"));
  t.strictSame(div.classList.length, 2);
  t.ok(div.classList.contains("bar"));
  t.ok(div.classList.contains("baz"));
  t.notOk(div.classList.contains("qux"));
  t.strictSame(div.getAttribute("class"), "bar baz");
  t.strictSame(div.classList.value, "bar baz");
  t.strictSame(div.className, "bar baz");

  div.classList.value = "hello world";
  t.strictSame(div.classList.length, 2);
  t.notOk(div.classList.contains("bar"));
  t.notOk(div.classList.contains("baz"));
  t.notOk(div.classList.contains("qux"));
  t.ok(div.classList.contains("hello"));
  t.ok(div.classList.contains("world"));
  t.strictSame(div.getAttribute("class"), "hello world");
  t.strictSame(div.classList.value, "hello world");
  t.strictSame(div.className, "hello world");

  t.throws(() => div.classList.add(""));
  t.throws(() => div.classList.remove(""));
  t.throws(() => div.classList.toggle(""));

  t.throws(() => div.classList.add(" "));
  t.throws(() => div.classList.remove(" "));
  t.throws(() => div.classList.toggle(" "));

  t.throws(() => div.classList.add("\t"));
  t.throws(() => div.classList.remove("\t"));
  t.throws(() => div.classList.toggle("\t"));

  t.throws(() => div.classList.add("\n"));
  t.throws(() => div.classList.remove("\n"));
  t.throws(() => div.classList.toggle("\n"));
});