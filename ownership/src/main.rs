fn main() {
    // Allocates on the heap, so it can grow because it's owned
    // let mut s = String::from("hello");

    // s.push_str(" world!");

    // println!("{}", s);

    // Will break because there is no ownership
    // let mut x = "hello";

    // x = x + " world";

    // println!("{}", x);

    // let x = 5;
    // let mut y = x;

    // y = y + 1;

    // println!("{} {}", x, y);

    // let s1 = String::from("hello");
    // let mut s2 = s1.clone();

    // s2 = s2 + " world";
    // println!("{} {}", s1, s2);

    // let s = String::from("hello");

    // take_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // println!("{} {}", x, s);
    // let mut s = String::from("world??");
    // change(&mut s);
    // println!("{}",s);
    let mut s = String::from("hello world");
    let word = find_first_word(&s);
    s.clear();
    println!("First word is {}", word);

}


fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}




fn take_ownership(s: String) {
    println!("I own your string: {}", s);
}

fn makes_copy(x: i32) {
    println!("I don't own your int: {}", x);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
