fn main() {
    let s = String::from("hello adkljfs"); // s comes into scope
    let f = first_word(&s);

    println!("f: {f}");

    let mut ty = TestStruct {
        uint32: 5,
        tuple_1: (0, -2),
        str_slice: String::from("ABcd"),
        str_ref: s,
    };

    let mut rec1 = Rectangle{
        width: 10,
        height: 5
    };
    println!("Rec1 fields {:?}", rec1);
    rec1 = dbg!(rec1);
    rec1.can_hold(&rec1);
    let sq1 = Rectangle::square(12);
    // println!("ref: {ty.str_ref}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &c) in s_bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    s
}

struct TestStruct {
    uint32: u32,
    str_slice: String,
    str_ref: String,
    tuple_1: (u8, i32),
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum SubEnum {
    E1,
    E2,
}

enum MainEnum {
    t1(SubEnum),
}