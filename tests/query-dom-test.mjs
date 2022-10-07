// this is based on the test from https://raw.githubusercontent.com/micnews/query-dom/master/test.js

import tap, { skip } from "tap";

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
test.skip = skip;

const parseFragment = (input) => {
  return Html5EverDom.createDocumentFragment(input);
};

test("tagNames & nodeNames are upper case", (t) => {
  const fragment = parseFragment("<div>beep boop</div><DIV></DIV>hello, world");
  const actual = fragment.childNodes;

  t.strictSame(fragment.nodeName, "#document-fragment", "fragment.nodeName");
  t.strictSame(actual.length, 3);
  t.strictSame(actual[0].tagName, "DIV");
  t.strictSame(actual[0].nodeName, "DIV");
  t.strictSame(actual[1].tagName, "DIV");
  t.strictSame(actual[1].nodeName, "DIV");
});

/*
test("nested has correct tagNames & nodeNames", (t) => {
  const actual = parseFragment("<div><span></span></div>").childNodes;

  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "div");
  t.strictSame(actual[0].nodeName, "div");

  t.strictSame(actual[0].childNodes[0].tagName, "span");
  t.strictSame(actual[0].childNodes[0].nodeName, "span");
});

test("getAttribute()", (t) => {
  const actual = parseFragment('<div foo="bar"></div>').childNodes;
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
  const actual = parseFragment('<div foo="bar"></div>').childNodes[0]
    .attributes;
  const expected = [{ name: "foo", value: "bar" }];
  t.same(actual, expected);
});

test("hasAttribute()", (t) => {
  const actual = parseFragment('<div foo="bar"></div>').childNodes[0];
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
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].parentNode.tagName, "div");
  t.strictSame(actual[1].tagName, "foo");
  t.strictSame(actual[1].parentNode.tagName, "beep");
});

test("getElementsByTagName() tricky", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).getElementsByTagName("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].parentNode.tagName, "beep");
  t.strictSame(actual[1].tagName, "foo");
  t.strictSame(actual[1].parentNode.tagName, "div");
});

test("classList.contains()", (t) => {
  const actual = parseFragment(`
      <div></div>
      <div class="foo"></div>
      <div class="foo bar"></div>
    `).childNodes;

  t.same(actual.length, 3);

  t.falsy(actual[0].classList.contains("foo"));
  t.truthy(actual[1].classList.contains("foo"));
  t.falsy(actual[1].classList.contains("bar"));
  t.truthy(actual[2].classList.contains("foo"));
  t.truthy(actual[2].classList.contains("bar"));
  t.falsy(actual[2].classList.contains("bas"));
});

test("classList.contains() whitespace in className", (t) => {
  const actual = parseFragment('<div class="  foo   bar  "/>').childNodes[0]
    .classList;

  t.truthy(actual.contains("foo"));
  t.truthy(actual.contains("bar"));
  t.falsy(actual.contains(""));
  t.falsy(actual.contains(" "));
});

test("style - single statement", (t) => {
  const actual = parseFragment('<div style="font-size: 14px"></div>')
    .childNodes[0].style;
  const expected = {
    fontSize: "14px",
  };
  t.same(actual, expected);
});

test("style - multiple statements", (t) => {
  const actual = parseFragment(`
      <div style="
        -webkit-border-radius: 10px;
        -moz-border-radius: 10px;
        -ms-border-radius: 10px;
        border-radius: 10px;
        border-color: ;
        : red;
      "></div>
    `).childNodes[0].style;
  const expected = {
    WebkitBorderRadius: "10px",
    MozBorderRadius: "10px",
    msBorderRadius: "10px",
    borderRadius: "10px",
  };
  t.same(actual, expected);
});

test("style - no style", (t) => {
  const actual = parseFragment("<div></div>").childNodes[0].style;
  const expected = {};
  t.same(actual, expected);
});

test("style - invalid", (t) => {
  const actual = parseFragment('<div style="foo"></div>').childNodes[0].style;
  const expected = {};
  t.same(actual, expected);
});

test("text element", (t) => {
  const expectedNodeName = "#text";
  const expectedData = "beep boop";

  const actual = parseFragment("beep boop").childNodes[0];
  const actualNodeName = actual.nodeName;
  const actualData = actual.data;

  t.strictSame(actualNodeName, expectedNodeName);
  t.strictSame(actualData, expectedData);
});

test("parseFragment().querySelectorAll()", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].parentNode.tagName, "beep");
  t.strictSame(actual[1].tagName, "foo");
  t.strictSame(actual[1].parentNode.tagName, "div");
});

test("parse().querySelectorAll()", (t) => {
  const actual = parse(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].parentNode.tagName, "beep");
  t.strictSame(actual[1].tagName, "foo");
  t.strictSame(actual[1].parentNode.tagName, "div");
});

test("element.querySelectorAll()", (t) => {
  const actual = parseFragment(`<div>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).childNodes[0].querySelectorAll("foo");
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].parentNode.tagName, "beep");
  t.strictSame(actual[1].tagName, "foo");
  t.strictSame(actual[1].parentNode.tagName, "div");
});

test("parseFragment().querySelector()", (t) => {
  const actual = parseFragment(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelector("foo");
  t.strictSame(actual.tagName, "foo");
  t.strictSame(actual.parentNode.tagName, "beep");
});

test("parse().querySelector()", (t) => {
  const actual = parse(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).querySelector("foo");
  t.strictSame(actual.tagName, "foo");
  t.strictSame(actual.parentNode.tagName, "beep");
});

test("element().querySelector()", (t) => {
  const actual = parseFragment(`<div>
      <flipp><flopp></flopp></flipp>
      <beep><foo></foo></beep>
      <foo></foo>
    </div>`).childNodes[0].querySelector("foo");
  t.strictSame(actual.tagName, "foo");
  t.strictSame(actual.parentNode.tagName, "beep");
});

test("element().innerHTML", (t) => {
  const innerHTML =
    '<flipp hello="world"><flopp foo="bar">text</flopp></flipp>';
  const actual = parseFragment(`<div>${innerHTML}</div>`).childNodes[0]
    .innerHTML;
  const expected = innerHTML;
  t.strictSame(expected, actual);
});

test("element().outerHTML", (t) => {
  const outerHTML =
    '<div><flipp hello="world"><flopp foo="bar">text</flopp></flipp></div>';
  const actual = parseFragment(outerHTML).childNodes[0].outerHTML;
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
    `).childNodes[0].textContent;
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
  const expected = "\n    Foo\n      Bar\n    \n    Fred\n  ";
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
  t.strictSame(actual[0].tagName, "foo");
});

test("parse().querySelectorAll(.class)", (t) => {
  const actual = parse(`<div>
      <beep><foo class="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll(".bar");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "foo");
});

test("parse().querySelectorAll(tag > #id)", (t) => {
  const actual = parse(`<div>
      <beep><foo id="bar"></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("beep #bar");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "foo");
});

test("parse().querySelectorAll(tag + tag)", (t) => {
  const actual = parse(`<div>
      <beep><span></span><foo></foo></beep>
      <foo></foo>
    </div>`).querySelectorAll("span + foo");
  t.strictSame(actual.length, 1);
  t.strictSame(actual[0].tagName, "foo");
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
  t.strictSame(actual[0].tagName, "foo");
  t.strictSame(actual[0].getAttribute("data-name"), "foo-bar");
});

test("parse().querySelectorAll(:contains())", (t) => {
  const actual = parse(`<div>
      <beep>qux it</beep>
      <beep><bar></bar>it qux</beep>
      <foo>quuux</foo>
    </div>`).querySelectorAll('beep:contains("qux")');
  t.strictSame(actual.length, 2);
  t.strictSame(actual[0].textContent, "qux it");
  t.strictSame(actual[1].textContent, "it qux");
});
*/
