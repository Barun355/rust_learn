use std::{sync::mpsc, thread::spawn};


fn main() {
    // create_thread();
    pass_data_in_thread();
    thread_message_passing();
}

fn create_thread() {
    spawn(|| {
        let mut c = 0;
        for _i in 1..=500000000 {
            for _j in 1..=500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    // handle.join().unwrap();

    let mut c = 0;
    for _i in 1..=500000000 {
        for _j in 1..=500000000 {
            c = c + 1;
            c = c - 1;
        }
    }
}

fn pass_data_in_thread() {
    let v = vec![1, 2, 3];

    spawn(move || {
        println!("{v:?}");
    });

}

fn thread_message_passing(){
    
    let (tx, rx) = mpsc::channel();
    
    spawn(move || {
        tx.send("Test".to_string()).unwrap();
    });

    let value = rx.recv();

    match value {
        Ok(val) => println!("{val}"),
        Err(_) => println!("Some error occured")
    }
}