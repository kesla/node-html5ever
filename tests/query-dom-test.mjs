// this is based on the test from https://raw.githubusercontent.com/micnews/query-dom/master/test.js

import tap, { skip } from "tap";
import { DocumentFragment, Text, Html5EverDom, QuirksMode, Element, Document } from "../index.js";

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
test.skip = skip;

/**
 * 
 * @param {String} input 
 * @returns {DocumentFragment}
 */
function parseFragment(input) {
  return Html5EverDom.createDocumentFragment(input);
}

/**
 * 
 * @param {String} input 
 * @returns {Document}
 */
function parse(input) {
  return new Html5EverDom(input).document;
}

test("tagNames & nodeNames are upper case", (t) => {
  const fragment = parseFragment("<div>beep boop</div><DIV></DIV>hello, world");
  const actual = fragment.childNodes;

  t.strictSame(fragment.nodeName, "#document-fragment", "fragment.nodeName");
  t.strictSame(actual.length, 3);
  t.strictSame(actual[0] instanceof Element && actual[0].tagName, "DIV");
  t.strictSame(actual[0].nodeName, "DIV");
  t.strictSame(actual[1] instanceof Element && actual[1].tagName, "DIV");
  t.strictSame(actual[1].nodeName, "DIV");
});

test("nested has correct tagNames & nodeNames", (t) => {
  const actual = parseFragment("<div><span></span></div>").children;

  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "DIV");
  t.strictSame(actual[0].nodeName, "DIV");

  t.strictSame(actual[0].children[0].tagName, "SPAN");
  t.strictSame(actual[0].children[0].nodeName, "SPAN");
});

test("getAttribute()", (t) => {
  const actual = parseFragment('<div foo="bar"></div>').children;
  t.strictSame(
    actual[0].getAttribute("does-not-exists"),
    null,
    "none existing attribute"
  );
  t.strictSame(
    actual[0].getAttribute("does-not-exists"),
    null,
    "none existing attribute (cached)"
  );
  t.strictSame(actual[0].getAttribute("foo"), "bar", "existing attribute");
  t.strictSame(
    actual[0].getAttribute("foo"),
    "bar",
    "existing attribute (cached)"
  );
});

test("attributes", (t) => {
  const actual = parseFragment('<div foo="bar"></div>').children[0]
    .attributes;

  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].name, "foo");
  t.strictSame(actual[0].value, "bar");
});

test("hasAttribute()", (t) => {
  const actual = parseFragment('<div foo="bar"></div>').children[0];
  t.strictSame(
    actual.hasAttribute("does-not-exists"),
    false,
    "none existing attribute"
  );
  t.strictSame(
    actual.hasAttribute("does-not-exists"),
    false,
    "none existing attribute (cached)"
  );
  t.strictSame(actual.hasAttribute("foo"), true, "existing attribute");
  t.strictSame(actual.hasAttribute("foo"), true, "existing attribute (cached)");
});

test("getElementsByTagName()", (t) => {
  const fragment = parseFragment(`<div>
      <foo></foo>
      <beep><foo></foo></beep>
    </div>`);
  const actual = fragment.getElementsByTagName("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].parentElement?.tagName, "DIV");
  t.strictSame(actual[1].tagName, "FOO");
  t.strictSame(actual[1].parentElement?.tagName, "BEEP");
});

test("getElementsByTagName() tricky", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).getElementsByTagName("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].parentElement?.tagName, "BEEP");
  t.strictSame(actual[1].tagName, "FOO");
  t.strictSame(actual[1].parentElement?.tagName, "DIV");
});

test("classList.contains()", (t) => {
  const actual = parseFragment(`
      <div></div>
      <div class="foo"></div>
      <div class="foo bar"></div>
    `).children;

  t.same(actual.length, 3);

  t.notOk(actual[0].classList.contains("foo"));
  t.ok(actual[1].classList.contains("foo"));
  t.notOk(actual[1].classList.contains("bar"));
  t.ok(actual[2].classList.contains("foo"));
  t.ok(actual[2].classList.contains("bar"));
  t.notOk(actual[2].classList.contains("bas"));
});

test("classList.contains() whitespace in className", (t) => {
  const actual = parseFragment('<div class="  foo   bar  "/>').children[0]
    .classList;

  t.ok(actual.contains("foo"));
  t.ok(actual.contains("bar"));
  t.notOk(actual.contains(""));
  t.notOk(actual.contains(" "));
});

test("style - single statement", (t) => {
  const actual = parseFragment('<div style="font-size: 14px"></div>')
    .children[0].style;
  const expected = {
    fontSize: "14px",
  };
  t.same(actual, expected);
  t.strictSame(actual.fontSize, "14px");
});

test("style - multiple statements", (t) => {
  const actual = parseFragment(`
      <div style="
        -webkit-border-radius: 10px;
        border-radius: 10px;
        border-color: ;
        font: 14px/1.5 Helvetica, Arial, sans-serif;
        : red;
      "></div>
    `).children[0].style;
  const expected = {
    webkitBorderRadius: "10px",
    borderRadius: "10px",
    font: "14px/1.5 Helvetica, Arial, sans-serif",
  };
  t.same(actual, expected);
  t.strictSame(actual.borderRadius, "10px");
  t.strictSame(actual.webkitBorderRadius, "10px");
  t.strictSame(actual.font, "14px/1.5 Helvetica, Arial, sans-serif");
});

test("style - no style", (t) => {
  const actual = parseFragment("<div></div>").children[0].style;
  const expected = {};
  t.same(actual, expected);
});

