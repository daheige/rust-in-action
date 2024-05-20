use once_cell::{self, sync::Lazy};
use std::collections::HashMap;

/// mysql type => rust type
static MYSQL_TYPES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // mysql类型映射到rust数据类型，可根据实际情况更改
    let types: Vec<(&str, &str)> = vec![
        ("int", "i64"),
        ("integer", "i64"),
        ("tinyint", "i64"),
        ("smallint", "i64"),
        ("mediumint", "i64"),
        ("bigint", "i64"),
        ("int unsigned", "i64"),
        ("integer unsigned", "i64"),
        ("tinyint unsigned", "i64"),
        ("smallint unsigned", "i64"),
        ("mediumint unsigned", "i64"),
        ("bigint unsigned", "i64"),
        ("bit", "i64"),
        ("bool", "bool"),
        ("enum", "String"),
        ("set", "String"),
        ("varchar", "String"),
        ("char", "String"),
        ("tinytext", "String"),
        ("mediumtext", "String"),
        ("text", "String"),
        ("longtext", "String"),
        ("blob", "String"),
        ("tinyblob", "String"),
        ("mediumblob", "String"),
        ("longblob", "String"),
        ("date", "Duration"),
        ("datetime", "Duration"),
        ("timestamp", "Duration"),
        ("time", "Duration"),
        ("float", "f64"),
        ("double", "f64"),
        ("decimal", "f64"),
        ("binary", "String"),
        ("varbinary", "String"),
        ("json", "String"),
    ];

    for (key, val) in types {
        m.insert(key, val);
    }

    m
});

/// get rust type by mysql type
pub fn get_type(t: &str) -> String {
    let val = MYSQL_TYPES
        .get(t)
        .expect(format!("not found mysql type:{}", t).as_str());

    val.to_string()
}

#[test]
fn test_sql_type() {
    println!("mysql bigint for rust type:{}", get_type("bigint"));
}
