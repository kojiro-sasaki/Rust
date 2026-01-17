use std::io;

fn main() {
    println!("Print first number:");
    let mut number_one = String::new();
    io::stdin().read_line(&mut number_one).unwrap();
    let number_one: f64 = number_one.trim().parse().expect("Print number");

    println!("Print second number:");
    let mut number_two = String::new();
    io::stdin().read_line(&mut number_two).unwrap();
    let number_two: f64 = number_two.trim().parse().expect("Print number");

    println!("Print operation:");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();
    let operation: char = operation.trim().chars().next().expect("Print operation");

    let result: f64 = match operation {
        '+' => number_one + number_two,
        '-' => number_one - number_two,
        '*' => number_one * number_two,
        '/' => {
            if number_two == 0.0 {
                println!("Error!");
                return;
            }
            number_one / number_two
        }
        _ => {
            println!("Unknown operation");
            return;
        }
    };

    println!("{} {} {} = {}", number_one, operation, number_two, result);
}
