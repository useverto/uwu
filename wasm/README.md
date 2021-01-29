The `uwu` compiler compiled to WASM for Node.js and Browsers.

## Building

Install `wasm-pack`

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
and use the build script...
```shell
./build.sh
```

## Usage

```typescript
import { compile } from "uwu_wasm";
let js = compile("1 + 1");
```

