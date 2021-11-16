#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // association function
    // Rectangle::suare(x)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let recta = Rectangle {
        length: 50,
        width: 30,
    };
    let rectb = Rectangle {
        length: 40,
        width: 10,
    };
    let rectc = Rectangle {
        length: 45,
        width: 60,
    };
    println!("Can recta hold rectb? {}", recta.can_hold(&rectb));
    println!("Can recta hold rectc? {}", recta.can_hold(&rectc));

    // association function
    let square = Rectangle::square(3);
}
