// Convert --web generated WASM package to Deno compatible library.
import { encode } from "https://deno.land/std@0.76.0/encoding/base64.ts";
const encoder = new TextEncoder();

const wasm = await Deno.readFile(`pkg/uwu_wasm_bg.wasm`);
const encoded = encode(wasm);
const init = await Deno.readTextFile(`pkg/uwu_wasm.js`);
const source = `
 export const source = Uint8Array.from(atob("${encoded}"), c => c.charCodeAt(0));
                ${init}`;
await Deno.writeFile("wasm.js", encoder.encode(source));
