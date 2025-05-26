fn main() {
    let favourite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_colour {
        println!("Using {color}");
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Using blue");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{index} {value}");
    }

    let print_coordinates = |&(x, y): &(i32, i32)| println!("{x} {y}");
    print_coordinates(&(5, -5));

    enum Message {
        Hello {id : i32 },
    }

    let msg = Message::Hello { id: 7 };

    match msg {
        Message::Hello {
            id: id_var @ 3..=7,
        } => println!("{id_var}"),
        _ => todo!(),
    }
}
