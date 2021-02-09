const { build } = require("esbuild");

build({
  entryPoints: ["./lib/index.ts"],
  outdir: "./dist",
  minify: false,
  bundle: false,
  format: "cjs"
}).catch(() => process.exit(1));