mod lib;

fn main() {
    println!("1st is {}", lib::fibonacci(1)); // 0
    println!("2nd is {}", lib::fibonacci(2)); // 1
    println!("3rd is {}", lib::fibonacci(3)); // 1
    println!("4th is {}", lib::fibonacci(4)); // 2
    println!("5th is {}", lib::fibonacci(5)); // 3
    println!("10th is {}", lib::fibonacci(10)); // 34
    println!("100th is {}", lib::fibonacci(100));
    println!("1000th is {}", lib::fibonacci(1000));
}