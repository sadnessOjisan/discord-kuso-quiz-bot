require("dotenv").config();

const token = process.env.DISCORD_TOKEN;
const { Client, Intents } = require("discord.js");
const client = new Client({ intents: Object.keys(Intents.FLAGS) });

client.on("ready", () => {
  console.log(`${client.user.tag} でログインしています。`);
});

const quizSet = [
  { id: 1, title: "test?", answer: "fuga" },
  { id: 2, title: "test2?", answer: "fugafuga" },
];

const result = {};

let quizing = false;
let waitingAnswer = false;

let currentQuiz = undefined;

client.on("messageCreate", async (msg) => {
  console.info(`${msg.author.tag}がコマンドを送信しました: ${msg.content}`);
  if (msg.author.tag === "kuso-quiz#1299") return;
  if (msg.content === "quiz!") {
    msg.channel.send("quizをスタートします");
    quizing = true;
  }
  if (quizing) {
    if (waitingAnswer) {
      const userAnswer = msg.content;
      result[currentQuiz.id] = currentQuiz.answer === userAnswer;
      waitingAnswer = false;
      if (quizSet.length === 0) {
        msg.channel.send("quizを終了します");
        console.info(result);
        msg.channel.send(
          `${Object.values(result).length}問中${
            Object.values(result).filter((i) => i).length
          }問正解`
        );
        quizing = false;
        return;
      } else if (quizSet.length > 0) {
        currentQuiz = quizSet.pop();
        msg.channel.send(currentQuiz.title);
        waitingAnswer = true;
        return;
      }
    } else {
      if (quizSet.length === 0) {
        msg.channel.send("quizを終了します");
        quizing = false;
        return;
      } else if (quizSet.length > 0) {
        currentQuiz = quizSet.pop();
        msg.channel.send(currentQuiz.title);
        waitingAnswer = true;
        return;
      }
    }
  }
});

client.login(token);
