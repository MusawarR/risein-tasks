use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(calc_operation: Operation) -> f64 {
    match calc_operation {
        Operation::Add(num1, num2) => num1 + num2,
        Operation::Subtract(num1, num2) => num1 - num2,
        Operation::Multiply(num1, num2) => num1 * num2,
        Operation::Divide(num1, num2) => num1 / num2
    }
}

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    println!("FIRST NUM: ");
    io::stdin().read_line(&mut num1).expect("Please provide the first number");

    println!("SECOND NUM: ");
    io::stdin().read_line(&mut num2).expect("Please provide the second number");

    println!("OPERATION: ");
    io::stdin().read_line(&mut op).expect("Expected an operation");

    let parsed_num1: f64 = num1.trim().parse().expect("Expected a number");
    let parsed_num2: f64 = num2.trim().parse().expect("Expected a number");
    let parsed_op = op.trim();

    let op = match parsed_op {
        "add_nums" => Operation::Add(parsed_num1, parsed_num2),
        "subtract_nums" => Operation::Subtract(parsed_num1, parsed_num2),
        "multiply_nums" => Operation::Multiply(parsed_num1, parsed_num2),
        "divide_nums" => Operation::Divide(parsed_num1, parsed_num2),
        _ => {
            println!("Unknown operation!");
            return;
        }
    };

    let calculation = calculate(op);

    println!("{}", calculation);
}