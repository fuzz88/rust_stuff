fn main() {
    let s = "hello";
    
    println!("{s}!");

    let mut s = String::from(s);
    s.push_str(", world");

    println!("{s}!");

    let mut s1 = String::from("hello, world!");
    let s2 = s1.clone();

    println!("{s1} {s2}");

    let len = calculate_len(&s2);

    println!("The length of s2 is: {len}");
    change(&mut s1);

    let s3 = &mut s1;
    println!("{s3}");
    {
        let s4 = &mut s1;
        change(s4);
        println!("{s4}");
    }
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" fuck, yeah!");
}
