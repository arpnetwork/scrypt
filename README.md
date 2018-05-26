# scrypt

Command line wrapper for [scrypt](https://en.wikipedia.org/wiki/Scrypt).

## Building

```bash
cargo build --release
```

## Usage

```bash
$ target/release/scrypt
# <seq> <password> <salt> <log_n> <r> <p> <dk_len>
42 70617373776f7264 73616c74 18 8 1 32
# <seq> <status> <result>
42 0 d36e883d93698af49daa529419bb1d97da262bbaa225c12fcf05651268659f42
```
