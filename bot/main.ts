import { scan, md, start } from "./deps.ts";

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
      if (message.content.startsWith("!uwu eval")) {
        let tokens = md.parse(message.content);
        for (let node in tokens) {
          let t = tokens[node];
          if (t.tag == "code") {
            try {
              let result = scan(t.content);
              if(result === "true") {
                // Hmmm
                message.reply("```js\n" + JSON.stringify(eval(t.content)) + "```");
              } else {
                message.reply("```js\n" + result + "```");
              }
              
            } catch (e) {
              message.reply(`Compiler panicked\n \`\`\`${e}\`\`\``);
            }
          }
        }
      }
    },
  },
});
