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

    {
        // Move
        let s1: String = String::from("Hello!");
        let s2: String = s1;
        println!("s2: {s2}");
        // println!("s1: {s1}");  Complie error!, 소유권이 이동(s1 -> s2)했기 때문에
    }

    {
        // Move & Clone
        fn say_hello(name: String) {
            println!("Hello {name}")
        }

        let name = String::from("Alice");
        say_hello(name.clone());
        say_hello(name.clone());
        println!("Name: {name}")
    }

    {
        // Copy & Clone
        let x = 42;
        let y = x;
        println!("x: {x}");
        println!("y: {y}");

        #[derive(Copy, Clone, Debug)]
        struct Point(i32, i32);

        let p1 = Point(3, 4);
        let p2 = p1;
        println!("p1: {p1:?}");
        println!("p2: {p2:?}");
    }

    {
        // Borrowing
        #[derive(Debug)]
        struct Point(i32, i32);

        fn add(p1: &Point, p2: &Point) -> Point {
            Point(p1.0 + p2.0, p1.1 + p2.1)
        }

        let p1 = Point(3, 4);
        let p2 = Point(10, 20);
        let p3 = add(&p1, &p2);
        println!("{p1:?} + {p2:?} = {p3:?}");
    }

    {
        let mut a: i32 = 10;
        let b: &i32 = &a;
        println!("b: {b}");

        {
            let c: &mut i32 = &mut a;
            *c = 20;
        }

        println!("a: {a}");
    }

    {
        // 함수 호출에서의 수명
        #[derive(Debug)]
        struct Point(i32, i32);

        fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
            if p1.0 < p2.0 {
                p1
            } else {
                p2
            }
        }

        let p1: Point = Point(10, 10);
        let p2: Point = Point(20, 20);
        let p3: &Point = left_most(&p1, &p2);
        println!("left-most point: {:?}", p3);
    }

    {
        // 구조체에서의 수명
        #[derive(Debug)]
        struct Highlight<'doc>(&'doc str);

        // fn erase(text: String) {
        //     println!("Bye {text}!");
        // }

        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        // erase(text);
        println!("{fox:?}");
        println!("{dog:?}");
    }

    {
        struct Library {
            books: Vec<Book>,
        }

        struct Book {
            title: String,
            year: u16,
        }

        impl Book {
            // This is a constructor, used below.
            fn new(title: &str, year: u16) -> Book {
                Book {
                    title: String::from(title),
                    year,
                }
            }
        }

        // Implement the methods below. Update the `self` parameter to
        // indicate the method's required level of ownership over the object:
        //
        // - `&self` for shared read-only access,
        // - `&mut self` for unique and mutable access,
        // - `self` for unique access by value.
        impl Library {
            fn new(books: Vec<Book>) -> Library {
                Library { books }
            }

            fn len(&self) -> usize {
                self.books.len()
            }

            fn is_empty(&self) -> bool {
                if self.len() == 0 {
                    true
                } else {
                    false
                }
            }

            fn add_book(&mut self, book: Book) {
                self.books.push(book)
            }

            fn print_books(&self) {
                for book in self.books.iter() {
                    println!("Title: {}, Year: {}", book.title, book.year)
                }
            }

            fn oldest_book(&self) -> Option<&Book> {
                let mut oldest: Option<&Book> = None;
                for book in self.books.iter() {
                    if oldest.is_none() || book.year < oldest.unwrap().year {
                        oldest = Some(book);
                    }
                }
                oldest
            }
        }

        // This shows the desired behavior. Uncomment the code below and
        // implement the missing methods. You will need to update the
        // method signatures, including the "self" parameter! You may
        // also need to update the variable bindings within main.
        let mut library = Library::new(vec![]);

        println!(
            "The library is empty: library.is_empty() -> {}",
            library.is_empty()
        );

        library.add_book(Book::new("Lord of the Rings", 1954));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

        println!(
            "The library is no longer empty: library.is_empty() -> {}",
            library.is_empty()
        );

        library.print_books();

        match library.oldest_book() {
            Some(book) => println!("The oldest book is {}", book.title),
            None => println!("The library is empty!"),
        }

        println!("The library has {} books", library.len());
        library.print_books();
    }
}
