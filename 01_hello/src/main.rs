/**
 * main function
 */
fn main() {
    println!("Hello, world!");

    // comments
    /*
        multiline
        comments yay
    */

    // signed i8, i16, i32, i64, i128
    // unsigned u8, u16, u32, u64, u128
    let a = 42;
    let x: u64 = 64;
    let mut b: i64 = -64;
    println!("b {}", b);
    b = 64;
    println!("b {}", b);
    println!("a {} x {}", a, x);

    // floats : f32 -- f64
    let pi: f64 = 3.145159265359;
    let radius = 4.0;
    let area = 2.0 * pi * (radius * radius);
    println!("area is {}", area);

    // boolean true and false
    let to_b: bool = false;
    if to_b {
        println!("true!");
    } else if !to_b {
        println!("false");
    }

    // char
    let c: char = 'c';
    println!("c is {}", c);
    let star_face = '🤩';
    println!("star face: {}", star_face);

    // strings
    // 1st string slice
    let hello = "Hello";

    // 2nd String
    // "world".to_string()
    let world: String = String::from("World");
    println!("{}, {}!", hello, world);

}
