use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    // Not a perfect "minification", but better than none.

    println!("cargo::rerun-if-changed=game.xslt");

    let mut result = String::new();

    let mut reader = BufReader::new(fs::File::open("game.xslt").expect("Reading xslt file"));
    let mut buffer = String::new();
    while let Ok(read) = reader.read_line(&mut buffer) {
        if read == 0 {
            break;
        }
        if !buffer.contains("<!--") {
            let s = buffer.trim().replace("\n", "");
            result.push_str(&s);
        }
        buffer.clear();
    }
    println!("cargo:rustc-env=GAME_XSLT={}", result);
}
