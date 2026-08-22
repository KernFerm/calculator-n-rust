use std::io::{self, ErrorKind, Read, Write};

const MAX_INPUT_LEN: usize = 1024;

fn read_input(prompt: &str) -> io::Result<Option<String>> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = Vec::new();
    let mut byte = [0_u8; 1];

    loop {
        let bytes_read = handle.read(&mut byte)?;

        if bytes_read == 0 {
            if input.is_empty() {
                return Ok(None);
            }

            return String::from_utf8(input)
                .map(Some)
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "input is not valid UTF-8"));
        }

        if byte[0] == b'\n' {
            break;
        }

        if byte[0] == b'\r' {
            continue;
        }

        if input.len() >= MAX_INPUT_LEN {
            while handle.read(&mut byte)? != 0 && byte[0] != b'\n' {}
            return Err(io::Error::new(ErrorKind::InvalidInput, "input is too long"));
        }

        input.push(byte[0]);
    }

    String::from_utf8(input)
        .map(Some)
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "input is not valid UTF-8"))
}

fn parse_number(input: &str) -> Result<f64, ()> {
    input
        .parse::<f64>()
        .map_err(|_| ())
        .and_then(|num| if num.is_finite() { Ok(num) } else { Err(()) })
}

fn main() {
    println!("Welcome to the Rust Command-Line Calculator!");
    println!("===========================================\n");

    loop {
        let num1_str = match read_input("Enter the first number (or 'q' to quit): ") {
            Ok(Some(input)) => input,
            Ok(None) => {
                println!("\nThank you for using the calculator! Created by Bubbles The Dev , Goodbye.");
                break;
            }
            Err(err) if err.kind() == ErrorKind::InvalidInput => {
                println!("Warning: input is too long.\n");
                continue;
            }
            Err(err) => {
                eprintln!("Input error: {}", err);
                break;
            }
        };

        if num1_str.trim().eq_ignore_ascii_case("q") {
            println!("\nThank you for using the calculator! Created by Bubbles The Dev , Goodbye.");
            break;
        }

        let num1: f64 = match parse_number(num1_str.trim()) {
            Ok(num) => num,
            Err(_) => {
                println!("Warning: please enter a valid finite number.\n");
                continue;
            }
        };

        println!("\nOperators:");
        println!("  + : Addition");
        println!("  - : Subtraction");
        println!("  * : Multiplication");
        println!("  / : Division");
        println!("  % : Modulus");
        println!("  ^ : Power");
        println!(" sqrt : Square Root");

        let operator = match read_input("\nEnter an operator: ") {
            Ok(Some(input)) => input,
            Ok(None) => {
                println!("\nThank you for using the calculator! Created by Bubbles The Dev , Goodbye.");
                break;
            }
            Err(err) if err.kind() == ErrorKind::InvalidInput => {
                println!("Warning: input is too long.\n");
                continue;
            }
            Err(err) => {
                eprintln!("Input error: {}", err);
                break;
            }
        };
        let operator = operator.trim();

        let num2 = if operator != "sqrt" {
            let num2_str = match read_input("Enter the second number: ") {
                Ok(Some(input)) => input,
                Ok(None) => {
                    println!("\nThank you for using the calculator! Created by Bubbles The Dev , Goodbye.");
                    break;
                }
                Err(err) if err.kind() == ErrorKind::InvalidInput => {
                    println!("Warning: input is too long.\n");
                    continue;
                }
                Err(err) => {
                    eprintln!("Input error: {}", err);
                    break;
                }
            };

            match parse_number(num2_str.trim()) {
                Ok(num) => num,
                Err(_) => {
                    println!("Warning: please enter a valid finite number.\n");
                    continue;
                }
            }
        } else {
            0.0
        };

        let result: f64 = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Warning: error: division by zero!\n");
                    continue;
                }
            }
            "%" => {
                if num2 != 0.0 {
                    num1 % num2
                } else {
                    println!("Warning: error: modulus by zero!\n");
                    continue;
                }
            }
            "^" => num1.powf(num2),
            "sqrt" => {
                if num1 >= 0.0 {
                    num1.sqrt()
                } else {
                    println!("Warning: error: square root of a negative number!\n");
                    continue;
                }
            }
            _ => {
                println!("Warning: invalid operator!\n");
                continue;
            }
        };

        println!("\n===============================");
        println!("  The result is: {:.2}", result);
        println!("===============================\n");
    }
}
