fn main() {
    //prompt the user to input the first number, the operation to be performed, and the second number
    println!("Enter the first number: ");
    let mut first_number = String::new();
    std::io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    let mut second_number = String::new();
    std::io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    let add = Operation::Add(first_number, second_number);
    let subtract = Operation::Subtract(first_number, second_number);
    let multiply = Operation::Multiply(first_number, second_number);
    let divide = Operation::Divide(first_number, second_number);

    println!("Add: {}", calculate(add));
    println!("Subtract: {}", calculate(subtract));
    println!("Multiply: {}", calculate(multiply));
    println!("Divide: {}", calculate(divide));
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}