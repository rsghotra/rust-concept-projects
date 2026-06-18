use std::slice;
#[allow(static_mut_refs)]

static HELLO_WORLD: &str = "Hello World";
static mut COUNTER: u32 = 0;

//unsafe traits: any method implementing unsafe block will make whole trait unsafe
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

fn main() {
    let mut num = 5;

    //declaring unsafe pointers

    let r1 = &raw const num;
    let r2 = &raw mut num;

    let address = 0x012345usize;
    let r = address as *mut i32;

    //let values = unsafe{slice::from_raw_parts_mut(r, 50)};

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    //calling unsafe method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    //using exten Functions to Call External Code
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Abs value of -3 according to C: {}", abs(-3));
    }

    //if we know any function which is safe to call then we can prefix it with safe then we will not need to use unsafe block
    unsafe extern "C" {
        safe fn fabs(input: f64) -> f64;
    }

    println!("Absolute value of -3.5 according to C: {}", fabs(-3.5));

    //Accessing or modifying a Mutable Static variable
    println!("value is: {HELLO_WORLD}");

    unsafe {
        //SAFETY: This is only called from a single thread in main
        add_to_count(5);
        println!("COUNTER = {}", *(&raw const COUNTER));
    }


}


//creating safe abstration for unsafe code -> function is safe because of assertion and unsafe code is part of its block
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();


    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    
    }
}

//letting other languages to call c fundtion
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

//SAFETY: Calling this from more than a SINGLE thread at a time is undefined behavior, so you must gurrantee you only call it froma single thread at atime

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}