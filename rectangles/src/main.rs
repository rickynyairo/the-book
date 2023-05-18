#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        call_area(area, &rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Rect 1 is {:#?}", rect1)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn call_area(area: fn(&Rectangle) -> u32, rect: &Rectangle) -> u32{
    area(rect)
}