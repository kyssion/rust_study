use crate::rust0004_enum_and_match::{test_match, Coin};
use std::convert::TryInto;
use std::any::Any;
use std::ops::Add;

mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;
mod rust0003_struct_1;
mod rust0003_struct_2;
mod rust0004_enum_and_match;
mod rust0005_pakage_module_other;
mod rust0006_collection;

fn main() {
    let mut string1 = String::from("A");
    let mut string2 = String::from("B");
    let mut string3 = String::from("C");
    let s = format!("{}-{}-{}",string1,string2,string3);

    print!("{}",s);
    print!("{}",string1);
    print!("{}",string2);
    print!("{}",string3);

}
