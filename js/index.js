require("dotenv").config();

const token = process.env.DISCORD_TOKEN;
const { Client, Intents } = require("discord.js");
const client = new Client({ intents: Object.keys(Intents.FLAGS) });

client.on("ready", () => {
  console.log(`${client.user.tag} でログインしています。`);
});

client.on("messageCreate", async (msg) => {
  if (msg.content === "!ping") {
    msg.channel.send("Pong!");
  }
});

client.login(token);
