use crate::rust0004_enum_and_match::{test_match, Coin};
use std::convert::TryInto;
use std::any::Any;

mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;
mod rust0003_struct_1;
mod rust0003_struct_2;
mod rust0004_enum_and_match;
mod rust0005_pakage_module_other;
mod rust0006_collection;

fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8];
    for i in &mut v{

    }
    v.push(1) // 这里因为有作用域的问题，所以可以v.push
}
