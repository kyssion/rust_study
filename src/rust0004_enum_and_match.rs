use crate::rust0004_enum_and_match::TestEnum::{NO_MESSAGE, INT_MESSAGE};
use std::borrow::Borrow;
use std::convert::TryInto;
use std::any::Any;

//定义一个枚举类型
//注意枚举类型的所有的变量都位于枚举名称的标识空间中
//这也就是说，任何一个枚举值都可以使用的枚举名称+:: 使用
enum IpAddrKind{
    V4,
    V6
}

struct TestStruct{
    itemStr:String
}

//todo 枚举值在其中的语义 - 在rust中的枚举的语义不单单是简单的一种类型定义
//todo 而且还是一种通用数据类型的封装

enum TestEnum{
    NO_MESSAGE(),
    INT_MESSAGE(i32,i32),
    STR_MESSAGE(String,String,String),
    FLOAT_MESSAGE(f32,f64),
    STRUCT_MESSAGE(TestStruct), //todo 结构体元组
    DEFAULT_MESSAGE{item_int:i32} //直接定义对象
}

//todo 枚举的一种使用技巧,通过枚举封装不同类型的值
fn test2(test :TestEnum){
    if test.type_id()== NO_MESSAGE.type_id(){

    }else if test.type_id() == INT_MESSAGE.type_id(){

    }
}


fn test1(){
    //使用枚举类型
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //todo rust中广泛运用的 枚举类型 option
    let mut one = Some(12);
    let tow = Some("sdfsdf");
    let sss = one.expect("ddd");

}

//todo 枚举在rust中其实是一种类型，可以直接在变种中使用
fn route(ip_type: IpAddrKind){

}


//todo rust 流程控制运算 match

//todo rust中match 和switch 不一样 rust 中 match是一种模式匹配
pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(i32,f64)
}
pub fn test_match (item : Coin) -> i32{
    //match 模式有很多东西,后面说
    //todo match会计算每一个分支,将匹配的执行
    match item {
        Penny => { //支持括号模式
            println!("this is Penny");
            23
        },
        Nickel=>12,
        Dime=>33,
        Coin::Quarter(one,tow)=>{
            println!("{},{}",one,tow);//todo 枚举展开, 这里rust的match将会展开Quarter中的枚举并且将数据加入到后面的逻辑里
            222
        },
        _ =>{ //todo　使用通配符_ 可以匹配其他未知情况

        }
    }
}