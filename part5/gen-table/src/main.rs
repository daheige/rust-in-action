// 定义engine和sql_type模块
mod engine;
mod sql_type;

use clap::{Arg, ArgAction, Command};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, welcome to gen-table");

    let matches = Command::new("clap demo")
        .version("0.1.0")
        .author("gen-table by daheige")
        .about("gen-table for mysql table structures convert to rust code")
        .arg(
            Arg::new("dsn")
                .short('d')
                .long("dsn")
                .action(ArgAction::Set)
                .help("mysql dsn,eg:mysql://root:root1234@localhost/test")
                .required(true),
        )
        .arg(
            Arg::new("out_dir")
                .short('o')
                .long("out_dir")
                .help("gen code output dir")
                .default_value("src/model"),
        )
        .arg(
            Arg::new("table")
                .short('t')
                .long("table")
                .help("tables eg:orders,users")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("enable_tab_name")
                .short('e')
                .long("enable_tab_name")
                .help("whether to generate table_name method for struct")
                .default_value("true")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("no_null_field")
                .short('n')
                .long("no_null")
                .help("whether to allow a field of null type")
                .default_value("false")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("is_serde")
                .short('s')
                .long("serde")
                .help("whether to use serde serialization and deserialization")
                .default_value("false")
                .action(ArgAction::Set),
        )
        .get_matches();

    let dsn = matches.get_one::<String>("dsn").expect("dsn invalid");
    let out_dir = matches
        .get_one::<String>("out_dir")
        .expect("out_dir invalid");
    let table = matches.get_one::<String>("table").expect("table invalid");

    let enable_table_name = str_to_bool(
        matches
            .get_one::<String>("enable_tab_name")
            .expect("enable_tab_name invalid")
            .as_str(),
    );
    let no_null_field = str_to_bool(
        matches
            .get_one::<String>("no_null_field")
            .expect("no_null_field invalid")
            .as_str(),
    );
    let is_serde = str_to_bool(
        matches
            .get_one::<String>("is_serde")
            .expect("is_serde invalid")
            .as_str(),
    );

    println!(
        "tables:{} enable_table_name:{} no_null_field:{} is_serde:{}",
        table, enable_table_name, no_null_field, is_serde
    );

    let tables: Vec<&str> = table.split(",").collect();
    let mut entry = engine::Engine::new(&dsn, &out_dir)
        .with_enable_tab_name(enable_table_name)
        .with_no_null_field(no_null_field)
        .with_serde(is_serde);
    entry.gen_code(tables).await;

    Ok(())
}

fn str_to_bool(s: &str) -> bool {
    let v = if s.to_lowercase().eq("true") {
        true
    } else {
        false
    };

    v
}
