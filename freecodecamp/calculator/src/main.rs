use std::env::{args, Args};
// if returns an option, we can unwrap if the value exists
fn main() {
    let mut args: Args = args();  // args needs to be muted for .nth() to work

    // Parse args
    let num0_str: String = args.nth(1).unwrap();
    let op: char = args.nth(0).unwrap().chars().next().unwrap();
    let num1_str: String = args.nth(0).unwrap();

    // Convert to float 32 bit
    let num0: f32 = num0_str.parse().unwrap();
    let num1: f32 = num1_str.parse().unwrap();
    let result: f32 = operation(op, num0, num1);

    println!("{}", output(num0, op, num1, result));
    /* https://doc.rust-lang.org/rust-by-example/hello/print.html
    Formatted print macros
    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as format! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.
    */

    // Precision (std::fmt) https://doc.rust-lang.org/std/fmt/
    // println!("{}, {num:.precision$}", "Hello", num=12.12112, precision=3)
}

fn operation(operator: char,
             num0: f32,
             num1: f32) -> f32{
    match operator {
        '+' => num0 + num1,
        '-' => num0 - num1,
        '*' | 'x' | 'X' => num0 * num1,  // Bitwise OR
        '/' => num0 / num1,
        _ => panic!("Invalid operator passed")
    }
}

fn output(num0: f32,
          operator: char,
          num1: f32,
          result: f32) -> String {
            /* Set precision to 3 decimal points in arg before next() arg */
            format!("{} {} {} = {:.*}", num0, operator, num1, 3, result)
}
