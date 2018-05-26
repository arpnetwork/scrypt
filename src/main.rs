// Copyright 2018 ARP Network
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate num_cpus;
extern crate scrypt;
extern crate threadpool;

use scrypt::cmd;
use std::sync::mpsc;
use std::thread;
use threadpool::ThreadPool;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let size = num_cpus::get() * 2;
        let pool = ThreadPool::new(size);

        for cmd in cmd::incoming() {
            let job_tx = tx.clone();
            pool.execute(move || {
                let res = match scrypt::process(&cmd.data) {
                    Ok(res) => format!("{} 0 {}", cmd.id, res),
                    Err(err) => format!("{} 1 {}", cmd.id, err.description()),
                };
                job_tx.send(res).unwrap();
            });
        }

        std::process::exit(0);
    });

    while let Ok(res) = rx.recv() {
        println!("{}", res);
    }
}
