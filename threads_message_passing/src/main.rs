use std::thread;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = Sender::clone(&tx);

    send_messages(tx, vec![
                  String::from("hi"),
                  String::from("from"),
                  String::from("the"),
                  String::from("thread")
    ]);

    send_messages(tx1, vec![
                  String::from("more"),
                  String::from("messages"),
                  String::from("for"),
                  String::from("you")
    ]);

    for received in rx {
        println!("Got: {}", received);
    }
}

fn send_messages<T: 'static>(sender: Sender<T>, messages: Vec<T>)
    where T: Send
{
    thread::spawn(move || {
        for msg in messages {
            sender.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
