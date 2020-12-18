
mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;

fn main() {
    println!("this is main function");

    let str = String::from("this main func");
    let item = &str;
    println!("{}",str)
}