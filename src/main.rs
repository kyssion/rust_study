
mod rust0001_var_and_control_flow;
mod rust0000_official_example;
mod rust0002_ownership;

fn main() {
    let mut string = String::from("test item");
    let word = test3_fn(&string);
    string.clear();
    println!(word);
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