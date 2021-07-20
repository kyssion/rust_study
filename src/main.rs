use crate::rust05_trait_lifetione_types::Summary;

mod rust00_base;
mod rust01_ownership;
mod rust02_struct_enum;
mod rust03_collection_string_map;
mod rust04_panic;
mod rust05_trait_lifetione_types;
mod rust06_functional_features;

fn main(){
    let mut string_item = String::from("this is test");
    let mut fn_once = ||{
        string_item.push_str("fffdsf");
        println!("{}",string_item);
    };
    fn_once();
    fn_once();
    println!()
}

fn test(str: String){
    println!("{}",str)
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
