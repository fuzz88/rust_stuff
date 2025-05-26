use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("[spawned] {i}");
            thread::sleep(Duration::from_millis(1));
            println!("[spawned] {v:?}");
        }
    });

    // println!("{v:?}");

    for i in 1..5 {
        println!("[main] {i}");
        thread::sleep(Duration::from_millis(2));
    }
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![String::from("hi"), String::from("there")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(3));

        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("how"),
            String::from("are"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    use std::sync::Mutex;

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    use std::sync::Arc;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
