fn main() {
    {
        let a = String::from("testststst");
        println!("Hello 🌍!");
        println!("{a}");
    }

    {
        let mut x: i32 = 0;
        let ref_x: &mut i32 = &mut x; // 가변 값에 대한 참조
        *ref_x = 20;
        println!("x: {x}");
    }

    {
        let x: i8 = 15;
        let y: i16 = 1000;

        println!("[x.into()] {x} * {y} = {}", multiply(x.into(), y));
        println!("[i16::from(x)] {x} * {y} = {}", multiply(i16::from(x), y));
    }
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
