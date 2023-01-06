fn main() {
    println!("Hello, world!");
    variables();
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

    println!("{}", final_string);
    println!("array's size is {}. array's last value is {}", array.len(), array[array.len() -1]);
    println!("vec's size is {}. vec's last value is {}", vec.len(), vec[vec.len() -1]);
}
