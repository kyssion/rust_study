use crate::rust05_trait_lifetione_types::Summary;

mod rust00_base;
mod rust01_ownership;
mod rust02_struct_enum;
mod rust03_collection_string_map;
mod rust04_panic;
mod rust05_trait_lifetione_types;

fn main(){
    let ppp = P{};
    ppp.summarize();
    let a = String::from("fff");
    println!("{}",a.summarize());
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
