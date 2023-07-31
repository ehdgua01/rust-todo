fn main() {
    {
        // Structure
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Person {
            fn new(name: String, age: u8) -> Self {
                Person { name, age }
            }
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

        let david = Person::new(String::from("David"), 27);
        println!("{david:#?}");
    }

    {
        // Tuple
        struct Point(i32, i32);
        let p = Point(17, 23);
        println!("({}, {})", p.0, p.1);

        // 단일 필드의 래퍼(Wrapper) a.k.a. Newtype
        // struct PoundsOfForce(f64);
        // struct Newtons(f64);

        // fn compute_thruster_force() -> PoundsOfForce {
        //     todo!("Ask a rocket scientist at NASA")
        // }

        // fn set_thruster_force(force: Newtons) {
        //     // ...
        // }

        // let force = compute_thruster_force();
        // set_thruster_force(force);
    }
}
