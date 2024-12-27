use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

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

    let mut all_paths = vec![];
    for entry in read_dir(&args.input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // フォルダは処理しない
            continue;
        }

        all_paths.push(path)
    }

    let processed_count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for check in all_paths.chunks((all_paths.len() + 3) / 4) {
        let check = check.to_vec();
        let processed_count = processed_count.clone();
        let output = args.output.clone();

        handles.push(thread::spawn(move || {
            let mut local_count = 0;
            for path in check {
                let output_path = output.join(path.file_name().unwrap());
                let img = image::open(&path);
                if let Ok(img) = img {
                    let thumbnail = img.thumbnail(64, 64);
                    thumbnail.save(output_path).unwrap();
                    local_count += 1
                }
            }
            let mut writer = processed_count.lock().unwrap();
            *writer += local_count
        }));
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!(
        "Processed {} Images",
        processed_count.as_ref().lock().unwrap()
    )
}
