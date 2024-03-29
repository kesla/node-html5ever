// Taken from https://raw.githubusercontent.com/jsdom/jsdom/master/test/to-port-to-wpts/selectors.js

"use strict";
const { assert } = require("chai");
const { describe, specify } = require("tap").mocha;

const { Html5EverDom } = require("../..");

// require("chai").use(require("../chai-helpers.js"));

// These tests are mostly random regression tests, not systematic parsing tests. They are compiled from the bug tracker.
describe("jsdom/selectors", () => {
    specify("div:last-child > span[title] (GH-972)", () => {
        const { document } = new Html5EverDom(
            "<div><div><span title='title text'>text</span></div></div>",
        ).window;

        assert.doesNotThrow(() => {
            document.firstElementChild?.querySelector(
                "div:last-child > span[title]",
            );
        });
    });

    specify("div[title=''] (GH-1163)", () => {
        const { document } =
            new Html5EverDom(`<!doctype html><html><head></head><body>
      <div></div><div title=""></div><div title="yes"></div>
    </body></html>`).window;

        assert.strictEqual(
            document.querySelectorAll("div[title='']").length,
            1,
        );
        assert.strictEqual(
            document.querySelectorAll("div[title][title='']").length,
            1,
        );
    });

    specify("matches smoke test", () => {
        const html = `<html><body><div id="main"><p class="foo">Foo</p><p>Bar</p></div></body></html>`;
        const { document } = new Html5EverDom(html).window;
        const div = document.body.children[0];

        const element = document.querySelector("#main p");
        assert(element);
        assert.strictEqual(element.matches("#main p"), true, "p and first-p");
        assert.strictEqual(
            element.matches("#asdf"),
            false,
            "doesn't match wrong selector",
        );

        const element2 = div.querySelector("p");
        assert(element2);
        assert.strictEqual(element2.matches("p"), true, "p and first-p");
        assert.strictEqual(
            element2.matches("#asdf"),
            false,
            "doesn't match wrong selector",
        );

        const element3 = document.querySelector("#main p:not(.foo)");
        assert(element3);
        assert.strictEqual(
            element3.matches("#main p:not(.foo)"),
            true,
            "p and second-p",
        );
        assert.strictEqual(
            element3.matches("#asdf"),
            false,
            "doesn't match wrong selector",
        );
    });

    specify("querySelector smoke test", () => {
        const html = `<html><body><div id="main"><p class="foo">Foo</p><p>Bar</p></div></body></html>`;
        const { document } = new Html5EverDom(html).window;
        const div = document.body.children[0];

        const element = document.querySelector("#main p");
        assert.equal(element, div.children[0], "p and first-p");

        const element2 = div.querySelector("p");
        assert.equal(element2, div.children[0], "p and first-p");

        const element3 = document.querySelector("#main p:not(.foo)");
        assert.equal(element3, div.children[1], "p and second-p");

        const element4 = document.querySelector("#asdf");
        assert.strictEqual(element4, null, "nonexistent becomes null");
    });

    specify("querySelector smoke test on a document fragment", () => {
        const html = `<html><body><div id="main"><p class="foo">Foo</p><p>Bar</p></div></body></html>`;
        const { document } = new Html5EverDom(html).window;
        const div = document.body.children[0];
        const fragment = document.createDocumentFragment();

        fragment.appendChild(div);
        assert.strictEqual(document.body.firstChild, null);

        const element = fragment.querySelector("#main p");
        assert.strictEqual(element, div.children[0], "p and first-p");

        const element2 = fragment.querySelector("p");
        assert.strictEqual(element2, div.children[0], "p and first-p");

        const element3 = fragment.querySelector("#main p:not(.foo)");
        assert.strictEqual(element3, div.children[1], "p and second-p");

        const element4 = fragment.querySelector("#asdf");
        assert.strictEqual(element4, null, "nonexistent becomes null");
    });

    specify("querySelectorAll smoke test", () => {
        const { document } = new Html5EverDom(
            `<html><body><div id="main"><p>Foo</p><p>Bar</p></div><div id="next">` +
                `<div id="next-child"><p>Baz</p></div></div></body></html>`,
        ).window;
        const div = document.body.children[0];

        const elements = document.querySelectorAll("#main p");
        assert.equal(elements.length, 2, "two results");
        assert.equal(elements[0], div.children[0], "p and first-p");
        assert.equal(elements[1], div.children[1], "p and second-p");

        const elements2 = div.querySelectorAll("p");
        assert.equal(elements2.length, 2, "two results");
        assert.equal(elements2[0], div.children[0], "p and first-p");
        assert.equal(elements2[1], div.children[1], "p and second-p");

        assert.equal(
            div.querySelectorAll("#main").length,
            0,
            "It should not return the base element",
        );

        assert.equal(
            div.querySelectorAll("div").length,
            0,
            "There are no div elements under div#main",
        );

        const elements3 = div.querySelectorAll("#main p");
        assert.equal(elements3.length, 2, "two results");
        assert.equal(elements3[0], div.children[0], "p and first-p");
        assert.equal(elements3[1], div.children[1], "p and second-p");

        const topNode = document.createElement("p");
        const newNode = document.createElement("p");
        topNode.id = "fuz";
        newNode.id = "buz";
        topNode.appendChild(newNode);
        assert.equal(
            topNode.querySelectorAll("#fuz").length,
            0,
            "It should not return the base element that is orphaned",
        );

        const elements4 = topNode.querySelectorAll("#fuz #buz");
        assert.equal(elements4.length, 1, "one result");
        assert.equal(elements4[0], newNode, "newNode and first-p");

        const elements5 = div.querySelectorAll("p");
        assert.equal(
            elements5.length,
            2,
            "It should not return elements that are not within the base element's subtrees",
        );
        assert.equal(elements5[0], div.children[0], "p and first-p");
        assert.equal(elements5[1], div.children[1], "p and second-p");
        assert.strictEqual(
            topNode.parentNode,
            null,
            "topNode.parentNode is null",
        );

        const nextChildDiv = document.getElementById("next-child");
        assert(nextChildDiv);
        const elements6 = nextChildDiv.querySelectorAll("p");
        assert.equal(elements6.length, 1, "p under div#next-child");
        assert.equal(
            elements6[0],
            nextChildDiv.children[0],
            "child of div#next-child",
        );
    });

    specify("querySelectorAll smoke test on a document fragment", () => {
        const html =
            `<html><body><div id="main"><p>Foo</p><p>Bar</p></div>` +
            `<div id="next"><div id="next-child"><p>Baz</p></div></div></body></html>`;
        const { document } = new Html5EverDom(html).window;
        const fragment = document.createDocumentFragment();
        // @ts-ignore
        fragment.appendChild(document.body.firstChild);
        // @ts-ignore
        fragment.appendChild(document.body.firstChild);

        assert.strictEqual(
            document.body.firstChild,
            null,
            "The body should now be empty",
        );

        const div = fragment.firstElementChild;
        assert(div);

        const elements = fragment.querySelectorAll("#main p");
        assert.equal(elements.length, 2, "two results");
        assert.equal(elements[0], div.children[0], "p and first-p");
        assert.equal(elements[1], div.children[1], "p and second-p");

        const elements2 = div.querySelectorAll("p");
        assert.equal(elements2.length, 2, "two results");
        assert.equal(elements2[0], div.children[0], "p and first-p");
        assert.equal(elements2[1], div.children[1], "p and second-p");
        assert.equal(
            div.querySelectorAll("#main").length,
            0,
            "It should not return the base element",
        );
        assert.equal(
            div.querySelectorAll("div").length,
            0,
            "There are no div elements under div#main",
        );

        const elements3 = div.querySelectorAll("#main p");
        assert.equal(elements3.length, 2, "two results");
        assert.equal(elements3[0], div.children[0], "p and first-p");
        assert.equal(elements3[1], div.children[1], "p and second-p");

        const topNode = document.createElement("p");
        const newNode = document.createElement("p");
        topNode.id = "fuz";
        newNode.id = "buz";
        topNode.appendChild(newNode);
        assert.equal(
            topNode.querySelectorAll("#fuz").length,
            0,
            "It should not return the base element that is orphaned",
        );

        const elements4 = topNode.querySelectorAll("#fuz #buz");
        assert.equal(elements4.length, 1, "one result");
        assert.equal(elements4[0], newNode, "newNode and first-p");

        const elements5 = div.querySelectorAll("p");
        assert.equal(
            elements5.length,
            2,
            "It should not return elements that are not within the base element's subtrees",
        );
        assert.equal(elements5[0], div.children[0], "p and first-p");
        assert.equal(elements5[1], div.children[1], "p and second-p");
        assert.equal(topNode.parentNode, null, "topNode.parentNode is null");

        const nextChildDiv = fragment.querySelectorAll("#next-child")[0];
        assert.notStrictEqual(
            nextChildDiv,
            null,
            "id selector on fragment not null",
        );

        const elements6 = nextChildDiv.querySelectorAll("p");
        assert.equal(elements6.length, 1, "p under div#next-child");
        assert.equal(
            elements6[0],
            nextChildDiv.children[0],
            "child of div#next-child",
        );

        const elements7 = fragment.querySelectorAll("p");
        assert.equal(elements7.length, 3, "all p");
        assert.equal(elements7[0], div.children[0], "p and first-p");
        assert.equal(elements7[1], div.children[1], "p and second-p");
        assert.equal(
            elements7[2],
            nextChildDiv.children[0],
            "child of div#next-child",
        );
    });

    // specify("invalid selector //MAIN MENU... (GH-1214)", () => {
    //     const { document } = new Html5EverDom().window;

    //     const selector =
    //         " //MAIN MENU - (used to keep mobile menu options hidden and keep weather/search and menu " +
    //         "on one line) // #tncms-region-nav-main-nav-right-nav";

    //     assert.throwsDomException(
    //         () => document.querySelector(selector),
    //         document,
    //         "SyntaxError",
    //     );
    //     assert.throwsDomException(
    //         () => document.querySelectorAll(selector),
    //         document,
    //         "SyntaxError",
    //     );
    //     assert.throwsDomException(
    //         () => document.body.matches(selector),
    //         document,
    //         "SyntaxError",
    //     );
    // });
});
