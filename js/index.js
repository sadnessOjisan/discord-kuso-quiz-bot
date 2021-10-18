require("dotenv").config();

const token = process.env.DISCORD_TOKEN;
const { Client, Intents } = require("discord.js");
const client = new Client({ intents: Object.keys(Intents.FLAGS) });

client.on("ready", () => {
  console.log(`${client.user.tag} でログインしています。`);
});

const quized = [
  { title: "test?", answer: "fuga" },
  { title: "test2?", answer: "fugafuga" },
];

let quizing = false;

client.on("messageCreate", async (msg) => {
  if (msg.content === "quiz!") {
    msg.channel.send("quizをスタートします");
    quizing = true;
  }
  if (quizing) {
    msg.channel.send(quized.pop().title);
    if (quized.length === 0) {
      msg.channel.send("quizを終了します");
      quizing = false;
    }
  }
});

client.login(token);
