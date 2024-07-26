use std::{env, fs};

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    println!(
        "{}",
        src.as_str()
            .replace(">", "skibidi ")
            .replace("<", "rizz ")
            .replace("+", "sigma ")
            .replace("-", "gyatt ")
            .replace(".", "yap ")
            .replace(",", "blud ")
            .replace("[", "grimaceshake ")
            .replace("]", "fanumtax ")
    );
}
