import tap from "tap";

import {
    Html5EverDom,
    QuirksMode,
    Element,
    Document,
    DocumentFragment,
    Comment,
    Text,
    DocumentType,
} from "../index.js";

/**
 *
 * @param {String} name
 * @param {(t:Tap.Test) => void} fn
 */
const test = (name, fn) => {
    tap.test(name, (t) => {
        // @ts-ignore
        t.test = test;

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
    t.equal(dom.document.nodeName, "#document");
    t.equal(dom.quirksMode, QuirksMode.Quirks, "Correct quirks mode");
    t.equal(dom.document.docType, null, ".document.docType is not set");

    t.equal(dom.document instanceof Document, true, ".document is a Document");
});

test("doc type / Quirks mode", (t) => {
    let dom = new Html5EverDom("<!DOCTYPE html><html></html>");
    t.ok(dom);
    t.equal(dom.quirksMode, QuirksMode.NoQuirks, "Correct quircks mode");
    t.ok(dom.document.docType, ".document.docType is truthy");
    t.equal(dom.document.docType?.name, "html");
    t.equal(dom.document.docType?.publicId, "");
    t.equal(dom.document.docType?.systemId, "");
    t.matchSnapshot(dom.serialize(), ".serialize()");

    let dom2 = new Html5EverDom(`
    <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
  `);
    t.ok(dom2.document.docType, ".document.docType is truthy");
    t.equal(dom2.document.docType?.name, "html");
    t.equal(
        dom2.document.docType?.publicId,
        "-//W3C//DTD HTML 4.01 Transitional//EN",
    );
    t.equal(
        dom2.document.docType?.systemId,
        "http://www.w3.org/TR/html4/loose.dtd",
    );
});

test(".document is initiated once", (t) => {
    let dom = new Html5EverDom("");
    let document1 = dom.document;
    let document2 = dom.document;
    t.equal(document1, document2);
});

test("element", (t) => {
    let dom = new Html5EverDom(
        `<!DOCTYPE html>
    <html id="main">
      <body class="foo"><div>Body content</div></body>
    </html>`,
    );
    let document = dom.document;

    let { documentElement: html, head, body } = document;
    let { documentElement: html2, head: head2, body: body2 } = document;

    t.equal(html.tagName, "HTML");
    t.equal(html.getAttribute("id"), "main");
    t.equal(body.tagName, "BODY");
    t.equal(body.nodeName, "BODY");
    t.equal(body.getAttribute("class"), "foo");

    t.equal(html instanceof Element, true);
    t.equal(body instanceof Element, true);

    t.matchSnapshot(html.outerHTML, "html.outerHTML");
    t.matchSnapshot(html.innerHTML, "html.innerHTML");
    t.matchSnapshot(body.outerHTML, "body.outerHTML");
    t.matchSnapshot(body.innerHTML, "body.innerHTML");

    t.equal(html, html2);
    t.equal(body, body2);
    t.equal(head, head2);

    t.equal(html.children[0], head);
    t.equal(html.children[1], body);

    t.equal(head.parentNode, html);
    t.equal(head.parentElement, head.parentElement);
    t.equal(head.parentElement, html);

    t.equal(body.parentNode, html);
    t.equal(body.parentElement, html);

    let div = body.children[0];
    t.equal(div.tagName, "DIV");
    t.equal(div.parentNode, body);
    t.equal(div.parentElement, body);

    t.equal(html.parentNode, document);
    t.equal(html.parentElement, null);
});

test("createElement + set attributes", (t) => {
    let dom = new Html5EverDom("");

    let element = dom.document.createElement("div");

    t.matchSnapshot(element.outerHTML, "empty div");
    t.equal(element.parentElement, null);
    t.equal(element.parentNode, null);

    t.equal(element.hasAttribute("foo"), false);
    t.equal(element.getAttribute("foo"), null);

    element.setAttribute("foo", "bar");
    t.equal(element.hasAttribute("foo"), true);
    t.equal(element.getAttribute("foo"), "bar");
    t.matchSnapshot(element.outerHTML, 'foo="bar"');

    element.setAttribute("foo", "baz");
    t.equal(element.hasAttribute("foo"), true);
    t.equal(element.getAttribute("foo"), "baz");
    t.matchSnapshot(element.outerHTML, 'foo="baz"');

    element.setAttribute("hello", "world");

    element.removeAttribute("foo");
    t.equal(element.hasAttribute("foo"), false);
    t.equal(element.getAttribute("foo"), null);
    t.equal(element.getAttribute("hello"), "world");
    t.matchSnapshot(element.outerHTML, "attribute foo removed, hello added");
});

test("createElement + .innerHTML setter", (t) => {
    let dom = new Html5EverDom("");
    let element = dom.document.createElement("div");
    t.equal(element.innerHTML, "");

    element.innerHTML = "Hello world";
    t.equal(element.innerHTML, "Hello world");
    t.ok(element.firstChild instanceof Text);

    element.innerHTML = "<div id='foo'></div><div id='bar'></div>";
    t.equal(element.innerHTML, '<div id="foo"></div><div id="bar"></div>');
    t.equal(element.childNodes.length, 2);
    t.equal(element.children.length, 2);
    t.equal(element.children[0].getAttribute("id"), "foo");
    t.equal(element.children[1].getAttribute("id"), "bar");
    t.equal(element.children[0].parentNode, element);
});

test(".outerHTML setter", (t) => {
    let dom = new Html5EverDom(
        '<div id="wrapper">BEFORE<div id="foo"></div>AFTER</div>',
    );
    const wrapper = dom.document.getElementById("wrapper");
    const element = wrapper?.getElementById("foo");
    if (element == null || wrapper == null) {
        throw new Error("element is null");
    }

    t.equal(element.outerHTML, '<div id="foo"></div>');

    element.outerHTML = "<span id='bar'></span><!-- comment -->Hello, world!";

    t.equal(
        wrapper.innerHTML,
        'BEFORE<span id="bar"></span><!-- comment -->Hello, world!AFTER',
    );
});

test("comment", (t) => {
    let dom = new Html5EverDom("<!-- Hello, world -->");

    t.matchSnapshot(dom.serialize(), "Comment dom");
});

test("Text node", (t) => {
    let dom = new Html5EverDom("");
    let text = dom.document.createTextNode("Hello, world");
    t.equal(text.parentElement, null);
    t.equal(text.parentNode, null);
    t.equal(text.ownerDocument, null);

    let text2 = dom.document.body.appendChild(text);
    t.equal(text2, text, "text2 is text");

    t.matchSnapshot(dom.serialize(), "Text node in body");
    t.equal(text.parentElement, dom.document.body);
    t.equal(text.parentNode, dom.document.body);
    t.equal(text.ownerDocument, dom.document);
});

test("basic appendChild & remove", (t) => {
    let { document } = new Html5EverDom("");

    let body = document.body;
    let child = document.createElement("div");
    body.appendChild(child);

    t.matchSnapshot(body.outerHTML, "body.outerHTML");

    t.ok(child.parentElement, "child.parentElement is truthy");
    t.equal(child.parentElement, body, "child.parentElement is body");

    t.ok(child.parentNode, "child.parentNode is truthy");
    t.equal(child.parentNode, body, "child.parentNode is body");

    t.equal(body.children[0], child, "body.children[0] is child");

    child.remove();

    t.equal(child.parentElement, null, "child.parentElement is null");
    t.equal(child.parentNode, null, "child.parentNode is null");
    t.equal(body.children.length, 0, "body.children.length is 0");
    t.matchSnapshot(body.outerHTML, "body.outerHTML after remove");
});

test("basic appendChild & removeChild", (t) => {
    let { document } = new Html5EverDom("");

    let body = document.body;
    let child = document.createElement("div");
    body.appendChild(child);

    t.matchSnapshot(body.outerHTML, "body.outerHTML");

    t.ok(child.parentElement, "child.parentElement is truthy");
    t.equal(child.parentElement, body, "child.parentElement is body");

    t.ok(child.parentNode, "child.parentNode is truthy");
    t.equal(child.parentNode, body, "child.parentNode is body");

    t.equal(body.children[0], child, "body.children[0] is child");

    let child2 = body.removeChild(child);

    t.equal(child2, child, "child2 is child");
    t.equal(child.parentElement, null, "child.parentElement is null");
    t.equal(child.parentNode, null, "child.parentNode is null");
    t.equal(body.children.length, 0, "body.children.length is 0");
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

    t.equal(child1.parentElement, parent2, "child1.parentElement is parent2");
    t.equal(parent1.children.length, 1, "parent1.children.length is 1");
    t.equal(parent2.children.length, 1, "parent2.children.length is 1");
    t.equal(
        child1.nextElementSibling,
        null,
        "child1.nextElementSibling is null",
    );
    t.equal(child1.previousElementSibling, null),
        "child1.previousElementSibling is null";
    t.equal(
        child2.nextElementSibling,
        null,
        "child2.nextElementSibling is null",
    );
    t.equal(
        child2.previousElementSibling,
        null,
        "child2.previousElementSibling is null",
    );
});

test(".append works w both strings and elements", (t) => {
    let { document } = new Html5EverDom("");
    let body = document.body;
    body.append("hello" /*, "world" */);
    t.ok(body.firstChild instanceof Text, "body.firstChild is a Text node");
    t.matchSnapshot(body.outerHTML, "body.outerHTML");
    const div = document.createElement("div");
    body.append(div);
    t.equal(div, body.lastChild, "div is body.lastChild");
    t.matchSnapshot(body.outerHTML, "body.outerHTML");
});

test(".prepend works w both strings and elements", (t) => {
    let { document } = new Html5EverDom("");
    let body = document.body;
    body.prepend("hello" /*, "world" */);
    t.ok(body.firstChild instanceof Text, "body.firstChild is a Text node");
    t.matchSnapshot(body.outerHTML, "body.outerHTML");
    const div = document.createElement("div");
    body.prepend(div);
    t.equal(div, body.firstChild, "div is body.firstChild");
    t.matchSnapshot(body.outerHTML, "body.outerHTML");
});

test("Element.id & Element.className", (t) => {
    let { document } = new Html5EverDom("");

    let div = document.createElement("div");
    t.equal(div.id, "");
    t.equal(div.className, "");

    div.id = "foo";
    div.className = "bar baz";

    t.matchSnapshot(div.outerHTML, "div.outerHTML");

    t.equal(div.id, "foo");
    t.equal(div.className, "bar baz");
});

test("Element.getElementById + Element.getElementsByClassName", (t) => {
    let { document } = new Html5EverDom(`
    <div id="foo">
      <div id="bar" class="baz">First</div>
    </div>
    <div class="baz">Second</div>
  `);

    let div = document.getElementById("foo");
    t.equal(div?.id, "foo");

    let bar = document.getElementById("bar");
    t.equal(bar?.id, "bar");

    let baz = document.getElementsByClassName("baz");
    t.equal(baz.length, 2);
    t.equal(baz[0].id, "bar");
    t.equal(baz[0].className, "baz");
    t.equal(baz[1].id, "");
    t.equal(baz[0].className, "baz");
});

test("previousSibling & nextSibling", (t) => {
    let { document } = new Html5EverDom(
        `
    <div id="foo"></div><div id="bar"></div>
    `.trim(),
    );

    let foo = document.getElementById("foo");
    let bar = document.getElementById("bar");

    t.equal(foo?.previousSibling, null);
    t.equal(foo?.previousElementSibling, null);
    t.equal(foo?.nextSibling, bar);
    t.equal(foo?.nextElementSibling, bar);

    t.equal(bar?.previousSibling, foo);
    t.equal(bar?.previousElementSibling, foo);
    t.equal(bar?.nextSibling, null);
    t.equal(bar?.nextElementSibling, null);
});

test("Instance of", (t) => {
    let fragment = Html5EverDom.createDocumentFragment(
        "text<div></div><!-- comment -->",
    );

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
});

test(".firstChild & .lastChild", (t) => {
    let { document } = new Html5EverDom(
        `<div id="foo"></div>
    <div id="bar">First text<div id="hello"></div><div id="world"></div>End text</div>
    `.trim(),
    );

    let foo = document.getElementById("foo");
    let bar = document.getElementById("bar");

    t.equal(foo?.firstChild, null);
    t.equal(foo?.lastChild, null);
    t.equal(foo?.firstElementChild, null);
    t.equal(foo?.lastElementChild, null);

    t.equal(bar?.firstChild, bar?.childNodes[0]);
    t.equal(bar?.lastChild, bar?.childNodes[bar.childNodes.length - 1]);
    t.equal(bar?.firstElementChild, bar?.children[0]);
    t.equal(bar?.lastElementChild, bar?.children[bar.children.length - 1]);
});

test(".removeChild errors if the node is not a child", (t) => {
    let { document } = new Html5EverDom(
        `<div id="foo"></div><div id="bar"></div>`,
    );

    const foo = document.getElementById("foo");
    const bar = document.getElementById("bar");

    if (!foo || !bar) {
        throw new Error("missing element");
    }

    t.throws(() => {
        foo.removeChild(bar);
    });
});

test("basic querySelectorAll", (t) => {
    let { document } = new Html5EverDom(`
    <div id="foo">
      <div id="bar" class="baz">First</div>
    </div>
    <div class="baz">Second</div>
  `);

    let div = document.querySelectorAll("div");
    t.equal(div.length, 3);
    t.equal(div[0].id, "foo");
    t.equal(div[1].id, "bar");
    t.equal(div[2].id, "");
});

test("ClassList", (t) => {
    function createDiv() {
        let { document } = new Html5EverDom(`
    <div id="foo" class="bar baz"></div>
    `);

        const div = document.getElementById("foo");

        if (!div) {
            throw new Error("missing element");
        }
        return div;
    }

    t.test("initial state", (t) => {
        const div = createDiv();

        t.equal(div.classList.length, 2);
        t.ok(div.classList.contains("bar"));
        t.ok(div.classList.contains("baz"));
        t.notOk(div.classList.contains("qux"));
        t.equal(div.getAttribute("class"), "bar baz");
        t.equal(div.classList.value, "bar baz");
        t.equal(div.className, "bar baz");
        t.equal(div.classList.item(0), "bar");
        t.equal(div.classList[0], "bar");
        t.equal(div.classList.item(1), "baz");
        t.equal(div.classList[1], "baz");
        t.equal(div.classList.item(2), null);
        t.equal(div.classList[2], undefined);
        t.equal(div.classList.item(-1), null);
    });

    t.test("add", (t) => {
        const div = createDiv();

        div.classList.add("qux");
        div.classList.add("qux");
        t.equal(div.classList.length, 3);
        t.ok(div.classList.contains("bar"));
        t.ok(div.classList.contains("baz"));
        t.ok(div.classList.contains("qux"));
        t.equal(div.getAttribute("class"), "bar baz qux");
        t.equal(div.classList.value, "bar baz qux");
        t.equal(div.className, "bar baz qux");
        t.equal(div.classList.item(0), "bar");
        t.equal(div.classList[0], "bar");
        t.equal(div.classList.item(1), "baz");
        t.equal(div.classList[1], "baz");
        t.equal(div.classList.item(2), "qux");
        t.equal(div.classList[2], "qux");
    });

    t.test("remove", (t) => {
        const div = createDiv();

        div.classList.remove("baz");
        div.classList.remove("baz");
        t.equal(div.classList.length, 1);
        t.ok(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.equal(div.getAttribute("class"), "bar");
        t.equal(div.classList.value, "bar");
        t.equal(div.className, "bar");
        t.equal(div.classList.item(0), "bar");
        t.equal(div.classList[0], "bar");
        t.equal(div.classList.item(1), null);
        t.equal(div.classList[1], undefined);
    });

    t.test("toggle (add)", (t) => {
        const div = createDiv();

        t.ok(div.classList.toggle("qux"));
        t.equal(div.classList.length, 3);
        t.ok(div.classList.contains("bar"));
        t.ok(div.classList.contains("baz"));
        t.ok(div.classList.contains("qux"));
        t.equal(div.getAttribute("class"), "bar baz qux");
        t.equal(div.classList.value, "bar baz qux");
        t.equal(div.className, "bar baz qux");
        t.equal(div.classList.item(0), "bar");
        t.equal(div.classList[0], "bar");
        t.equal(div.classList.item(1), "baz");
        t.equal(div.classList[1], "baz");
        t.equal(div.classList.item(2), "qux");
        t.equal(div.classList[2], "qux");
    });

    t.test("toggle (remove)", (t) => {
        const div = createDiv();

        div.classList.toggle("baz");
        t.equal(div.classList.length, 1);
        t.ok(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.equal(div.getAttribute("class"), "bar");
        t.equal(div.classList.value, "bar");
        t.equal(div.className, "bar");
        t.equal(div.classList.item(0), "bar");
        t.equal(div.classList[0], "bar");
        t.equal(div.classList.item(1), null);
        t.equal(div.classList[1], undefined);
    });

    t.test("set .value", (t) => {
        const div = createDiv();

        div.classList.value = "hello world";
        t.equal(div.classList.length, 2);
        t.notOk(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.notOk(div.classList.contains("qux"));
        t.ok(div.classList.contains("hello"));
        t.ok(div.classList.contains("world"));
        t.equal(div.getAttribute("class"), "hello world");
        t.equal(div.classList.value, "hello world");
        t.equal(div.className, "hello world");
        t.equal(div.classList.item(0), "hello");
        t.equal(div.classList[0], "hello");
        t.equal(div.classList.item(1), "world");
        t.equal(div.classList[1], "world");
    });

    t.test("set .className", (t) => {
        const div = createDiv();

        div.className = "hello world";
        t.equal(div.classList.length, 2);
        t.notOk(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.notOk(div.classList.contains("qux"));
        t.ok(div.classList.contains("hello"));
        t.ok(div.classList.contains("world"));
        t.equal(div.getAttribute("class"), "hello world");
        t.equal(div.classList.value, "hello world");
        t.equal(div.className, "hello world");
        t.equal(div.classList.item(0), "hello");
        t.equal(div.classList[0], "hello");
        t.equal(div.classList.item(1), "world");
        t.equal(div.classList[1], "world");
    });

    t.test("set .className to empty string", (t) => {
        const div = createDiv();

        div.className = "";
        t.equal(div.classList.length, 0);
        t.notOk(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.equal(div.getAttribute("class"), "");
        t.equal(div.classList.value, "");
        t.equal(div.className, "");
        t.equal(div.classList.item(0), null);
        t.equal(div.classList[0], undefined);
    });

    t.test('removeAttribute("class")', (t) => {
        const div = createDiv();

        div.removeAttribute("class");
        t.equal(div.classList.length, 0);
        t.notOk(div.classList.contains("bar"));
        t.notOk(div.classList.contains("baz"));
        t.equal(div.getAttribute("class"), null);
        t.equal(div.classList.value, "");
        t.equal(div.className, "");
        t.equal(div.classList.item(0), null);
        t.equal(div.classList[0], undefined);
    });

    t.test(
        'removeAttribute("class") after .classList has been created',
        (t) => {
            const div = createDiv();

            let classList = div.classList;

            div.removeAttribute("class");
            t.equal(classList.length, 0);
            t.notOk(classList.contains("bar"));
            t.notOk(classList.contains("baz"));
            t.equal(div.getAttribute("class"), null);
            t.equal(classList.value, "");
            t.equal(div.className, "");
            t.equal(classList.item(0), null);
            t.equal(classList[0], undefined);
        },
    );

    t.test("throw on invalid input", (t) => {
        const div = createDiv();
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
});

test(".style", (t) => {
    function createDiv() {
        let { document } = new Html5EverDom(`
    <div id="foo" style="font-size: 14px; -webkit-text-stroke-width: 12px"></div>
    `);

        const div = document.getElementById("foo");

        if (!div) {
            throw new Error("missing element");
        }
        return div;
    }

    t.test("get", (t) => {
        const div = createDiv();

        t.equal(div.style.fontSize, "14px", "fontSize");
        t.equal(
            div.style.webkitTextStrokeWidth,
            "12px",
            "webkitTextStrokeWidth",
        );
        t.equal(
            div.style.cssText,
            "font-size: 14px; -webkit-text-stroke-width: 12px;",
            "cssText",
        );
        t.equal(
            div.getAttribute("style"),
            "font-size: 14px; -webkit-text-stroke-width: 12px",
            "getAttribute is unchanged from input",
        );
        t.equal(div.style.length, 2, "length");
        t.equal(div.style.item(0), "font-size");
        t.equal(div.style[0], "font-size");
        t.equal(div.style.item(1), "-webkit-text-stroke-width");
        t.equal(div.style[1], "-webkit-text-stroke-width");
        t.equal(div.style.item(2), null);
        t.equal(div.style[2], undefined);
        t.equal(
            div.style.getPropertyValue("font-size"),
            "14px",
            "getPropertyValue('font-size')",
        );
        t.equal(
            div.style.getPropertyValue("-webkit-text-stroke-width"),
            "12px",
            "getPropertyValue('-webkit-text-stroke-width')",
        );
    });

    t.test("weird input", (t) => {
        const div = createDiv();
        div.style.cssText = "font:; : red";

        t.equal(div.style.length, 0, "length");
        t.equal(div.style.item(0), null);
        t.equal(div.style[0], undefined);
        t.equal(div.style.cssText, "", "cssText");
        t.equal(div.getAttribute("style"), "", "getAttribute");
    });

    t.test("set", (t) => {
        const div = createDiv();
        div.style.fontSize = "12px";
        div.style.fontWeight = "400";
        div.style.webkitTextStrokeWidth = null;

        t.equal(div.style.fontSize, "12px", "fontSize");
        t.equal(div.style.fontWeight, "400", "fontWeight");
        t.equal(div.style.webkitTextStrokeWidth, "", "webkitTextStrokeWidth");
        t.equal(div.style.length, 2, "length");
        t.equal(div.style.item(0), "font-size");
        t.equal(div.style[0], "font-size");
        t.equal(div.style.item(1), "font-weight");
        t.equal(div.style[1], "font-weight");
        t.equal(div.style.item(2), null);
        t.equal(div.style[2], undefined);
        t.equal(div.style.cssText, "font-size: 12px; font-weight: 400;");
        t.equal(
            div.getAttribute("style"),
            "font-size: 12px; font-weight: 400;",
        );
    });

    t.test(".setAttribte", (t) => {
        const div = createDiv();
        div.setAttribute("style", "font-size: 9px; font-weight: 100");

        t.equal(div.style.fontSize, "9px", "fontSize");
        t.equal(div.style.fontWeight, "100", "fontWeight");
        t.equal(div.style.length, 2, "length");
        t.equal(div.style.item(0), "font-size");
        t.equal(div.style[0], "font-size");
        t.equal(div.style.item(1), "font-weight");
        t.equal(div.style[1], "font-weight");
        t.equal(div.style.item(2), null);
        t.equal(div.style[2], undefined);
        t.equal(div.style.cssText, "font-size: 9px; font-weight: 100;");

        div.setAttribute("style", "background-color: red");
        t.equal(div.style.backgroundColor, "red", "backgroundColor");
        t.equal(div.style.length, 1, "length");
        t.equal(div.style.item(0), "background-color");
        t.equal(div.style[0], "background-color");
        t.equal(div.style.item(1), null);
        t.equal(div.style[1], undefined);
        t.equal(div.style.cssText, "background-color: red;");
    });

    t.test(".removeAttribute", (t) => {
        const div = createDiv();

        div.removeAttribute("style");
        //  check when .style is not set
        t.equal(div.style.length, 0, "length");
        t.equal(div.style.item(0), null);
        t.equal(div.style[0], undefined);
        t.equal(div.style.cssText, "");
        t.equal(div.getAttribute("style"), null);

        // do this so we can remove the style attribute and check that it is removed
        div.setAttribute("style", "font-size: 9px; font-weight: 100");
        t.equal(div.style.length, 2, "length");

        // remove the style attribute
        div.removeAttribute("style");
        t.equal(div.style.length, 0, "length");
        t.equal(div.style.item(0), null);
        t.equal(div.style[0], undefined);
        t.equal(div.style.cssText, "");
        t.equal(div.getAttribute("style"), null);
    });
});

test(".insertBefore()", function (t) {
    let { document } = new Html5EverDom("");
    let referenceNode = document.body.appendChild(
        document.createElement("div"),
    );
    let newNode = document.body.insertBefore(
        document.createElement("span"),
        referenceNode,
    );
    t.equal(
        referenceNode.previousSibling,
        newNode,
        "referenceNode.previousSibling",
    );
    t.equal(newNode.nextSibling, referenceNode, "newNode.nextSibling");
    t.equal(document.body.firstChild, newNode);
    t.equal(document.body.lastChild, referenceNode);
});

test(".cloneNode()", function (t) {
    const createDiv = () => {
        let { document } = new Html5EverDom(
            '<div id="foo"><span></span>Hello, World<!-- beep boop --></div>',
        );
        const div = document.getElementById("foo");
        if (!div) {
            throw new Error("div not found");
        }
        return div;
    };

    t.test("default", (tt) => {
        let div = createDiv();

        let clone = div.cloneNode();

        tt.not(clone, div);
        tt.equal(clone.id, "foo");
        tt.equal(clone.childNodes.length, 0, "defaults to shallow clone");

        div.setAttribute("class", "bar");
        tt.equal(clone.getAttribute("class"), null, "does not copy attributes");

        clone.setAttribute("class", "baz");
        tt.equal(div.getAttribute("class"), "bar", "does not copy attributes");
    });

    t.test("shallow", (tt) => {
        let div = createDiv();

        let clone = div.cloneNode(false);

        tt.not(clone, div);
        tt.equal(clone.id, "foo");
        tt.equal(clone.childNodes.length, 0, "shallow clone has no children");

        div.setAttribute("class", "bar");
        tt.equal(clone.getAttribute("class"), null, "does not copy attributes");

        clone.setAttribute("class", "baz");
        tt.equal(div.getAttribute("class"), "bar", "does not copy attributes");
    });

    t.test("deep", (tt) => {
        let div = createDiv();

        let clone = div.cloneNode(true);

        tt.not(clone, div);
        tt.equal(clone.id, "foo");
        tt.equal(clone.childNodes.length, 3, "deep clone has children");

        tt.equal(clone.outerHTML, div.outerHTML, "deep clone is equal");

        tt.not(
            clone.childNodes[0],
            div.childNodes[0],
            "deep clone has new children",
        );
        tt.not(
            clone.childNodes[1],
            div.childNodes[1],
            "deep clone has new children",
        );
        tt.not(
            clone.childNodes[2],
            div.childNodes[2],
            "deep clone has new children",
        );

        div.setAttribute("class", "bar");
        tt.equal(clone.getAttribute("class"), null, "does not copy attributes");

        clone.setAttribute("class", "baz");
        tt.equal(div.getAttribute("class"), "bar", "does not copy attributes");
    });
});

test(".normalize() element", (t) => {
    let { document } = new Html5EverDom("<div><span></span>Hello, </div>");
    const div = document.body.firstElementChild;
    const span = div?.firstElementChild;
    const hello = div?.lastChild;
    if (!div || !span) {
        throw new Error("div or span not found");
    }

    if (!(hello instanceof Text)) {
        throw new Error("hello is not a Text node");
    }

    div.prepend("");
    div.append("World!");

    span.append("beep");
    span.append("");
    span.append("-");
    span.append("");
    span.append("boop");
    span.append("");

    div.normalize();

    t.equal(div.childNodes.length, 2, "div has 2 children");
    t.equal(div.childNodes[0], span, "div.firstChild is span");
    t.equal(hello, div.childNodes[1]);
    t.equal(hello.data, "Hello, World!");

    t.equal(span.childNodes.length, 1, "span has 1 child");
    const text = span.childNodes[0];

    t.equal(text instanceof Text && text.data, "beep-boop");
});

test(".normalize() document", (t) => {
    let { document } = new Html5EverDom("<div><span></span>Hello, </div>");
    const div = document.body.firstElementChild;
    const span = div?.firstElementChild;
    const hello = div?.lastChild;
    if (!div || !span) {
        throw new Error("div or span not found");
    }

    if (!(hello instanceof Text)) {
        throw new Error("hello is not a Text node");
    }

    div.prepend("");
    div.append("World!");

    span.append("beep");
    span.append("");
    span.append("-");
    span.append("");
    span.append("boop");
    span.append("");

    document.normalize();

    t.equal(div.childNodes.length, 2, "div has 2 children");
    t.equal(div.childNodes[0], span, "div.firstChild is span");
    t.equal(hello, div.childNodes[1]);
    t.equal(hello.data, "Hello, World!");

    t.equal(span.childNodes.length, 1, "span has 1 child");
    const text = span.childNodes[0];

    t.equal(text instanceof Text && text.data, "beep-boop");
});
