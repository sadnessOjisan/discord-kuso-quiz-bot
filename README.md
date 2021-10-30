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

### チャンネルの数だけ spawn したい

各チャンネルが、各タスクで動くイベントループとやりとりするイメージ

単純にこう書くと、

```rust
let task = tokio::spawn(async move {
   let framework = StandardFramework::new()
       .configure(|c| c.case_insensitivity(true))
       .group(&GENERAL_GROUP);
   let initial_state = BotState::new();
   let mut client = Client::builder(&token)
       .event_handler(Handler)
       .framework(framework)
       .type_map_insert::<BotState>(Arc::new(Mutex::new(initial_state))) // new!
       .await
       .expect("Failed to build client");
   if let Err(why) = client.start().await {
       println!("Client error: {:?}", why);
   }
});
task.await;
```

チャンネルの追加と spawn されたタスクが紐づかない

もしかして、

```rust

```

### mpsc::channel を async ブロック内で定義すると怒られる

```
type inside `async` block must be known in this context
cannot infer type for type parameter `T` declared on the function `channel`rustcE0698
```

ジェネリクスがないからなのだが、サンプルコードは動いていそう。

### :: の意味がよく分からない

module を辿っているのはわかるがジェネリクスの指定もそうなっていて、そういうものか？と言う気持ち

- ` .type_map_insert::<BotState>(Arc::new(Mutex::new(initial_state)))`
- ` let (tx, mut rx) = mpsc::channel::<String>(32);`

## memo

-
