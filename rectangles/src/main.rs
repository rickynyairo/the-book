#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self{
        Self {
            width,
            height,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(40);
    let rec = Rectangle::new(40, 90);
    dbg!(&rec);
    println!(
        "The area of the rectangle is {} square pixels.",
        call_area(area, &rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Rect 1 is {:#?}", rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area of square is {}", sq.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn call_area(area: fn(&Rectangle) -> u32, rect: &Rectangle) -> u32 {
    area(rect)
}
