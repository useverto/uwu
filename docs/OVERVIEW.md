## Getting started

In short, `uwu` strictly analyzes ECMAScript code and produces diagnostics.

### Why

`uwu` is when you want to:

* Prevent malicious code execution.
* Scope arbitary code from its outer enviornment.

### Usage

`uwu` will be available as an NPM module to scan aribtary code on-the-fly.

```sh
npm i --save @useverto/uwu
```

```js
import { scan } from "@useverto/uwu";

// Unknown code.
let input = `window["local" + "Storage"].getItem("apiKey");`

// Scan, collect diagnostics.
let diagnostics = scan(input);
// If no diagnostics, evalutate code...
if(diagnostics.length < 0) {
  let execute = new Function(input);
  execute();
} 
// ...Otherwise show what's wrong.
else {
  console.error(diagnostics);
}

// [
//   {
//     kind: "ItemNotFound",
//     loc: [0, 6],
//     msg: "`window` not found in scope",
//   }
// ]
```

### Smartweave

`uwu` comes with out-of-the-box support for writing & verifying SmartWeave contracts. 
SmartWeave is a crucial component for most dApps on the [permaweb](https://arweave.org). It evaluates smartcontracts which can have certain side-effects.

To prevent possible malicious code to be executed on client engines, you can scan a contract source using `uwu`.

```js
// your_dApp/contract_handler.ts
import { scanTx } from "@useverto/uwu";

let diagnostics = scanTx(contractID);
```
