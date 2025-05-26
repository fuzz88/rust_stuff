fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    println!("Hello, world!");
    let hello = String::from("hello world");

    let first_word = first_word(&hello);

    println!("{first_word}");
}
