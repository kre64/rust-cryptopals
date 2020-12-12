mod initialization;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let selected = &args[1];

    let challenges = initialization::init_hash_map();

    println!("Running rust-cryptopals!");

    // println!("Selected challenge {}", challenge);

    println!("Selected challenge: {}", "e");
    println!("Review for Jane: {}", challenges[1]);
}
