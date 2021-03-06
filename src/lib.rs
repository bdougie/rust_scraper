#![feature(test)]
extern crate hyper;
use std::sync::Arc;
use std::thread;
extern crate test;
use hyper::Client;
use std::io::Read;
extern crate time;

#[no_mangle]
pub extern fn run_threads() {
    let start_time = time::now();
    let client = Arc::new(Client::new());
    let threads: Vec<_> = (0..5).map(|i| {
        let client = client.clone();
        thread::spawn(move || {
            println!("Requesting {}", i.to_string());
            let mut response = client.get("http://wikipedia.com").send().unwrap();
            let mut body = String::new();
            response.read_to_string(&mut body).unwrap();
            body.len().to_string()
        })
    }).collect();

    let responses: Vec<_> = threads
        .into_iter()
        .map(|thread| thread.join())
        .collect();
    println!("All threads joined. Full responses are:");
    for response in responses.into_iter() {
        println!("The response have the following lengths: {:?}", response.ok());
    }
    let end_time = time::now();
    println!("{:?}", (end_time - start_time));
}
