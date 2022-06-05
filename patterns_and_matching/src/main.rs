fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using your fave color {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple");
        } else {
            println!("using orange");
        }
    } else {
        println!("using blue");
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("default val, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let numbers = (2, 4, 6, 8, 10);

    match numbers {
        (_, second, ..) => {
            println!("Second number is {}", second);
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => {let y = x;
            println!("x is less than 5: {}", x)},
        Some(_) => println!("lol"),
        None => (),
    }
    println!("{:?}", num);
}
