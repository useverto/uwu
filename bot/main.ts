import { scan, md, start } from "./deps.ts";

const BOT_TOKEN = await Deno.readTextFile("bot/token.txt");

console.info("Starting Bot...");

const GLOBALS: Array<String> = ["Math", "parseInt", "parseFloat"];
const globalsInfo = GLOBALS.join('\n');

start({
  token: BOT_TOKEN,
  intents: ["GUILDS", "GUILD_MESSAGES"],
  eventHandlers: {
    ready() {
      console.log("Successfully connected to gateway");
    },
    messageCreate(message) {
      if(message.content.startsWith("!uwu scope")) {
          message.reply("The following GLOBALS were found: " + "```js\n" + globalsInfo + "```")
      }
      if (message.content.startsWith("!uwu eval")) {
        let tokens = md.parse(message.content);
        for (let node in tokens) {
          let t = tokens[node];
          if (t.tag == "code") {
            try {
              let result = scan(t.content, GLOBALS);
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
