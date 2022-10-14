import { Html5EverDom } from './index.js';

function whee() {
    let dom = new Html5EverDom(
        '<!DOCTYPE html><div id="foo">Hello, world!</div><div id="bar"></div>',
    );

    let { document } = dom;

    let foo = document.getElementById('foo');
    console.log(foo?.outerHTML)
    // foo?.firstChild?.remove();
    foo?.remove();

    let bar = document.getElementById('bar');
    bar?.remove();
}
whee();

// document.createElement('div').id = 'baz';
// console.log(dom.serialize());

global.gc();

setTimeout(() => { process.exit() }, 5000)