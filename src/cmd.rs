// Copyright 2018 ARP Network
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

/// A standard command builder with sequence support.
#[derive(Debug)]
pub struct Command {
    pub id: String,
    pub data: String,
}

impl Command {
    pub fn new(s: &str) -> Option<Command> {
        let items: Vec<_> = s.trim().splitn(2, ' ').collect();
        if items.len() == 2 {
            Some(Command {
                id: items[0].to_string(),
                data: items[1].to_string(),
            })
        } else {
            None
        }
    }
}

/// An iterator that infinitely read lines on stdin.
pub struct Incoming {
    buf: String,
}

/// Returns an iterator over the lines being received on stdin.
pub fn incoming() -> Incoming {
    Incoming { buf: String::new() }
}

impl Iterator for Incoming {
    type Item = Command;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buf.clear();
            match io::stdin().read_line(&mut self.buf) {
                Ok(n) if n > 0 => {
                    if let Some(cmd) = Command::new(&self.buf) {
                        return Some(cmd);
                    }
                }
                _ => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands() {
        assert!(Command::new("").is_none());
        assert!(Command::new(" \n").is_none());
        assert!(Command::new("abc").is_none());
        let cmd = Command::new("abc def").unwrap();
        assert_eq!(("abc", "def"), (&cmd.id[..], &cmd.data[..]));
    }
}
