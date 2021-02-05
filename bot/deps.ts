import { startBot as start } from "https://deno.land/x/discordeno/mod.ts";
import Markdown from "https://esm.sh/markdown-it";
import init, { compile, source } from "../wasm/wasm.js";

await init(source);
const md = new Markdown();

export { compile, md, start };
