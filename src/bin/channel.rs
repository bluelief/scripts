use async_std::channel::{bounded,unbounded,RecvError};
use async_std::task;
use std::panic;

#[async_std::main]
async fn main() {
    let (s, r) = bounded(50);
    let mut counter = 0;
    let handle1 = task::spawn(async move {
        //s.send(counter).await;
        loop {
            s.send(counter).await;
            println!("[+] send: {}", counter);
            counter += 1;

            if counter == 50 {
                println!("[*] OK, that's enough");
                break;
            }
        }
    });

    let handle2 = task::spawn(async move {
        loop {

            if r.is_empty() {
                continue
            } else {
                let reciver = r.recv().await;
                let reciver = match reciver {
                    Ok(recv) => recv,
                    Err(RecvError) => {
                        panic!("{:?}", RecvError);
                        //println!("[!] Task error: {:?}", RecvError);
                    },
                };
                println!("[-] recv: {:?}", reciver);
            }
        }
    });

    task::block_on(async {
        handle1.await;
        handle2.await;
    });
}
