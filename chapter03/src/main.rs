fn main() {
    {
        // Structure
        struct Person {
            name: String,
            age: u8,
        }

        let mut peter = Person {
            name: String::from("Peter"),
            age: 27,
        };
        println!("{} is {} years old", peter.name, peter.age);

        peter.age = 28;
        println!("{} is {} years old", peter.name, peter.age);

        let jackie = Person {
            name: String::from("Jackie"),
            ..peter
        };
        println!("{} is {} years old", jackie.name, jackie.age);
    }
}
