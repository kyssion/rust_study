
mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;

fn main() {
    let mut i = String::from("sdfsdf");
    let k = &i;
    let j = &i;
    println!("{},{}",k,j);
    let mut p = &mut i;
    println!("{}",p);// true

    i.push_str("ssss");
    //println!("{}",p);// error

    let j = &mut i;


}