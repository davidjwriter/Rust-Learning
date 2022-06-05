use std::fmt;

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut HELLO: &str = "Hello, world";

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len + 4));
    }
}
fn test_outline_print() {
    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point: ({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    let p = Point{x: 2, y: 3};
    p.outline_print();
}

fn test_unsafe_stuff() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 6;
        println!("r1 is now: {}", *r1);
    }

    unsafe {
        println!("Absolute value of -3 is: {}", abs(-3));
    }
    unsafe {
        println!("{}", HELLO);
    }
    unsafe {
        HELLO = "Hacked, world!";
    }
    unsafe {
        println!("{}", HELLO);
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_function_pointers() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 2)
}
fn main() {
    test_unsafe_stuff();
    test_outline_print();
    test_function_pointers();
    let x = returns_closure()(3);
    println!("x is: {}", x);
}
