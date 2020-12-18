
pub fn ownership_main(){
    // 所有权基本逻辑
    //1. rust中每一个值都有一个对应的变量作为所有者
    //2. 在同一个书剑内，值仅有一个所有者
    //3. 当所有者离开作用域的时候将会释放这个值

    let mut string  = String::from("string item");
    string.push_str(" push start");
    //todo 和 &str 相比，&str 不可变
    let mut str = "str item";


}