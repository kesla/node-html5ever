import test from "ava";
import { Html5EverDom } from "../index.js";

import { parse, QuirksMode, Element, Document } from "../index.js";

test("parse works", (t) => {
  let dom = parse("<html></html>");
  t.truthy(dom);
  t.snapshot(dom, "dom");
  t.snapshot(dom.serialize(), ".serialize()");
  t.snapshot(dom.document, ".document");
  t.is(dom.document.nodeName, "#document");
  t.is(dom.quirksMode, QuirksMode.Quirks, "Correct quirks mode");
  t.is(dom.document.docType, null, ".document.docType is not set");

  t.true(dom.document instanceof Document)
});

test("doc type / Quirks mode", (t) => {
  let dom = parse("<!DOCTYPE html><html></html>");
  t.truthy(dom);
  t.is(dom.quirksMode, QuirksMode.NoQuirks, "Correct quircks mode");
  t.truthy(dom.document.docType, ".document.docType is truthy");
  t.is(dom.document.docType?.name, "html");
  t.is(dom.document.docType?.publicId, "");
  t.is(dom.document.docType?.systemId, "");
  t.snapshot(dom.serialize(), ".serialize()");

  let dom2 = parse(`
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
  let dom = parse("");
  let document1 = dom.document;
  let document2 = dom.document;
  t.is(document1, document2);
});

test("element", (t) => {
  let dom = parse(
    `<!DOCTYPE html>
    <html id="main">
      <body class="foo">Body content</body>
    </html>`
  );
  let document = dom.document;

  let {documentElement: html, head, body} = document;
  let {documentElement: html2, head: head2, body: body2} = document;

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


  t.is(html, html2)
  t.is(body, body2)
  t.is(head, head2)

  t.is(html.children[0], head)
  t.is(html.children[1], body)

  t.is(head.parentNode, html)
  t.is(head.parentElement, head.parentElement)
  t.is(head.parentElement, html)

  t.is(body.parentNode, html)
  t.is(body.parentElement, html)

  t.is(html.parentNode, document)
  t.is(html.parentElement, null)
});
