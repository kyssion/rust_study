use crate::rust0004_enum_and_match::{test_match, Coin};
use std::convert::TryInto;
use std::any::Any;
use std::ops::Add;
use std::collections::HashMap;
use crate::rust0008_generics::Summary;

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

trait SummaryItem {
    fn show(&self);
}
struct One{

}

struct Two{

}

impl SummaryItem for One{
    fn show(&self) {
        println!("fffffff")
    }
}

impl SummaryItem for Two{
    fn show(&self) {
        println!("fffffff")
    }
}

fn test_trait_main_1 <T: SummaryItem>(a:T,b:T){
    a.show();
    b.show();
}

fn test_trait_main_2(a : impl SummaryItem,b:impl SummaryItem){
    a.show();
    b.show();
}

fn main() {
    let str1 = String::from("a");
    let str2 = String::from("a");

    let vector = [1, 2];
    let a = vector[0];
}
