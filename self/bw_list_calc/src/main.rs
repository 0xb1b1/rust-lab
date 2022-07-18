use std::env::{args, Args};
use std::convert::TryFrom;

fn main() {
    let mut args: Args = args();

    // Parse args
    let num0: u32 = args.nth(1).unwrap().parse::<u32>().unwrap();
    let num1: u32 = args.nth(0).unwrap().parse::<u32>().unwrap();

    // Convert parsed args to vec
    let num0vec: Vec<u8> = num_to_u8_vec(&num0);
    let num1vec: Vec<u8> = num_to_u8_vec(&num1);

    // Convert bwls to u32
    let num0_conv: u32 = bw_vec_to_u32(&num0vec);
    let num1_conv: u32 = bw_vec_to_u32(&num1vec);
    println!("Result: {}", num0_conv + num1_conv);
}

fn num_to_u8_vec(num: &u32) -> Vec<u8> {
    let mut ret_vec: Vec<u8> = vec![];
    let num_string: String = num.to_owned().to_string();
    let num_charvec: Vec<char> = num_string.chars().collect();
    for char in num_charvec {
        let ch_i32: i32 = char.to_string().parse().unwrap();
        ret_vec.push(u8::try_from(ch_i32).ok().unwrap());
    }
    ret_vec
}

fn bw_vec_to_u32(bwl: &Vec<u8>) -> u32 {
    let mut ret_string: String = "".to_string();
    for num in (0..bwl.len()).rev() {
        let num_string: String = bwl[num].to_owned().to_string();
        let num_vec: Vec<char> = num_string.chars().collect();
        ret_string.push(num_vec[0]);
    }
    let ret_num: u32 = ret_string.parse().unwrap();
    ret_num
}
