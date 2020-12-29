use crate::rust0004_enum_and_match::{test_match, Coin};
use std::convert::TryInto;
use std::any::Any;

mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;
mod rust0003_struct_1;
mod rust0003_struct_2;
mod rust0004_enum_and_match;

fn main() {
    let item = Some(12);
    let item2 = Some(Coin::Dime);
    if let Some(i) = item {
        println!("{}",i)
    } else if let Some(ddd) = item2 {
        let p = ddd.type_id()==Coin::Dime.type_id();
        println!("{:?}",p)
    } else{

    }

}

pub fn test3_fn(s:&String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}