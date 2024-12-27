use std::{fs::{create_dir_all, read_dir}, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// サムネイル化する画像が入るフォルダ
    input: PathBuf,
    /// サムネイルにした画像を保存するフォルダ
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    // 出力先フォルダ
    create_dir_all(&args.output).unwrap();

    let mut processed_count = 0;
    for item in read_dir(&args.input).unwrap() {
        let item = item.unwrap();
        let input_path = item.path();
        if input_path.is_dir() {
            // フォルダは処理しない
            continue;
        }

        let img = image::open(&input_path);
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64);
            let output_path = args.output.join(
                input_path.file_name().unwrap()
            );
            thumbnail.save(output_path).unwrap();
            processed_count += 1
        }
    }

    println!("Processed {} Images", processed_count)
}
