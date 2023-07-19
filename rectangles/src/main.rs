fn main() {
    println!("Hello recangles !");

    //Not using tuples or struct
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square units.", area(width, height));

    //Using tuples
    let rectangle = (40,60);

    println!("The area of the 2nd rectangle is {} square units.", area_tuple(rectangle));

    //Using structs
    let rectangle_struct = Rectangle {
        width: 45,
        height: 25,
    };

    println!("The area of the struct rectangle is {} square units.", 
    //We use a reference so that the main function keeps the ownership of rectangle_struct
    area_struct(&rectangle_struct));

    //Using structs & methods
    println!("The area of the struct rectangle using method is {} square units.", rectangle_struct.area());

    //Can hold exercise 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 25,
        height: 55,
    };

    println!("Can rect2 hold rect1 ? {}", rect2.can_hold(&rect1));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        if (self.width > rectangle.width) && (self.height > rectangle.height) {
            true
        }
        else {
            false
        } 
    }
}
