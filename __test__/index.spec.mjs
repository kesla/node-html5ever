import test from "ava";

import { parseDocument, QuirksMode } from "../index.js";

test("parseDocument works", (t) => {
  let dom = parseDocument("<html></html>");
  t.truthy(dom);
  t.snapshot(dom, "dom");
  t.snapshot(dom.serialize(), ".serialize()");
  t.snapshot(dom.document, ".document");
  t.is(dom.document.nodeName, "#document");
  t.is(dom.quirksMode, QuirksMode.Quirks, "Correct quirks mode");
  t.is(dom.document.docType, null, ".document.docType is not set");
});

test("doc type / Quirks mode", (t) => {
  let dom = parseDocument("<!DOCTYPE html><html></html>");
  t.truthy(dom);
  t.is(dom.quirksMode, QuirksMode.NoQuirks, "Correct quircks mode");
  t.truthy(dom.document.docType, ".document.docType is truthy");
  t.is(dom.document.docType.name, "html");
  t.is(dom.document.docType.publicId, "");
  t.is(dom.document.docType.systemId, "");
  t.snapshot(dom.serialize(), ".serialize()");

  let dom2 = parseDocument(`
    <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
  `);
  t.truthy(dom2.document.docType, ".document.docType is truthy");
  t.is(dom2.document.docType.name, "html");
  t.is(
    dom2.document.docType.publicId,
    "-//W3C//DTD HTML 4.01 Transitional//EN"
  );
  t.is(dom2.document.docType.systemId, "http://www.w3.org/TR/html4/loose.dtd");
});

test.skip(".document is initiated once", (t) => {
  let dom = parseDocument("");
  let document1 = dom.document;
  let document2 = dom.document;
  t.is(document1, document2);
});

test("element", (t) => {
  let dom = parseDocument(
    `<!DOCTYPE html>
    <html id="main">
      <body class="foo">Body content</body>
    </html>`
  );
  let document = dom.document;

  let {documentElement: html, body} = document;

  t.is(html.tagName, "HTML");
  t.is(html.getAttribute("id"), "main");
  t.is(body.tagName, "BODY");
  t.is(body.nodeName, "BODY");
  t.is(body.getAttribute("class"), "foo");

  t.snapshot(html.outerHtml);
  t.snapshot(body.outerHtml);
});
