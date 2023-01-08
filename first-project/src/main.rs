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
