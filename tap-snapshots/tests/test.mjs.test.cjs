/* IMPORTANT
 * This snapshot file is auto-generated, but designed for humans.
 * It should be checked into source control and tracked carefully.
 * Re-generate by setting TAP_SNAPSHOT=1 and running tests.
 * Make sure to inspect the output below.  Do not ignore changes!
 */
'use strict'
exports[`tests/test.mjs TAP .append works w both strings and elements > body.outerHTML 1`] = `
<body>hello</body>
`

exports[`tests/test.mjs TAP .append works w both strings and elements > body.outerHTML 2`] = `
<body>hello<div></div></body>
`

exports[`tests/test.mjs TAP .prepend works w both strings and elements > body.outerHTML 1`] = `
<body>hello</body>
`

exports[`tests/test.mjs TAP .prepend works w both strings and elements > body.outerHTML 2`] = `
<body><div></div>hello</body>
`

exports[`tests/test.mjs TAP Element.id & Element.className > div.outerHTML 1`] = `
<div id="foo" class="bar baz"></div>
`

exports[`tests/test.mjs TAP Text node > Text node in body 1`] = `
<html><head></head><body>Hello, world</body></html>
`

exports[`tests/test.mjs TAP basic appendChild & remove > body.outerHTML 1`] = `
<body><div></div></body>
`

exports[`tests/test.mjs TAP basic appendChild & remove > body.outerHTML after remove 1`] = `
<body></body>
`

exports[`tests/test.mjs TAP basic appendChild & removeChild > body.outerHTML 1`] = `
<body><div></div></body>
`

exports[`tests/test.mjs TAP basic appendChild & removeChild > body.outerHTML after remove 1`] = `
<body></body>
`

exports[`tests/test.mjs TAP comment > Comment dom 1`] = `
<!-- Hello, world --><html><head></head><body></body></html>
`

exports[`tests/test.mjs TAP createElement + set attributes > attribute foo removed, hello added 1`] = `
<div hello="world"></div>
`

exports[`tests/test.mjs TAP createElement + set attributes > empty div 1`] = `
<div></div>
`

exports[`tests/test.mjs TAP createElement + set attributes > foo="bar" 1`] = `
<div foo="bar"></div>
`

exports[`tests/test.mjs TAP createElement + set attributes > foo="baz" 1`] = `
<div foo="baz"></div>
`

exports[`tests/test.mjs TAP doc type / Quirks mode > .serialize() 1`] = `
<!DOCTYPE html><html><head></head><body></body></html>
`

exports[`tests/test.mjs TAP element > body.innerHTML 1`] = `
<div>Body content</div>
    
`

exports[`tests/test.mjs TAP element > body.outerHTML 1`] = `
<body class="foo"><div>Body content</div>
    </body>
`

exports[`tests/test.mjs TAP element > html.innerHTML 1`] = `
<head></head><body class="foo"><div>Body content</div>
    </body>
`

exports[`tests/test.mjs TAP element > html.outerHTML 1`] = `
<html id="main"><head></head><body class="foo"><div>Body content</div>
    </body></html>
`

exports[`tests/test.mjs TAP parse works > .document 1`] = `
Document {}
`

exports[`tests/test.mjs TAP parse works > .serialize() 1`] = `
<html><head></head><body></body></html>
`

exports[`tests/test.mjs TAP parse works > dom 1`] = `
Html5EverDom {}
`
