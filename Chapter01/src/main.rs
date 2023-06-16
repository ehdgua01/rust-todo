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
        let x_bool: bool = true;

        println!("[x.into()] {x} * {y} = {}", multiply(x.into(), y));
        println!("[i16::from(x)] {x} * {y} = {}", multiply(i16::from(x), y));
        println!(
            "[bool.into()] {x_bool} * {y} = {}",
            multiply(x_bool.into(), y)
        );
        // println!(
        //     "[i128.into()] {x_i128} * {y} = {}",
        //     multiply(x_i128.into(), y)  # Complie error
        // );
        // println!(
        //     "[f32.into(x)] {x_float} * {y} = {}",
        //     multiply(x_float.into(), y)  # Complie error
        // )
    }
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
