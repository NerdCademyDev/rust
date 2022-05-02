fn main() {
    
    // tuples
    let point = (3, 4, 5);
    println!("x {} y {} z {}", point.0, point.1, point.2);

    // cant do
    // let i = 0;
    // (point).i

    let rgba: (u8, u8, u8, f32) = (100, 255, 25, 0.5);
    let (red, blue, green, alpha) = rgba;
    println!("rgba: ({}, {}, {}, {})", red, blue, green, alpha);

    let data = (10, "hello", (true, 0.1));
    println!("{} {}", data.2.0, (data.2).1);

    // () unit type, returned from functions/expressions that dont
    // return anything

    // arrays:
    let months = ["Jan", "Feb", "Mar", "Apr"];
    let mut x: [i32; 5] = [10, 12, 4, 32, 99];
    let y = [0.00; 100];

    println!("months[1]: {}", months[1]);
    println!("y[99] {}", y[99]);
    println!("x {:?}", x);
    x.sort();
    println!("x {:?}", x);

    let mut evens = vec![2, 4, 6, 8];
    let mut odds = Vec::new();
    odds.push(1);
    odds.push(3);
    evens.push(10);
    println!("Odds: {:?} Evens: {:?}", odds, evens);

    let mut numbers: Vec<i32> = Vec::with_capacity(4);
    println!("numbers len: {}", numbers.len());
    println!("numbers cap: {}", numbers.capacity());

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);

    println!("numbers len: {}", numbers.len());
    println!("numbers cap: {}", numbers.capacity());

    numbers.push(5);

    println!("numbers len: {}", numbers.len());
    println!("numbers cap: {}", numbers.capacity());

    let last = numbers.pop();

    println!("last {:?}", last);

    println!("numbers len: {}", numbers.len());
    println!("numbers cap: {}", numbers.capacity());
    
}
