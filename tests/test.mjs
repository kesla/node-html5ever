import tap from "tap";

import { Html5EverDom, QuirksMode, Element, Document } from "../index.js";

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

  t.matchSnapshot(html.outerHtml, "html.outerHtml");
  t.matchSnapshot(html.innerHtml, "html.innerHtml");
  t.matchSnapshot(body.outerHtml, "body.outerHtml");
  t.matchSnapshot(body.innerHtml, "body.innerHtml");

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

  t.matchSnapshot(element.outerHtml, "empty div");
  t.strictSame(element.parentElement, null);
  t.strictSame(element.parentNode, null);

  t.strictSame(element.hasAttribute("foo"), false);
  t.strictSame(element.getAttribute("foo"), null);

  element.setAttribute("foo", "bar");
  t.strictSame(element.hasAttribute("foo"), true);
  t.strictSame(element.getAttribute("foo"), "bar");
  t.matchSnapshot(element.outerHtml, 'foo="bar"');

  element.setAttribute("foo", "baz");
  t.strictSame(element.hasAttribute("foo"), true);
  t.strictSame(element.getAttribute("foo"), "baz");
  t.matchSnapshot(element.outerHtml, 'foo="baz"');

  element.setAttribute("hello", "world");

  element.removeAttribute("foo");
  t.strictSame(element.hasAttribute("foo"), false);
  t.strictSame(element.getAttribute("foo"), null);
  t.strictSame(element.getAttribute("hello"), "world");
  t.matchSnapshot(element.outerHtml, "attribute foo removed, hello added");
});

test("Text node", (t) => {
  let dom = new Html5EverDom("");
  let text = dom.document.createTextNode("Hello, world");
  t.strictSame(text.parentElement, null);
  t.strictSame(text.parentNode, null);
  t.strictSame(text.ownerDocument, null);

  dom.document.body.appendChild(text);
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

  t.matchSnapshot(body.outerHtml, "body.outerHtml");

  t.ok(child.parentElement, "child.parentElement is truthy");
  t.strictSame(child.parentElement, body, "child.parentElement is body");

  t.ok(child.parentNode, "child.parentNode is truthy");
  t.strictSame(child.parentNode, body, "child.parentNode is body");

  t.strictSame(body.children[0], child, "body.children[0] is child");

  child.remove();

  t.strictSame(child.parentElement, null, "child.parentElement is null");
  t.strictSame(child.parentNode, null, "child.parentNode is null");
  t.strictSame(body.children.length, 0, "body.children.length is 0");
  t.matchSnapshot(body.outerHtml, "body.outerHtml after remove");
});

test("basic appendChild & removeElement", (t) => {
  let { document } = new Html5EverDom("");

  let body = document.body;
  let child = document.createElement("div");
  body.appendChild(child);

  t.matchSnapshot(body.outerHtml, "body.outerHtml");

  t.ok(child.parentElement, "child.parentElement is truthy");
  t.strictSame(child.parentElement, body, "child.parentElement is body");

  t.ok(child.parentNode, "child.parentNode is truthy");
  t.strictSame(child.parentNode, body, "child.parentNode is body");

  t.strictSame(body.children[0], child, "body.children[0] is child");

  body.removeElement(child);

  t.strictSame(child.parentElement, null, "child.parentElement is null");
  t.strictSame(child.parentNode, null, "child.parentNode is null");
  t.strictSame(body.children.length, 0, "body.children.length is 0");
  t.matchSnapshot(body.outerHtml, "body.outerHtml after remove");
});

test("Element.id & Element.className", (t) => {
  let { document } = new Html5EverDom("");

  let div = document.createElement("div");
  t.strictSame(div.id, "");
  t.strictSame(div.className, "");

  div.id = "foo";
  div.className = "bar baz";

  t.matchSnapshot(div.outerHtml, "div.outerHtml");

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
  // t.strictSame(foo?.nextSibling, bar);

  t.strictSame(bar?.previousSibling, foo);
  // t.strictSame(bar?.nextSibling, null);
});
