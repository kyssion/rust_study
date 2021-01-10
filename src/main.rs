use crate::rust0004_enum_and_match::{test_match, Coin};
use std::convert::TryInto;
use std::any::Any;
use std::ops::Add;
use std::collections::HashMap;

mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;
mod rust0003_struct_1;
mod rust0003_struct_2;
mod rust0004_enum_and_match;
mod rust0005_pakage_module_other;
mod rust0006_collection;
mod rust0007_error;
mod rust0008_generics;

fn main() {
    let mut my_hash_map = HashMap::new();
    my_hash_map.insert("key","value");
    let mut valueOption = my_hash_map.get("key");
    let mut value = *valueOption.unwrap();
    println!("{:?}",value)
}
