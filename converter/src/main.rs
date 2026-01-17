enum Operation{
    Temperature,
    Weight,
    Distance,
}
fn main(){
    println!("Choose operation: Temperature / Weight / Distance");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input=input.trim();
    let operation : Operation = match input{
        "Temperature" => Operation::Temperature,
        "Weight" => Operation::Weight,
        "Distance" => Operation::Distance,
        _=>{
            println!("Unrecognized operation");
            return;
        }
    };

    match operation{
        Operation::Temperature=> {
            println!("1 - Celsius to Fahrenheit");
            println!("2 - Fahrenheit to Celsius");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice: u32 = choice.trim().parse().unwrap();

            println!("Enter temperature value:");

            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number: f64 = number.trim().parse().unwrap();

            match choice {
                1 => {
                    let result = number * 9.0 / 5.0 + 32.0;
                    println!("The temperature in Fahrenheit is {}", result);
                }
                2 => {
                    let result = (number - 32.0) * 5.0 / 9.0;
                    println!("The temperature in Celsius is {:.2}", result);
                }
                _ => {
                    println!("Unrecognized operation");
                }
            }
        }
        Operation::Weight=> {
            println!("1 - Kg to Lb");
            println!("2 - Lb to Kg");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice: u32 = choice.trim().parse().unwrap();

            println!("Enter a weight value:");

            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number: f64 = number.trim().parse().unwrap();

            match choice{
                1=>{
                    let _result = number * 2.205;
                    println!("The weight in Lb is {}", _result);
                }
                2=>{
                    let result = number * 0.45359237;
                    println!("The weight in Kg is {}", result);
                }
                _=>{
                    println!("Unrecognized operation");
                }
            }
        }
        Operation::Distance=> {
            println!("1 - Km to M");
            println!("2 - M to Km");

            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice :u32 = choice.trim().parse().unwrap();

            println!("Enter a distance value:");
            let mut number = String::new();
            std::io::stdin().read_line(&mut number).unwrap();
            let number: f64 = number.trim().parse().unwrap();

            match choice{
                1=>{
                    let result = number * 0.621371;
                    println!("The distance from km to m is: {}", result);
                }
                2=>{
                    let result = number * 1.60934;
                    println!("The distance from m to km is: {}", result);
                }
                _=>{
                    println!("Unrecognized operation");
                }
            }

        }
    }
}