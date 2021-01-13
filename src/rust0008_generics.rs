// 范型

//结构体范型

use std::fmt;
use std::fmt::{Display, Formatter};

//在结构体中的相同的类型需要保证同样的使用时候的类型，比如都是T ，那么在真实使用的时候都必须是 相同的类型
//todo rust 的范型就是符号相同
struct Point<T,U>{
    x: T,
    y: U
}

impl <T,U> Point<T,U> {
    fn test(&mut self) ->&T{
        return &(self.x);
    }
}

//在枚举中的范型

enum TestEnum<T,U>{
    One(T),Two(U)
}


//trait 共享行为 -  和接口相同，但是又何接口不同

pub trait Summary {
    fn summarize(&self) -> String;
    //todo 制定一个默认的实现方法
    fn summarize2(&self) -> String{
        println!("heheh");
        return String::from("ttt");
    }

}

//类型实现trait

pub struct TestSummary{

}

impl Summary for TestSummary{
    fn summarize(&self) -> String {
        unimplemented!()
    }
}

fn test_generics(){

}

// 用 display 扩展自己定义的结构体
impl Display for TestSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self)
    }
}

//用自己的定义的trait 扩展String
impl Summary for String{
    fn summarize(&self) -> String {
        return String::from("ffff")
    }

}
//todo trait 不支持 trait和对应的结构体都不在当前包下进行定义
// impl Display for String{
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<String,Error> {
//         return Ok(String::from("ffff"))
//     }
// }


//todo 使用trait 作为参数
//1. 简化版
fn test_trait_params(t : impl Summary){

}
//2. 高级版 rust 使用范型来规定 trait 类型
fn test_trait_params_up <T : Summary>(t: T,p:T){ //todo 注意这里，这里使用了范型，那么，rust会强制要求t.p 大类型相同，而不是都实现了trait即可

}

