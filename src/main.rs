mod initialization;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let challenge = &args[1];
    initialization::hello();

    println!("Running rust-cryptopals!");
    println!("Selected challenge {}", challenge);
}
