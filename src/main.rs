mod Ch01;
use crate::Ch01::integer;
use crate::Ch01::floating;

fn main() {
    println!("Hello, world!");
    println!("{}", integer::add_i(1,2));
    println!("{}", integer::subtract_i(1,2));
    println!("{}", integer::multiply_i(1,2));
    println!("{}", integer::divide_i(1,2));
    println!("{}", floating::add_fl(1.2,2.2));
}
