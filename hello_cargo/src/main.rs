use std::{collections::HashMap, u64::MIN};

fn main() {
    println!("Hello world!");
    let field_name = String::from("abc");
    let field_val = 13;

    let mut map = HashMap::new();


    println!("{}, {}", field_name, field_val);

    let mut int_vec = vec![10, 9, 8, 7, 10];

    int_vec.sort();

    for i in &int_vec{
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut maxKey: Option<i32> = None;
    let mut maxVal: Option<i32> = None;

    for (key, val) in &map{
        match maxVal {
            Some(v) => {
                if v < *val {
                    maxVal = Some(*val);
                    maxKey = Some(*key);
                }
            }
            None => {
                maxKey = Some(*key);
                maxVal = Some(*val);
            }
        }
    }
    println!("median: {}, mode: {:?}", int_vec[int_vec.len()/2], maxKey.unwrap_or(i32::MIN));
}
