use std::collections::{HashMap};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Department {
    name: String,
    employee_count: i32
}

struct Employee {
    name: String,
}

fn main() {
    let mut employee_dir: HashMap<Department, Vec<Employee>>  = HashMap::new();
    let d1 = Department {
        name: "test1".to_string(),
        employee_count: 0
    };

    let e1  = Employee {
        name: "anc".to_string(),
    };

    let e2 = Employee {
        name: "e2".to_string()
    };

    employee_dir.insert(d1.clone(), vec![e1]);
    let e = employee_dir.entry(d1.clone()).or_insert(vec![]);
    e.push(e2);
    println!("{}", employee_dir[&d1][1].name);

    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
    let result;
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    {
        result = longest(string1.as_str(), string2.as_str(),25);
    }
    println!("The longest string is {}", result);

}

fn longest<'a, 'b>(x :&'a str, y:&'a str, _c: i32) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}