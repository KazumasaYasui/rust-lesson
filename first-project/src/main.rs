struct Fruit {
    name: String,
}

impl Fruit {
    fn get_name(&self) -> &str {
        &self.name
    }
}

struct Rectangle(i32, i32);

impl Rectangle {
    fn calc_area(&self) -> i32 {
    self.0 * self.1
    }
}

struct Unit;

enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

impl Color {
    fn color_code(&self) -> String {
        match self {
            Color::Red => String::from("#ff0000"),
            Color::Green => String::from("#00ff00"),
            Color::Blue => String::from("#0000ff"),
            Color::Custom(r, g, b) => {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    r, g, b
                )
            }
        }
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

trait Greeter {
    fn greet(&self);
}

struct Person(String);

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, I am {}!", self.0);
    }
}

#[derive(Debug)]
struct Hours(u32);

fn main() {
    println!("Hello, world!");
    variables();
    conditional_branch();
    iteration();
    pattern_match();

    let added = add(10, 20);
    println!("{}", added);

    let z = 20;
    let add_z = |x: i32| x + z;
    println!("{}", add_z(10));

    let banana = Fruit {
        name: String::from("Banana"),
    };
    println!("this fruit name is {}", banana.get_name());

    let rect = Rectangle(10, 20);
    println!("this rectangle area is {}", rect.calc_area());

    let _unit = Unit;

    let color_b = Color::Blue;
    println!("blue's color code is {}", color_b.color_code());
    let color_g = Color::Green;
    println!("green's color code is {}", color_g.color_code());
    let color_r = Color::Red;
    println!("red's color code is {}", color_r.color_code());

    let color_custom = Color::Custom(10, 123, 255);
    println!("this custom color code is {}", color_custom.color_code());

    let some_value: Option<i32> = Some(10);
    let none_value: Option<i32> = None;
    match some_value {
        Some(v) => println!("some value = {}", v),
        None => println!("none value"),
    }
    if let None = none_value {
        println!("none value");
    }

    let ok_value: Result<i32, &'static str> = Ok(200);
    let err_value: Result<i32, &'static str> = Err("error message");
    match ok_value {
        Ok(v) => println!("ok value = {}", v),
        Err(e) => println!("err value = {}", e),
    }
    if let Err(e) = err_value {
        println!("err value = {}", e);
    }

    ownership();
    lifetime_parameter();

    let person = Person(String::from("Kazumasa"));
    person.greet();

    let h = Hours(5);
    println!("{:?}", h);
    println!("{}", h.0);
}

fn variables() {
    let x = 100;
    // x = 100;

    let mut y = 50;
    if x == y {
        println!("x is equal to y.")
    }
    y = 300;

    let str_slice: &str = "world";
    let _string: String = String::from(str_slice);
    let _string_format: String = format!("Hello, {}", str_slice);
    let final_string: String = format!("Variables!, {}, {}, {}", _string_format, x, y);

    let mut array: [i32; 3] = [1, 2, 3];
    array[0] = 10;
    // array.push(10);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec[0] = 10;
    vec.push(10);

    let t: (i32, &str) = (120, "文字列");
    let t0 = t.0;
    let t1 = t.1;

    println!("{}", final_string);
    println!("array's size is {}. array's last value is {}", array.len(), array[array.len() -1]);
    println!("vec's size is {}. vec's last value is {}", vec.len(), vec[vec.len() -1]);
    println!("tuple t's elements are {} and {}", t0, t1);
}

fn conditional_branch() {
    let x = 100;
    let y = 50;

    if x == y {
        println!("x and y are same value!")
    } else {
        println!("x and y are not same value!")
    }

    let z = if x != y { 500 } else { 300 };
    println!("z value is {}", z);
}

fn iteration() {
    for i in 0..10 {
        println!("in for-loop: {}", i);
    }

    let mut count = 0;
    while count < 10 {
        println!("current count is {}", count);
        count += 1;
    }

    loop {
        count -= 1;
        println!("current count is {}", count);
        if count == 0 {
            break;
        }
    }
}

fn pattern_match() {
    let i = 5;
    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=10 => println!("four to ten"),
        _ => println!("other"),
    }

    let is_zero_str = match i {
        0 => "zero",
        _ => "not zero",
    };

    println!("{}", is_zero_str);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn ownership() {
    // 所有権の移動
    {
        let x = String::from("hello");
        // ここで移動する
        let y = x;

        // 所有権がxからyに移動しているのでxは使えない
        // println!("{}", x);
        println!("y is {}", y);
    } // yはこの時点で解放される

    // 借用
    {
        let z = String::from("hello");
        {
            // ここで借用する
            let w = &z;

            // zはwが借用しているのでsに所有権を移動できない
            // let s = z;
            // println!("{}", s);

            println!("w is {}", w);
        } // wはこの時点で解放される

        println!("z is {}", z);
    } // zはこの時点で解放される
}

struct Greet<'a> {
    word: &'a str,
}

impl<'a> Greet<'a> {
    fn say(&self) {
        println!("{}", self.word);
    }
}

fn lifetime_parameter() {
    let hello: &str = "Hello!";
    {
        let greet = Greet { word: hello };
        greet.say();
    } // greetはここで解放される

    println!("{}", hello);
} // helloはここで解放される
