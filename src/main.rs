mod ch01;
use crate::ch01::{integer, floating};
mod ch02;
use crate::ch02::{slice, palindrome};

fn main() {
    let str = String::from("lol me");
    println!("Hello, world!");
    println!("{}", integer::add_i(1,2));
    println!("{}", integer::subtract_i(1,2));
    println!("{}", integer::multiply_i(1,2));
    println!("{}", integer::divide_i(1,2));
    println!("{}", floating::add_fl(1.2,2.2));
    println!("{}", slice::first_word(&str));
    println!("{}", palindrome::check_palindrome("madam"));
}
