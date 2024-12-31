fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.", area(width1, height1)
    );
    // Tuple approach
    let rect1 = (30, 50);
    area_tuple(rect1);

    // Struct approach
    let rect2 = Rectangle {
        width: 30, 
        height: 50,
    };
    area_struct(&rect2);

    rect2.area();

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        height: 40, 
        width: 10,
    };

    let rect3 = Rectangle {
        width: 60, 
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}   
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
