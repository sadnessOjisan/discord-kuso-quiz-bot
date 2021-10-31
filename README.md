# discord-kuso-quiz-bot

just [kuso quiz](https://twitter.com/hashtag/kuso%E3%81%AA%E3%81%9E%E3%81%AA%E3%81%9E?src=hashtag_click&f=live) bot.

```sh
rustup toolchain install beta

cargo init --edition 2021
```

Run

```
cargo +beta run
```

and open discord.

## 質問

### tokio そのものってマルチスレッド + event loop の合わせ技ってこと？

### dereference と Copy の関係

ごまかそうとして dereference しようとすると Copy を要求される。String が混じっているとできない。

### :: の意味がよく分からない

module を辿っているのはわかるがジェネリクスの指定もそうなっていて、そういうものか？と言う気持ち

- ` .type_map_insert::<BotState>(Arc::new(Mutex::new(initial_state)))`
- ` let (tx, mut rx) = mpsc::channel::<String>(32);`

## memo

-
