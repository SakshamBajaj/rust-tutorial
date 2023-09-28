use std::collections::HashMap;

fn main() {
    println!("Hello world!");
    let field_name = String::from("abc");
    let field_val = 13;

    let mut map = HashMap::new();

    map.insert(field_name, field_val);
}
