# このプロジェクトについて
Rustを使ったシンプルなWebサーバーを実装しました。

# 環境
- rustc 1.71.1 (eb26296b5 2023-08-03)
- cargo 1.71.1 (7f1d04c00 2023-07-29)

# 起動方法

```bash
% cargo run      
   Compiling myfirstrustapp v0.1.0 (/Users/koyamayoshito/CLionProjects/myfirstrustapp)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/myfirstrustapp`
略
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
🚀 Rocket has launched from http://127.0.0.1:8000
```

# 動作確認

```shell
% curl http://localhost:8000/current-time -i
HTTP/1.1 200 OK
content-type: application/json
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 33
date: Mon, 14 Aug 2023 05:08:32 GMT

{"message":"2023-08-14 14:08:32"}
```

```shell
% curl http://localhost:8000/hello -i
HTTP/1.1 200 OK
content-type: application/json
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 27
date: Mon, 14 Aug 2023 05:08:54 GMT

{"message":"Hello, world!"}
```
```shell
% curl "http://localhost:8000/greeting?country=japan" -i
HTTP/1.1 200 OK
content-type: application/json
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 29
date: Mon, 14 Aug 2023 05:08:08 GMT

{"message":"こんにちは"}%
```

