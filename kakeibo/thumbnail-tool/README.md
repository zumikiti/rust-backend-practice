#  サムネイル画像生成ツール

直列で処理した場合
```sh
$ time ./target/debug/thumbnail-tool ~/Desktop ./output
Processed 72 Images

________________________________________________________
Executed in   31.75 secs    fish           external
   usr time   31.60 secs   57.00 micros   31.60 secs
   sys time    0.14 secs  682.00 micros    0.14 secs
```

並列処理の場合
```sh
$ time ./target/debug/thumbnail-tool ~/Desktop ./output
Processed 72 Images

________________________________________________________
Executed in   10.53 secs    fish           external
   usr time   33.73 secs   78.00 micros   33.73 secs
   sys time    0.13 secs  807.00 micros    0.13 secs
```

rayon クレートをしようすると、裏でCPUやメモリ数をチェックして最大限までスレッドを設定してくれる。
ただ、最大限活用しても速度はさほど変化しない。
そのため、実際にはスレッド数の設定し細かくチューニングする必要はある。
