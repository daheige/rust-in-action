use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Yellow"), 60);
    println!("scores = {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score = {}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("field_name:{},field_value:{}", field_name, field_value);

    scores.insert(String::from("Blue"), 25);
    println!("scores = {:?}", scores);

    scores.entry(String::from("green")).or_insert(80);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);

    let text = "rust hello world wonderful world hello test rust";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
