fn main() {
    println!("Hello, world!");
    variables();
    conditional_branch();
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
