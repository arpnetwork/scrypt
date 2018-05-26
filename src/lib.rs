// Copyright 2018 ARP Network
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate crypto;
extern crate hex;

pub mod cmd;

use std::error::Error;
use std::io;

/// Fetch next params
macro_rules! next {
    ($p:ident) => ($p.next().ok_or(invalid_error())?)
}

/// Fetch next integer params
macro_rules! next_int {
    ($p:ident) => (next!($p).parse()?)
}

/// Fetch next hexadecimal params
macro_rules! next_hex {
    ($p:ident) => (hex::decode(next!($p))?)
}

/// Parses command line and returns scrypt result
pub fn process(s: &str) -> Result<String, Box<Error>> {
    let mut params = s.split_whitespace();

    let password = next_hex!(params);
    let salt = next_hex!(params);
    let log_n: u8 = next_int!(params);
    let r: u32 = next_int!(params);
    let p: u32 = next_int!(params);
    let dk_len: usize = next_int!(params);

    if log_n == 0 {
        return Err(Box::new(invalid_error()));
    }

    let params = crypto::scrypt::ScryptParams::new(log_n, r, p);
    let mut output = vec![0; dk_len];
    crypto::scrypt::scrypt(&password, &salt, &params, &mut output);
    Ok(hex::encode(output))
}

fn invalid_error() -> io::Error {
    io::Error::from(io::ErrorKind::InvalidInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrypt() {
        let hash = process("aa aa 1 1 1 32").unwrap();
        assert_eq!(
            hash,
            "7bd181d19c802a11eb3d524584fc049b576e32df533ccc94582768939a0689d7"
        );
    }
}
