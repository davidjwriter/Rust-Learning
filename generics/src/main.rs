pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

fn create_article() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanely Cup Championship!"),
        location: String::from("Pittspurggg"),
        author: String::from("Icy Boy"),
        content: String::from("No surprise"),
    }
}

fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("davidjwriter"),
        content: String::from("no"),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(create_article())
    } else {
        Box::new(create_tweet())
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn do_stuff<'a>() -> &'a str {
    let x = "hello";
    let y = "world!";

    longest(x, y)
}
fn main() {
    let summary = returns_summarizable(true);
    println!("{}", summary.summarize());

    let num_list = vec![34, 55, 1, 28, 111];

    let result = largest(&num_list);
    println!("The largest number in list is {}", result);

    println!("The longest string is {}", longest("hello", "world"));

    let r = do_stuff();
    println!("r is {}", r);

    let string1 = String::from("longest string in world");
    let result;
    {
        let string2 = String::from("blah");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("result is {}", result);
}
