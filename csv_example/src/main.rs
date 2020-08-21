extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::{error::Error, ffi::OsString, process};

// fn main_not_recover() {
//     println!("Hello, world!");
//     let mut rds = csv::Reader::from_reader(io::stdin());
//     for result in rds.records() {
//         // expectは、Error時にpanicを発生させるので、バッドプラクティスである。
//         let record = result.expect("a csv record");
//         println!("{:?}", record);
//     }
// }

fn main() {
    println!("Hello, world!");
    match performance_up_read_csv_to_model() {
        Ok(count) => println!("{:?}", count),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
// error 処理の練習
fn main_recorver() {
    println!("Hellssdfgsdf");

    let mut rds = csv::Reader::from_reader(io::stdin());
    for result in rds.records() {
        match result {
            Ok(r) => println!("{:?}", r),
            // こうすることで、回復可能なエラー処理になる。
            Err(e) => println!("{:?}", e),
        }
    }
}
// read and write csv test
fn main_csv() {
    println!("Hello, world!");

    // if let 文で、Errの場合のみの処理を、{}内に記載できる。＜これ便利だ！
    if let Err(err) = read_and_write_csv_model() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run_match() -> Result<(), Box<dyn Error>> {
    let mut rds = csv::Reader::from_reader(io::stdin());
    for result in rds.records() {
        match result {
            //　先に書いて、returnするんだって。
            Err(e) => return Err(From::from(e)),
            Ok(r) => println!("{:?}", r),
        }
    }
    Ok(())
}

fn run_question() -> Result<(), Box<dyn Error>> {
    let mut rds = csv::Reader::from_reader(io::stdin());
    for result in rds.records() {
        //  ?を使うことで可読性が上がる！
        let a = result?;
        println!("{:?}", a);
    }
    Ok(())
}

fn read_csv_file() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    // ここでヘッダーを読み込みたいとする。
    // ① clone()する。
    // ただし、メモリにコピーをとる代償が伴う。
    // let headers = rdr.headers()?.clone();
    {
        // lifetimeのために、この呼び出しはそれ所有スコープでネストされている。
        // ② スコープをネストさせる。
        // 所有権が奪われて、以降のイテレーションができなくなる。
        // ＜なるほど。逆にこういうテクニックがあるということか。
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn read_csv_file2() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_file_path() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn read_csv_file3() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());
    // setting可能。＜柔軟
}

type Record = (String, String, Option<u64>, f64, f64);

fn read_csv_file4() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record3 = result?;
        println!("{:?}", record);
    }
    Ok(())
}

type Record2 = HashMap<String, String>;

fn read_csv_file5() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record2 = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record3 {
    latitude: f64,
    longitude: f64,
    // error時に、自動的にNoneにしてくれるオプション
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<f64>,
    city: String,
    state: String,
}

fn write_csv() -> Result<(), Box<dyn Error>> {
    // let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(io::stdout());

    // AsRef<[u8]>境界はString, &str, Vec<u8>のような型がすべて条件を満たすため有用である。
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    wtr.flush()?;
    Ok(())
}

// borrowされた&strを、ownedなString型で置き換えるということは、
// レコードを書き込むたびにcityとstate双方の新しいStringをアロケートしなければならないことを意味する。
// これでも書き込みはできるにはできるのだが、メモリとパフォーマンスを少しばかり無駄遣いしている。
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct WriteRecord<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn write_csv2() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.serialize(WriteRecord {
        city: "Davidsons Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;
    wtr.serialize(WriteRecord {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(WriteRecord {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;
    Ok(())
}

fn read_and_write_csv() -> Result<(), Box<dyn Error>> {
    let argss = match env::args_os().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(argument) => argument,
    };

    // CSVリーダー(stdin)とCSVライター(stdout)を構築する
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|r| r == &argss) {
            wtr.write_record(&record);
        }
    }

    wtr.flush()?;
    Ok(())
}

// utf-8に変換できない場合の対処法。
// byteで読み込む！！！
fn read_and_write_byte_csv() -> Result<(), Box<dyn Error>> {
    let argss = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(argument) => argument,
    };

    // CSVリーダー(stdin)とCSVライター(stdout)を構築する
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.byte_headers()?)?;

    for result in rdr.byte_records() {
        let record = result?;
        // argss.as_bytes() 戻りが、参照なのね。
        if record.iter().any(|r| r == argss.as_bytes()) {
            wtr.write_record(&record);
        }
    }

    wtr.flush()?;
    Ok(())
}

