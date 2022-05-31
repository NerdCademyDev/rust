fn main() {

    let check = false;

    if check {
        println!("check is good!");
    } else {
        println!("check is not good!");
    }

//    We don't have 'truthy' values in Rust
//    if 1 {
//        println!("we have 1");
//    }

    let num = 10;

    if num >= 15 {
        println!("num is greater than or eq to 15");
    } else if num < 15 {
        println!("num is less than 15");
    } else {
        println!("?");
    }

    let mut number = if num != 15 { 0 } else { 100 };
    println!("number is {}", number);

    // error  must be same type!
    // let n = if num == 15 { 0 } else { "word up" };

    let value = 0;
    match value {
        0 => println!("value is 0"),
        1 => println!("value is 1"),
        _ => println!("uknown value {}", value)
    }

    // Loops:
    // loop expression
    // for, while always return () unit type

//    loop {
//        println!("looping....");
//    }

    let mut count = 10;
    loop {
        if count == 0 {
            println!("lift off!");
            break;
        } else {
            println!("{}", count);
            count -= 1;
        }
    }

    let answer = loop {
        count += 1;

        if count == 10 {
            break count;
        }
    };
    println!("answer: {}", answer);

    // reverse a number
    number = 145;
    let mut reverse = 0;
    println!("number: {}", number);
    while number != 0 {
        reverse = reverse * 10;
        reverse = reverse + number % 10;
        number = number / 10;
    }
    println!("reversed number {}", reverse);

    // for loops
    let numbers = [10, 100, 1000, 10000];
//    for i in 0..4 {
//        println!("{}", numbers[i]);
//    }
    for num in numbers {
        println!("{}", num);
    }






}
