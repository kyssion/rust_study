use crate::rust05_trait_lifetione_types::Summary;

mod rust00_base;
mod rust01_ownership;
mod rust02_struct_enum;
mod rust03_collection_string_map;
mod rust04_panic;
mod rust05_trait_lifetione_types;
mod rust06_functional_features;

fn main(){
    let x = 4;
    let p = |z| {x==z};
    // fn qual_to_x(z :i32)->bool{ z==x }
    println!("{}",p(123))
}

struct P{}

impl Summary for P{
    fn summarize(&self) -> String {
        String::from("zzz")
    }

    fn prn() -> String {
        return String::from("P")
    }
}
