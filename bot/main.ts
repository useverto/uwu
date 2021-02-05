import { compile, md, start } from "./deps.ts";

const BOT_TOKEN = await Deno.readTextFile("bot/token.txt");

console.info("Starting Bot...");

start({
  token: BOT_TOKEN,
  intents: ["GUILDS", "GUILD_MESSAGES"],
  eventHandlers: {
    ready() {
      console.log("Successfully connected to gateway");
    },
    messageCreate(message) {
      if (message.content.startsWith("!uwu compile")) {
        let tokens = md.parse(message.content);
        for (let node in tokens) {
          let t = tokens[node];
          if (t.tag == "code") {
            try {
              const out = compile(t.content);
              message.reply("```js\n" + out + "```");
            } catch (e) {
              message.reply(`Compiler panicked\n \`\`\`${e}\`\`\``);
            }
          }
        }
      } else if (message.content.startsWith("!uwu eval")) {
        let tokens = md.parse(message.content);
        for (let node in tokens) {
          let t = tokens[node];
          if (t.tag == "code") {
            try {
              const out = compile(t.content);
              message.reply("```js\n" + JSON.stringify(eval(out)) + "```");
            } catch (e) {
              message.reply(`Compiler panicked\n \`\`\`${e}\`\`\``);
            }
          }
        }
      }
    },
  },
});
