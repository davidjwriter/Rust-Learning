struct Person {
    name: String,
    age: u16,
    home: String,
}

impl Person {
    fn print_person(&self) {
        println!("{} is {} years old and lives in {}.", self.name, self.age, self.home);
    }

    fn happy_birthday(&mut self) {
        println!("Happy Birthday, {}!!", self.name);
        self.age = self.age + 1;
    }

    fn create_person(name: String, age: u16, home: String) -> Person {
        Person {name, age, home}
    }
}


fn main() {
    println!("Hello, world!");
    let mut p = Person::create_person("David".to_string(), 25, "Durham".to_string());
    p.print_person();
    p.happy_birthday();
    p.print_person();
}
