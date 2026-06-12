use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with Data: {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Hell Yeah")
    };

    let d = CustomSmartPointer {
        data: String::from("Dell Yeah"),
    };

    //dropping c before it goes out of scope
    std::mem::drop(c);

    println!("Customer Pointer Created!")
}