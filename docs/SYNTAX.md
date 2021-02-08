## Syntax

`uwu` use [swc_ecma_parser](https://crates.io/crates/swc_ecma_parser) for parsing ECMAScript2019. It implements certain syntax restrictions on top of the AST.
Any code that passes those syntax restrictions is considered safe for execution.

> If you find an edge case that bypasses the scanner, please report them.

### Undefined refrences

The scope is at `"zero"` by default. Thus, nothing that's not in the scope of the script is accessible to it.

```javascript
window.navigator.getUserMedia(); // `window` not in current scope.
```

### Computed Call Expressions & IIFEs

```javascript
["dummy", eval][1](); // Call expressions can have computed callees.
// including IIFEs
(() => {e: eval})().e("window.location.href = 'https://letsstealyourmoney.com'");
```

