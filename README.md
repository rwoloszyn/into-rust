Install Rust
============
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Init new project
================
```
cargo new PROJECT_NAME
```


Adding dependencies
===================
```
cargo add ferris-says
```

Compile
=======
```
cargo build
```

Run
===
```
cargo run
```

Example
=======
```
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```


Where I finished this time....
==============================
11.03.2024 Ownership, memory managment -> https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type

References:
* https://www.rust-lang.org/learn/get-started
* https://doc.rust-lang.org/rust-by-example/primitives.html
* https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type