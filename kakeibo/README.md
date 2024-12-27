# 家計簿 CLI コマンド

ビルド
```sh
cargo build
```

```sh
$ ./target/debug/kakeibo -h

Usage: kakeibo <COMMAND>

Commands:
  new       新しい講座を作る
  deposit   口座に入金する
  withdraw  口座から出金する
  import    CSV からインポートする
  report    レポートを出力する
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
