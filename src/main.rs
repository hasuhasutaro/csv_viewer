use std::{error::Error, io::Write};
use std::fs::File;
use csv::StringRecord;
use prettytable::{Table, Row, Cell};

fn main() {
    loop {
        main_process();

        print!("終了しますか？ (Y/N):");
        std::io::stdout().flush().unwrap(); // 表示するためにバッファをフラッシュ
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("入力の読み取りに失敗しました");
        
        if input.trim().to_lowercase().as_str() == "y" {
            break;
        }
    }
}

fn main_process() {
    print!("csvファイルのパスを入力してください: ");
    std::io::stdout().flush().unwrap(); // 表示するためにバッファをフラッシュ

    let mut path = String::new();
    std::io::stdin().read_line(&mut path).expect("入力の読み取りに失敗しました");
    path = path.trim().to_string(); // 末尾の改行文字を消す

    let file = match file_open(&path) {
        Ok(f) => f,
        Err(err) => {
            println!("ファイルの展開に失敗しました: {}", err);
            return;
        }
    };

    let csv_data = match parse_csv(file) {
        Ok(data) => data,
        Err(err) => {
            println!("CSVファイルの解析に失敗しました: {}", err);
            return;
        }
    };

    let csv_key_index_max = csv_data[0].len() - 1;

    let mut print_data = csv_data;

    print!("ソートしますか？ (Y/N): ");
    std::io::stdout().flush().unwrap(); // 表示するためにバッファをフラッシュ

    let mut sort_check = String::new();
    std::io::stdin().read_line(&mut sort_check).expect("入力の読み取りに失敗しました");

    if let "Y" = sort_check.trim().to_uppercase().as_str() {
        println!("------キー一覧------");
        for (index, record) in print_data[0].iter().enumerate() {
            println!("[{}] {}", index, record);
        }
        println!("-------------------");

        let mut sort_index = 0;
        loop {
            print!("ソートするキーの番号を入力してください: ");
            std::io::stdout().flush().unwrap();
    
            let mut input_index = String::new();
            std::io::stdin().read_line(&mut input_index).expect("入力の読み取りに失敗しました");
            let index = input_index.trim().parse::<usize>();

            match index {
                Err(_) => {
                    println!("整数を入力してください。");
                    continue;
                },
                Ok(i) if i > csv_key_index_max => {
                    println!("{}以上のキーは指定できません。", csv_key_index_max);
                    continue;
                },
                Ok(i) => {
                    sort_index = i;
                    break;
                }
            }
        }

        print!("昇順にソートしますか？ (Y/N): ");
        std::io::stdout().flush().unwrap(); // 表示するためにバッファをフラッシュ

        let mut input_asc_check = String::new();
        std::io::stdin().read_line(&mut input_asc_check).expect("入力の読み取りに失敗しました");
        let asc_check = input_asc_check.trim();

        print_data = sort_records(&print_data, sort_index, asc_check.to_uppercase().as_str() == "Y");
    }

    let table = create_table_from_csv(&print_data);
    table.printstd();
}

fn create_table_from_csv(csv_data: &Vec<StringRecord>) -> Table {
    let mut table = Table::new();

    // ヘッダーを表のタイトルに
    if let Some(header) = csv_data.first() {
        let header_cells = header.iter().map(|h| Cell::new(h)).collect::<Vec<Cell>>();
        table.set_titles(Row::new(header_cells));
    }

    //データの追加 最初はヘッダーなのでスキップ。
    for records in csv_data.iter().skip(1) {
        let cells = records.iter().map(|d| Cell::new(d)).collect::<Vec<Cell>>();
        table.add_row(Row::new(cells));
    }
    table
}

fn file_open(path: &String) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?; // return error
    Ok(file)
}

fn parse_csv(file: File) -> Result<Vec<csv::StringRecord>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    // csvファイル全体をvectorにまとめる
    let records_iter = rdr.records().collect::<Result<Vec<_>, _>>()?;
    Ok(records_iter)
}

fn sort_records(data: &Vec<StringRecord>, key_index: usize, is_asc: bool) -> Vec<StringRecord> {
    let mut non_header_data = data[1..].to_vec();

    if is_asc {
        non_header_data.sort_by(|a, b| a.get(key_index).unwrap().cmp(b.get(key_index).unwrap()));
    } else {
        non_header_data.sort_by(|a, b| b.get(key_index).unwrap().cmp(a.get(key_index).unwrap()));
    }
    
    let mut sorted_data = Vec::new();
    sorted_data.push(data[0].clone());
    sorted_data.extend(non_header_data);
    sorted_data
}
