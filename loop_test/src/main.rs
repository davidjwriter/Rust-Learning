fn main() {
    let n = 9;
    println!("32F is {}", convert_f_to_c(32.0));
    println!("Fib of {}: {}", n, fib(n));
}

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn print_12_days_of_christmas(n: i32) {
    let mut num = 0;
    if (n < 0) {
        num = 12;
    } else {
        num = n;
    }

    if (num == 1) {
        "a Partridge in a Pear Tree"
    }
//     On the twelfth day of Christmas
// my true love sent to me:
// 12 Drummers Drumming
// 11 Pipers Piping
// 10 Lords a Leaping
// 9 Ladies Dancing
// 8 Maids a Milking
// 7 Swans a Swimming
// 6 Geese a Laying
// 5 Golden Rings
// 4 Calling Birds
// 3 French Hens
// 2 Turtle Doves
// and a Partridge in a Pear Tree
}
