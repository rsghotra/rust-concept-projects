#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width & self.height
    }

    fn contain_rectangle(&self, rectangle: &Rectangle) -> bool {
            self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
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

    println!("The area of the rectangle is: {}", rect1.area()); //rust automatically add as many dereferences or references with a dot operator aka method syntax
    println!("The area of the rectangle is: {}", Rectangle::area(&rect1)); //with namespace type call - we need ourself to match what the method is expecting est for &self

    if rect1.width() {
        println!("The rectangle has a non zero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.contain_rectangle(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.contain_rectangle(&rect3));

    println!("Calling rectangle's square method {:?}", Rectangle::square(3));

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    let area1 = r.area();
    let area2 = Rectangle::area(&r);

    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    let r = &mut Box::new(Rectangle{
        width: 1,
        height: 20,
    });

    let area1 = r.area();
    let area2 = Rectangle::area(&**r);

    assert_eq!(area1, area2);



}