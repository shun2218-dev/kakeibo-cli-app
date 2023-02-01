use crate::services;
use chrono::NaiveDate;
use std::io;
use std::str::FromStr;

fn input_register_type() -> u8 {
    println!("登録種別を入力してください（0:収入、1:支出 ）");
    let mut register_type = String::new();
    io::stdin()
        .read_line(&mut register_type)
        .expect("登録種別の入力に失敗しました");
    let register_type: u8 = register_type
        .trim()
        .parse()
        .expect("登録種別は数値で入力してください");
    services::validate::InputValidator::validate_register_type(register_type);
    register_type
}

fn input_name() -> String {
    println!("品目名を入力してください");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("品目名の入力に失敗しました");
    name.trim().to_string()
}

fn input_category_type(register_type: u8) -> u8 {
    println!("カテゴリーを入力してください");
    if register_type == 0 {
        println!("(0:給与、1:ボーナス、2:その他)");
    } else {
        println!("（0:食費、1:趣味、2:その他）");
    }
    let mut category_type = String::new();
    io::stdin()
        .read_line(&mut category_type)
        .expect("カテゴリ種別の入力に失敗しました");
    let category_type: u8 = category_type
        .trim()
        .parse()
        .expect("カテゴリ種別は数値で入力してください");
    services::validate::InputValidator::validate_category_type(register_type, category_type);
    category_type
}

fn input_price() -> u32 {
    println!("金額を入力してください");
    let mut price = String::new();
    io::stdin()
        .read_line(&mut price)
        .expect("金額の入力に失敗しました");
    price.trim().parse().expect("金額を数値で入浴してください")
}

fn input_date() -> NaiveDate {
    println!("日付を入力してください（yyyy-mm-dd）");
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    NaiveDate::from_str(&date).expect("日付はyyyy-mm-ddの形式で入力してください")
}
