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


trait Summary2 {
    fn summarize(&self);
}
//todo trait 不支持 trait和对应的结构体都不在当前包下进行定义
// impl Display for String{
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<String,Error> {
//         return Ok(String::from("ffff"))
//     }
// }


//todo 使用trait 作为参数 -  使用+ 同时支持两种trait
//1. 简化版
fn test_trait_params(t : impl Summary+Summary2){

}
//2. 高级版 rust 使用范型来规定 trait 类型
fn test_trait_params_up <T : Summary+Summary2>(t: T,p:T){ //todo 注意这里，这里使用了范型，那么，rust会强制要求t.p 大类型相同，而不是都实现了trait即可

}

//3. 将约束使用where包装简化操作

fn test_trait_params_end <T,U> (t:T,u:U) ->String where T:Summary+Summary2,U:Summary+Summary2{
    return String::from("ffff")
}

//todo rust 的返回值限制 - 只能返回相同类型的trait 实现类

fn test_trait_reture()->impl Summary{
    return String::from("f")
}


//rust 牛逼功能，有条件的实现trait

struct Pair<T>{
    x:T,
    y:T
}
impl <T> Pair<T>{
    fn new(x:T,y:T)->Self{
        return Self{
            x,
            y,
        }
    }
}
//只有当 T 同时满足了 display + paritalOrd 约束的时候 才能 满足这个 cmp_display
impl <T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x>self.y{
            println!("xxxx = {}",self.x)
        }else{
            println!("yyyy = {}",self.y)
        }
    }
}

//todo rust lift style - rust 生命周期

// rust 通过给变量增加作用域来解决的悬空引用的问题的
// todo 这段代码是无法通过编译的因为 x 和 y 这两个参数作为了 str的返回值 -> str 的无法在一开始就确定生命周是使用 x的还是使用y的 所以报错
// todo 原则 - 编译期决定生命周期的规则 ， 默认情况下，返回值的生命周期必须为参数中的一个- 下面例子的str产生了二义性
// fn lift_test(x:&str,y:&str)-> &str{
//     if x.len()>y.len(){
//         x
//     }else{
//         y
//     }
// }

//这样就能进行修正了 str 返回值的生命周期就是  传入参数 x 和 y 的最小值
fn  lift_test<'a>(x:&'a str,y :&'a str)-> &'a str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}

//todo 结构体生命周期标注 - 结构体在rust相当于一个数据的集合， 结构体中的生命周期用来描述 当前的结构体的生命周器和那个变量的生命周期相同
//todo ?? 标注了一个表示当前生命周期最大是一个变量的生命周期，如果两个呢
struct ImportantExcerpt<'a,'b>{
    part: &'a str,
    part2: &'b str
}
//todo 全局变量， 静态生命周期 'static