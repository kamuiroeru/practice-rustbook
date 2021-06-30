struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old.", self.age);
        self
    }
}

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 },
}

fn main() {
    // str と String の関係
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", s3);

    // タプルの話
    let mut t = (1, "2");
    t.0 = 3;
    t.1 = "123";
    println!("{}", t.0);
    println!("{}", t.1);

    // 配列の話
    let a: [i32; 3] = [0, 1, 2];
    let b: [i32; 6] = [2; 6];
    println!("{:?}", &a[..]);
    println!("{:?}", &b[..]);

    let p = Person{
        name: String::from("John"),
        age: 14,
    };

    p.say_name().say_age();
    // p.say_age();

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x:10, y: 10};

    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 2);
    debug_assert!(false);
    debug_assert_eq!(1, 0);
    debug_assert_ne!(1, 1);
}
