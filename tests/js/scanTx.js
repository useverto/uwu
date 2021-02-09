const { scanTx } = require("../..");
const arweave = require("arweave");

// requires Node v14.15.4 or above
const assert = require("assert").strict;

const client = new arweave({
    host: "arweave.net",
    port: 443,
    protocol: "https",
  });
async function runTests() {
    console.log(await scanTx(client, "usjm4PCxUd5mtaon7zc97-dt-3qf67yPyqgzLnLqk5A"));
}

runTests()