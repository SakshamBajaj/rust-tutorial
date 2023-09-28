
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
    fn width_less_than(&self, other: &Rectangle) -> Result<(), String> {
        if(self.width < other.width) {
            Ok(())
        } else {
            Err(String::from("width is greater"))
        }
        
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        assert_ne!(2 + 2, 5);
    }

    fn setup_rectangles() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        
    }

    // #[test]
    // fn can_hold(){
    //     setup_rectangles();
    //     assert!(rect1.can_hold(&rect2));
    // }

    #[test]
    fn width_less() -> Result<(), String> {
        let rect1 = Rectangle {
            width: 9,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        }; 
        rect1.width_less_than(&rect2)?;
        Ok(())
    }
}