test("style - invalid", (t) => {
  const actual = parseFragment('<div style="foo"></div>').children[0].style;
  const expected = {};
  t.same(actual, expected);
});

test("text element", (t) => {
  const expectedNodeName = "#text";
  const expectedData = "beep boop";

  const actual = parseFragment("beep boop").childNodes[0];
  const actualNodeName = actual.nodeName;
  const actualData = actual instanceof Text && actual.data;

  t.strictSame(actualNodeName, expectedNodeName);
  t.strictSame(actualData, expectedData);
});

test("parseFragment().querySelectorAll()", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].parentElement?.tagName, "BEEP");
  t.strictSame(actual[1].tagName, "FOO");
  t.strictSame(actual[1].parentElement?.tagName, "DIV");
});

test("parse().querySelectorAll()", (t) => {
  const actual = parse(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].parentElement?.tagName, "BEEP");
  t.strictSame(actual[1].tagName, "FOO");
  t.strictSame(actual[1].parentElement?.tagName, "DIV");
});

test("element.querySelectorAll()", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).children[0].querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].parentElement?.tagName, "BEEP");
  t.strictSame(actual[1].tagName, "FOO");
  t.strictSame(actual[1].parentElement?.tagName, "DIV");
});

test("parseFragment().querySelector()", (t) => {
  const actual = parseFragment(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelector("foo");
  t.strictSame(actual?.tagName, "FOO");
  t.strictSame(actual?.parentElement?.tagName, "BEEP");
});

test("parse().querySelector()", (t) => {
  const actual = parse(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelector("foo");
  t.strictSame(actual?.tagName, "FOO");
  t.strictSame(actual?.parentElement?.tagName, "BEEP");
});

test("element().querySelector()", (t) => {
  const actual = parseFragment(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).children[0].querySelector("foo");
  t.strictSame(actual?.tagName, "FOO");
  t.strictSame(actual?.parentElement?.tagName, "BEEP");
});

test("element().innerHTML", (t) => {
  const innerHTML =
    '<flipp hello="world"><flopp foo="bar">text</flopp></flipp>';
  const actual = parseFragment(`<div>${innerHTML}</div>`).children[0]
    .innerHTML;
  const expected = innerHTML;
  t.strictSame(expected, actual);
});

test("element().outerHTML", (t) => {
  const outerHTML =
    '<div><flipp hello="world"><flopp foo="bar">text</flopp></flipp></div>';
  const actual = parseFragment(outerHTML).children[0].outerHTML;
  const expected = outerHTML;
  t.strictSame(expected, actual);
});

test("#text.textContent", (t) => {
  const actual = parseFragment("beep beep").childNodes[0].textContent;
  const expected = "beep beep";
  t.strictSame(actual, expected);
});

test("element().textContent", (t) => {
  const actual = parseFragment(`
      <div><flipp>Foo <flopp>Bar</flopp></flipp>Fred</div>
    `).children[0].textContent;
  const expected = "Foo BarFred";
  t.strictSame(actual, expected);
});

test("element().textContent preserves whitespace", (t) => {
  const actual = parseFragment(`<div>
      <flipp>Foo
        <flopp>Bar</flopp>
      </flipp>
      Fred
    </div>`).childNodes[0].textContent;
  const expected = "\n      Foo\n        Bar\n      \n      Fred\n    ";
  t.strictSame(actual, expected);
});

test("document().textContent is null", (t) => {
  const actual = parse(`<!DOCTYPE html5>
      <p>Hello</p>
    </div>`).textContent;
  t.strictSame(actual, null);
});

test("documentFragment().textContent is null", (t) => {
  const actual = parseFragment(`<p>Hello</p>
    </div>`).textContent;
  t.strictSame(actual, null);
});

test("parse().querySelectorAll(#id)", (t) => {
  const actual = parse(`<div>
      <beep><foo id="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("#bar");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "FOO");
});

test("parse().querySelectorAll(.class)", (t) => {
  const actual = parse(`<div>
      <beep><foo class="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll(".bar");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "FOO");
});

test("parse().querySelectorAll(tag > #id)", (t) => {
  const actual = parse(`<div>
      <beep><foo id="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("beep #bar");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "FOO");
});

test("parse().querySelectorAll(tag + tag)", (t) => {
  const actual = parse(`<div>
      <beep><span></span><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("span + foo");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "FOO");
});

test("parse().querySelectorAll([attr=value])", (t) => {
  const actual = parse(`<div>
      <beep><foo data-name="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll('[data-name="bar"]');
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].getAttribute("data-name"), "bar");
});

test("parse().querySelectorAll([attr^=value])", (t) => {
  const actual = parse(`<div>
      <beep><foo data-name="foo-bar"></foo></beep>
      <beep><bar data-name="bar-foo"></bar></beep>
      <foo></foo>
    </div>`).querySelectorAll('[data-name^="foo"]');
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "FOO");
  t.strictSame(actual[0].getAttribute("data-name"), "foo-bar");
});

// - not proper css
// test("parse().querySelectorAll(:contains())", (t) => {
//   const actual = parse(`<div>
//       <beep>qux it</beep>
//       <beep><bar></bar>it qux</beep>
//       <foo>quuux</foo>
//     </div>`).querySelectorAll('beep:contains("qux")');
//   t.strictSame(actual.length, 2);
//   t.strictSame(actual[0].textContent, "qux it");
//   t.strictSame(actual[1].textContent, "it qux");
// });
