Install Rust
============
<code>
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
</code>

Init new project
================
<code>
cargo new PROJECT_NAME
<code>


Adding dependencies
===================
<code>
cargo add ferris-says
</code>

Compile
=======
<code>
cargo build
</code>

Run
===
<code>
cargo run
</code>

Example
=======
<code>
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
</code>


References:
* https://www.rust-lang.org/learn/get-started
* https://doc.rust-lang.org/rust-by-example/primitives.html