use super::utils::{do_smt};
use std::{thread::{sleep, spawn}, time::Duration};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let mut handles:Vec<_> = vec![];
    for i in 0..1000000{
        handles.push(
            spawn(move ||{
                sleep(Duration::from_millis(1));
                println!("{i} added");
            })
        )
    }
    for handle in handles{
        handle.join().unwrap();
    }

    do_smt().await;
}
