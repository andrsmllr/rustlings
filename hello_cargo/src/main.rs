fn main() {
    println!("Hello, world!");

    let mut i = 0;
    let mut x = 1;
    let y = if loop {
        x += x;
        i += 1;
        if i == 3 { break x }
    } == 8 { 1 } else { 0 };

    println!("y = {y}");
}
