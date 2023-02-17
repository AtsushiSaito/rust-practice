# RerunViewer にテキストを送信して記録するテストプログラム

- このプログラムは RerunSDK から RerunViewer に TextEntry を送るプログラムです。
- send_text_1: 送信プログラム 1 個目
  - 100ms でテキストを送信するプログラム
- send_text_2: 送信プログラム 2 個目
  - 1000ms でテキストを送信するプログラム
- 各送信プログラムの`set_recording_id`を`8a658439-8b9b-0ca8-e76f-96c5b4f8e5cf`に固定している。

## Usage

1. Rerun Viewer のインストール

```
cargo install rerun
```

2. 送信プログラムの実行（send_text_1）

```
cd rust-practice/rerun/send_text_recording/send_text_1
cargo run
```

3. Rerun Viewer にテキストが表示されます。

4. 送信プログラムの実行（send_text_2）

```
cd rust-practice/rerun/send_text_recording/send_text_2
cargo run
```
