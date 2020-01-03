mod str_utils;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use log::{debug, info, trace};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use str_utils::{edit_distance, is_digits, split_command};

fn calc_score(a: &str, b: &str) -> usize {
    use std::cmp::min;
    let ed = edit_distance(a, b);
    let score = if is_digits(a) && is_digits(b) {
        // 両方数字のときは違いを無視
        0
    } else if a.starts_with('-') && b.starts_with('-') {
        // オプションの違いは2倍で評価
        ed * 2
    } else if ed >= min(a.len(), b.len()) {
        // 文字の長さに対して編集距離が大きい場合は追加ペナルティ
        ed + 3
    } else {
        // 基本は編集距離
        ed
    };
    debug!("\t{} {} {} {}", ed, score, a, b);
    score
}

fn near_aux(a: &[&str], b: &[&str]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let total: usize = a.iter().zip(b).map(|(a, b)| calc_score(a, b)).sum();
    total < a.len()
}

fn near(a: &[Vec<&str>], b: &[Vec<&str>]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| near_aux(a, b))
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .init();
    let default_path = directories::BaseDirs::new()
        .unwrap()
        .home_dir()
        .join(".bash_history");
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Target history file")
                .takes_value(true)
                .default_value(default_path.to_str().unwrap()),
        )
        .get_matches();
    let history_file = Path::new(matches.value_of("file").unwrap());
    debug!("{:?}", &history_file);

    let f = BufReader::new(File::open(history_file).unwrap());
    let mut old: Option<String> = None;
    for line in f.lines() {
        let l = line.unwrap();
        if let Some(last_line) = old {
            let ov = split_command(&last_line);
            let v = split_command(&l);
            trace!("{:?}", &ov);
            trace!("{:?}", &v);
            if !near(&ov, &v) {
                println!("{}", last_line);
            } else {
                info!("{}", last_line);
            }
        }
        old = Some(l);
    }
    if let Some(l) = old {
        println!("{}", l);
    }
}