// 前回の例と違い、デシリアライズとシリアライズ両方をderiveする
// これは型から自動的にデシリアライズとシリアライズを行えるということである
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RecordMulti {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
}

fn read_and_write_csv_model() -> Result<(), Box<dyn Error>> {
    // クエリとなる固定引数を受け取る
    // もし引数が与えられないか整数でない場合はエラーを返す
    let minimum_pop: u64 = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(arg) => arg.parse::<u64>()?,
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for result in rdr.deserialize() {
        let record: RecordMulti = result?;
        if record.population.map_or(false, |f| f >= minimum_pop) {
            wtr.serialize(&record)?;
        }
    }

    wtr.flush()?;
    Ok(())
}

// ./csv_example < worldcitiespop.csv  2.12s user 0.09s system 70% cpu 3.125 total
fn performance_read_csv() -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in reader.records() {
        let record = result?;
        if &record[0] == "us" && &record[3] == "MA" {
            count += 1;
        }
    }
    Ok(count)
}

// ./csv_example < worldcitiespop.csv  1.69s user 0.05s system 34% cpu 5.094 total
// String からbyteで処理をするように変更した。
fn performance2_read_csv() -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    let mut count = 0;
    for result in reader.byte_records() {
        let record = result?;
        if &record[0] == b"us" && &record[3] == b"MA" {
            count += 1;
        }
    }
    Ok(count)
}

// ./csv_example < worldcitiespop.csv  0.44s user 0.04s system 22% cpu 2.142 total
// reader.record()は、イテレータをどんどん返す（アロケートしながら）
// だから、１回だけにして、アロケーションの回数を減らす。
fn performance3_read_csv() -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());

    // 一度だけ、メモリにアロケーションする。読み込まれるたびに上書きされていくため、高速化する。
    let mut record = csv::ByteRecord::new();
    let mut count = 0;

    while reader.read_byte_record(&mut record)? {
        if &record[0] == b"us" && &record[3] == b"MA" {
            count += 1;
        }
    }

    Ok(count)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RecordPerformance {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

// ./csv_example < worldcitiespop.csv  3.66s user 0.11s system 85% cpu 4.396 total
fn performance_read_csv_to_model() -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut count = 0;

    for result in reader.deserialize() {
        let record: RecordPerformance = result?;
        if &record.country == "us" && &record.region == "MA" {
            count += 1;
        }
    }

    Ok(count)
}

// 生存期間をつけて、さらに参照型のstrに変更する。
//tutorial-perf-serde-02.rs
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RecordPerfomanceUp<'a> {
    city: &'a str,
    country: &'a str,
    accent_city: &'a str,
    region: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

// ./csv_example < worldcitiespop.csv  1.14s user 0.04s system 97% cpu 1.216 total
fn performance_up_read_csv_to_model() -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut raw_record = csv::StringRecord::new();
    let headers = reader.headers()?.clone();

    let mut count = 0;
    // while reader.read_record(&mut raw_record)? {
    //     let record: RecordPerfomanceUp = raw_record.deserialize(Some(&headers))?;
    //     if record.country == "us" && record.region == "MA" {
    //         count += 1;
    //     }
    // }

    for result in reader.deserialize() {
        let record: RecordPerformance = result?;
        if &record.country == "us" && &record.region == "MA" {
            count += 1;
        }
    }
    Ok(count)
}
