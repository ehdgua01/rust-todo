fn main() {
    {
        let x = 10;
        // x doesn't move into `println!` because `println!` is a macro
        println!("x: {x}");
    }

    {
        // Type inference
        fn takes_u32(x: u32) {
            println!("u32: {x}");
        }

        fn takes_i8(y: i8) {
            println!("i8: {y}");
        }

        let x = 10;
        let y = 20;

        takes_u32(x);
        takes_i8(y);
    }

    {
        // Generic type annotation (_)
        let mut v = Vec::new();
        v.push((10, false));
        v.push((20, true));
        println!("v: {v:?}");

        let vv = v.iter().collect::<std::collections::HashSet<_>>(); // _: generic type annotation
        println!("vv: {vv:?}");
    }

    {
        const DIGEST_SIZE: usize = 3;
        const ZERO: Option<u8> = Some(42);

        fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
            let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
            for (idx, &b) in text.as_bytes().iter().enumerate() {
                digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
            }
            digest
        }

        let digest = compute_digest("Hello");
        println!("Digest: {digest:?}");
    }

    {
        static BANNER: &str = "Welcome to RustOS 3.14";
        println!("{BANNER}");
    }
}
