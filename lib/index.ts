import Arweave from "arweave";
import Transaction from "arweave/node/lib/transaction";

/**
 * List of verified inbuilt JavaScript functions.
 * TODO: expand this list...
 */
export const GLOBALS: Scope = ["Math", "parseInt", "parseFloat"];

/**
 * Smartweave contract API.
 */
export const SmartWeaveGLOBALS: Scope = [...GLOBALS, "Smartweave"];

let _scan: (source: string) => string;

if(typeof window == "undefined") {
    _scan = require("../crates/uwu-wasm/pkg-node/uwu_wasm").scan;
} else {
    _scan = require("../crates/uwu-wasm/pkg/uwu_wasm").scan;
}

/**
 * Scope is a pre-defined set of items exposed to the script's scope.
 * eg: ['String', 'Math', ...]
 */
export type Scope = Array<string>;

// https://github.com/ArweaveTeam/SmartWeave/blob/78dd343228511161ae820cf6bd6661bf7fa6b6b3/src/utils.ts#L9
export function getTag(tx: Transaction, name: string) {
    const tags = tx.get('tags') as any;
  
    for (const tag of tags) {
      // decoding tags can throw on invalid utf8 data.
      try {
        if (tag.get('name', { decode: true, string: true }) === name) {
          return tag.get('value', { decode: true, string: true });
        }
      } catch (e) {}
    }
  
    return false;
}

export function scan(source: string, scope?: Scope) {
    scope = scope || [];
    // Pass in scope to the scanner. todo.
    return _scan(source);
}

export async function scanTx(arweave: Arweave, txID: string, scope?: Scope) {
    scope = scope || ["ContractAssert", "ContractThrow"];
    let contractTx = await arweave.transactions.get(txID);
    let contractSrc = getTag(contractTx, "Contract-Src");
    const contractSrcTX = await arweave.transactions.get(contractSrc);
    const source = contractSrcTX.get('data', { decode: true, string: true });
    return _scan(source as string);
}