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

fn main() {
    let mut keys = vec!["key1","key2"];
    let mut values = vec!["value1","value1"];
    let my_hash_map :HashMap<_,_> = keys.iter().zip(values.iter()).collect();
    println!("{:?}",my_hash_map.get(&"key"))
}
