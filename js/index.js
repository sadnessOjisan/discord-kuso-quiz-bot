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
let waitingAnswer = false;

client.on("messageCreate", async (msg) => {
  console.info(`${msg.author.tag}がコマンドを送信しました: ${msg.content}`);
  if (msg.author.tag === "kuso-quiz#1299") return;
  if (msg.content === "quiz!") {
    msg.channel.send("quizをスタートします");
    quizing = true;
  }
  if (quizing) {
    if (quized.length === 0 && !waitingAnswer) {
      msg.channel.send("quizを終了します");
      quizing = false;
      return;
    } else if (quized.length > 0) {
      msg.channel.send(quized.pop().title);
      waitingAnswer = true;
      return;
    }
  }
});

client.login(token);
