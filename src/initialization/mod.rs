use std::collections::HashMap;

pub fn init_hash_map() -> HashMap<i32, &'static str> {
    let mut challenges = HashMap::new();

    challenges.insert(1, "Convert hex to base64");
    
    return challenges;
}
