use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい講座を作る
    New,
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSV からインポートする
    Import,
    /// レポートを出力する
    Report,
}

fn main() {
    let _args = App::parse();
}
