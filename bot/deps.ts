import { startBot as start } from "https://deno.land/x/discordeno/mod.ts";
import Markdown from "https://esm.sh/markdown-it";
import init, { scan, source } from "../crates/uwu-wasm/wasm.js";

await init(source);
const md = new Markdown();

export { scan, md, start };
