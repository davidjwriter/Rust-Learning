use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // let dne = &v[100];
    // let dne = v.get(100);
    // let dne = match v.get(100) {
    //     Some(x) => x,
    //     None => &0
    // };
    // println!("dne is {}", dne);

    let mut v = vec![1, 2, 3, 4, 5];

    if check_num(&v[0]) {
        v.push(6);
    }

    println!("The first element is {}", &v[0]);

    for i in &v {
        println!("{}", i);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
}

fn check_num(num: &u32) -> bool {
    num < &2
}
