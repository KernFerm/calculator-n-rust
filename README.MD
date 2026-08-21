# Rust Command-Line Calculator

Welcome to the Rust Command-Line Calculator! This is a simple command-line application written in Rust that allows you to perform basic arithmetic operations.


## Features

- Addition
- Subtraction
- Multiplication
- Division
- Percentage
- SquareRoot
- Modulus
- Power

## Screenshot

![Screenshot 1 of Rust Calculator](https://github.com/KernFerm/calculator-n-rust/blob/main/screenshots/Screenshot-1.png)
![Screenshot 2 of Rust Calculator](https://github.com/KernFerm/calculator-n-rust/blob/main/screenshots/Screenshot-2.png)
![Screenshot 3 of Rust Calculator](https://github.com/KernFerm/calculator-n-rust/blob/main/screenshots/Screenshot-3.png)
![Screenshot 4 of Rust Calculator](https://github.com/KernFerm/calculator-n-rust/blob/main/screenshots/Screenshot-4.png)


## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Make sure you have Rust installed on your machine)
- **`WINDOWS OS`**

## How to Use

1. **Download the project files**:
    - Download the project files from the provided source (e.g., a ZIP file).
    - Extract the files to a directory of your choice.

2. **Navigate to the project directory**:
    ```sh
    cd <project-directory>
    ```

3. **Build the project**:
    ```sh
    cargo build
    ```

4. **Run the project**:
    ```sh
    cargo run
    ```

5. **Using the Calculator**:
    - When you run the program, you will be greeted with a welcome message.
    - You will be prompted to enter the first number. Type the number and press Enter.
    - Next, you will be prompted to enter an operator. Choose from the following:
        - `+` for addition
        - `-` for subtraction
        - `*` for multiplication
        - `/` for division
    - Finally, you will be prompted to enter the second number. Type the number and press Enter.
    - The result of the operation will be displayed.
    - You can continue performing calculations or type `q` to quit the program.

## Example

Welcome to the Rust Command-Line Calculator!
- Enter the first number (or 'q' to quit): 10 Operators:
- Addition
- Subtraction
- Multiplication / Division Enter the operator: + Enter the second number: 5 Result: 10 + 5 = 15
- Enter the first number (or 'q' to quit): q

`Thank you for using the calculator! Created by Bubbles The Dev, Goodbye.`


## Building on a 64-bit System

This project is built and tested on a 64-bit system. Ensure that your Rust installation is configured for 64-bit architecture. You can verify this by running:

```sh
rustup target list --installed
```

- You should see `x86_64-unknown-linux-gnu` (or a similar 64-bit target) in the list of installed targets.
- **`WINDOWS OS`**

## License

- This project is licensed under the [MIT License](https://github.com/KernFerm/calculator-n-rust/blob/main/LICENSE). See the LICENSE file for details

## Acknowledgements

- Created by [Bubbles The Dev](https://www.github.com/kernferm)

------

-----
```rust
use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust Command-Line Calculator!");
    println!("===========================================\n");

    loop {
        // Prompt for the first number
        print!("Enter the first number (or 'q' to quit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
        let mut num1_str = String::new();
        io::stdin().read_line(&mut num1_str).expect("Failed to read line");
        if num1_str.trim().to_lowercase() == "q" {
            println!("\nThank you for using the calculator! Created by Bubbles The Dev, Goodbye.");
            break;
        }
        let num1: f64 = match num1_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Please enter a valid number.\n");
                continue;
            }
        };

        // Prompt for the operator
        println!("\nOperators:");
        println!("  + : Addition");
        println!("  - : Subtraction");
        println!("  * : Multiplication");
        println!("  / : Division");
        println!("  % : Modulus");
        println!("  ^ : Power");
        println!(" sqrt : Square Root");
        print!("\nEnter an operator: ");
        io::stdout().flush().unwrap();
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        // Prompt for the second number if not using sqrt
        let num2: f64;
        if operator != "sqrt" {
            print!("Enter the second number: ");
            io::stdout().flush().unwrap();
            let mut num2_str = String::new();
            io::stdin().read_line(&mut num2_str).expect("Failed to read line");
            num2 = match num2_str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("⚠️  Please enter a valid number.\n");
                    continue;
                }
            };
        } else {
            num2 = 0.0; // Placeholder for sqrt operation
        }

        // Perform the calculation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("⚠️  Division by zero is not allowed.\n");
                    continue;
                }
                num1 / num2
            }
            "%" => num1 % num2,
            "^" => num1.powf(num2),
            "sqrt" => num1.sqrt(),
            _ => {
                println!("⚠️  Invalid operator.\n");
                continue;
            }
        };

        // Display the result
        if operator == "sqrt" {
            println!("\nResult: sqrt({}) = {}\n", num1, result);
        } else {
            println!("\nResult: {} {} {} = {}\n", num1, operator, num2, result);
        }
    }
}
```

