fn main() {
    // コマンドラインの引数一覧出力
    for arg in std::env::args() {
        println!("{}", arg)
    }
}
