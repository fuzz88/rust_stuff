use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3, 4, 5];

    for (i, el) in v.iter().enumerate() {
        match i {
            last if last == v.len() - 1 => print!("{el}"),
            _ => print!("{el} "),
        }
    }

    let third: &i32 = &v[2];
    println!("\nThe third element is {third}");

    let third : Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let s = String::from("this is a string, Юникод");

    println!("{s}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert("Red".to_string(), 15);

    println!("{scores:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
