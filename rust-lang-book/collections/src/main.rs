use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // println!("{}", field_name);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
    // match score {
    //     Some(x) => println!("{}", x),
    //     None => println!("None"),
    // }

    // for (key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", *i);
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    // for i in &row {
    //     match i {
    //         SpreadsheetCell::Int(x) => println!("{}", x),
    //         SpreadsheetCell::Float(x) => println!("{}", x),
    //         SpreadsheetCell::Text(s) => println!("{}", s),
    //     }
    // }
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2: {}", s2);

    // let mut s = String::from("lo");
    // s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);

    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }
    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }
}
