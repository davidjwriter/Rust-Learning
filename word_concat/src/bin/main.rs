fn main() {
    use word_concat::find_word_concatenation;
    let test = find_word_concatenation(
        String::from("catcatfoxfox"),
        Vec::from(["cat".to_string(), "fox".to_string()]),
    );
    print!("{:?}", test);
}
