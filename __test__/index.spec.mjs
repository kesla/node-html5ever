import test from "ava";

import { Html5EverDom, QuirksMode, Element, Document } from "../index.js";

test("parse works", (t) => {
  let dom = new Html5EverDom("<html></html>");
  t.truthy(dom);
  t.snapshot(dom, "dom");
  t.snapshot(dom.serialize(), ".serialize()");
  t.snapshot(dom.document, ".document");
  t.is(dom.document.nodeName, "#document");
  t.is(dom.quirksMode, QuirksMode.Quirks, "Correct quirks mode");
  t.is(dom.document.docType, null, ".document.docType is not set");

  t.true(dom.document instanceof Document);
});

test("doc type / Quirks mode", (t) => {
  let dom = new Html5EverDom("<!DOCTYPE html><html></html>");
  t.truthy(dom);
  t.is(dom.quirksMode, QuirksMode.NoQuirks, "Correct quircks mode");
  t.truthy(dom.document.docType, ".document.docType is truthy");
  t.is(dom.document.docType?.name, "html");
  t.is(dom.document.docType?.publicId, "");
  t.is(dom.document.docType?.systemId, "");
  t.snapshot(dom.serialize(), ".serialize()");

  let dom2 = new Html5EverDom(`
    <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
  `);
  t.truthy(dom2.document.docType, ".document.docType is truthy");
  t.is(dom2.document.docType?.name, "html");
  t.is(
    dom2.document.docType?.publicId,
    "-//W3C//DTD HTML 4.01 Transitional//EN"
  );
  t.is(dom2.document.docType?.systemId, "http://www.w3.org/TR/html4/loose.dtd");
});

test(".document is initiated once", (t) => {
  let dom = new Html5EverDom("");
  let document1 = dom.document;
  let document2 = dom.document;
  t.is(document1, document2);
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

  t.is(html.tagName, "HTML");
  t.is(html.getAttribute("id"), "main");
  t.is(body.tagName, "BODY");
  t.is(body.nodeName, "BODY");
  t.is(body.getAttribute("class"), "foo");

  t.true(html instanceof Element);
  t.true(body instanceof Element);

  t.snapshot(html.outerHtml, "html.outerHtml");
  t.snapshot(html.innerHtml, "html.innerHtml");
  t.snapshot(body.outerHtml, "body.outerHtml");
  t.snapshot(body.innerHtml, "body.innerHtml");

  t.is(html, html2);
  t.is(body, body2);
  t.is(head, head2);

  t.is(html.children[0], head);
  t.is(html.children[1], body);

  t.is(head.parentNode, html);
  t.is(head.parentElement, head.parentElement);
  t.is(head.parentElement, html);

  t.is(body.parentNode, html);
  t.is(body.parentElement, html);

  let div = body.children[0];
  t.is(div.tagName, "DIV");
  t.is(div.parentNode, body);
  t.is(div.parentElement, body);

  t.is(html.parentNode, document);
  t.is(html.parentElement, null);
});

test("comment", (t) => {
  let dom = new Html5EverDom("<!-- Hello, world -->");

  t.snapshot(dom.serialize(), "Comment dom");
});

test("createElement + set attributes", (t) => {
  let dom = new Html5EverDom("");

  let element = dom.document.createElement("div");

  t.snapshot(element.outerHtml, "empty div");
  t.is(element.parentElement, null);
  t.is(element.parentNode, null);

  t.false(element.hasAttribute("foo"));
  t.is(element.getAttribute("foo"), null);

  element.setAttribute("foo", "bar");
  t.true(element.hasAttribute("foo"));
  t.is(element.getAttribute("foo"), "bar");
  t.snapshot(element.outerHtml, 'foo="bar"');

  element.setAttribute("foo", "baz");
  t.true(element.hasAttribute("foo"));
  t.is(element.getAttribute("foo"), "baz");
  t.snapshot(element.outerHtml, 'foo="baz"');

  element.setAttribute("hello", "world");

  element.removeAttribute("foo");
  t.false(element.hasAttribute("foo"));
  t.is(element.getAttribute("foo"), null);
  t.is(element.getAttribute("hello"), "world");
  t.snapshot(element.outerHtml, "attribute foo removed, hello added");
});

test("basic appendElement & remove", (t) => {
  let { document } = new Html5EverDom("");

  let body = document.body;
  let child = document.createElement("div");
  body.appendElement(child);

  t.snapshot(body.outerHtml, "body.outerHtml");

  t.truthy(child.parentElement, "child.parentElement is truthy");
  t.is(child.parentElement, body, "child.parentElement is body");

  t.truthy(child.parentNode, "child.parentNode is truthy");
  t.is(child.parentNode, body, "child.parentNode is body");

  t.is(body.children[0], child, "body.children[0] is child");

  child.remove();

  t.is(child.parentElement, null, "child.parentElement is null");
  t.is(child.parentNode, null, "child.parentNode is null");
  t.is(body.children.length, 0, "body.children.length is 0");
  t.snapshot(body.outerHtml, "body.outerHtml after remove");
});

test("basic appendElement & removeElement", (t) => {
  let { document } = new Html5EverDom("");

  let body = document.body;
  let child = document.createElement("div");
  body.appendElement(child);

  t.snapshot(body.outerHtml, "body.outerHtml");

  t.truthy(child.parentElement, "child.parentElement is truthy");
  t.is(child.parentElement, body, "child.parentElement is body");

  t.truthy(child.parentNode, "child.parentNode is truthy");
  t.is(child.parentNode, body, "child.parentNode is body");

  t.is(body.children[0], child, "body.children[0] is child");

  body.removeElement(child);

  t.is(child.parentElement, null, "child.parentElement is null");
  t.is(child.parentNode, null, "child.parentNode is null");
  t.is(body.children.length, 0, "body.children.length is 0");
  t.snapshot(body.outerHtml, "body.outerHtml after remove");
});
