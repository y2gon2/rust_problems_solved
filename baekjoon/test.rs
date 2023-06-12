fn main() {
    let name = String::from("Alice");
    let person = Person::from(name);

    println!("{}", person.name);
}