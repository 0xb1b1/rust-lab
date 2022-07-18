use std::io::{stdin,stdout,Write, Read};
//use std::convert::TryFrom;

fn main() {
    // User interaction
    let mut matrix_size_str: String = String::new();
    // Ask for matrix size
    print!("Input matrix size (`horiz vert`): ");
    let _ = stdout().flush();
    // Get user input
    stdin().read_line(&mut matrix_size_str).expect("Not a correct string");
    let mut matrix_size_str_vec: Vec<u8> = vec![];
    for dim in matrix_size_str.split_whitespace() {
        matrix_size_str_vec.push(dim.parse::<u8>().unwrap());
    }
    assert_eq!(2, matrix_size_str_vec.len());  // Check if we have ! 2 dims
    //println!("{:?}", matrix_size_str_vec);

    // Ask for matrix contents
    print!("Input matrix (sep: ` `, rowsep: `\\n`");
    let _ = stdout().flush();
    // Get user input
    stdin().read_line()
}
