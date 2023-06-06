struct Person {
    name: String,
}

impl From<String> for Person {
    fn from(item: String) -> Self {
        Person { name: item }
    }
} 

fn main() {
    let name = String::from("Alice");
    let person = Person::from(name);

    println!("{}", person.name);
}