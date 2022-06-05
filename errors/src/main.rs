use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    //let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", error),
    //     },
    // };

    // More seasoned Rustacean would do this

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
