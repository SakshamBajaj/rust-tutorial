use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let x = [3; 5];
    let mut idx = String::new();
    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");
    let index: usize = idx.trim().parse().expect("Index NaN");
    let e = x[index];
    println!("Value at index {index}: {e}");
    plus_one(e);
    let y = {
        let m = 4;
        m + 3
    };
    println!("y: {y}");
    if y < 2 {
        println!("y < 2")
    } else {
        println!("y >= 2")
    }
    let mut ctr = 0;

    let result = 'c_up: loop {
        // infinite loop
        println!("Inf loop!");
        if ctr > 2 {
            break 'c_up ctr;
        }
        ctr += 1;
    };

    println!("result: {result}");
    let c = convert_to_celsius(f64::MAX);
    let f = convert_to_fahrenheit(f64::MIN);
    println!("24 F in C: {c}");
    println!("0.0 C in F: {f}");
    fibonnaci(10);
}

fn plus_one(x: u8) -> u16 {
    x as u16 + 1
}

fn convert_to_celsius(f: f64) -> f64 {
    (5.0 / 9.0) * (f - 32.0)
}

fn convert_to_fahrenheit(c: f64) -> f64 {
    (9.0 / 5.0) * c + 32.0
}

fn fibonnaci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonnaci(n - 1) + fibonnaci(n - 2)
}
