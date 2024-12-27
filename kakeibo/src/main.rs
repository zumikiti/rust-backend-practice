use std::{collections::HashMap, fs::OpenOptions};

use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい講座を作る
    New(NewArgs),
    /// 口座に入金する
    Deposit(DepositArgs),
    /// 口座から出金する
    Withdraw(WithdrawArgs),
    /// CSV からインポートする
    Import(ImportAges),
    /// レポートを出力する
    Report(ReportArgs),
}

#[derive(Args)]
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut witer = Writer::from_path(file_name).unwrap();
        witer.write_record(["日付", "用途", "金額"]).unwrap();
        witer.flush().unwrap()
    }
}

#[derive(Args)]
struct DepositArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl DepositArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap()
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl WithdrawArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount),
            ])
            .unwrap();
        writer.flush().unwrap()
    }
}

#[derive(Args)]
struct ImportAges {
    src_file_name: String,    // インモートするファイル
    dst_account_name: String, // インポート先として登録する口座名
}

impl ImportAges {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.deserialize() {
            // Readder は先頭行をヘッダーとして扱うので、ここで読まれるのは2ぎょうめから
            let record: Record = result.unwrap();
            writer.serialize(record).unwrap();
        }
        writer.flush().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct Record {
    日付: NaiveDate,
    用途: String,
    金額: i32,
}

#[derive(Args)]
struct ReportArgs {
    files: Vec<String>,
}

impl ReportArgs {
    fn run(&self) {
        let mut map = HashMap::new();
        for file in &self.files {
            let mut reader = Reader::from_path(file).unwrap();
            for result in reader.records() {
                let record = result.unwrap();
                let amount: i32 = record[2].parse().unwrap();
                let date: NaiveDate = record[0].parse().unwrap();
                let sum = map.entry(date.format("%Y-%m").to_string()).or_insert(0);

                *sum += amount;
            }
        }

        println!("{:?}", map)
    }
}

fn main() {
    let args = App::parse();

    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
