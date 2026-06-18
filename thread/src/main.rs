
use std::thread;
use std::time::Duration;
use std::sync::mpsc;


enum ClientMessage { Incr, Get, Quit }
enum ServerMessage { Get(usize) }


fn main() {

    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
            println!("Here is a vector: {v:?}");
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("first"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }

    });

    thread::spawn(move || {
        let vals = vec![
            String::from("From"),
            String::from("Second"),
            String::from("Thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
        }
    });

    for received in rx {
        println!("Got: {received}");
    }


    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::spawn(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
            }
        }
    });
    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }
    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("{}", n)
    }
    server.join().unwrap();

}
